use failure::{err_msg, Error};
use serde::{de, Deserialize, Deserializer};
use std::{
  collections::{BTreeMap, HashSet},
  fs::File,
  io::Write,
  ops::Range,
};

const BIT_BAND: Range<u32> = 0x4000_0000..0x4010_0000;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct Device {
  name: String,
  peripherals: Peripherals,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Default, Deserialize)]
struct Peripherals {
  #[serde(deserialize_with = "deserialize_peripheral", default)]
  peripheral: BTreeMap<String, Peripheral>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct Peripheral {
  pub derived_from: Option<String>,
  pub name: String,
  pub description: Option<String>,
  #[serde(deserialize_with = "deserialize_hex")]
  pub base_address: u32,
  #[serde(default)]
  pub interrupt: Vec<Interrupt>,
  registers: Option<Registers>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct Interrupt {
  pub name: String,
  pub description: String,
  #[serde(deserialize_with = "deserialize_dec")]
  pub value: u32,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
struct Registers {
  register: Vec<Register>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct Register {
  pub name: String,
  pub description: String,
  #[serde(deserialize_with = "deserialize_hex")]
  pub address_offset: u32,
  #[serde(deserialize_with = "deserialize_hex")]
  pub size: u32,
  pub access: Option<Access>,
  #[serde(deserialize_with = "deserialize_hex")]
  pub reset_value: u32,
  fields: Option<Fields>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
struct Fields {
  field: Vec<Field>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct Field {
  pub name: String,
  pub description: String,
  pub bit_offset: usize,
  pub bit_width: usize,
  pub access: Option<Access>,
}

#[serde(rename_all = "kebab-case")]
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Access {
  WriteOnly,
  ReadOnly,
  ReadWrite,
}

impl Device {
  pub fn new(name: String) -> Self {
    Self {
      name,
      peripherals: Peripherals::default(),
    }
  }

  pub fn peripheral_mut(&mut self, name: &str) -> Option<&mut Peripheral> {
    self.peripherals.peripheral.get_mut(name)
  }

  pub fn set_peripheral(&mut self, peripheral: Peripheral) {
    self
      .peripherals
      .peripheral
      .insert(peripheral.name.clone(), peripheral);
  }

  pub fn generate_regs(
    self,
    regs: &mut File,
    except: &[&str],
    pool_number: usize,
    pool_size: usize,
  ) -> Result<(), Error> {
    let mut counter = 0;
    for peripheral in self.peripherals.peripheral.values() {
      if except.iter().any(|&name| name == peripheral.name) {
        continue;
      }
      peripheral.generate_regs(
        &self.peripherals,
        regs,
        pool_number,
        pool_size,
        &mut counter,
      )?;
    }
    Ok(())
  }

  pub fn generate_rest(
    self,
    reg_index: &mut File,
    interrupts: &mut File,
    except: &[&str],
  ) -> Result<(), Error> {
    let mut int_names = HashSet::new();
    writeln!(reg_index, "reg::unsafe_tokens! {{")?;
    writeln!(
      reg_index,
      "  /// Defines an index of {} register tokens.",
      self.name
    )?;
    writeln!(reg_index, "  ///")?;
    writeln!(reg_index, "  /// # Safety")?;
    writeln!(reg_index, "  ///")?;
    writeln!(
      reg_index,
      "  /// See [`::drone_core::reg::unsafe_tokens!`]."
    )?;
    writeln!(reg_index, "  pub macro unsafe_stm32_reg_tokens;")?;
    writeln!(
      reg_index,
      "  use macro ::drone_cortex_m::unsafe_cortex_m_reg_tokens;"
    )?;
    writeln!(reg_index, "  super::inner; reg;")?;
    for peripheral in self.peripherals.peripheral.values() {
      peripheral.generate_rest(
        &self.peripherals,
        &mut int_names,
        reg_index,
        interrupts,
        except,
      )?;
    }
    writeln!(reg_index, "}}")?;
    Ok(())
  }
}

impl Peripheral {
  pub fn registers_mut(&mut self) -> impl Iterator<Item = &mut Register> + '_ {
    self
      .registers
      .iter_mut()
      .flat_map(|registers| &mut registers.register)
  }

  fn generate_regs(
    &self,
    peripherals: &Peripherals,
    regs: &mut File,
    pool_number: usize,
    pool_size: usize,
    counter: &mut usize,
  ) -> Result<(), Error> {
    let &Self {
      ref derived_from,
      ref name,
      base_address,
      ref registers,
      ..
    } = self;
    let parent = if let Some(derived_from) = derived_from {
      Some(
        peripherals
          .peripheral
          .get(derived_from)
          .ok_or_else(|| err_msg("Peripheral `derivedFrom` not found"))?,
      )
    } else {
      None
    };
    registers
      .as_ref()
      .or_else(|| parent.and_then(|x| x.registers.as_ref()))
      .ok_or_else(|| err_msg("Peripheral registers not found"))?
      .generate_regs(
        name,
        base_address,
        regs,
        pool_number,
        pool_size,
        counter,
      )?;
    Ok(())
  }

  fn generate_rest(
    &self,
    peripherals: &Peripherals,
    int_names: &mut HashSet<String>,
    reg_index: &mut File,
    interrupts: &mut File,
    except: &[&str],
  ) -> Result<(), Error> {
    let &Self {
      ref derived_from,
      ref name,
      ref description,
      ref interrupt,
      ref registers,
      ..
    } = self;
    let parent = if let Some(derived_from) = derived_from {
      Some(
        peripherals
          .peripheral
          .get(derived_from)
          .ok_or_else(|| err_msg("Peripheral `derivedFrom` not found"))?,
      )
    } else {
      None
    };
    if except.iter().all(|&except| except != name) {
      let description = description
        .as_ref()
        .or_else(|| parent.and_then(|x| x.description.as_ref()))
        .ok_or_else(|| err_msg("Peripheral description not found"))?;
      for line in description.lines() {
        writeln!(reg_index, "  /// {}", line.trim())?;
      }
      writeln!(reg_index, "  pub mod {} {{", name)?;
      registers
        .as_ref()
        .or_else(|| parent.and_then(|x| x.registers.as_ref()))
        .ok_or_else(|| err_msg("Peripheral registers not found"))?
        .generate_reg_index(reg_index)?;
      writeln!(reg_index, "  }}")?;
    }
    interrupt.generate(int_names, interrupts)?;
    Ok(())
  }
}

trait Interrupts {
  fn generate(
    &self,
    int_names: &mut HashSet<String>,
    interrupts: &mut File,
  ) -> Result<(), Error>;
}

impl Interrupts for Vec<Interrupt> {
  fn generate(
    &self,
    int_names: &mut HashSet<String>,
    interrupts: &mut File,
  ) -> Result<(), Error> {
    for interrupt in self {
      if int_names.insert(interrupt.name.to_owned()) {
        let &Interrupt {
          ref name,
          ref description,
          value,
        } = interrupt;
        writeln!(interrupts, "int! {{")?;
        for line in description.lines() {
          writeln!(interrupts, "  /// {}", line.trim())?;
        }
        writeln!(interrupts, "  pub trait {}: {};", name, value)?;
        writeln!(interrupts, "}}")?;
      }
    }
    Ok(())
  }
}

impl Register {
  pub fn fields_mut(&mut self) -> impl Iterator<Item = &mut Field> + '_ {
    self.fields.iter_mut().flat_map(|fields| &mut fields.field)
  }
}

impl Registers {
  fn generate_regs(
    &self,
    peripheral_name: &str,
    base_address: u32,
    regs: &mut File,
    pool_number: usize,
    pool_size: usize,
    counter: &mut usize,
  ) -> Result<(), Error> {
    for register in &self.register {
      *counter += 1;
      if *counter % pool_size != pool_number - 1 {
        continue;
      }
      let &Register {
        ref name,
        ref description,
        address_offset,
        size,
        access,
        reset_value,
        ref fields,
      } = register;
      let address = base_address + address_offset;
      writeln!(regs, "reg! {{")?;
      for line in description.lines() {
        writeln!(regs, "  /// {}", line.trim())?;
      }
      writeln!(regs, "  pub mod {} {};", peripheral_name, name)?;
      writeln!(
        regs,
        "  0x{:04X}_{:04X} {} 0x{:04X}_{:04X}",
        address >> 16,
        address & 0xFFFF,
        size,
        reset_value >> 16,
        reset_value & 0xFFFF,
      )?;
      write!(regs, " ")?;
      match access {
        Some(Access::WriteOnly) => {
          write!(regs, " WReg")?;
          write!(regs, " WoReg")?;
        }
        Some(Access::ReadOnly) => {
          write!(regs, " RReg")?;
          write!(regs, " RoReg")?;
        }
        Some(Access::ReadWrite) | None => {
          write!(regs, " RReg")?;
          write!(regs, " WReg")?;
        }
      }
      if BIT_BAND.contains(&address) {
        write!(regs, " RegBitBand")?;
      }
      writeln!(regs, ";")?;
      if let Some(fields) = fields {
        fields.generate(access, regs)?;
      }
      writeln!(regs, "}}")?;
    }
    Ok(())
  }

  fn generate_reg_index(&self, reg_index: &mut File) -> Result<(), Error> {
    for register in &self.register {
      let Register { name, .. } = register;
      writeln!(reg_index, "    {};", name)?;
    }
    Ok(())
  }
}

impl Fields {
  fn generate(
    &self,
    base_access: Option<Access>,
    regs: &mut File,
  ) -> Result<(), Error> {
    for field in &self.field {
      let &Field {
        ref name,
        ref description,
        bit_offset,
        bit_width,
        access,
      } = field;
      for line in description.lines() {
        writeln!(regs, "  /// {}", line.trim())?;
      }
      write!(regs, "  {} {{ {} {}", name, bit_offset, bit_width)?;
      match access.as_ref().or_else(|| base_access.as_ref()) {
        Some(&Access::WriteOnly) => {
          write!(regs, " WWRegField")?;
          write!(regs, " WoWRegField")?;
        }
        Some(&Access::ReadOnly) => {
          write!(regs, " RRRegField")?;
          write!(regs, " RoRRegField")?;
        }
        Some(&Access::ReadWrite) | None => {
          write!(regs, " RRRegField")?;
          write!(regs, " WWRegField")?;
        }
      }
      writeln!(regs, " }}")?;
    }
    Ok(())
  }
}

fn deserialize_peripheral<'de, D>(
  deserializer: D,
) -> Result<BTreeMap<String, Peripheral>, D::Error>
where
  D: Deserializer<'de>,
{
  let mut map = BTreeMap::new();
  for peripheral in Vec::<Peripheral>::deserialize(deserializer)? {
    map.insert(peripheral.name.clone(), peripheral);
  }
  Ok(map)
}

fn deserialize_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  let s = s.trim_start_matches("0x").trim_start_matches("0X");
  u32::from_str_radix(s, 16).map_err(de::Error::custom)
}

fn deserialize_dec<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  u32::from_str_radix(&s, 10).map_err(de::Error::custom)
}

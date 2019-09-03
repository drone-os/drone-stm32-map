use failure::{err_msg, Error};
use serde::{de, Deserialize, Deserializer};
use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::Write,
    ops::{Generator, GeneratorState, Range},
    pin::Pin,
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
#[derive(Debug, Default, Deserialize)]
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
#[derive(Debug, Default, Deserialize)]
struct Registers {
    register: Vec<Register>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Default, Deserialize)]
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
#[derive(Debug, Clone, Default, Deserialize)]
struct Fields {
    field: Vec<Field>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Field {
    pub name: String,
    pub description: String,
    pub bit_offset: usize,
    pub bit_width: usize,
    pub access: Option<Access>,
}

#[serde(rename_all = "kebab-case")]
#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
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

    pub fn peripheral(&self, peripheral_name: &str) -> &Peripheral {
        &self.peripherals.peripheral[peripheral_name]
    }

    pub fn peripheral_mut(&mut self, peripheral_name: &str) -> &mut Peripheral {
        self.peripherals
            .peripheral
            .get_mut(peripheral_name)
            .unwrap()
    }

    pub fn add_peripheral(&mut self, peripheral: Peripheral) {
        self.peripherals
            .peripheral
            .insert(peripheral.name.clone(), peripheral);
    }

    pub fn new_peripheral(&mut self, f: impl FnOnce(&mut Peripheral)) {
        let mut peripheral = Peripheral::default();
        f(&mut peripheral);
        self.add_peripheral(peripheral);
    }

    pub fn register(&self, peripheral_name: &str, register_name: &str) -> &Register {
        self.peripheral(peripheral_name).register(register_name)
    }

    pub fn add_register(&mut self, peripheral_name: &str, register: Register) {
        self.peripheral_mut(peripheral_name).add_register(register)
    }

    pub fn new_register(&mut self, peripheral_name: &str, f: impl FnOnce(&mut Register)) {
        let mut reg = Register::default();
        f(&mut reg);
        self.add_register(peripheral_name, reg);
    }

    pub fn remove_register(&mut self, peripheral_name: &str, register_name: &str) -> Register {
        self.peripheral_mut(peripheral_name)
            .remove_register(register_name)
    }

    pub fn field(&self, peripheral_name: &str, register_name: &str, field_name: &str) -> &Field {
        self.peripheral(peripheral_name)
            .register(register_name)
            .field(field_name)
    }

    pub fn field_mut(
        &mut self,
        peripheral_name: &str,
        register_name: &str,
        field_name: &str,
    ) -> &mut Field {
        self.peripheral_mut(peripheral_name)
            .register_mut(register_name)
            .field_mut(field_name)
    }

    pub fn add_field(&mut self, peripheral_name: &str, register_name: &str, field: Field) {
        self.peripheral_mut(peripheral_name)
            .register_mut(register_name)
            .add_field(field)
    }

    pub fn remove_field(
        &mut self,
        peripheral_name: &str,
        register_name: &str,
        field_name: &str,
    ) -> Field {
        self.peripheral_mut(peripheral_name)
            .register_mut(register_name)
            .remove_field(field_name)
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
        writeln!(reg_index, "reg::tokens! {{")?;
        writeln!(
            reg_index,
            "  /// Defines an index of {} register tokens.",
            self.name
        )?;
        writeln!(reg_index, "  pub macro stm32_reg_tokens;")?;
        writeln!(
            reg_index,
            "  use macro ::drone_cortex_m::map::cortex_m_reg_tokens;"
        )?;
        writeln!(reg_index, "  super::inner;")?;
        writeln!(reg_index, "  crate::reg;")?;
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
    pub fn register(&self, register_name: &str) -> &Register {
        self.registers
            .as_ref()
            .unwrap()
            .register
            .iter()
            .find(|register| register.name == register_name)
            .unwrap()
    }

    pub fn register_mut(&mut self, register_name: &str) -> &mut Register {
        self.registers
            .as_mut()
            .unwrap()
            .register
            .iter_mut()
            .find(|register| register.name == register_name)
            .unwrap()
    }

    pub fn add_register(&mut self, register: Register) {
        self.registers
            .get_or_insert_with(Default::default)
            .register
            .push(register);
    }

    pub fn remove_register(&mut self, register_name: &str) -> Register {
        let index = self
            .registers
            .as_ref()
            .unwrap()
            .register
            .iter()
            .position(|register| register.name == register_name)
            .unwrap();
        self.registers.as_mut().unwrap().register.remove(index)
    }

    fn generate_regs(
        &self,
        peripherals: &Peripherals,
        regs: &mut File,
        pool_number: usize,
        pool_size: usize,
        counter: &mut usize,
    ) -> Result<(), Error> {
        let parent = self.derived_from(peripherals)?;
        for register in self.registers(parent) {
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
            let address = self.base_address + address_offset;
            writeln!(regs, "reg! {{")?;
            for line in description.lines() {
                writeln!(regs, "  /// {}", line.trim())?;
            }
            writeln!(regs, "  pub mod {} {};", self.name, name)?;
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

    fn generate_rest(
        &self,
        peripherals: &Peripherals,
        int_names: &mut HashSet<String>,
        reg_index: &mut File,
        interrupts: &mut File,
        except: &[&str],
    ) -> Result<(), Error> {
        let parent = self.derived_from(peripherals)?;
        if except.iter().all(|&except| except != self.name) {
            let description = self
                .description
                .as_ref()
                .or_else(|| parent.and_then(|x| x.description.as_ref()))
                .ok_or_else(|| err_msg("Peripheral description not found"))?;
            for line in description.lines() {
                writeln!(reg_index, "  /// {}", line.trim())?;
            }
            writeln!(reg_index, "  pub mod {} {{", self.name)?;
            for register in self.registers(parent) {
                let Register { name, .. } = register;
                writeln!(reg_index, "    {};", name)?;
            }
            writeln!(reg_index, "  }}")?;
        }
        for interrupt in &self.interrupt {
            if int_names.insert(interrupt.name.to_owned()) {
                let &Interrupt {
                    ref name,
                    ref description,
                    value,
                } = interrupt;
                writeln!(interrupts, "thr::int! {{")?;
                for line in description.lines() {
                    writeln!(interrupts, "  /// {}", line.trim())?;
                }
                writeln!(interrupts, "  pub trait {}: {};", name, value)?;
                writeln!(interrupts, "}}")?;
            }
        }
        Ok(())
    }

    fn derived_from<'a>(&'a self, peripherals: &'a Peripherals) -> Result<Option<&'a Self>, Error> {
        Ok(if let Some(derived_from) = &self.derived_from {
            Some(
                peripherals
                    .peripheral
                    .get(derived_from)
                    .ok_or_else(|| err_msg("Peripheral `derivedFrom` not found"))?,
            )
        } else {
            None
        })
    }

    fn registers<'a>(&'a self, parent: Option<&'a Self>) -> impl Iterator<Item = &'a Register> {
        GenIter::new(static move || {
            let mut visited = HashSet::new();
            let direct = self.registers.iter().flat_map(|x| x.register.iter());
            for register in direct {
                visited.insert(&register.name);
                yield register;
            }
            let inherited = parent
                .iter()
                .flat_map(|x| x.registers.iter().flat_map(|x| x.register.iter()));
            for register in inherited {
                if !visited.contains(&register.name) {
                    yield register;
                }
            }
        })
    }
}

impl Register {
    pub fn field(&self, field_name: &str) -> &Field {
        self.fields
            .as_ref()
            .unwrap()
            .field
            .iter()
            .find(|field| field.name == field_name)
            .unwrap()
    }

    pub fn field_mut(&mut self, field_name: &str) -> &mut Field {
        self.fields
            .as_mut()
            .unwrap()
            .field
            .iter_mut()
            .find(|field| field.name == field_name)
            .unwrap()
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields
            .get_or_insert_with(Default::default)
            .field
            .push(field);
    }

    pub fn remove_field(&mut self, field_name: &str) -> Field {
        let index = self
            .fields
            .as_ref()
            .unwrap()
            .field
            .iter()
            .position(|field| field.name == field_name)
            .unwrap();
        self.fields.as_mut().unwrap().field.remove(index)
    }
}

impl Fields {
    fn generate(&self, base_access: Option<Access>, regs: &mut File) -> Result<(), Error> {
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

fn deserialize_peripheral<'de, D>(deserializer: D) -> Result<BTreeMap<String, Peripheral>, D::Error>
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

struct GenIter<T, G: Generator<Yield = T, Return = ()>>(G);

impl<T, G> GenIter<T, G>
where
    G: Generator<Yield = T, Return = ()>,
{
    pub fn new(gen: G) -> Pin<Box<Self>> {
        Box::pin(Self(gen))
    }
}

impl<T, G> Iterator for Pin<Box<GenIter<T, G>>>
where
    G: Generator<Yield = T, Return = ()>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let gen = unsafe { self.as_mut().map_unchecked_mut(|x| &mut x.0) };
        match gen.resume() {
            GeneratorState::Yielded(item) => Some(item),
            GeneratorState::Complete(()) => None,
        }
    }
}

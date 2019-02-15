//! Drone STM32 SVD bindings generator.

#![feature(generators)]
#![feature(generator_trait)]
#![feature(range_contains)]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

mod device;

use crate::device::{Access, Device, Field, Register};
use failure::Error;
use std::{
  env,
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
  process,
};

const REG_EXCLUDE: &[&str] = &[
  "FPU",
  "FPU_CPACR",
  "ITM",
  "MPU",
  "NVIC",
  "SCB",
  "STK",
  "TPIU",
];

/// Returns a device string based on features.
#[macro_export]
macro_rules! svd_feature {
  () => {
    if cfg!(feature = "stm32f100") {
      "stm32f100"
    } else if cfg!(feature = "stm32f101") {
      "stm32f101"
    } else if cfg!(feature = "stm32f102") {
      "stm32f102"
    } else if cfg!(feature = "stm32f103") {
      "stm32f103"
    } else if cfg!(feature = "stm32f107") {
      "stm32f107"
    } else if cfg!(feature = "stm32l4x1") {
      "stm32l4x1"
    } else if cfg!(feature = "stm32l4x2") {
      "stm32l4x2"
    } else if cfg!(feature = "stm32l4x3") {
      "stm32l4x3"
    } else if cfg!(feature = "stm32l4x5") {
      "stm32l4x5"
    } else if cfg!(feature = "stm32l4x6") {
      "stm32l4x6"
    } else if cfg!(feature = "stm32l4r5") {
      "stm32l4r5"
    } else if cfg!(feature = "stm32l4r7") {
      "stm32l4r7"
    } else if cfg!(feature = "stm32l4r9") {
      "stm32l4r9"
    } else if cfg!(feature = "stm32l4s5") {
      "stm32l4s5"
    } else if cfg!(feature = "stm32l4s7") {
      "stm32l4s7"
    } else if cfg!(feature = "stm32l4s9") {
      "stm32l4s9"
    } else {
      ""
    }
  };
}

/// Generates code for register mappings.
pub fn generate_regs(feature: &str, pool_number: usize, pool_size: usize) {
  let run = || {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize(feature)?;
    let mut regs = File::create(out_dir.join("svd_regs.rs"))?;
    dev.generate_regs(&mut regs, REG_EXCLUDE, pool_number, pool_size)?;
    Ok::<(), Error>(())
  };
  if let Err(error) = run() {
    eprintln!("{}", error);
    process::exit(1);
  }
}

/// Generates code for interrupts and register tokens struct.
pub fn generate_rest(feature: &str) {
  let run = || {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize(feature)?;
    let mut reg_tokens = File::create(out_dir.join("svd_reg_index.rs"))?;
    let mut interrupts = File::create(out_dir.join("svd_interrupts.rs"))?;
    dev.generate_rest(&mut reg_tokens, &mut interrupts, REG_EXCLUDE)?;
    Ok::<(), Error>(())
  };
  if let Err(error) = run() {
    eprintln!("{}", error);
    process::exit(1);
  }
}

fn svd_deserialize(feature: &str) -> Result<Device, Error> {
  match feature {
    "stm32f100" => parse_svd("STM32F100.svd"),
    "stm32f101" => parse_svd("STM32F101.svd"),
    "stm32f102" => parse_svd("STM32F102.svd"),
    "stm32f103" => parse_svd("STM32F103.svd"),
    "stm32f107" => parse_svd("STM32F107.svd"),
    "stm32l4x1" => patch_stm32l4(parse_svd("STM32L4x1.svd")?),
    "stm32l4x2" => patch_stm32l4(parse_svd("STM32L4x2.svd")?),
    "stm32l4x3" => patch_stm32l4(parse_svd("STM32L4x3.svd")?),
    "stm32l4x5" => patch_stm32l4(parse_svd("STM32L4x5.svd")?),
    "stm32l4x6" => patch_stm32l4(parse_svd("STM32L4x6.svd")?),
    "stm32l4r5" => patch_stm32l4plus(parse_svd("STM32L4R5.svd")?),
    "stm32l4r7" => patch_stm32l4plus(parse_svd("STM32L4R7.svd")?),
    "stm32l4r9" => patch_stm32l4plus(parse_svd("STM32L4R9.svd")?),
    "stm32l4s5" => patch_stm32l4plus(parse_svd("STM32L4S5.svd")?),
    "stm32l4s7" => patch_stm32l4plus(parse_svd("STM32L4S7.svd")?),
    "stm32l4s9" => patch_stm32l4plus(parse_svd("STM32L4S9.svd")?),
    _ => Ok(Device::new("Generic STM32".to_string())),
  }
}

fn patch_stm32l4(mut dev: Device) -> Result<Device, Error> {
  fix_spi3(&mut dev)?;
  fix_adc(&mut dev)?;
  fix_tim(&mut dev)?;
  Ok(dev)
}

fn patch_stm32l4plus(mut dev: Device) -> Result<Device, Error> {
  add_dmamux(&mut dev)?;
  fix_spi3(&mut dev)?;
  fix_adc(&mut dev)?;
  fix_tim(&mut dev)?;
  Ok(dev)
}

fn add_dmamux(dev: &mut Device) -> Result<(), Error> {
  dev.add_peripheral(serde_xml_rs::deserialize(
    read_svd("patch/add_dmamux.xml")?.as_bytes(),
  )?);
  Ok(())
}

fn fix_spi3(dev: &mut Device) -> Result<(), Error> {
  dev.field_mut("RCC", "APB1ENR1", "SP3EN").name = "SPI3EN".to_string();
  dev.field_mut("RCC", "APB1SMENR1", "SP3SMEN").name = "SPI3SMEN".to_string();
  Ok(())
}

fn fix_adc(dev: &mut Device) -> Result<(), Error> {
  dev.field_mut("RCC", "AHB2SMENR", "ADCFSSMEN").name = "ADCSMEN".to_string();
  Ok(())
}

fn fix_tim(dev: &mut Device) -> Result<(), Error> {
  for &name in &["TIM1", "TIM8"] {
    dev.field_mut(name, "CCMR1_Input", "IC2PCS").name = "IC2PSC".to_string();
    dev.field_mut(name, "CCMR1_Input", "ICPCS").name = "IC1PSC".to_string();
    dev.remove_field(name, "OR1", "ETR_ADC3_RMP");
    if name == "TIM8" {
      dev.remove_field(name, "OR1", "ETR_ADC2_RMP");
    }
  }
  for &name in &["TIM15", "TIM16"] {
    dev.field_mut(name, "CCMR1_Output", "OC1M").name = "OC1M0_2".to_string();
    dev.field_mut(name, "CCMR1_Output", "OC1M_2").name = "OC1M3".to_string();
    dev.remove_field(name, "BDTR", "BKF");
  }
  add_third_bit(dev, "TIM2", "SMCR", "SMS", 16);
  add_third_bit(dev, "TIM2", "CCMR1_Output", "OC1M", 16);
  add_third_bit(dev, "TIM2", "CCMR1_Output", "OC2M", 24);
  copy_field(dev, "TIM15", "TIM2", "CR1", "UIFREMAP");
  copy_field(dev, "TIM2", "TIM15", "DIER", "CC2DE");
  copy_field(dev, "TIM2", "TIM15", "DIER", "CC2IE");
  copy_field(dev, "TIM2", "TIM15", "SR", "CC2IF");
  copy_field(dev, "TIM2", "TIM15", "SR", "CC2OF");
  copy_field(dev, "TIM2", "TIM15", "CCMR1_Output", "CC2S");
  copy_field(dev, "TIM2", "TIM15", "CCMR1_Output", "OC2CE");
  copy_field(dev, "TIM2", "TIM15", "CCMR1_Output", "OC2FE");
  copy_field(dev, "TIM2", "TIM15", "CCMR1_Output", "OC2M0_2");
  copy_field(dev, "TIM2", "TIM15", "CCMR1_Output", "OC2M3");
  copy_field(dev, "TIM2", "TIM15", "CCMR1_Output", "OC2PE");
  copy_field(dev, "TIM2", "TIM15", "CCMR1_Input", "CC2S");
  copy_field(dev, "TIM2", "TIM15", "CCMR1_Input", "IC2F");
  copy_field(dev, "TIM2", "TIM15", "CCMR1_Input", "IC2PSC");
  copy_field(dev, "TIM2", "TIM15", "CCER", "CC2NP");
  copy_field(dev, "TIM2", "TIM15", "CCER", "CC2P");
  copy_field(dev, "TIM2", "TIM15", "CCER", "CC2E");
  copy_field(dev, "TIM15", "TIM2", "CNT", "UIFCPY");
  dev.field_mut("TIM2", "CNT", "CNT_H").bit_width = 15;
  let field = dev.field_mut("TIM2", "CNT", "UIFCPY");
  field.name = "UIFCPY_CNT31".to_string();
  field.access = Some(Access::ReadWrite);
  dev.remove_field("TIM2", "DIER", "COMDE");
  dev.remove_field("TIM16", "DIER", "TDE");
  dev.remove_field("TIM16", "DIER", "TIE");
  dev.remove_field("TIM16", "SR", "TIF");
  dev.remove_field("TIM16", "EGR", "TG");
  let mut reg = dev.register("TIM2", "SMCR").clone();
  reg.remove_field("ETP");
  reg.remove_field("ECE");
  reg.remove_field("ETPS");
  reg.remove_field("ETF");
  dev.add_register("TIM15", reg);
  let reg = dev.register("TIM16", "OR2").clone();
  dev.add_register("TIM15", reg);
  for &field in &["OIS2", "TI1S", "MMS"] {
    copy_field(dev, "TIM1", "TIM15", "CR2", field);
  }
  dev.remove_register("TIM2", "OR");
  {
    let mut reg = Register::default();
    reg.name = "OR1".to_string();
    reg.description = "TIM2 option register 1".to_string();
    reg.address_offset = 0x50;
    reg.size = 0x20;
    reg.access = Some(Access::ReadWrite);
    reg.reset_value = 0x0000;
    reg.add_field(Field {
      name: "ETR1_RMP".to_string(),
      description: "External trigger remap".to_string(),
      bit_offset: 1,
      bit_width: 1,
      access: None,
    });
    reg.add_field(Field {
      name: "ITR1_RMP".to_string(),
      description: "Internal trigger 1 remap".to_string(),
      bit_offset: 0,
      bit_width: 1,
      access: None,
    });
    reg.add_field(Field {
      name: "TI4_RMP".to_string(),
      description: "Input Capture 4 remap".to_string(),
      bit_offset: 2,
      bit_width: 2,
      access: None,
    });
    dev.add_register("TIM2", reg);
  }
  for &name in &["TIM2", "TIM3"] {
    let mut reg = Register::default();
    reg.name = "OR2".to_string();
    reg.description = format!("{} option register 2", name);
    reg.address_offset = 0x60;
    reg.size = 0x20;
    reg.access = Some(Access::ReadWrite);
    reg.reset_value = 0x0000;
    reg.add_field(Field {
      name: "ETRSEL".to_string(),
      description: "ETR source selection".to_string(),
      bit_offset: 14,
      bit_width: 3,
      access: None,
    });
    dev.add_register(name, reg);
  }
  {
    let mut reg = Register::default();
    reg.name = "OR1".to_string();
    reg.description = "TIM3 option register 1".to_string();
    reg.address_offset = 0x50;
    reg.size = 0x20;
    reg.access = Some(Access::ReadWrite);
    reg.reset_value = 0x0000;
    reg.add_field(Field {
      name: "TI1_RMP".to_string(),
      description: "Input Capture 1 remap".to_string(),
      bit_offset: 0,
      bit_width: 2,
      access: None,
    });
    dev.add_register("TIM3", reg);
  }
  {
    let mut reg = Register::default();
    reg.name = "OR1".to_string();
    reg.description = "TIM15 option register 1".to_string();
    reg.address_offset = 0x50;
    reg.size = 0x20;
    reg.access = Some(Access::ReadWrite);
    reg.reset_value = 0x0000;
    reg.add_field(Field {
      name: "ENCODER_MODE".to_string(),
      description: "Encoder mode".to_string(),
      bit_offset: 1,
      bit_width: 2,
      access: None,
    });
    reg.add_field(Field {
      name: "TI1_RMP".to_string(),
      description: "Input Capture 1 remap".to_string(),
      bit_offset: 0,
      bit_width: 1,
      access: None,
    });
    dev.add_register("TIM15", reg);
  }
  for &name in &["LPTIM1", "LPTIM2"] {
    let mut reg = Register::default();
    reg.name = "OR".to_string();
    reg.description = format!("{} option register", name);
    reg.address_offset = 0x20;
    reg.size = 0x20;
    reg.access = Some(Access::ReadWrite);
    reg.reset_value = 0x0000;
    reg.add_field(Field {
      name: "OR_0".to_string(),
      description: "Option register bit 0".to_string(),
      bit_offset: 0,
      bit_width: 1,
      access: None,
    });
    reg.add_field(Field {
      name: "OR_1".to_string(),
      description: "Option register bit 1".to_string(),
      bit_offset: 1,
      bit_width: 1,
      access: None,
    });
    dev.add_register(name, reg);
  }
  Ok(())
}

fn copy_field(
  dev: &mut Device,
  periph_from: &str,
  periph_to: &str,
  reg_name: &str,
  field_name: &str,
) {
  let field = dev.field(periph_from, reg_name, field_name).clone();
  dev.add_field(periph_to, reg_name, field);
}

fn add_third_bit(
  dev: &mut Device,
  periph_name: &str,
  reg_name: &str,
  field_name: &str,
  bit_offset: usize,
) {
  let field = dev.field_mut(periph_name, reg_name, field_name);
  field.name = format!("{}0_2", field_name);
  let mut field = field.clone();
  field.name = format!("{}3", field_name);
  field.bit_offset = bit_offset;
  field.bit_width = 1;
  dev.add_field(periph_name, reg_name, field);
}

fn parse_svd(input: &str) -> Result<Device, Error> {
  Ok(serde_xml_rs::deserialize(read_svd(input)?.as_bytes())?)
}

fn read_svd(path: &str) -> Result<String, Error> {
  let mut input = BufReader::new(File::open(format!(
    "{}/../svd_files/{}",
    env!("CARGO_MANIFEST_DIR"),
    path
  ))?);
  let mut xml = String::new();
  input.read_to_string(&mut xml)?;
  Ok(xml)
}

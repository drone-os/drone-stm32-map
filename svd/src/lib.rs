//! Drone STM32 SVD bindings generator.

#![feature(range_contains)]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

mod device;

use crate::device::Device;
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
    let device = svd_deserialize(feature)?;
    let mut regs = File::create(out_dir.join("svd_regs.rs"))?;
    device.generate_regs(&mut regs, REG_EXCLUDE, pool_number, pool_size)?;
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
    let device = svd_deserialize(feature)?;
    let mut reg_tokens = File::create(out_dir.join("svd_reg_index.rs"))?;
    let mut interrupts = File::create(out_dir.join("svd_interrupts.rs"))?;
    device.generate_rest(&mut reg_tokens, &mut interrupts, REG_EXCLUDE)?;
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

fn patch_stm32l4(mut device: Device) -> Result<Device, Error> {
  fix_spi3(&mut device)?;
  fix_adc(&mut device)?;
  Ok(device)
}

fn patch_stm32l4plus(mut device: Device) -> Result<Device, Error> {
  add_dmamux(&mut device)?;
  fix_spi3(&mut device)?;
  fix_adc(&mut device)?;
  Ok(device)
}

fn add_dmamux(device: &mut Device) -> Result<(), Error> {
  device.set_peripheral(serde_xml_rs::deserialize(
    read_svd("patch/add_dmamux.xml")?.as_bytes(),
  )?);
  Ok(())
}

fn fix_spi3(device: &mut Device) -> Result<(), Error> {
  device
    .peripheral_mut("RCC")
    .unwrap()
    .registers_mut()
    .find(|register| register.name == "APB1ENR1")
    .unwrap()
    .fields_mut()
    .find(|field| field.name == "SP3EN")
    .unwrap()
    .name = "SPI3EN".to_string();
  device
    .peripheral_mut("RCC")
    .unwrap()
    .registers_mut()
    .find(|register| register.name == "APB1SMENR1")
    .unwrap()
    .fields_mut()
    .find(|field| field.name == "SP3SMEN")
    .unwrap()
    .name = "SPI3SMEN".to_string();
  Ok(())
}

fn fix_adc(device: &mut Device) -> Result<(), Error> {
  device
    .peripheral_mut("RCC")
    .unwrap()
    .registers_mut()
    .find(|register| register.name == "AHB2SMENR")
    .unwrap()
    .fields_mut()
    .find(|field| field.name == "ADCFSSMEN")
    .unwrap()
    .name = "ADCSMEN".to_string();
  Ok(())
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

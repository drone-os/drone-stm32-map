//! STM32 SVD to Drone bindings converter.

#![feature(generators)]
#![feature(generator_trait)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

mod device;

use crate::device::{Access, Device, Field, Interrupt};
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
        "stm32f102" => patch_stm32f102(parse_svd("STM32F102.svd")?),
        "stm32f103" => parse_svd("STM32F103.svd"),
        "stm32f107" => parse_svd("STM32F107.svd"),
        "stm32l4x1" => patch_stm32l4x1(parse_svd("STM32L4x1.svd")?),
        "stm32l4x2" => patch_stm32l4x2(parse_svd("STM32L4x2.svd")?),
        "stm32l4x3" => patch_stm32l4x3(parse_svd("STM32L4x3.svd")?),
        "stm32l4x5" => patch_stm32l4x5(parse_svd("STM32L4x5.svd")?),
        "stm32l4x6" => patch_stm32l4x6(parse_svd("STM32L4x6.svd")?),
        "stm32l4r5" => patch_stm32l4plus(parse_svd("STM32L4R5.svd")?),
        "stm32l4r7" => patch_stm32l4plus(parse_svd("STM32L4R7.svd")?),
        "stm32l4r9" => patch_stm32l4plus(parse_svd("STM32L4R9.svd")?),
        "stm32l4s5" => patch_stm32l4plus(parse_svd("STM32L4S5.svd")?),
        "stm32l4s7" => patch_stm32l4plus(parse_svd("STM32L4S7.svd")?),
        "stm32l4s9" => patch_stm32l4plus(parse_svd("STM32L4S9.svd")?),
        _ => Ok(Device::new("Generic STM32".to_string())),
    }
}

fn patch_stm32f102(mut dev: Device) -> Result<Device, Error> {
    fix_spi2_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4x1(mut dev: Device) -> Result<Device, Error> {
    fix_adc(&mut dev)?;
    fix_lptim1(&mut dev)?;
    fix_lptim2(&mut dev)?;
    fix_lpuart1(&mut dev)?;
    fix_rcc(&mut dev)?;
    fix_spi2_2(&mut dev)?;
    fix_spi3_1(&mut dev)?;
    fix_tim1(&mut dev)?;
    fix_tim16(&mut dev)?;
    fix_tim2_and_tim15(&mut dev)?;
    fix_tim3_1(&mut dev)?;
    fix_tim3_2(&mut dev)?;
    fix_uart4(&mut dev)?;
    fix_usart1(&mut dev)?;
    fix_usart3(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4x2(mut dev: Device) -> Result<Device, Error> {
    fix_adc(&mut dev)?;
    fix_i2c(&mut dev)?;
    fix_lptim1(&mut dev)?;
    fix_lptim2(&mut dev)?;
    fix_lpuart1(&mut dev)?;
    fix_rcc(&mut dev)?;
    fix_spi2_2(&mut dev)?;
    fix_spi3_1(&mut dev)?;
    fix_tim1(&mut dev)?;
    fix_tim16(&mut dev)?;
    fix_tim2_and_tim15(&mut dev)?;
    fix_tim3_1(&mut dev)?;
    fix_tim3_2(&mut dev)?;
    fix_uart4(&mut dev)?;
    fix_usart1(&mut dev)?;
    fix_usart3(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4x3(mut dev: Device) -> Result<Device, Error> {
    add_tim3(&mut dev)?;
    fix_adc(&mut dev)?;
    fix_lptim1(&mut dev)?;
    fix_lptim2(&mut dev)?;
    fix_rcc(&mut dev)?;
    fix_spi3_2(&mut dev)?;
    fix_tim1(&mut dev)?;
    fix_tim16(&mut dev)?;
    fix_tim2_and_tim15(&mut dev)?;
    fix_tim3_1(&mut dev)?;
    fix_tim3_2(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4x5(mut dev: Device) -> Result<Device, Error> {
    fix_adc(&mut dev)?;
    fix_exti(&mut dev)?;
    fix_lptim1(&mut dev)?;
    fix_lptim2(&mut dev)?;
    fix_rcc(&mut dev)?;
    fix_rtc(&mut dev)?;
    fix_spi3_2(&mut dev)?;
    fix_tim1(&mut dev)?;
    fix_tim16(&mut dev)?;
    fix_tim2_and_tim15(&mut dev)?;
    fix_tim3_1(&mut dev)?;
    fix_tim8(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4x6(mut dev: Device) -> Result<Device, Error> {
    fix_adc(&mut dev)?;
    fix_exti(&mut dev)?;
    fix_lptim1(&mut dev)?;
    fix_lptim2(&mut dev)?;
    fix_rcc(&mut dev)?;
    fix_spi3_2(&mut dev)?;
    fix_tim1(&mut dev)?;
    fix_tim16(&mut dev)?;
    fix_tim2_and_tim15(&mut dev)?;
    fix_tim3_1(&mut dev)?;
    fix_tim8(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4plus(mut dev: Device) -> Result<Device, Error> {
    add_dmamux(&mut dev)?;
    fix_adc(&mut dev)?;
    fix_exti(&mut dev)?;
    fix_lptim1(&mut dev)?;
    fix_lptim2(&mut dev)?;
    fix_pwr(&mut dev)?;
    fix_spi3_2(&mut dev)?;
    fix_tim1(&mut dev)?;
    fix_tim16(&mut dev)?;
    fix_tim2_and_tim15(&mut dev)?;
    fix_tim3_1(&mut dev)?;
    fix_tim8(&mut dev)?;
    Ok(dev)
}

fn add_dmamux(dev: &mut Device) -> Result<(), Error> {
    dev.add_peripheral(serde_xml_rs::deserialize(
        read_svd("patch/add_dmamux.xml")?.as_bytes(),
    )?);
    Ok(())
}

fn add_tim3(dev: &mut Device) -> Result<(), Error> {
    dev.new_peripheral(|peripheral| {
        peripheral.derived_from = Some("TIM2".to_string());
        peripheral.name = "TIM3".to_string();
        peripheral.base_address = 0x4000_0400;
        peripheral.interrupt = vec![Interrupt {
            name: "TIM3".to_string(),
            description: "TIM3 global interrupt".to_string(),
            value: 29,
        }];
    });
    Ok(())
}

fn fix_adc(dev: &mut Device) -> Result<(), Error> {
    dev.field_mut("RCC", "AHB2SMENR", "ADCFSSMEN").name = "ADCSMEN".to_string();
    Ok(())
}

fn fix_exti(dev: &mut Device) -> Result<(), Error> {
    for (reg_name, field_name) in &[("IMR2", "MR39"), ("EMR2", "MR39")] {
        let mut field = dev.field("EXTI", reg_name, field_name).clone();
        field.name = field.name.replace("39", "40");
        field.description = field.description.replace("39", "40");
        field.bit_offset += 1;
        dev.add_field("EXTI", reg_name, field);
    }
    Ok(())
}

fn fix_i2c(dev: &mut Device) -> Result<(), Error> {
    dev.add_field("RCC", "APB1SMENR2", Field {
        name: "I2C4SMEN".to_string(),
        description: "I2C4 clocks enable during Sleep and Stop modes".to_string(),
        bit_offset: 1,
        bit_width: 1,
        access: None,
    });
    Ok(())
}

fn fix_lptim1(dev: &mut Device) -> Result<(), Error> {
    dev.new_register("LPTIM1", |reg| {
        reg.name = "OR".to_string();
        reg.description = format!("{} option register", "LPTIM1");
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
    });
    Ok(())
}

fn fix_lptim2(dev: &mut Device) -> Result<(), Error> {
    dev.new_register("LPTIM2", |reg| {
        reg.name = "OR".to_string();
        reg.description = format!("{} option register", "LPTIM2");
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
    });
    Ok(())
}

fn fix_lpuart1(dev: &mut Device) -> Result<(), Error> {
    copy_field(dev, "USART3", "LPUART1", "CR3", "UCESM");
    Ok(())
}

fn fix_pwr(dev: &mut Device) -> Result<(), Error> {
    dev.add_field("PWR", "CR1", Field {
        name: "RRSTP".to_string(),
        description: "SRAM3 retention in Stop 2 mode".to_string(),
        bit_offset: 4,
        bit_width: 1,
        access: None,
    });
    Ok(())
}

fn fix_rcc(dev: &mut Device) -> Result<(), Error> {
    dev.new_register("RCC", |reg| {
        reg.name = "CCIPR2".to_string();
        reg.description = "Peripherals independent clock configuration register".to_string();
        reg.address_offset = 0x9C;
        reg.size = 0x20;
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = 0x0000;
        reg.add_field(Field {
            name: "I2C4SEL".to_string(),
            description: "I2C4 clock source selection".to_string(),
            bit_offset: 0,
            bit_width: 2,
            access: None,
        });
    });
    Ok(())
}

fn fix_rtc(dev: &mut Device) -> Result<(), Error> {
    dev.add_field("RCC", "APB1ENR1", Field {
        name: "RTCAPBEN".to_string(),
        description: "RTC APB clock enable".to_string(),
        bit_offset: 10,
        bit_width: 1,
        access: None,
    });
    dev.add_field("RCC", "APB1SMENR1", Field {
        name: "RTCAPBSMEN".to_string(),
        description: "RTC APB clock enable during Sleep and Stop modes".to_string(),
        bit_offset: 10,
        bit_width: 1,
        access: None,
    });
    Ok(())
}

fn fix_spi2_1(dev: &mut Device) -> Result<(), Error> {
    dev.add_field("RCC", "APB1ENR", Field {
        name: "SPI2EN".to_string(),
        description: "SPI 2 clock enable".to_string(),
        bit_offset: 14,
        bit_width: 1,
        access: None,
    });
    dev.add_field("RCC", "APB1RSTR", Field {
        name: "SPI2RST".to_string(),
        description: "SPI2 reset".to_string(),
        bit_offset: 14,
        bit_width: 1,
        access: None,
    });
    copy_field(dev, "SPI1", "SPI2", "SR", "UDR");
    copy_field(dev, "SPI1", "SPI2", "SR", "CHSIDE");
    Ok(())
}

fn fix_spi2_2(dev: &mut Device) -> Result<(), Error> {
    dev.add_field("RCC", "APB1ENR1", Field {
        name: "SPI2EN".to_string(),
        description: "SPI2 clock enable".to_string(),
        bit_offset: 14,
        bit_width: 1,
        access: None,
    });
    Ok(())
}

fn fix_spi3_1(dev: &mut Device) -> Result<(), Error> {
    dev.field_mut("RCC", "APB1SMENR1", "SP3SMEN").name = "SPI3SMEN".to_string();
    Ok(())
}

fn fix_spi3_2(dev: &mut Device) -> Result<(), Error> {
    dev.field_mut("RCC", "APB1ENR1", "SP3EN").name = "SPI3EN".to_string();
    dev.field_mut("RCC", "APB1SMENR1", "SP3SMEN").name = "SPI3SMEN".to_string();
    Ok(())
}

fn fix_tim1(dev: &mut Device) -> Result<(), Error> {
    dev.field_mut("TIM1", "CCMR1_Input", "IC2PCS").name = "IC2PSC".to_string();
    dev.field_mut("TIM1", "CCMR1_Input", "ICPCS").name = "IC1PSC".to_string();
    dev.remove_field("TIM1", "OR1", "ETR_ADC3_RMP");
    Ok(())
}

fn fix_tim2(dev: &mut Device) -> Result<(), Error> {
    add_third_bit(dev, "TIM2", "SMCR", "SMS", 16);
    add_third_bit(dev, "TIM2", "CCMR1_Output", "OC1M", 16);
    add_third_bit(dev, "TIM2", "CCMR1_Output", "OC2M", 24);
    copy_field(dev, "TIM15", "TIM2", "CR1", "UIFREMAP");
    copy_field(dev, "TIM15", "TIM2", "CNT", "UIFCPY");
    dev.field_mut("TIM2", "CNT", "CNT_H").bit_width = 15;
    let field = dev.field_mut("TIM2", "CNT", "UIFCPY");
    field.name = "UIFCPY_CNT31".to_string();
    field.access = Some(Access::ReadWrite);
    dev.remove_field("TIM2", "DIER", "COMDE");
    dev.remove_register("TIM2", "OR");
    dev.new_register("TIM2", |reg| {
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
    });
    dev.new_register("TIM2", |reg| {
        reg.name = "OR2".to_string();
        reg.description = format!("{} option register 2", "TIM2");
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
    });
    Ok(())
}

fn fix_tim2_and_tim15(dev: &mut Device) -> Result<(), Error> {
    fix_tim2(dev)?;
    dev.field_mut("TIM15", "CCMR1_Output", "OC1M").name = "OC1M0_2".to_string();
    dev.field_mut("TIM15", "CCMR1_Output", "OC1M_2").name = "OC1M3".to_string();
    dev.remove_field("TIM15", "BDTR", "BKF");
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
    {
        let mut reg = dev.register("TIM2", "SMCR").clone();
        reg.remove_field("ETP");
        reg.remove_field("ECE");
        reg.remove_field("ETPS");
        reg.remove_field("ETF");
        dev.add_register("TIM15", reg);
    }
    {
        let reg = dev.register("TIM16", "OR2").clone();
        dev.add_register("TIM15", reg);
    }
    for &field in &["OIS2", "TI1S", "MMS"] {
        copy_field(dev, "TIM1", "TIM15", "CR2", field);
    }
    dev.new_register("TIM15", |reg| {
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
    });
    Ok(())
}

fn fix_tim3_1(dev: &mut Device) -> Result<(), Error> {
    dev.new_register("TIM3", |reg| {
        reg.name = "OR2".to_string();
        reg.description = format!("{} option register 2", "TIM3");
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
    });
    dev.new_register("TIM3", |reg| {
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
    });
    Ok(())
}

fn fix_tim3_2(dev: &mut Device) -> Result<(), Error> {
    dev.add_field("RCC", "APB1RSTR1", Field {
        name: "TIM3RST".to_string(),
        description: "TIM3 timer reset".to_string(),
        bit_offset: 1,
        bit_width: 1,
        access: None,
    });
    dev.add_field("RCC", "APB1SMENR1", Field {
        name: "TIM3SMEN".to_string(),
        description: "TIM3 timer clocks enable during Sleep and Stop modes".to_string(),
        bit_offset: 1,
        bit_width: 1,
        access: None,
    });
    Ok(())
}

fn fix_tim8(dev: &mut Device) -> Result<(), Error> {
    dev.field_mut("TIM8", "CCMR1_Input", "IC2PCS").name = "IC2PSC".to_string();
    dev.field_mut("TIM8", "CCMR1_Input", "ICPCS").name = "IC1PSC".to_string();
    dev.remove_field("TIM8", "OR1", "ETR_ADC3_RMP");
    dev.remove_field("TIM8", "OR1", "ETR_ADC2_RMP");
    Ok(())
}

fn fix_tim16(dev: &mut Device) -> Result<(), Error> {
    dev.field_mut("TIM16", "CCMR1_Output", "OC1M").name = "OC1M0_2".to_string();
    dev.field_mut("TIM16", "CCMR1_Output", "OC1M_2").name = "OC1M3".to_string();
    dev.remove_field("TIM16", "BDTR", "BKF");
    dev.remove_field("TIM16", "DIER", "TDE");
    dev.remove_field("TIM16", "DIER", "TIE");
    dev.remove_field("TIM16", "SR", "TIF");
    dev.remove_field("TIM16", "EGR", "TG");
    Ok(())
}

fn fix_uart4(dev: &mut Device) -> Result<(), Error> {
    dev.add_field("RCC", "APB1RSTR1", Field {
        name: "UART4RST".to_string(),
        description: "UART4 reset".to_string(),
        bit_offset: 19,
        bit_width: 1,
        access: None,
    });
    dev.add_field("RCC", "APB1SMENR1", Field {
        name: "UART4SMEN".to_string(),
        description: "UART4 clocks enable during Sleep and Stop modes".to_string(),
        bit_offset: 19,
        bit_width: 1,
        access: None,
    });
    dev.field_mut("RCC", "CCIPR", "USART4SEL").name = "UART4SEL".to_string();
    Ok(())
}

fn fix_usart1(dev: &mut Device) -> Result<(), Error> {
    copy_field(dev, "USART3", "USART1", "CR3", "UCESM");
    Ok(())
}

fn fix_usart3(dev: &mut Device) -> Result<(), Error> {
    dev.add_field("RCC", "APB1ENR1", Field {
        name: "USART3EN".to_string(),
        description: "USART3 clock enable".to_string(),
        bit_offset: 18,
        bit_width: 1,
        access: None,
    });
    dev.add_field("RCC", "APB1RSTR1", Field {
        name: "USART3RST".to_string(),
        description: "USART3 reset".to_string(),
        bit_offset: 18,
        bit_width: 1,
        access: None,
    });
    dev.add_field("RCC", "APB1SMENR1", Field {
        name: "USART3SMEN".to_string(),
        description: "USART3 clocks enable during Sleep and Stop modes".to_string(),
        bit_offset: 18,
        bit_width: 1,
        access: None,
    });
    dev.remove_field("USART3", "BRR", "BRR");
    dev.add_field("USART3", "BRR", Field {
        name: "DIV_Mantissa".to_string(),
        description: "DIV_Mantissa".to_string(),
        bit_offset: 4,
        bit_width: 12,
        access: None,
    });
    dev.add_field("USART3", "BRR", Field {
        name: "DIV_Fraction".to_string(),
        description: "DIV_Fraction".to_string(),
        bit_offset: 0,
        bit_width: 4,
        access: None,
    });
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
        "{}/files/{}",
        env!("CARGO_MANIFEST_DIR"),
        path
    ))?);
    let mut xml = String::new();
    input.read_to_string(&mut xml)?;
    Ok(xml)
}

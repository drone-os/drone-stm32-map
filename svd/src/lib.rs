//! STM32 SVD to bindings for Drone, an Embedded Operating System.

#![feature(generators)]
#![feature(generator_trait)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

use drone_svd::{Access, Device, Interrupt};
use failure::Error;
use std::{env, fs::File, path::Path, process};

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

/// Returns the selected device feature.
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
        dev.generate_rest(
            &mut reg_tokens,
            &mut interrupts,
            REG_EXCLUDE,
            "stm32_reg_tokens",
        )?;
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
    dev.add_periph(parse_svd("patch/add_dmamux.xml")?.periph("DMAMUX1").clone());
    Ok(())
}

fn add_tim3(dev: &mut Device) -> Result<(), Error> {
    dev.new_periph(|peripheral| {
        peripheral.derived_from = Some("TIM2".to_string());
        peripheral.name = "TIM3".to_string();
        peripheral.base_address = 0x4000_0400;
        peripheral.interrupt.push({
            let mut interrupt = Interrupt::default();
            interrupt.name = "TIM3".to_string();
            interrupt.description = "TIM3 global interrupt".to_string();
            interrupt.value = 29;
            interrupt
        });
    });
    Ok(())
}

fn fix_adc(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").reg("AHB2SMENR").field("ADCFSSMEN").name = "ADCSMEN".to_string();
    Ok(())
}

fn fix_exti(dev: &mut Device) -> Result<(), Error> {
    for (reg_name, field_name) in &[("IMR2", "MR39"), ("EMR2", "MR39")] {
        let mut field = dev.periph("EXTI").reg(reg_name).field(field_name).clone();
        field.name = field.name.replace("39", "40");
        field.description = field.description.replace("39", "40");
        field.bit_offset = Some(field.bit_offset.unwrap() + 1);
        dev.periph("EXTI").reg(reg_name).add_field(field);
    }
    Ok(())
}

fn fix_i2c(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").reg("APB1SMENR2").new_field(|field| {
        field.name = "I2C4SMEN".to_string();
        field.description = "I2C4 clocks enable during Sleep and Stop modes".to_string();
        field.bit_offset = Some(1);
        field.bit_width = Some(1);
    });
    Ok(())
}

fn fix_lptim1(dev: &mut Device) -> Result<(), Error> {
    dev.periph("LPTIM1").new_reg(|reg| {
        reg.name = "OR".to_string();
        reg.description = "LPTIM1 option register".to_string();
        reg.address_offset = 0x20;
        reg.size = Some(0x20);
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = Some(0x0000);
        reg.new_field(|field| {
            field.name = "OR_0".to_string();
            field.description = "Option register bit 0".to_string();
            field.bit_offset = Some(0);
            field.bit_width = Some(1);
        });
        reg.new_field(|field| {
            field.name = "OR_1".to_string();
            field.description = "Option register bit 1".to_string();
            field.bit_offset = Some(1);
            field.bit_width = Some(1);
        });
    });
    Ok(())
}

fn fix_lptim2(dev: &mut Device) -> Result<(), Error> {
    dev.periph("LPTIM2").new_reg(|reg| {
        reg.name = "OR".to_string();
        reg.description = "LPTIM2 option register".to_string();
        reg.address_offset = 0x20;
        reg.size = Some(0x20);
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = Some(0x0000);
        reg.new_field(|field| {
            field.name = "OR_0".to_string();
            field.description = "Option register bit 0".to_string();
            field.bit_offset = Some(0);
            field.bit_width = Some(1);
        });
        reg.new_field(|field| {
            field.name = "OR_1".to_string();
            field.description = "Option register bit 1".to_string();
            field.bit_offset = Some(1);
            field.bit_width = Some(1);
        });
    });
    Ok(())
}

fn fix_lpuart1(dev: &mut Device) -> Result<(), Error> {
    copy_field(dev, "USART3", "LPUART1", "CR3", "UCESM");
    Ok(())
}

fn fix_pwr(dev: &mut Device) -> Result<(), Error> {
    dev.periph("PWR").reg("CR1").new_field(|field| {
        field.name = "RRSTP".to_string();
        field.description = "SRAM3 retention in Stop 2 mode".to_string();
        field.bit_offset = Some(4);
        field.bit_width = Some(1);
    });
    Ok(())
}

fn fix_rcc(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").new_reg(|reg| {
        reg.name = "CCIPR2".to_string();
        reg.description = "Peripherals independent clock configuration register".to_string();
        reg.address_offset = 0x9C;
        reg.size = Some(0x20);
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = Some(0x0000);
        reg.new_field(|field| {
            field.name = "I2C4SEL".to_string();
            field.description = "I2C4 clock source selection".to_string();
            field.bit_offset = Some(0);
            field.bit_width = Some(2);
        });
    });
    Ok(())
}

fn fix_rtc(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").reg("APB1ENR1").new_field(|field| {
        field.name = "RTCAPBEN".to_string();
        field.description = "RTC APB clock enable".to_string();
        field.bit_offset = Some(10);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1SMENR1").new_field(|field| {
        field.name = "RTCAPBSMEN".to_string();
        field.description = "RTC APB clock enable during Sleep and Stop modes".to_string();
        field.bit_offset = Some(10);
        field.bit_width = Some(1);
    });
    Ok(())
}

fn fix_spi2_1(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").reg("APB1ENR").new_field(|field| {
        field.name = "SPI2EN".to_string();
        field.description = "SPI 2 clock enable".to_string();
        field.bit_offset = Some(14);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1RSTR").new_field(|field| {
        field.name = "SPI2RST".to_string();
        field.description = "SPI2 reset".to_string();
        field.bit_offset = Some(14);
        field.bit_width = Some(1);
    });
    copy_field(dev, "SPI1", "SPI2", "SR", "UDR");
    copy_field(dev, "SPI1", "SPI2", "SR", "CHSIDE");
    Ok(())
}

fn fix_spi2_2(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").reg("APB1ENR1").new_field(|field| {
        field.name = "SPI2EN".to_string();
        field.description = "SPI2 clock enable".to_string();
        field.bit_offset = Some(14);
        field.bit_width = Some(1);
    });
    Ok(())
}

fn fix_spi3_1(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").reg("APB1SMENR1").field("SP3SMEN").name = "SPI3SMEN".to_string();
    Ok(())
}

fn fix_spi3_2(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").reg("APB1ENR1").field("SP3EN").name = "SPI3EN".to_string();
    dev.periph("RCC").reg("APB1SMENR1").field("SP3SMEN").name = "SPI3SMEN".to_string();
    Ok(())
}

fn fix_tim1(dev: &mut Device) -> Result<(), Error> {
    dev.periph("TIM1").reg("CCMR1_Input").field("IC2PCS").name = "IC2PSC".to_string();
    dev.periph("TIM1").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    dev.periph("TIM1").reg("OR1").remove_field("ETR_ADC3_RMP");
    Ok(())
}

fn fix_tim2(dev: &mut Device) -> Result<(), Error> {
    fn add_third_bit(
        dev: &mut Device,
        periph_name: &str,
        reg_name: &str,
        field_name: &str,
        bit_offset: u32,
    ) {
        let field = dev.periph(periph_name).reg(reg_name).field(field_name);
        field.name = format!("{}0_2", field_name);
        let mut field = field.clone();
        field.name = format!("{}3", field_name);
        field.bit_offset = Some(bit_offset);
        field.bit_width = Some(1);
        dev.periph(periph_name).reg(reg_name).add_field(field);
    }
    add_third_bit(dev, "TIM2", "SMCR", "SMS", 16);
    add_third_bit(dev, "TIM2", "CCMR1_Output", "OC1M", 16);
    add_third_bit(dev, "TIM2", "CCMR1_Output", "OC2M", 24);
    copy_field(dev, "TIM15", "TIM2", "CR1", "UIFREMAP");
    copy_field(dev, "TIM15", "TIM2", "CNT", "UIFCPY");
    dev.periph("TIM2").reg("CNT").field("CNT_H").bit_width = Some(15);
    dev.periph("TIM2").reg("CNT").field("UIFCPY").access = Some(Access::ReadWrite);
    dev.periph("TIM2").reg("CNT").field("UIFCPY").name = "UIFCPY_CNT31".to_string();
    dev.periph("TIM2").reg("DIER").remove_field("COMDE");
    dev.periph("TIM2").remove_reg("OR");
    dev.periph("TIM2").new_reg(|reg| {
        reg.name = "OR1".to_string();
        reg.description = "TIM2 option register 1".to_string();
        reg.address_offset = 0x50;
        reg.size = Some(0x20);
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = Some(0x0000);
        reg.new_field(|field| {
            field.name = "ETR1_RMP".to_string();
            field.description = "External trigger remap".to_string();
            field.bit_offset = Some(1);
            field.bit_width = Some(1);
        });
        reg.new_field(|field| {
            field.name = "ITR1_RMP".to_string();
            field.description = "Internal trigger 1 remap".to_string();
            field.bit_offset = Some(0);
            field.bit_width = Some(1);
        });
        reg.new_field(|field| {
            field.name = "TI4_RMP".to_string();
            field.description = "Input Capture 4 remap".to_string();
            field.bit_offset = Some(2);
            field.bit_width = Some(2);
        });
    });
    dev.periph("TIM2").new_reg(|reg| {
        reg.name = "OR2".to_string();
        reg.description = "TIM2 option register 2".to_string();
        reg.address_offset = 0x60;
        reg.size = Some(0x20);
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = Some(0x0000);
        reg.new_field(|field| {
            field.name = "ETRSEL".to_string();
            field.description = "ETR source selection".to_string();
            field.bit_offset = Some(14);
            field.bit_width = Some(3);
        });
    });
    Ok(())
}

fn fix_tim2_and_tim15(dev: &mut Device) -> Result<(), Error> {
    fix_tim2(dev)?;
    dev.periph("TIM15").reg("CCMR1_Output").field("OC1M").name = "OC1M0_2".to_string();
    dev.periph("TIM15").reg("CCMR1_Output").field("OC1M_2").name = "OC1M3".to_string();
    dev.periph("TIM15").reg("BDTR").remove_field("BKF");
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
    copy_reg(dev, "TIM2", "TIM15", "SMCR");
    dev.periph("TIM15").reg("SMCR").remove_field("ETP");
    dev.periph("TIM15").reg("SMCR").remove_field("ECE");
    dev.periph("TIM15").reg("SMCR").remove_field("ETPS");
    dev.periph("TIM15").reg("SMCR").remove_field("ETF");
    copy_reg(dev, "TIM16", "TIM15", "OR2");
    for &field in &["OIS2", "TI1S", "MMS"] {
        copy_field(dev, "TIM1", "TIM15", "CR2", field);
    }
    dev.periph("TIM15").new_reg(|reg| {
        reg.name = "OR1".to_string();
        reg.description = "TIM15 option register 1".to_string();
        reg.address_offset = 0x50;
        reg.size = Some(0x20);
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = Some(0x0000);
        reg.new_field(|field| {
            field.name = "ENCODER_MODE".to_string();
            field.description = "Encoder mode".to_string();
            field.bit_offset = Some(1);
            field.bit_width = Some(2);
        });
        reg.new_field(|field| {
            field.name = "TI1_RMP".to_string();
            field.description = "Input Capture 1 remap".to_string();
            field.bit_offset = Some(0);
            field.bit_width = Some(1);
        });
    });
    Ok(())
}

fn fix_tim3_1(dev: &mut Device) -> Result<(), Error> {
    dev.periph("TIM3").new_reg(|reg| {
        reg.name = "OR2".to_string();
        reg.description = "TIM3 option register 2".to_string();
        reg.address_offset = 0x60;
        reg.size = Some(0x20);
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = Some(0x0000);
        reg.new_field(|field| {
            field.name = "ETRSEL".to_string();
            field.description = "ETR source selection".to_string();
            field.bit_offset = Some(14);
            field.bit_width = Some(3);
        });
    });
    dev.periph("TIM3").new_reg(|reg| {
        reg.name = "OR1".to_string();
        reg.description = "TIM3 option register 1".to_string();
        reg.address_offset = 0x50;
        reg.size = Some(0x20);
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = Some(0x0000);
        reg.new_field(|field| {
            field.name = "TI1_RMP".to_string();
            field.description = "Input Capture 1 remap".to_string();
            field.bit_offset = Some(0);
            field.bit_width = Some(2);
        });
    });
    Ok(())
}

fn fix_tim3_2(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").reg("APB1RSTR1").new_field(|field| {
        field.name = "TIM3RST".to_string();
        field.description = "TIM3 timer reset".to_string();
        field.bit_offset = Some(1);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1SMENR1").new_field(|field| {
        field.name = "TIM3SMEN".to_string();
        field.description = "TIM3 timer clocks enable during Sleep and Stop modes".to_string();
        field.bit_offset = Some(1);
        field.bit_width = Some(1);
    });
    Ok(())
}

fn fix_tim8(dev: &mut Device) -> Result<(), Error> {
    dev.periph("TIM8").reg("CCMR1_Input").field("IC2PCS").name = "IC2PSC".to_string();
    dev.periph("TIM8").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    dev.periph("TIM8").reg("OR1").remove_field("ETR_ADC3_RMP");
    dev.periph("TIM8").reg("OR1").remove_field("ETR_ADC2_RMP");
    Ok(())
}

fn fix_tim16(dev: &mut Device) -> Result<(), Error> {
    dev.periph("TIM16").reg("CCMR1_Output").field("OC1M").name = "OC1M0_2".to_string();
    dev.periph("TIM16").reg("CCMR1_Output").field("OC1M_2").name = "OC1M3".to_string();
    dev.periph("TIM16").reg("BDTR").remove_field("BKF");
    dev.periph("TIM16").reg("DIER").remove_field("TDE");
    dev.periph("TIM16").reg("DIER").remove_field("TIE");
    dev.periph("TIM16").reg("SR").remove_field("TIF");
    dev.periph("TIM16").reg("EGR").remove_field("TG");
    Ok(())
}

fn fix_uart4(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").reg("APB1RSTR1").new_field(|field| {
        field.name = "UART4RST".to_string();
        field.description = "UART4 reset".to_string();
        field.bit_offset = Some(19);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1SMENR1").new_field(|field| {
        field.name = "UART4SMEN".to_string();
        field.description = "UART4 clocks enable during Sleep and Stop modes".to_string();
        field.bit_offset = Some(19);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("CCIPR").field("USART4SEL").name = "UART4SEL".to_string();
    Ok(())
}

fn fix_usart1(dev: &mut Device) -> Result<(), Error> {
    copy_field(dev, "USART3", "USART1", "CR3", "UCESM");
    Ok(())
}

fn fix_usart3(dev: &mut Device) -> Result<(), Error> {
    dev.periph("RCC").reg("APB1ENR1").new_field(|field| {
        field.name = "USART3EN".to_string();
        field.description = "USART3 clock enable".to_string();
        field.bit_offset = Some(18);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1RSTR1").new_field(|field| {
        field.name = "USART3RST".to_string();
        field.description = "USART3 reset".to_string();
        field.bit_offset = Some(18);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1SMENR1").new_field(|field| {
        field.name = "USART3SMEN".to_string();
        field.description = "USART3 clocks enable during Sleep and Stop modes".to_string();
        field.bit_offset = Some(18);
        field.bit_width = Some(1);
    });
    dev.periph("USART3").reg("BRR").remove_field("BRR");
    dev.periph("USART3").reg("BRR").new_field(|field| {
        field.name = "DIV_Mantissa".to_string();
        field.description = "DIV_Mantissa".to_string();
        field.bit_offset = Some(4);
        field.bit_width = Some(12);
    });
    dev.periph("USART3").reg("BRR").new_field(|field| {
        field.name = "DIV_Fraction".to_string();
        field.description = "DIV_Fraction".to_string();
        field.bit_offset = Some(0);
        field.bit_width = Some(4);
    });
    Ok(())
}

fn copy_reg(dev: &mut Device, periph_from: &str, periph_to: &str, reg_name: &str) {
    let reg = dev.periph(periph_from).reg(reg_name).clone();
    dev.periph(periph_to).add_reg(reg);
}

fn copy_field(
    dev: &mut Device,
    periph_from: &str,
    periph_to: &str,
    reg_name: &str,
    field_name: &str,
) {
    let field = dev
        .periph(periph_from)
        .reg(reg_name)
        .field(field_name)
        .clone();
    dev.periph(periph_to).reg(reg_name).add_field(field);
}

fn parse_svd(path: &str) -> Result<Device, Error> {
    drone_svd::parse(format!("{}/files/{}", env!("CARGO_MANIFEST_DIR"), path))
}

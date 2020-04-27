//! RCC peripheral patches.

use anyhow::Result;
use drone_svd::{Access, Device};

pub fn fix_1(dev: &mut Device) -> Result<()> {
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

pub fn fix_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLQ3");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLQ2");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLQ1");
    dev.periph("RCC").reg("PLLCFGR").field("PLLQ0").bit_width = Some(4);
    dev.periph("RCC").reg("PLLCFGR").field("PLLQ0").name = "PLLQ".to_string();
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLP1");
    dev.periph("RCC").reg("PLLCFGR").field("PLLP0").bit_width = Some(2);
    dev.periph("RCC").reg("PLLCFGR").field("PLLP0").name = "PLLP".to_string();
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLN8");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLN7");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLN6");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLN5");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLN4");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLN3");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLN2");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLN1");
    dev.periph("RCC").reg("PLLCFGR").field("PLLN0").bit_width = Some(9);
    dev.periph("RCC").reg("PLLCFGR").field("PLLN0").name = "PLLN".to_string();
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLM5");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLM4");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLM3");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLM2");
    dev.periph("RCC").reg("PLLCFGR").remove_field("PLLM1");
    dev.periph("RCC").reg("PLLCFGR").field("PLLM0").bit_width = Some(6);
    dev.periph("RCC").reg("PLLCFGR").field("PLLM0").name = "PLLM".to_string();
    dev.periph("RCC").reg("CFGR").remove_field("SWS1");
    dev.periph("RCC").reg("CFGR").field("SWS0").bit_width = Some(2);
    dev.periph("RCC").reg("CFGR").field("SWS0").name = "SWS".to_string();
    dev.periph("RCC").reg("CFGR").remove_field("SW1");
    dev.periph("RCC").reg("CFGR").field("SW0").bit_width = Some(2);
    dev.periph("RCC").reg("CFGR").field("SW0").name = "SW".to_string();
    Ok(())
}

pub fn fix_3(dev: &mut Device) -> Result<()> {
    for (reg_name, field_name) in
        &[("AHB1ENR", "GPIOIEN"), ("AHB1RSTR", "GPIOIRST"), ("AHB1LPENR", "GPIOILPEN")]
    {
        for (name, description, offset) in &[("GPIOJ", "port J", 1), ("GPIOK", "port K", 2)] {
            let mut field = dev.periph("RCC").reg(reg_name).field(field_name).clone();
            field.name = field.name.replace("GPIOI", name);
            field.description = field.description.replace("port I", description);
            field.bit_offset = Some(field.bit_offset.unwrap() + offset);
            dev.periph("RCC").reg(reg_name).add_field(field);
        }
    }
    Ok(())
}

pub fn fix_4(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("AHB2SMENR").field("ADCFSSMEN").name = "ADCSMEN".to_string();
    Ok(())
}

pub fn fix_5(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1ENR").field("I2C4EN").name = "I2CFMP1EN".to_string();
    dev.periph("RCC").reg("APB1RSTR").field("I2C4RST").name = "I2CFMP1RST".to_string();
    dev.periph("RCC").reg("APB1LPENR").field("I2C4LPEN").name = "I2CFMP1LPEN".to_string();
    Ok(())
}

pub fn fix_6(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").new_reg(|reg| {
        reg.name = "DCKCFGR2".to_string();
        reg.description = "DCKCFGR2 register".to_string();
        reg.address_offset = 0x94;
        reg.size = Some(0x20);
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = Some(0x0000_0000);
        reg.new_field(|field| {
            field.name = "I2CFMP1SEL".to_string();
            field.description = "I2CFMP1SEL".to_string();
            field.bit_offset = Some(22);
            field.bit_width = Some(2);
        });
    });
    Ok(())
}

pub fn fix_7(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("DCKCFGR2").field("I2CFMP1SEL").bit_width = Some(2);
    Ok(())
}

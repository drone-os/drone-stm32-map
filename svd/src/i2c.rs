//! I2C peripheral patches.

use drone_config::Result;
use drone_svd::{Access, Device};

pub fn fix_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1SMENR2").new_field(|field| {
        field.name = "I2C4SMEN".to_string();
        field.description = "I2C4 clocks enable during Sleep and Stop modes".to_string();
        field.bit_offset = Some(1);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_2(dev: &mut Device) -> Result<()> {
    add_fltr(dev, "I2C3")
}

pub fn fix_3(dev: &mut Device) -> Result<()> {
    add_fltr(dev, "I2C2")
}

pub fn fix_4(dev: &mut Device) -> Result<()> {
    fix_i2cfmp1(dev, "I2C4")?;
    dev.periph("I2C4").reg("ICR").field("ALERTC").name = "ALERTCF".to_string();
    dev.periph("I2C4").name = "I2CFMP1".to_string();
    Ok(())
}

pub fn fix_5(dev: &mut Device) -> Result<()> {
    fix_i2cfmp1(dev, "FMPI2C")?;
    dev.periph("FMPI2C").name = "I2CFMP1".to_string();
    Ok(())
}

pub fn fix_6(dev: &mut Device) -> Result<()> {
    dev.periph("I2C4").reg("CR1").remove_field("WUPEN");
    Ok(())
}

fn fix_i2cfmp1(dev: &mut Device, periph: &str) -> Result<()> {
    dev.periph(periph).reg("CR1").field("ADDRE").name = "ADDRIE".to_string();
    dev.periph(periph).reg("CR1").field("TCDMAEN").name = "TXDMAEN".to_string();
    dev.periph(periph).reg("CR2").remove_field("SADD0");
    dev.periph(periph).reg("CR2").remove_field("SADD1_7");
    dev.periph(periph).reg("CR2").remove_field("SADD8_9");
    dev.periph(periph).reg("CR2").new_field(|field| {
        field.name = "SADD".to_string();
        field.description = "Slave address bit".to_string();
        field.bit_offset = Some(0);
        field.bit_width = Some(10);
    });
    dev.periph(periph).reg("OAR1").field("OA1").bit_width = Some(10);
    dev.periph(periph).reg("OAR1").remove_field("OA11_7");
    dev.periph(periph).reg("OAR1").remove_field("OA18_9");
    dev.periph(periph).reg("OAR2").field("OA21_7").name = "OA2".to_string();
    Ok(())
}

fn add_fltr(dev: &mut Device, periph: &str) -> Result<()> {
    dev.periph(periph).new_reg(|reg| {
        reg.name = "FLTR".to_string();
        reg.description = "FLTR register".to_string();
        reg.address_offset = 0x24;
        reg.size = Some(0x20);
        reg.access = Some(Access::ReadWrite);
        reg.reset_value = Some(0x0000);
        reg.new_field(|field| {
            field.name = "DNF".to_string();
            field.description = "Digital noise filter".to_string();
            field.bit_offset = Some(0);
            field.bit_width = Some(4);
        });
        reg.new_field(|field| {
            field.name = "ANOFF".to_string();
            field.description = "Analog noise filter OFF".to_string();
            field.bit_offset = Some(4);
            field.bit_width = Some(1);
        });
    });
    Ok(())
}

//! SPI peripheral patches.

use drone_config::Result;
use drone_svd::Device;

use crate::copy_field;

pub fn fix_astren(dev: &mut Device, periph: &str) -> Result<()> {
    dev.periph(periph).reg("I2SCFGR").new_field(|field| {
        field.name = "ASTREN".to_string();
        field.description = "Asynchronous start enable".to_string();
        field.bit_offset = Some(12);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_spi2_1(dev: &mut Device) -> Result<()> {
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

pub fn fix_spi2_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1ENR1").new_field(|field| {
        field.name = "SPI2EN".to_string();
        field.description = "SPI2 clock enable".to_string();
        field.bit_offset = Some(14);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_spi3_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1SMENR1").field("SP3SMEN").name = "SPI3SMEN".to_string();
    Ok(())
}

pub fn fix_spi3_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1ENR1").field("SP3EN").name = "SPI3EN".to_string();
    dev.periph("RCC").reg("APB1SMENR1").field("SP3SMEN").name = "SPI3SMEN".to_string();
    Ok(())
}

pub fn fix_spi4_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB2ENR").new_field(|field| {
        field.name = "SPI4EN".to_string();
        field.description = "SPI4 clock enable".to_string();
        field.bit_offset = Some(13);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_spi4_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB2RSTR").new_field(|field| {
        field.name = "SPI4RST".to_string();
        field.description = "SPI4 reset".to_string();
        field.bit_offset = Some(13);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_spi4_3(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB2LPENR").new_field(|field| {
        field.name = "SPI4LPEN".to_string();
        field.description = "SPI4 clock enable during Sleep mode".to_string();
        field.bit_offset = Some(13);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_spi5_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB2ENR").new_field(|field| {
        field.name = "SPI5EN".to_string();
        field.description = "SPI5 clock enable".to_string();
        field.bit_offset = Some(20);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_spi5_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB2RSTR").new_field(|field| {
        field.name = "SPI5RST".to_string();
        field.description = "SPI5 reset".to_string();
        field.bit_offset = Some(20);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB2LPENR").new_field(|field| {
        field.name = "SPI5LPEN".to_string();
        field.description = "SPI5 clock enable during Sleep mode".to_string();
        field.bit_offset = Some(20);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_spi6_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB2ENR").new_field(|field| {
        field.name = "SPI6EN".to_string();
        field.description = "SPI6 clock enable".to_string();
        field.bit_offset = Some(21);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_spi6_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB2RSTR").new_field(|field| {
        field.name = "SPI6RST".to_string();
        field.description = "SPI6 reset".to_string();
        field.bit_offset = Some(21);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB2LPENR").new_field(|field| {
        field.name = "SPI6LPEN".to_string();
        field.description = "SPI6 clock enable during Sleep mode".to_string();
        field.bit_offset = Some(21);
        field.bit_width = Some(1);
    });
    Ok(())
}

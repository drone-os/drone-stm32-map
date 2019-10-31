//! UART peripheral patches.

use crate::copy_field;
use anyhow::Result;
use drone_svd::Device;

pub fn fix_usart1(dev: &mut Device) -> Result<()> {
    copy_field(dev, "USART3", "USART1", "CR3", "UCESM");
    Ok(())
}

pub fn fix_usart3(dev: &mut Device) -> Result<()> {
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

pub fn fix_uart4(dev: &mut Device) -> Result<()> {
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

pub fn fix_lpuart1(dev: &mut Device) -> Result<()> {
    copy_field(dev, "USART3", "LPUART1", "CR3", "UCESM");
    Ok(())
}

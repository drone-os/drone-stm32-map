//! UART peripheral patches.

use crate::{copy_field, copy_reg};
use drone_config::Result;
use drone_svd::Device;

pub fn fix_usart1_1(dev: &mut Device) -> Result<()> {
    copy_field(dev, "USART3", "USART1", "CR3", "UCESM");
    Ok(())
}

pub fn fix_usart1_2(dev: &mut Device) -> Result<()> {
    dev.periph("USART1").reg("BRR").remove_field("DIV_Mantissa");
    dev.periph("USART1").reg("BRR").remove_field("DIV_Fraction");
    dev.periph("USART1").reg("BRR").new_field(|field| {
        field.name = "BRR".to_string();
        field.description = "BRR".to_string();
        field.bit_offset = Some(0);
        field.bit_width = Some(16);
    });
    Ok(())
}

pub fn fix_usart3_1(dev: &mut Device) -> Result<()> {
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
    Ok(())
}

pub fn fix_usart3_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1RSTR").field("UART3RST").name = "USART3RST".to_string();
    Ok(())
}

pub fn fix_uart4_1(dev: &mut Device) -> Result<()> {
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

pub fn fix_uart4_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1RSTR").new_field(|field| {
        field.name = "UART4RST".to_string();
        field.description = "UART4 reset".to_string();
        field.bit_offset = Some(19);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1LPENR").new_field(|field| {
        field.name = "UART4LPEN".to_string();
        field.description = "UART4 clock enable during Sleep mode".to_string();
        field.bit_offset = Some(19);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_uart4_3(dev: &mut Device) -> Result<()> {
    copy_reg(dev, "USART3", "UART4", "SR");
    copy_reg(dev, "USART3", "UART4", "DR");
    copy_reg(dev, "USART3", "UART4", "BRR");
    copy_reg(dev, "USART3", "UART4", "CR1");
    copy_reg(dev, "USART3", "UART4", "CR2");
    copy_reg(dev, "USART3", "UART4", "CR3");
    dev.periph("UART4").derived_from = None;
    dev.periph("UART4").description =
        Some("Universal asynchronous receiver transmitter".to_string());
    dev.periph("UART4").reg("SR").remove_field("CTS");
    dev.periph("UART4").reg("CR2").remove_field("CLKEN");
    dev.periph("UART4").reg("CR2").remove_field("CPOL");
    dev.periph("UART4").reg("CR2").remove_field("CPHA");
    dev.periph("UART4").reg("CR2").remove_field("LBCL");
    dev.periph("UART4").reg("CR3").remove_field("CTSIE");
    dev.periph("UART4").reg("CR3").remove_field("CTSE");
    dev.periph("UART4").reg("CR3").remove_field("RTSE");
    dev.periph("UART4").reg("CR3").remove_field("SCEN");
    dev.periph("UART4").reg("CR3").remove_field("NACK");
    Ok(())
}

pub fn fix_uart5_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1RSTR").new_field(|field| {
        field.name = "UART5RST".to_string();
        field.description = "UART5 reset".to_string();
        field.bit_offset = Some(20);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1LPENR").new_field(|field| {
        field.name = "UART5LPEN".to_string();
        field.description = "UART5 clock enable during Sleep mode".to_string();
        field.bit_offset = Some(20);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_uart5_2(dev: &mut Device) -> Result<()> {
    dev.periph("UART5").derived_from = Some("UART4".to_string());
    Ok(())
}

pub fn fix_uart7_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1ENR").new_field(|field| {
        field.name = "UART7EN".to_string();
        field.description = "UART7 clock enable".to_string();
        field.bit_offset = Some(30);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_uart7_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1RSTR").new_field(|field| {
        field.name = "UART7RST".to_string();
        field.description = "UART7 reset".to_string();
        field.bit_offset = Some(30);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1LPENR").new_field(|field| {
        field.name = "UART7LPEN".to_string();
        field.description = "UART7 clock enable during Sleep mode".to_string();
        field.bit_offset = Some(30);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_uart7_3(dev: &mut Device) -> Result<()> {
    dev.periph("UART7").derived_from = Some("UART4".to_string());
    Ok(())
}

pub fn fix_uart8_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1ENR").new_field(|field| {
        field.name = "UART8EN".to_string();
        field.description = "UART8 clock enable".to_string();
        field.bit_offset = Some(31);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_uart8_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1RSTR").new_field(|field| {
        field.name = "UART8RST".to_string();
        field.description = "UART8 reset".to_string();
        field.bit_offset = Some(32);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1LPENR").new_field(|field| {
        field.name = "UART8LPEN".to_string();
        field.description = "UART8 clock enable during Sleep mode".to_string();
        field.bit_offset = Some(31);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_uart8_3(dev: &mut Device) -> Result<()> {
    dev.periph("UART8").derived_from = Some("UART4".to_string());
    Ok(())
}

pub fn fix_uart9_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB2ENR").new_field(|field| {
        field.name = "UART9EN".to_string();
        field.description = "UART9 clock enable".to_string();
        field.bit_offset = Some(6);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB2RSTR").new_field(|field| {
        field.name = "UART9RST".to_string();
        field.description = "UART9 reset".to_string();
        field.bit_offset = Some(6);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB2LPENR").new_field(|field| {
        field.name = "UART9LPEN".to_string();
        field.description = "UART9 clock enable during Sleep mode".to_string();
        field.bit_offset = Some(6);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_uart9_2(dev: &mut Device) -> Result<()> {
    dev.periph("UART9").derived_from = Some("UART4".to_string());
    Ok(())
}

pub fn fix_uart10_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB2ENR").new_field(|field| {
        field.name = "UART10EN".to_string();
        field.description = "UART10 clock enable".to_string();
        field.bit_offset = Some(7);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB2RSTR").new_field(|field| {
        field.name = "UART10RST".to_string();
        field.description = "UART10 reset".to_string();
        field.bit_offset = Some(7);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB2LPENR").new_field(|field| {
        field.name = "UART10LPEN".to_string();
        field.description = "UART10 clock enable during Sleep mode".to_string();
        field.bit_offset = Some(7);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_uart10_2(dev: &mut Device) -> Result<()> {
    dev.periph("UART10").derived_from = Some("UART4".to_string());
    Ok(())
}

pub fn fix_lpuart1(dev: &mut Device) -> Result<()> {
    copy_field(dev, "USART3", "LPUART1", "CR3", "UCESM");
    Ok(())
}

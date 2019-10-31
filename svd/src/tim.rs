//! TIM peripheral patches.

use crate::{copy_field, copy_reg};
use anyhow::Result;
use drone_svd::{Access, Device, Interrupt};

pub fn fix_tim1(dev: &mut Device) -> Result<()> {
    dev.periph("TIM1").reg("CCMR1_Input").field("IC2PCS").name = "IC2PSC".to_string();
    dev.periph("TIM1").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    dev.periph("TIM1").reg("OR1").remove_field("ETR_ADC3_RMP");
    Ok(())
}

pub fn fix_tim2(dev: &mut Device) -> Result<()> {
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

pub fn fix_tim2_and_tim15(dev: &mut Device) -> Result<()> {
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

pub fn add_tim3(dev: &mut Device) -> Result<()> {
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

pub fn fix_tim3_1(dev: &mut Device) -> Result<()> {
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

pub fn fix_tim3_2(dev: &mut Device) -> Result<()> {
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

pub fn fix_tim8(dev: &mut Device) -> Result<()> {
    dev.periph("TIM8").reg("CCMR1_Input").field("IC2PCS").name = "IC2PSC".to_string();
    dev.periph("TIM8").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    dev.periph("TIM8").reg("OR1").remove_field("ETR_ADC3_RMP");
    dev.periph("TIM8").reg("OR1").remove_field("ETR_ADC2_RMP");
    Ok(())
}

pub fn fix_tim16(dev: &mut Device) -> Result<()> {
    dev.periph("TIM16").reg("CCMR1_Output").field("OC1M").name = "OC1M0_2".to_string();
    dev.periph("TIM16").reg("CCMR1_Output").field("OC1M_2").name = "OC1M3".to_string();
    dev.periph("TIM16").reg("BDTR").remove_field("BKF");
    dev.periph("TIM16").reg("DIER").remove_field("TDE");
    dev.periph("TIM16").reg("DIER").remove_field("TIE");
    dev.periph("TIM16").reg("SR").remove_field("TIF");
    dev.periph("TIM16").reg("EGR").remove_field("TG");
    Ok(())
}

pub fn fix_lptim1(dev: &mut Device) -> Result<()> {
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

pub fn fix_lptim2(dev: &mut Device) -> Result<()> {
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

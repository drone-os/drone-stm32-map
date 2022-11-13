//! TIM peripheral patches.

use crate::{copy_field, copy_reg};
use drone_config::Result;
use drone_svd::{Access, Device, Register};

pub fn fix_tim1_1(dev: &mut Device) -> Result<()> {
    dev.periph("TIM1").reg("CCMR1_Input").field("IC2PCS").name = "IC2PSC".to_string();
    dev.periph("TIM1").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    Ok(())
}

pub fn fix_tim1_2(dev: &mut Device) -> Result<()> {
    dev.periph("TIM1").reg("OR1").remove_field("ETR_ADC3_RMP");
    Ok(())
}

pub fn fix_tim2_1(dev: &mut Device) -> Result<()> {
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

pub fn fix_tim2_2(dev: &mut Device) -> Result<()> {
    dev.periph("TIM2").reg("CCMR1_Input").field("IC2PCS").name = "IC2PSC".to_string();
    dev.periph("TIM2").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    dev.periph("TIM2").reg("CCMR2_Output").field("O24CE").name = "OC4CE".to_string();
    Ok(())
}

pub fn fix_tim2_3(dev: &mut Device) -> Result<()> {
    merge_high_low(dev.periph("TIM2").reg("CNT"), "CNT_H", "CNT_L", "CNT")?;
    merge_high_low(dev.periph("TIM2").reg("ARR"), "ARR_H", "ARR_L", "ARR")?;
    merge_high_low(dev.periph("TIM2").reg("CCR1"), "CCR1_H", "CCR1_L", "CCR1")?;
    merge_high_low(dev.periph("TIM2").reg("CCR2"), "CCR2_H", "CCR2_L", "CCR2")?;
    merge_high_low(dev.periph("TIM2").reg("CCR3"), "CCR3_H", "CCR3_L", "CCR3")?;
    merge_high_low(dev.periph("TIM2").reg("CCR4"), "CCR4_H", "CCR4_L", "CCR4")?;
    Ok(())
}

pub fn add_tim3(dev: &mut Device) -> Result<()> {
    dev.new_periph(|peripheral| {
        peripheral.derived_from = Some("TIM2".to_string());
        peripheral.name = "TIM3".to_string();
        peripheral.base_address = 0x4000_0400;
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

pub fn fix_tim3_3(dev: &mut Device) -> Result<()> {
    dev.periph("TIM3").reg("CCMR1_Input").field("IC2PCS").name = "IC2PSC".to_string();
    dev.periph("TIM3").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    dev.periph("TIM3").reg("CCMR2_Output").field("O24CE").name = "OC4CE".to_string();
    merge_high_low(dev.periph("TIM3").reg("CNT"), "CNT_H", "CNT_L", "CNT")?;
    merge_high_low(dev.periph("TIM3").reg("ARR"), "ARR_H", "ARR_L", "ARR")?;
    merge_high_low(dev.periph("TIM3").reg("CCR1"), "CCR1_H", "CCR1_L", "CCR1")?;
    merge_high_low(dev.periph("TIM3").reg("CCR2"), "CCR2_H", "CCR2_L", "CCR2")?;
    merge_high_low(dev.periph("TIM3").reg("CCR3"), "CCR3_H", "CCR3_L", "CCR3")?;
    merge_high_low(dev.periph("TIM3").reg("CCR4"), "CCR4_H", "CCR4_L", "CCR4")?;
    Ok(())
}

pub fn fix_tim5_1(dev: &mut Device) -> Result<()> {
    dev.periph("TIM5").reg("CCMR1_Input").field("IC2PCS").name = "IC2PSC".to_string();
    dev.periph("TIM5").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    dev.periph("TIM5").reg("CCMR2_Output").field("O24CE").name = "OC4CE".to_string();
    merge_high_low(dev.periph("TIM5").reg("CNT"), "CNT_H", "CNT_L", "CNT")?;
    merge_high_low(dev.periph("TIM5").reg("ARR"), "ARR_H", "ARR_L", "ARR")?;
    merge_high_low(dev.periph("TIM5").reg("CCR1"), "CCR1_H", "CCR1_L", "CCR1")?;
    merge_high_low(dev.periph("TIM5").reg("CCR2"), "CCR2_H", "CCR2_L", "CCR2")?;
    merge_high_low(dev.periph("TIM5").reg("CCR3"), "CCR3_H", "CCR3_L", "CCR3")?;
    merge_high_low(dev.periph("TIM5").reg("CCR4"), "CCR4_H", "CCR4_L", "CCR4")?;
    dev.periph("TIM5").reg("OR").field("IT4_RMP").name = "TI4_RMP".to_string();
    Ok(())
}

pub fn fix_tim5_2(dev: &mut Device) -> Result<()> {
    dev.periph("TIM5").reg("SMCR").remove_field("ETP");
    dev.periph("TIM5").reg("SMCR").remove_field("ECE");
    dev.periph("TIM5").reg("SMCR").remove_field("ETPS");
    dev.periph("TIM5").reg("SMCR").remove_field("ETF");
    dev.periph("TIM5").reg("CCMR1_Output").remove_field("OC1CE");
    dev.periph("TIM5").reg("CCMR1_Output").remove_field("OC2CE");
    dev.periph("TIM5").reg("CCMR2_Output").remove_field("OC3CE");
    dev.periph("TIM5").reg("CCMR2_Output").remove_field("OC4CE");
    Ok(())
}

pub fn fix_tim6_1(dev: &mut Device) -> Result<()> {
    dev.periph("TIM6").reg("CR1").remove_field("UIFREMAP");
    dev.periph("TIM6").reg("CNT").remove_field("UIFCPY");
    Ok(())
}

pub fn fix_tim6_2(dev: &mut Device) -> Result<()> {
    let mut field = dev.periph("DBG").reg("DBGMCU_APB1_FZ").field("DBG_TIM5_STOP").clone();
    field.name = "DBG_TIM6_STOP".to_string();
    field.description = "TIM6 counter stopped when core is halted".to_string();
    field.bit_offset = Some(4);
    dev.periph("DBG").reg("DBGMCU_APB1_FZ").add_field(field);
    Ok(())
}

pub fn fix_tim7_1(dev: &mut Device) -> Result<()> {
    dev.periph("TIM7").reg("CR1").remove_field("UIFREMAP");
    dev.periph("TIM7").reg("CNT").remove_field("UIFCPY");
    Ok(())
}

pub fn fix_tim7_2(dev: &mut Device) -> Result<()> {
    let mut field = dev.periph("DBG").reg("DBGMCU_APB1_FZ").field("DBG_TIM6_STOP").clone();
    field.name = "DBG_TIM7_STOP".to_string();
    field.description = "TIM7 counter stopped when core is halted".to_string();
    field.bit_offset = Some(5);
    dev.periph("DBG").reg("DBGMCU_APB1_FZ").add_field(field);
    Ok(())
}

pub fn fix_tim8_1(dev: &mut Device) -> Result<()> {
    dev.periph("TIM8").reg("CCMR1_Input").field("IC2PCS").name = "IC2PSC".to_string();
    dev.periph("TIM8").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    dev.periph("TIM8").reg("OR1").remove_field("ETR_ADC3_RMP");
    dev.periph("TIM8").reg("OR1").remove_field("ETR_ADC2_RMP");
    Ok(())
}

pub fn fix_tim8_2(dev: &mut Device) -> Result<()> {
    let mut field = dev.periph("DBG").reg("DBGMCU_APB2_FZ").field("DBG_TIM1_STOP").clone();
    field.name = "DBG_TIM8_STOP".to_string();
    field.description = "TIM8 counter stopped when core is halted".to_string();
    field.bit_offset = Some(1);
    dev.periph("DBG").reg("DBGMCU_APB2_FZ").add_field(field);
    Ok(())
}

pub fn fix_tim9_1(dev: &mut Device) -> Result<()> {
    dev.periph("TIM9").reg("CCMR1_Input").field("IC2PCS").name = "IC2PSC".to_string();
    dev.periph("TIM9").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    Ok(())
}

pub fn fix_tim10_1(dev: &mut Device) -> Result<()> {
    dev.periph("TIM10").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    Ok(())
}

pub fn fix_tim10_2(dev: &mut Device) -> Result<()> {
    copy_field(dev, "TIM9", "TIM10", "CR1", "OPM");
    Ok(())
}

pub fn fix_tim11_1(dev: &mut Device) -> Result<()> {
    dev.periph("TIM11").reg("CCMR1_Input").field("ICPCS").name = "IC1PSC".to_string();
    dev.periph("TIM11").reg("OR").field("RMP").name = "TI1_RMP".to_string();
    Ok(())
}

pub fn fix_tim11_2(dev: &mut Device) -> Result<()> {
    copy_field(dev, "TIM9", "TIM11", "CR1", "OPM");
    Ok(())
}

pub fn fix_tim12(dev: &mut Device) -> Result<()> {
    let mut field = dev.periph("DBG").reg("DBGMCU_APB1_FZ").field("DBG_TIM7_STOP").clone();
    field.name = "DBG_TIM12_STOP".to_string();
    field.description = "TIM12 counter stopped when core is halted".to_string();
    field.bit_offset = Some(6);
    dev.periph("DBG").reg("DBGMCU_APB1_FZ").add_field(field);
    Ok(())
}

pub fn fix_tim13(dev: &mut Device) -> Result<()> {
    let mut field = dev.periph("DBG").reg("DBGMCU_APB1_FZ").field("DBG_TIM12_STOP").clone();
    field.name = "DBG_TIM13_STOP".to_string();
    field.description = "TIM13 counter stopped when core is halted".to_string();
    field.bit_offset = Some(7);
    dev.periph("DBG").reg("DBGMCU_APB1_FZ").add_field(field);
    Ok(())
}

pub fn fix_tim14(dev: &mut Device) -> Result<()> {
    let mut field = dev.periph("DBG").reg("DBGMCU_APB1_FZ").field("DBG_TIM13_STOP").clone();
    field.name = "DBG_TIM14_STOP".to_string();
    field.description = "TIM14 counter stopped when core is halted".to_string();
    field.bit_offset = Some(8);
    dev.periph("DBG").reg("DBGMCU_APB1_FZ").add_field(field);
    Ok(())
}

pub fn fix_tim15(dev: &mut Device) -> Result<()> {
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

fn merge_high_low(reg: &mut Register, high: &str, low: &str, merged: &str) -> Result<()> {
    let high_offset = reg.field(high).bit_offset.unwrap();
    let high_width = reg.field(high).bit_width.unwrap();
    let low_offset = reg.field(low).bit_offset.unwrap();
    let low_width = reg.field(low).bit_width.unwrap();
    assert!(low_offset + low_width == high_offset);
    reg.remove_field(high);
    reg.field(low).name = merged.to_string();
    reg.field(merged).bit_width = Some(high_width + low_width);
    Ok(())
}

fn add_third_bit(
    dev: &mut Device,
    periph_name: &str,
    reg_name: &str,
    field_name: &str,
    bit_offset: u32,
) {
    let field = dev.periph(periph_name).reg(reg_name).field(field_name);
    field.name = format!("{field_name}0_2");
    let mut field = field.clone();
    field.name = format!("{field_name}3");
    field.bit_offset = Some(bit_offset);
    field.bit_width = Some(1);
    dev.periph(periph_name).reg(reg_name).add_field(field);
}

//! ADC peripheral patches.

use drone_config::Result;
use drone_svd::Device;

pub fn fix_adc1_1(dev: &mut Device) -> Result<()> {
    dev.periph("ADC1").reg("SMPR1").remove_field("SMPx_x");
    for i in 0..=8 {
        dev.periph("ADC1").reg("SMPR1").new_field(|field| {
            field.name = format!("SMP{}", 10 + i);
            field.description = "Channel x sampling time selection".to_string();
            field.bit_offset = Some(i * 3);
            field.bit_width = Some(3);
        });
    }
    dev.periph("ADC1").reg("SMPR2").remove_field("SMPx_x");
    for i in 0..=9 {
        dev.periph("ADC1").reg("SMPR2").new_field(|field| {
            field.name = format!("SMP{i}");
            field.description = "Channel x sampling time selection".to_string();
            field.bit_offset = Some(i * 3);
            field.bit_width = Some(3);
        });
    }
    Ok(())
}

pub fn fix_adc_1(dev: &mut Device) -> Result<()> {
    dev.periph("ADC").reg("SMPR1").remove_field("SMPPLUS");
    for i in 1..=4 {
        dev.periph("ADC").reg(&format!("JDR{i}")).field(&format!("JDATA{i}")).name =
            "JDATA".to_string();
    }
    Ok(())
}

pub fn fix_adc_com(dev: &mut Device) -> Result<()> {
    dev.periph("C_ADC").name = "ADC_Common".to_string();
    Ok(())
}

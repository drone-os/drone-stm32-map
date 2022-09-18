//! EXTI peripheral patches.

use drone_config::Result;
use drone_svd::Device;

pub fn fix_exti_1(dev: &mut Device) -> Result<()> {
    for (reg_name, field_name) in &[("IMR2", "MR39"), ("EMR2", "MR39")] {
        let mut field = dev.periph("EXTI").reg(reg_name).field(field_name).clone();
        field.name = field.name.replace("39", "40");
        field.description = field.description.replace("39", "40");
        field.bit_offset = Some(field.bit_offset.unwrap() + 1);
        dev.periph("EXTI").reg(reg_name).add_field(field);
    }
    Ok(())
}

pub fn fix_exti_2(dev: &mut Device) -> Result<()> {
    for (reg_name, field_name) in &[
        ("IMR", "MR22"),
        ("EMR", "MR22"),
        ("RTSR", "TR22"),
        ("FTSR", "TR22"),
        ("SWIER", "SWIER22"),
        ("PR", "PR22"),
    ] {
        let mut field = dev.periph("EXTI").reg(reg_name).field(field_name).clone();
        field.name = field.name.replace("22", "23");
        field.description = field.description.replace("22", "23");
        field.bit_offset = Some(field.bit_offset.unwrap() + 1);
        dev.periph("EXTI").reg(reg_name).add_field(field);
    }
    Ok(())
}

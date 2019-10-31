//! I2C peripheral patches.

use anyhow::Result;
use drone_svd::Device;

pub fn fix(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1SMENR2").new_field(|field| {
        field.name = "I2C4SMEN".to_string();
        field.description = "I2C4 clocks enable during Sleep and Stop modes".to_string();
        field.bit_offset = Some(1);
        field.bit_width = Some(1);
    });
    Ok(())
}

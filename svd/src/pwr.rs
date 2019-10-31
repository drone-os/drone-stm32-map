//! PWR peripheral patches.

use anyhow::Result;
use drone_svd::Device;

pub fn fix(dev: &mut Device) -> Result<()> {
    dev.periph("PWR").reg("CR1").new_field(|field| {
        field.name = "RRSTP".to_string();
        field.description = "SRAM3 retention in Stop 2 mode".to_string();
        field.bit_offset = Some(4);
        field.bit_width = Some(1);
    });
    Ok(())
}

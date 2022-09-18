//! DMA peripheral patches.

use drone_config::Result;
use drone_svd::{Access, Device};

pub fn fix_dma1_1(dev: &mut Device) -> Result<()> {
    for reg_name in &["S1CR", "S2CR", "S3CR", "S4CR", "S5CR", "S6CR", "S7CR"] {
        dev.periph("DMA1").reg(reg_name).remove_field("ACK");
    }
    Ok(())
}

pub fn fix_dma2_1(dev: &mut Device) -> Result<()> {
    for reg_name in &["S1CR", "S2CR", "S3CR", "S4CR", "S5CR", "S6CR", "S7CR"] {
        dev.periph("DMA2").reg(reg_name).remove_field("ACK");
    }
    Ok(())
}

pub fn fix_dma2_2(dev: &mut Device) -> Result<()> {
    dev.periph("DMA2").reg("LIFCR").access = Some(Access::WriteOnly);
    dev.periph("DMA2").reg("HIFCR").access = Some(Access::WriteOnly);
    Ok(())
}

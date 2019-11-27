//! DMA peripheral patches.

use anyhow::Result;
use drone_svd::{Access, Device, Interrupt};

pub fn fix_dma1_1(dev: &mut Device) -> Result<()> {
    for reg_name in &["S1CR", "S2CR", "S3CR", "S4CR", "S5CR", "S6CR", "S7CR"] {
        dev.periph("DMA1").reg(reg_name).remove_field("ACK");
    }
    Ok(())
}

pub fn fix_dma1_2(dev: &mut Device) -> Result<()> {
    for (number, &position) in [11, 12, 13, 14, 15, 16, 17, 47].iter().enumerate() {
        dev.periph("DMA1").interrupt.push({
            let mut interrupt = Interrupt::default();
            interrupt.name = format!("DMA1_Stream{}", number);
            interrupt.description = format!("DMA1 Stream{} global interrupt", number);
            interrupt.value = position;
            interrupt
        });
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

pub fn fix_dma2_3(dev: &mut Device) -> Result<()> {
    for (number, &position) in [56, 57, 58, 59, 60, 68, 69, 70].iter().enumerate() {
        dev.periph("DMA2").interrupt.push({
            let mut interrupt = Interrupt::default();
            interrupt.name = format!("DMA2_Stream{}", number);
            interrupt.description = format!("DMA2 Stream{} global interrupt", number);
            interrupt.value = position;
            interrupt
        });
    }
    Ok(())
}

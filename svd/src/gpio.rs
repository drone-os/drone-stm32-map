//! GPIO peripheral patches.

use crate::parse_svd;
use anyhow::Result;
use drone_svd::Device;

pub fn add_ascr(dev: &mut Device) -> Result<()> {
    let mut patch = parse_svd("patch/add_gpio_ascr.xml")?;
    let ascr = patch.periph("GPIO").reg("ASCR");
    dev.periph("GPIOA").add_reg(ascr.clone());
    dev.periph("GPIOB").add_reg(ascr.clone());
    dev.periph("GPIOC").add_reg(ascr.clone());
    Ok(())
}

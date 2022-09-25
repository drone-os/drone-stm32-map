//! GPIO peripheral patches.

use drone_config::Result;
use drone_svd::Device;

use crate::parse_svd;

pub fn add_ascr(dev: &mut Device) -> Result<()> {
    let mut patch = parse_svd("patch/add_gpio_ascr.xml")?;
    let ascr = patch.periph("GPIO").reg("ASCR");
    dev.periph("GPIOA").add_reg(ascr.clone());
    dev.periph("GPIOB").add_reg(ascr.clone());
    dev.periph("GPIOC").add_reg(ascr.clone());
    Ok(())
}

//! DMAMUX peripheral patches.

use crate::parse_svd;
use drone_config::Result;
use drone_svd::Device;

pub fn add_dmamux1(dev: &mut Device) -> Result<()> {
    dev.add_periph(parse_svd("patch/add_dmamux.xml")?.periph("DMAMUX1").clone());
    Ok(())
}

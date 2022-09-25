//! DMAMUX peripheral patches.

use drone_config::Result;
use drone_svd::Device;

use crate::parse_svd;

pub fn add_dmamux1(dev: &mut Device) -> Result<()> {
    dev.add_periph(parse_svd("patch/add_dmamux.xml")?.periph("DMAMUX1").clone());
    Ok(())
}

//! RTC peripheral patches.

use drone_config::Result;
use drone_svd::Device;

pub fn fix(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1ENR1").new_field(|field| {
        field.name = "RTCAPBEN".to_string();
        field.description = "RTC APB clock enable".to_string();
        field.bit_offset = Some(10);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1SMENR1").new_field(|field| {
        field.name = "RTCAPBSMEN".to_string();
        field.description = "RTC APB clock enable during Sleep and Stop modes".to_string();
        field.bit_offset = Some(10);
        field.bit_width = Some(1);
    });
    Ok(())
}

//! Analog-to-digital converters dual mode common registers.

#[cfg(any(stm32_mcu = "stm32f303"))]
mod f3;

#[cfg(any(stm32_mcu = "stm32f303"))]
pub use self::f3::*;

//! Analog-to-digital converters dual mode common registers.

#[cfg(any(drone_stm32_map = "stm32f303"))]
mod f3;

#[cfg(any(drone_stm32_map = "stm32f303"))]
pub use self::f3::*;

//! General-purpose timers.

#[cfg(any(stm32_mcu = "stm32f411"))]
mod f4;
#[cfg(any(stm32_mcu = "stm32f411"))]
pub use self::f4::*;

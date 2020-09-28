//! USB on the go full speed.

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
mod f4;
#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
pub use self::f4::*;

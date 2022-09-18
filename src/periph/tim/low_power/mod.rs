//! Low-power timers.

#[cfg(any(
    drone_stm32_map = "stm32l4x1",
    drone_stm32_map = "stm32l4x2",
    drone_stm32_map = "stm32l4x3",
    drone_stm32_map = "stm32l4x5",
    drone_stm32_map = "stm32l4x6",
    drone_stm32_map = "stm32l4r5",
    drone_stm32_map = "stm32l4r7",
    drone_stm32_map = "stm32l4r9",
    drone_stm32_map = "stm32l4s5",
    drone_stm32_map = "stm32l4s7",
    drone_stm32_map = "stm32l4s9"
))]
mod l4_all;

#[cfg(any(
    drone_stm32_map = "stm32l4x1",
    drone_stm32_map = "stm32l4x2",
    drone_stm32_map = "stm32l4x3",
    drone_stm32_map = "stm32l4x5",
    drone_stm32_map = "stm32l4x6",
    drone_stm32_map = "stm32l4r5",
    drone_stm32_map = "stm32l4r7",
    drone_stm32_map = "stm32l4r9",
    drone_stm32_map = "stm32l4s5",
    drone_stm32_map = "stm32l4s7",
    drone_stm32_map = "stm32l4s9"
))]
pub use self::l4_all::*;

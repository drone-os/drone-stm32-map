//! Analog-to-digital converters common registers.

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f410",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469"
))]
mod f4;
#[cfg(any(
    drone_stm32_map = "stm32l4r5",
    drone_stm32_map = "stm32l4r7",
    drone_stm32_map = "stm32l4r9",
    drone_stm32_map = "stm32l4s5",
    drone_stm32_map = "stm32l4s7",
    drone_stm32_map = "stm32l4s9"
))]
mod l4_plus;

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f410",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469"
))]
pub use self::f4::*;
#[cfg(any(
    drone_stm32_map = "stm32l4r5",
    drone_stm32_map = "stm32l4r7",
    drone_stm32_map = "stm32l4r9",
    drone_stm32_map = "stm32l4s5",
    drone_stm32_map = "stm32l4s7",
    drone_stm32_map = "stm32l4s9"
))]
pub use self::l4_plus::*;

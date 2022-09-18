//! Universal Asynchronous Receiver/Transmitter.

#![feature(proc_macro_hygiene)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds, clippy::wildcard_imports)]
#![no_std]

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
    drone_stm32_map = "stm32f469",
))]
pub mod f4;
#[cfg(any(
    drone_stm32_map = "stm32l4x1",
    drone_stm32_map = "stm32l4x2",
    drone_stm32_map = "stm32l4x3",
    drone_stm32_map = "stm32l4x5",
    drone_stm32_map = "stm32l4x6",
))]
pub mod l4;
#[cfg(any(
    drone_stm32_map = "stm32l4r5",
    drone_stm32_map = "stm32l4r7",
    drone_stm32_map = "stm32l4r9",
    drone_stm32_map = "stm32l4s5",
    drone_stm32_map = "stm32l4s7",
    drone_stm32_map = "stm32l4s9",
))]
pub mod l4_plus;

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
    drone_stm32_map = "stm32f469",
))]
pub use self::f4::*;
#[cfg(any(
    drone_stm32_map = "stm32l4x1",
    drone_stm32_map = "stm32l4x2",
    drone_stm32_map = "stm32l4x3",
    drone_stm32_map = "stm32l4x5",
    drone_stm32_map = "stm32l4x6",
))]
pub use self::l4::*;
#[cfg(any(
    drone_stm32_map = "stm32l4r5",
    drone_stm32_map = "stm32l4r7",
    drone_stm32_map = "stm32l4r9",
    drone_stm32_map = "stm32l4s5",
    drone_stm32_map = "stm32l4s7",
    drone_stm32_map = "stm32l4s9",
))]
pub use self::l4_plus::*;

//! General Purpose I/Os.

#![feature(proc_macro_hygiene)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds, clippy::wildcard_imports)]
#![no_std]

pub mod head;
pub mod pin;

#[cfg(any(
    drone_stm32_map = "stm32f100",
    drone_stm32_map = "stm32f101",
    drone_stm32_map = "stm32f102",
    drone_stm32_map = "stm32f103",
    drone_stm32_map = "stm32f107",
))]
mod f1;
#[cfg(any(drone_stm32_map = "stm32f303"))]
mod f3;
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
    drone_stm32_map = "stm32f100",
    drone_stm32_map = "stm32f101",
    drone_stm32_map = "stm32f102",
    drone_stm32_map = "stm32f103",
    drone_stm32_map = "stm32f107",
))]
pub use self::f1::*;
#[cfg(any(drone_stm32_map = "stm32f303"))]
pub use self::f3::*;
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

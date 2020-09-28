//! USB on the go full speed.
//!

#![feature(proc_macro_hygiene)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds, clippy::wildcard_imports)]
#![no_std]

#[cfg(any(
    stm32_mcu = "stm32f411",
))]
pub mod global;
#[cfg(any(
    stm32_mcu = "stm32f411",
))]
pub mod device;
#[cfg(any(
    stm32_mcu = "stm32f411",
))]
pub mod host;
#[cfg(any(
    stm32_mcu = "stm32f411",
))]
pub mod pwrclk;

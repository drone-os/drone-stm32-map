//! Drone for STM32. Mappings.

#![feature(uniform_paths)]
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

pub use drone_stm32_map_pieces::unsafe_stm32_reg_tokens;

/// Memory-mapped register mappings.
pub mod reg {
  pub use drone_stm32_map_pieces::reg::*;
}

/// Interrupt mappings.
pub mod thr {
  pub use drone_stm32_map_pieces::thr::*;
}

/// Resource mappings.
pub mod res {
  pub use drone_cortex_m::map::res::*;
  pub extern crate drone_stm32_map_res_gpio as gpio;
}

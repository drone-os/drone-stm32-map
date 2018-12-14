//! Drone for STM32. Mappings.

#![no_std]
#![warn(missing_docs)]

extern crate drone_cortex_m;
extern crate drone_stm32_map_pieces;

pub use drone_stm32_map_pieces::stm32_reg_index;

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

//! Drone for STM32. Mappings.

#![no_std]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
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

/// Peripheral mappings.
pub mod periph {
  pub use drone_cortex_m::map::periph::*;
  pub extern crate drone_stm32_map_periph_adc as adc;
  pub extern crate drone_stm32_map_periph_dma as dma;
  pub extern crate drone_stm32_map_periph_gpio as gpio;
  pub extern crate drone_stm32_map_periph_i2c as i2c;
  pub extern crate drone_stm32_map_periph_rtc as rtc;
  pub extern crate drone_stm32_map_periph_spi as spi;
  pub extern crate drone_stm32_map_periph_uart as uart;
}

//! Drone for STM32. Mappings.

#![feature(marker_trait_attr)]
#![feature(proc_macro_hygiene)]
#![no_std]
#![warn(missing_docs)]

extern crate drone_core;
extern crate drone_cortex_m;
extern crate drone_stm32_map_piece_1;
extern crate drone_stm32_map_piece_10;
extern crate drone_stm32_map_piece_11;
extern crate drone_stm32_map_piece_12;
extern crate drone_stm32_map_piece_2;
extern crate drone_stm32_map_piece_3;
extern crate drone_stm32_map_piece_4;
extern crate drone_stm32_map_piece_5;
extern crate drone_stm32_map_piece_6;
extern crate drone_stm32_map_piece_7;
extern crate drone_stm32_map_piece_8;
extern crate drone_stm32_map_piece_9;

/// Memory-mapped register mappings.
pub mod reg {
  mod inner {
    pub use drone_stm32_map_piece_1::reg::*;
    pub use drone_stm32_map_piece_10::reg::*;
    pub use drone_stm32_map_piece_11::reg::*;
    pub use drone_stm32_map_piece_12::reg::*;
    pub use drone_stm32_map_piece_2::reg::*;
    pub use drone_stm32_map_piece_3::reg::*;
    pub use drone_stm32_map_piece_4::reg::*;
    pub use drone_stm32_map_piece_5::reg::*;
    pub use drone_stm32_map_piece_6::reg::*;
    pub use drone_stm32_map_piece_7::reg::*;
    pub use drone_stm32_map_piece_8::reg::*;
    pub use drone_stm32_map_piece_9::reg::*;
  }

  pub use drone_core::reg;
  pub use drone_cortex_m::map::reg::*;

  include!(concat!(env!("OUT_DIR"), "/svd_reg_index.rs"));
}

/// Interrupt mappings.
pub mod thr {
  pub use drone_cortex_m::map::thr::*;

  mod map {
    #[allow(unused_imports)]
    use drone_cortex_m::thr::int;
    #[allow(unused_imports)]
    use drone_cortex_m::thr::prelude::*;

    include!(concat!(env!("OUT_DIR"), "/svd_interrupts.rs"));
  }

  pub use self::map::*;
}

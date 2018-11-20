//! Interrupt mappings.

pub use drone_stm32::thr::map::*;

mod map {
  #[allow(unused_imports)]
  use drone_stm32::thr::int;
  #[allow(unused_imports)]
  use drone_stm32::thr::prelude::*;

  include!(concat!(env!("OUT_DIR"), "/svd_interrupts.rs"));
}

pub use self::map::*;

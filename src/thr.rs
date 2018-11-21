//! Interrupt mappings.

pub use drone_cortex_m::thr::map::*;

mod map {
  #[allow(unused_imports)]
  use drone_cortex_m::thr::int;
  #[allow(unused_imports)]
  use drone_cortex_m::thr::prelude::*;

  include!(concat!(env!("OUT_DIR"), "/svd_interrupts.rs"));
}

pub use self::map::*;

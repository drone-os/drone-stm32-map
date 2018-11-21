#![no_std]
#![allow(clippy::precedence, clippy::doc_markdown)]

#[allow(unused_imports)]
#[macro_use]
extern crate drone_core;
extern crate drone_cortex_m;

pub mod reg {
  #[allow(unused_imports)]
  use drone_core::reg::map;
  #[allow(unused_imports)]
  use drone_cortex_m::reg::prelude::*;

  include!(concat!(env!("OUT_DIR"), "/svd_reg_map.rs"));
}

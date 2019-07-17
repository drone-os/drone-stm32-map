#![no_std]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::pedantic)]

pub mod reg {
    #[allow(unused_imports)]
    use drone_core::reg;
    #[allow(unused_imports)]
    use drone_cortex_m::reg::prelude::*;

    include!(concat!(env!("OUT_DIR"), "/svd_regs.rs"));
}

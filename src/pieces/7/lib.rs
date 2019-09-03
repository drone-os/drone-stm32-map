#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::pedantic)]
#![allow(intra_doc_link_resolution_failure)]
#![no_std]

#[doc(hidden)]
pub mod reg {
    #[allow(unused_imports)]
    use drone_core::reg;
    #[allow(unused_imports)]
    use drone_cortex_m::reg::prelude::*;

    include!(concat!(env!("OUT_DIR"), "/svd_regs.rs"));
}

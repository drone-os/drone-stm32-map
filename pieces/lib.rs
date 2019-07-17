//! Drone for STM32. Mappings.

#![feature(marker_trait_attr)]
#![feature(proc_macro_hygiene)]
#![no_std]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

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

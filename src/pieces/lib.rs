//! STM32 peripheral mappings for Drone, an Embedded Operating System.

#![feature(marker_trait_attr)]
#![feature(proc_macro_hygiene)]
#![warn(clippy::pedantic)]
#![allow(broken_intra_doc_links)]
#![no_std]

#[doc(hidden)]
pub mod reg {
    mod inner {
        pub use drone_stm32_map_pieces_1::reg::*;
        pub use drone_stm32_map_pieces_10::reg::*;
        pub use drone_stm32_map_pieces_11::reg::*;
        pub use drone_stm32_map_pieces_12::reg::*;
        pub use drone_stm32_map_pieces_2::reg::*;
        pub use drone_stm32_map_pieces_3::reg::*;
        pub use drone_stm32_map_pieces_4::reg::*;
        pub use drone_stm32_map_pieces_5::reg::*;
        pub use drone_stm32_map_pieces_6::reg::*;
        pub use drone_stm32_map_pieces_7::reg::*;
        pub use drone_stm32_map_pieces_8::reg::*;
        pub use drone_stm32_map_pieces_9::reg::*;
    }

    #[allow(unused_imports)]
    use drone_core::reg;

    include!(concat!(env!("OUT_DIR"), "/svd_reg_index.rs"));
}

#[doc(hidden)]
pub mod thr {
    mod map {
        #[allow(unused_imports)]
        use drone_cortexm::thr;
        #[allow(unused_imports)]
        use drone_cortexm::thr::prelude::*;
    }

    pub use self::map::*;
}

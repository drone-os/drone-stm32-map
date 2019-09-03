//! Direct Memory Access.

#![feature(proc_macro_hygiene)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds)]
#![no_std]

pub mod ch;
#[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
))]
pub mod mux;

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
    /// Generic DMA head peripheral variant.
    pub trait DmaMap {}

    /// Generic DMA head peripheral.
    pub struct DmaPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            DMAEN { RwRwRegFieldBitBand }
        }
        #[cfg(any(
            feature = "stm32l4x1",
            feature = "stm32l4x2",
            feature = "stm32l4x3",
            feature = "stm32l4x5",
            feature = "stm32l4x6",
            feature = "stm32l4r5",
            feature = "stm32l4r7",
            feature = "stm32l4r9",
            feature = "stm32l4s5",
            feature = "stm32l4s7",
            feature = "stm32l4s9"
        ))]
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            DMARST { RwRwRegFieldBitBand }
        }
        #[cfg(any(
            feature = "stm32l4x1",
            feature = "stm32l4x2",
            feature = "stm32l4x3",
            feature = "stm32l4x5",
            feature = "stm32l4x6",
            feature = "stm32l4r5",
            feature = "stm32l4r7",
            feature = "stm32l4r9",
            feature = "stm32l4s5",
            feature = "stm32l4s7",
            feature = "stm32l4s9"
        ))]
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            DMASMEN { RwRwRegFieldBitBand }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_dma {
    (
        $dma_macro_doc:expr,
        $dma_macro:ident,
        $dma_ty_doc:expr,
        $dma_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $dmaen:ident,
        $dmarst:ident,
        $dmasmen:ident,
    ) => {
        periph::map! {
            #[doc = $dma_macro_doc]
            pub macro $dma_macro;

            #[doc = $dma_ty_doc]
            pub struct $dma_ty;

            impl DmaMap for $dma_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    DMAEN { $dmaen }
                }
                #[cfg(any(
                    feature = "stm32l4x1",
                    feature = "stm32l4x2",
                    feature = "stm32l4x3",
                    feature = "stm32l4x5",
                    feature = "stm32l4x6",
                    feature = "stm32l4r5",
                    feature = "stm32l4r7",
                    feature = "stm32l4r9",
                    feature = "stm32l4s5",
                    feature = "stm32l4s7",
                    feature = "stm32l4s9"
                ))]
                BUSRSTR {
                    $busrstr Shared;
                    DMARST { $dmarst }
                }
                #[cfg(any(
                    feature = "stm32l4x1",
                    feature = "stm32l4x2",
                    feature = "stm32l4x3",
                    feature = "stm32l4x5",
                    feature = "stm32l4x6",
                    feature = "stm32l4r5",
                    feature = "stm32l4r7",
                    feature = "stm32l4r9",
                    feature = "stm32l4s5",
                    feature = "stm32l4s7",
                    feature = "stm32l4s9"
                ))]
                BUSSMENR {
                    $bussmenr Shared;
                    DMASMEN { $dmasmen }
                }
            }
        }
    };
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_dma! {
    "Extracts DMA1 head register tokens.",
    periph_dma1,
    "DMA1 head peripheral variant.",
    Dma1,
    AHBENR,
    AHBRSTR,
    AHBSMENR,
    DMA1EN,
    DMA1RST,
    DMA1SMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_dma! {
    "Extracts DMA2 head register tokens.",
    periph_dma2,
    "DMA2 head peripheral variant.",
    Dma2,
    AHBENR,
    AHBRSTR,
    AHBSMENR,
    DMA2EN,
    DMA2RST,
    DMA2SMEN,
}

#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6",
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
))]
map_dma! {
    "Extracts DMA1 head register tokens.",
    periph_dma1,
    "DMA1 head peripheral variant.",
    Dma1,
    AHB1ENR,
    AHB1RSTR,
    AHB1SMENR,
    DMA1EN,
    DMA1RST,
    DMA1SMEN,
}

#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6",
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
))]
map_dma! {
    "Extracts DMA2 head register tokens.",
    periph_dma2,
    "DMA2 head peripheral variant.",
    Dma2,
    AHB1ENR,
    AHB1RSTR,
    AHB1SMENR,
    DMA2EN,
    DMA2RST,
    DMA2SMEN,
}

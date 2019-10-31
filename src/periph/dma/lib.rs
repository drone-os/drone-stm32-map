//! Direct Memory Access.

#![feature(proc_macro_hygiene)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds)]
#![no_std]

pub mod ch;
#[cfg(any(
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
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
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            DMARST { RwRwRegFieldBitBand }
        }
        #[cfg(any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6",
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
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
                    stm32_mcu = "stm32f401",
                    stm32_mcu = "stm32f405",
                    stm32_mcu = "stm32f407",
                    stm32_mcu = "stm32f410",
                    stm32_mcu = "stm32f411",
                    stm32_mcu = "stm32f412",
                    stm32_mcu = "stm32f413",
                    stm32_mcu = "stm32f427",
                    stm32_mcu = "stm32f429",
                    stm32_mcu = "stm32f446",
                    stm32_mcu = "stm32f469",
                    stm32_mcu = "stm32l4x1",
                    stm32_mcu = "stm32l4x2",
                    stm32_mcu = "stm32l4x3",
                    stm32_mcu = "stm32l4x5",
                    stm32_mcu = "stm32l4x6",
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                BUSRSTR {
                    $busrstr Shared;
                    DMARST { $dmarst }
                }
                #[cfg(any(
                    stm32_mcu = "stm32f401",
                    stm32_mcu = "stm32f405",
                    stm32_mcu = "stm32f407",
                    stm32_mcu = "stm32f410",
                    stm32_mcu = "stm32f411",
                    stm32_mcu = "stm32f412",
                    stm32_mcu = "stm32f413",
                    stm32_mcu = "stm32f427",
                    stm32_mcu = "stm32f429",
                    stm32_mcu = "stm32f446",
                    stm32_mcu = "stm32f469",
                    stm32_mcu = "stm32l4x1",
                    stm32_mcu = "stm32l4x2",
                    stm32_mcu = "stm32l4x3",
                    stm32_mcu = "stm32l4x5",
                    stm32_mcu = "stm32l4x6",
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
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
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
    stm32_mcu = "stm32l4x1",
    stm32_mcu = "stm32l4x2",
    stm32_mcu = "stm32l4x3",
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
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
    stm32_mcu = "stm32l4x1",
    stm32_mcu = "stm32l4x2",
    stm32_mcu = "stm32l4x3",
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
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

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469"
))]
map_dma! {
    "Extracts DMA1 head register tokens.",
    periph_dma1,
    "DMA1 head peripheral variant.",
    Dma1,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    DMA1EN,
    DMA1RST,
    DMA1LPEN,
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469"
))]
map_dma! {
    "Extracts DMA2 head register tokens.",
    periph_dma2,
    "DMA2 head peripheral variant.",
    Dma2,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    DMA2EN,
    DMA2RST,
    DMA2LPEN,
}

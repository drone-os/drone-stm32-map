//! Direct Memory Access.
//!
//! For STM32L4+ series of ultra-low-power MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

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
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            DMARST { RwRwRegFieldBitBand }
        }
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
                BUSRSTR {
                    $busrstr Shared;
                    DMARST { $dmarst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    DMASMEN { $dmasmen }
                }
            }
        }
    };
}

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

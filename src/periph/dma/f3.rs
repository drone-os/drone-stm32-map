//! Direct Memory Access
//!
//! For STM32F3 Series of mixed-signal MCUs with DSP and FPU instructions.

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
        $dmaen:ident,
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
            }
        }
    };
}

map_dma! {
    "Extracts DMA1 head register tokens.",
    periph_dma1,
    "DMA1 head peripheral variant.",
    Dma1,
    AHBENR,
    DMAEN,
}

map_dma! {
    "Extracts DMA2 head register tokens.",
    periph_dma2,
    "DMA2 head peripheral variant.",
    Dma2,
    AHBENR,
    DMA2EN,
}

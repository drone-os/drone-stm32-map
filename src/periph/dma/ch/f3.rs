//! DMA channels
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic DMA channel peripheral variant.
    pub trait DmaChMap {
        /// DMA head peripheral variant.
        type DmaMap: super::super::DmaMap;
    }

    /// Generic DMA channel peripheral.
    pub struct DmaChPeriph;

    DMA {
        ISR {
            0x20 RoRegBitBand Shared;
            GIF { RoRoRegFieldBitBand }
            TCIF { RoRoRegFieldBitBand }
            HTIF { RoRoRegFieldBitBand }
            TEIF { RoRoRegFieldBitBand }
        }
        IFCR {
            0x20 WoRegBitBand Shared;
            CGIF { WoWoRegFieldBitBand }
            CTCIF { WoWoRegFieldBitBand }
            CHTIF { WoWoRegFieldBitBand }
            CTEIF { WoWoRegFieldBitBand }
        }
        CCR {
            0x20 RwRegBitBand;
            MEM2MEM { RwRwRegFieldBitBand }
            PL { RwRwRegFieldBits }
            MSIZE { RwRwRegFieldBits }
            PSIZE { RwRwRegFieldBits }
            MINC { RwRwRegFieldBitBand }
            PINC { RwRwRegFieldBitBand }
            CIRC { RwRwRegFieldBitBand }
            DIR { RwRwRegFieldBitBand }
            TEIE { RwRwRegFieldBitBand }
            HTIE { RwRwRegFieldBitBand }
            TCIE { RwRwRegFieldBitBand }
            EN { RwRwRegFieldBitBand }
        }
        CNDTR {
            0x20 RwRegBitBand;
            NDT { RwRwRegFieldBits }
        }
        CPAR {
            0x20 RwRegBitBand;
            PA { RwRwRegFieldBits }
        }
        CMAR {
            0x20 RwRegBitBand;
            MA { RwRwRegFieldBits }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_dma_ch {
    (
        $dma_ch_macro_doc:expr,
        $dma_ch_macro:ident,
        $dma_ch_ty_doc:expr,
        $dma_ch_ty:ident,
        $dma_ty:ident,
        $dma:ident,
        $isr:ident,
        $teif:ident,
        $htif:ident,
        $tcif:ident,
        $gif:ident,
        $ifcr:ident,
        $cteif:ident,
        $chtif:ident,
        $ctcif:ident,
        $cgif:ident,
        $ccr:ident,
        $cndtr:ident,
        $cpar:ident,
        $cmar:ident,
    ) => {
        periph::map! {
            #[doc = $dma_ch_macro_doc]
            pub macro $dma_ch_macro;

            #[doc = $dma_ch_ty_doc]
            pub struct $dma_ch_ty;

            impl DmaChMap for $dma_ch_ty {
                type DmaMap = super::super::$dma_ty;
            }

            drone_stm32_map_pieces::reg;
            crate::ch;

            DMA {
                $dma;
                ISR {
                    $isr Shared;
                    TEIF { $teif }
                    HTIF { $htif }
                    TCIF { $tcif }
                    GIF { $gif }
                }
                IFCR {
                    $ifcr Shared;
                    CGIF { $cgif }
                    CTCIF { $ctcif }
                    CHTIF { $chtif }
                    CTEIF { $cteif }
                }
                CCR {
                    $ccr;
                    MEM2MEM { MEM2MEM }
                    PL { PL }
                    MSIZE { MSIZE }
                    PSIZE { PSIZE }
                    MINC { MINC }
                    PINC { PINC }
                    CIRC { CIRC }
                    DIR { DIR }
                    TEIE { TEIE }
                    HTIE { HTIE }
                    TCIE { TCIE }
                    EN { EN }
                }
                CNDTR {
                    $cndtr;
                    NDT { NDT }
                }
                CPAR {
                    $cpar;
                    PA { PA }
                }
                CMAR {
                    $cmar;
                    MA { MA }
                }
            }
        }
    };
}

map_dma_ch! {
    "Extracts DMA1 channel 1 register tokens.",
    periph_dma1_ch1,
    "DMA1 channel 1 peripheral variant.",
    Dma1Ch1,
    Dma1,
    DMA1,
    ISR,
    TEIF1,
    HTIF1,
    TCIF1,
    GIF1,
    IFCR,
    CTEIF1,
    CHTIF1,
    CTCIF1,
    CGIF1,
    CCR1,
    CNDTR1,
    CPAR1,
    CMAR1,
}

map_dma_ch! {
    "Extracts DMA1 channel 2 register tokens.",
    periph_dma1_ch2,
    "DMA1 channel 2 peripheral variant.",
    Dma1Ch2,
    Dma1,
    DMA1,
    ISR,
    TEIF2,
    HTIF2,
    TCIF2,
    GIF2,
    IFCR,
    CTEIF2,
    CHTIF2,
    CTCIF2,
    CGIF2,
    CCR2,
    CNDTR2,
    CPAR2,
    CMAR2,
}

map_dma_ch! {
    "Extracts DMA1 channel 3 register tokens.",
    periph_dma1_ch3,
    "DMA1 channel 3 peripheral variant.",
    Dma1Ch3,
    Dma1,
    DMA1,
    ISR,
    TEIF3,
    HTIF3,
    TCIF3,
    GIF3,
    IFCR,
    CTEIF3,
    CHTIF3,
    CTCIF3,
    CGIF3,
    CCR3,
    CNDTR3,
    CPAR3,
    CMAR3,
}

map_dma_ch! {
    "Extracts DMA1 channel 4 register tokens.",
    periph_dma1_ch4,
    "DMA1 channel 4 peripheral variant.",
    Dma1Ch4,
    Dma1,
    DMA1,
    ISR,
    TEIF4,
    HTIF4,
    TCIF4,
    GIF4,
    IFCR,
    CTEIF4,
    CHTIF4,
    CTCIF4,
    CGIF4,
    CCR4,
    CNDTR4,
    CPAR4,
    CMAR4,
}

map_dma_ch! {
    "Extracts DMA1 channel 5 register tokens.",
    periph_dma1_ch5,
    "DMA1 channel 5 peripheral variant.",
    Dma1Ch5,
    Dma1,
    DMA1,
    ISR,
    TEIF5,
    HTIF5,
    TCIF5,
    GIF5,
    IFCR,
    CTEIF5,
    CHTIF5,
    CTCIF5,
    CGIF5,
    CCR5,
    CNDTR5,
    CPAR5,
    CMAR5,
}

map_dma_ch! {
    "Extracts DMA1 channel 6 register tokens.",
    periph_dma1_ch6,
    "DMA1 channel 6 peripheral variant.",
    Dma1Ch6,
    Dma1,
    DMA1,
    ISR,
    TEIF6,
    HTIF6,
    TCIF6,
    GIF6,
    IFCR,
    CTEIF6,
    CHTIF6,
    CTCIF6,
    CGIF6,
    CCR6,
    CNDTR6,
    CPAR6,
    CMAR6,
}

map_dma_ch! {
    "Extracts DMA1 channel 7 register tokens.",
    periph_dma1_ch7,
    "DMA1 channel 7 peripheral variant.",
    Dma1Ch7,
    Dma1,
    DMA1,
    ISR,
    TEIF7,
    HTIF7,
    TCIF7,
    GIF7,
    IFCR,
    CTEIF7,
    CHTIF7,
    CTCIF7,
    CGIF7,
    CCR7,
    CNDTR7,
    CPAR7,
    CMAR7,
}

map_dma_ch! {
    "Extracts DMA2 channel 1 register tokens.",
    periph_dma2_ch1,
    "DMA2 channel 1 peripheral variant.",
    Dma2Ch1,
    Dma2,
    DMA2,
    ISR,
    TEIF1,
    HTIF1,
    TCIF1,
    GIF1,
    IFCR,
    CTEIF1,
    CHTIF1,
    CTCIF1,
    CGIF1,
    CCR1,
    CNDTR1,
    CPAR1,
    CMAR1,
}

map_dma_ch! {
    "Extracts DMA2 channel 2 register tokens.",
    periph_dma2_ch2,
    "DMA2 channel 2 peripheral variant.",
    Dma2Ch2,
    Dma2,
    DMA2,
    ISR,
    TEIF2,
    HTIF2,
    TCIF2,
    GIF2,
    IFCR,
    CTEIF2,
    CHTIF2,
    CTCIF2,
    CGIF2,
    CCR2,
    CNDTR2,
    CPAR2,
    CMAR2,
}

map_dma_ch! {
    "Extracts DMA2 channel 3 register tokens.",
    periph_dma2_ch3,
    "DMA2 channel 3 peripheral variant.",
    Dma2Ch3,
    Dma2,
    DMA2,
    ISR,
    TEIF3,
    HTIF3,
    TCIF3,
    GIF3,
    IFCR,
    CTEIF3,
    CHTIF3,
    CTCIF3,
    CGIF3,
    CCR3,
    CNDTR3,
    CPAR3,
    CMAR3,
}

map_dma_ch! {
    "Extracts DMA2 channel 4 register tokens.",
    periph_dma2_ch4,
    "DMA2 channel 4 peripheral variant.",
    Dma2Ch4,
    Dma2,
    DMA2,
    ISR,
    TEIF4,
    HTIF4,
    TCIF4,
    GIF4,
    IFCR,
    CTEIF4,
    CHTIF4,
    CTCIF4,
    CGIF4,
    CCR4,
    CNDTR4,
    CPAR4,
    CMAR4,
}

map_dma_ch! {
    "Extracts DMA2 channel 5 register tokens.",
    periph_dma2_ch5,
    "DMA2 channel 5 peripheral variant.",
    Dma2Ch5,
    Dma2,
    DMA2,
    ISR,
    TEIF5,
    HTIF5,
    TCIF5,
    GIF5,
    IFCR,
    CTEIF5,
    CHTIF5,
    CTCIF5,
    CGIF5,
    CCR5,
    CNDTR5,
    CPAR5,
    CMAR5,
}

map_dma_ch! {
    "Extracts DMA2 channel 6 register tokens.",
    periph_dma2_ch6,
    "DMA2 channel 6 peripheral variant.",
    Dma2Ch6,
    Dma2,
    DMA2,
    ISR,
    TEIF6,
    HTIF6,
    TCIF6,
    GIF6,
    IFCR,
    CTEIF6,
    CHTIF6,
    CTCIF6,
    CGIF6,
    CCR6,
    CNDTR6,
    CPAR6,
    CMAR6,
}

map_dma_ch! {
    "Extracts DMA2 channel 7 register tokens.",
    periph_dma2_ch7,
    "DMA2 channel 7 peripheral variant.",
    Dma2Ch7,
    Dma2,
    DMA2,
    ISR,
    TEIF7,
    HTIF7,
    TCIF7,
    GIF7,
    IFCR,
    CTEIF7,
    CHTIF7,
    CTCIF7,
    CGIF7,
    CCR7,
    CNDTR7,
    CPAR7,
    CMAR7,
}

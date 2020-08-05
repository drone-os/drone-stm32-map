//! DMA channels.
//!
//! For STM32L4+ series of ultra-low-power MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic DMA channel peripheral variant.
    pub trait DmaChMap {
        /// DMA head peripheral variant.
        type DmaMap: super::super::DmaMap;

        /// DMAMUX channel peripheral variant.
        type DmamuxChMap: super::super::mux::ch::DmamuxChMap;
    }

    /// Generic DMA channel peripheral.
    pub struct DmaChPeriph;

    DMA {
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
        CMAR {
            0x20 RwRegBitBand;
            MA { RwRwRegFieldBits }
        }
        CNDTR {
            0x20 RwRegBitBand;
            NDT { RwRwRegFieldBits }
        }
        CPAR {
            0x20 RwRegBitBand;
            PA { RwRwRegFieldBits }
        }
        IFCR {
            0x20 WoRegBitBand Shared;
            CGIF { WoWoRegFieldBitBand }
            CHTIF { WoWoRegFieldBitBand }
            CTCIF { WoWoRegFieldBitBand }
            CTEIF { WoWoRegFieldBitBand }
        }
        ISR {
            0x20 RoRegBitBand Shared;
            GIF { RoRoRegFieldBitBand }
            HTIF { RoRoRegFieldBitBand }
            TCIF { RoRoRegFieldBitBand }
            TEIF { RoRoRegFieldBitBand }
        }
    }
}

macro_rules! map_dma_ch {
    (
        $dma_ch_macro_doc:expr,
        $dma_ch_macro:ident,
        $dma_ch_ty_doc:expr,
        $dma_ch_ty:ident,
        $dma_ty:ident,
        $dmamux_ch_ty:ident,
        $dma:ident,
        $ccr:ident,
        $cmar:ident,
        $cndtr:ident,
        $cpar:ident,
        $ifcr:ident,
        $cgif:ident,
        $chtif:ident,
        $ctcif:ident,
        $cteif:ident,
        $isr:ident,
        $gif:ident,
        $htif:ident,
        $tcif:ident,
        $teif:ident,
    ) => {
        periph::map! {
            #[doc = $dma_ch_macro_doc]
            pub macro $dma_ch_macro;

            #[doc = $dma_ch_ty_doc]
            pub struct $dma_ch_ty;

            impl DmaChMap for $dma_ch_ty {
                type DmaMap = super::super::$dma_ty;
                type DmamuxChMap = super::super::mux::ch::$dmamux_ch_ty;
            }

            drone_stm32_map_pieces::reg;
            crate::ch;

            DMA {
                $dma;
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
                CMAR {
                    $cmar;
                    MA { MA }
                }
                CNDTR {
                    $cndtr;
                    NDT { NDT }
                }
                CPAR {
                    $cpar;
                    PA { PA }
                }
                IFCR {
                    $ifcr Shared;
                    CGIF { $cgif }
                    CHTIF { $chtif }
                    CTCIF { $ctcif }
                    CTEIF { $cteif }
                }
                ISR {
                    $isr Shared;
                    GIF { $gif }
                    HTIF { $htif }
                    TCIF { $tcif }
                    TEIF { $teif }
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
    Dmamux1Ch0,
    DMA1,
    CCR1,
    CMAR1,
    CNDTR1,
    CPAR1,
    IFCR,
    CGIF1,
    CHTIF1,
    CTCIF1,
    CTEIF1,
    ISR,
    GIF1,
    HTIF1,
    TCIF1,
    TEIF1,
}

map_dma_ch! {
    "Extracts DMA1 channel 2 register tokens.",
    periph_dma1_ch2,
    "DMA1 channel 2 peripheral variant.",
    Dma1Ch2,
    Dma1,
    Dmamux1Ch1,
    DMA1,
    CCR2,
    CMAR2,
    CNDTR2,
    CPAR2,
    IFCR,
    CGIF2,
    CHTIF2,
    CTCIF2,
    CTEIF2,
    ISR,
    GIF2,
    HTIF2,
    TCIF2,
    TEIF2,
}

map_dma_ch! {
    "Extracts DMA1 channel 3 register tokens.",
    periph_dma1_ch3,
    "DMA1 channel 3 peripheral variant.",
    Dma1Ch3,
    Dma1,
    Dmamux1Ch2,
    DMA1,
    CCR3,
    CMAR3,
    CNDTR3,
    CPAR3,
    IFCR,
    CGIF3,
    CHTIF3,
    CTCIF3,
    CTEIF3,
    ISR,
    GIF3,
    HTIF3,
    TCIF3,
    TEIF3,
}

map_dma_ch! {
    "Extracts DMA1 channel 4 register tokens.",
    periph_dma1_ch4,
    "DMA1 channel 4 peripheral variant.",
    Dma1Ch4,
    Dma1,
    Dmamux1Ch3,
    DMA1,
    CCR4,
    CMAR4,
    CNDTR4,
    CPAR4,
    IFCR,
    CGIF4,
    CHTIF4,
    CTCIF4,
    CTEIF4,
    ISR,
    GIF4,
    HTIF4,
    TCIF4,
    TEIF4,
}

map_dma_ch! {
    "Extracts DMA1 channel 5 register tokens.",
    periph_dma1_ch5,
    "DMA1 channel 5 peripheral variant.",
    Dma1Ch5,
    Dma1,
    Dmamux1Ch4,
    DMA1,
    CCR5,
    CMAR5,
    CNDTR5,
    CPAR5,
    IFCR,
    CGIF5,
    CHTIF5,
    CTCIF5,
    CTEIF5,
    ISR,
    GIF5,
    HTIF5,
    TCIF5,
    TEIF5,
}

map_dma_ch! {
    "Extracts DMA1 channel 6 register tokens.",
    periph_dma1_ch6,
    "DMA1 channel 6 peripheral variant.",
    Dma1Ch6,
    Dma1,
    Dmamux1Ch5,
    DMA1,
    CCR6,
    CMAR6,
    CNDTR6,
    CPAR6,
    IFCR,
    CGIF6,
    CHTIF6,
    CTCIF6,
    CTEIF6,
    ISR,
    GIF6,
    HTIF6,
    TCIF6,
    TEIF6,
}

map_dma_ch! {
    "Extracts DMA1 channel 7 register tokens.",
    periph_dma1_ch7,
    "DMA1 channel 7 peripheral variant.",
    Dma1Ch7,
    Dma1,
    Dmamux1Ch6,
    DMA1,
    CCR7,
    CMAR7,
    CNDTR7,
    CPAR7,
    IFCR,
    CGIF7,
    CHTIF7,
    CTCIF7,
    CTEIF7,
    ISR,
    GIF7,
    HTIF7,
    TCIF7,
    TEIF7,
}

map_dma_ch! {
    "Extracts DMA2 channel 1 register tokens.",
    periph_dma2_ch1,
    "DMA2 channel 1 peripheral variant.",
    Dma2Ch1,
    Dma2,
    Dmamux1Ch7,
    DMA2,
    CCR1,
    CMAR1,
    CNDTR1,
    CPAR1,
    IFCR,
    CGIF1,
    CHTIF1,
    CTCIF1,
    CTEIF1,
    ISR,
    GIF1,
    HTIF1,
    TCIF1,
    TEIF1,
}

map_dma_ch! {
    "Extracts DMA2 channel 2 register tokens.",
    periph_dma2_ch2,
    "DMA2 channel 2 peripheral variant.",
    Dma2Ch2,
    Dma2,
    Dmamux1Ch8,
    DMA2,
    CCR2,
    CMAR2,
    CNDTR2,
    CPAR2,
    IFCR,
    CGIF2,
    CHTIF2,
    CTCIF2,
    CTEIF2,
    ISR,
    GIF2,
    HTIF2,
    TCIF2,
    TEIF2,
}

map_dma_ch! {
    "Extracts DMA2 channel 3 register tokens.",
    periph_dma2_ch3,
    "DMA2 channel 3 peripheral variant.",
    Dma2Ch3,
    Dma2,
    Dmamux1Ch9,
    DMA2,
    CCR3,
    CMAR3,
    CNDTR3,
    CPAR3,
    IFCR,
    CGIF3,
    CHTIF3,
    CTCIF3,
    CTEIF3,
    ISR,
    GIF3,
    HTIF3,
    TCIF3,
    TEIF3,
}

map_dma_ch! {
    "Extracts DMA2 channel 4 register tokens.",
    periph_dma2_ch4,
    "DMA2 channel 4 peripheral variant.",
    Dma2Ch4,
    Dma2,
    Dmamux1Ch10,
    DMA2,
    CCR4,
    CMAR4,
    CNDTR4,
    CPAR4,
    IFCR,
    CGIF4,
    CHTIF4,
    CTCIF4,
    CTEIF4,
    ISR,
    GIF4,
    HTIF4,
    TCIF4,
    TEIF4,
}

map_dma_ch! {
    "Extracts DMA2 channel 5 register tokens.",
    periph_dma2_ch5,
    "DMA2 channel 5 peripheral variant.",
    Dma2Ch5,
    Dma2,
    Dmamux1Ch11,
    DMA2,
    CCR5,
    CMAR5,
    CNDTR5,
    CPAR5,
    IFCR,
    CGIF5,
    CHTIF5,
    CTCIF5,
    CTEIF5,
    ISR,
    GIF5,
    HTIF5,
    TCIF5,
    TEIF5,
}

map_dma_ch! {
    "Extracts DMA2 channel peripheral varian tokens.",
    periph_dma2_ch6,
    "DMA2 channel 6.",
    Dma2Ch6,
    Dma2,
    Dmamux1Ch12,
    DMA2,
    CCR6,
    CMAR6,
    CNDTR6,
    CPAR6,
    IFCR,
    CGIF6,
    CHTIF6,
    CTCIF6,
    CTEIF6,
    ISR,
    GIF6,
    HTIF6,
    TCIF6,
    TEIF6,
}

map_dma_ch! {
    "Extracts DMA2 channel 7 register tokens.",
    periph_dma2_ch7,
    "DMA2 channel 7 peripheral variant.",
    Dma2Ch7,
    Dma2,
    Dmamux1Ch13,
    DMA2,
    CCR7,
    CMAR7,
    CNDTR7,
    CPAR7,
    IFCR,
    CGIF7,
    CHTIF7,
    CTCIF7,
    CTEIF7,
    ISR,
    GIF7,
    HTIF7,
    TCIF7,
    TEIF7,
}

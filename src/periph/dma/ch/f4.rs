//! DMA channels
//! for STM32F4 series of high-performance MCUs with DSP and FPU instructions.

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
        CCR {
            0x20 RwRegBitBand;
            CHSEL { RwRwRegFieldBits }
            MBURST { RwRwRegFieldBits }
            PBURST { RwRwRegFieldBits }
            CT { RwRwRegFieldBitBand }
            DBM { RwRwRegFieldBitBand }
            PL { RwRwRegFieldBits }
            PINCOS { RwRwRegFieldBitBand }
            MSIZE { RwRwRegFieldBits }
            PSIZE { RwRwRegFieldBits }
            MINC { RwRwRegFieldBitBand }
            PINC { RwRwRegFieldBitBand }
            CIRC { RwRwRegFieldBitBand }
            DIR { RwRwRegFieldBits }
            PFCTRL { RwRwRegFieldBitBand }
            TEIE { RwRwRegFieldBitBand }
            HTIE { RwRwRegFieldBitBand }
            TCIE { RwRwRegFieldBitBand }
            DMEIE { RwRwRegFieldBitBand }
            EN { RwRwRegFieldBitBand }
        }
        CFCR {
            0x20 RwRegBitBand;
            FEIE { RwRwRegFieldBitBand }
            FS { RoRwRegFieldBits }
            DMDIS { RwRwRegFieldBitBand }
            FTH { RwRwRegFieldBits }
        }
        CM0AR {
            0x20 RwRegBitBand;
            M0A { RwRwRegFieldBits }
        }
        CM1AR {
            0x20 RwRegBitBand;
            M1A { RwRwRegFieldBits }
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
            CDMEIF { WoWoRegFieldBitBand }
            CFEIF { WoWoRegFieldBitBand }
            CHTIF { WoWoRegFieldBitBand }
            CTCIF { WoWoRegFieldBitBand }
            CTEIF { WoWoRegFieldBitBand }
        }
        ISR {
            0x20 RoRegBitBand Shared;
            DMEIF { RoRoRegFieldBitBand }
            FEIF { RoRoRegFieldBitBand }
            HTIF { RoRoRegFieldBitBand }
            TCIF { RoRoRegFieldBitBand }
            TEIF { RoRoRegFieldBitBand }
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
        $dmamux_ch_ty:ident,
        $dma:ident,
        $ccr:ident,
        $cfcr:ident,
        $cm0ar:ident,
        $cm1ar:ident,
        $cndtr:ident,
        $cpar:ident,
        $cs:ident,
        $ifcr:ident,
        $cdmeif:ident,
        $cfeif:ident,
        $cgif:ident,
        $chtif:ident,
        $ctcif:ident,
        $cteif:ident,
        $isr:ident,
        $dmeif:ident,
        $feif:ident,
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

            }

            drone_stm32_map_pieces::reg;
            crate::ch;

            DMA {
                $dma;
                CCR {
                    $ccr;
                    CHSEL { CHSEL }
                    MBURST { MBURST }
                    PBURST { PBURST }
                    CT { CT }
                    DBM { DBM }
                    PL { PL }
                    PINCOS { PINCOS }
                    MSIZE { MSIZE }
                    PSIZE { PSIZE }
                    MINC { MINC }
                    PINC { PINC }
                    CIRC { CIRC }
                    DIR { DIR }
                    PFCTRL { PFCTRL }
                    TEIE { TEIE }
                    HTIE { HTIE }
                    TCIE { TCIE }
                    DMEIE { DMEIE }
                    EN { EN }
                }
                CFCR {
                    $cfcr;
                    FEIE { FEIE }
                    FS { FS }
                    DMDIS { DMDIS }
                    FTH { FTH }
                }
                CM0AR {
                    $cm0ar;
                    M0A { M0A }
                }
                CM1AR {
                    $cm1ar;
                    M1A { M1A }
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
                    CDMEIF { $cdmeif }
                    CFEIF { $cfeif }
                    CHTIF { $chtif }
                    CTCIF { $ctcif }
                    CTEIF { $cteif }
                }
                ISR {
                    $isr Shared;
                    DMEIF { $dmeif }
                    FEIF { $feif }
                    HTIF { $htif }
                    TCIF { $tcif }
                    TEIF { $teif }
                }
            }
        }
    };
}

map_dma_ch! {
    "Extracts DMA1 channel 0 register tokens.",
    periph_dma1_ch0,
    "DMA1 channel 0 peripheral variant.",
    Dma1Ch0,
    Dma1,
    Dmamux1Ch0,
    DMA1,
    S0CR,
    S0FCR,
    S0M0AR,
    S0M1AR,
    S0NDTR,
    S0PAR,
    C0S,
    LIFCR,
    CDMEIF0,
    CFEIF0,
    CGIF0,
    CHTIF0,
    CTCIF0,
    CTEIF0,
    LISR,
    DMEIF0,
    FEIF0,
    GIF0,
    HTIF0,
    TCIF0,
    TEIF0,
}

map_dma_ch! {
    "Extracts DMA1 channel 1 register tokens.",
    periph_dma1_ch1,
    "DMA1 channel 1 peripheral variant.",
    Dma1Ch1,
    Dma1,
    Dmamux1Ch1,
    DMA1,
    S1CR,
    S1FCR,
    S1M0AR,
    S1M1AR,
    S1NDTR,
    S1PAR,
    C1S,
    LIFCR,
    CDMEIF1,
    CFEIF1,
    CGIF1,
    CHTIF1,
    CTCIF1,
    CTEIF1,
    LISR,
    DMEIF1,
    FEIF1,
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
    Dmamux1Ch2,
    DMA1,
    S2CR,
    S2FCR,
    S2M0AR,
    S2M1AR,
    S2NDTR,
    S2PAR,
    C2S,
    LIFCR,
    CDMEIF2,
    CFEIF2,
    CGIF2,
    CHTIF2,
    CTCIF2,
    CTEIF2,
    LISR,
    DMEIF2,
    FEIF2,
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
    Dmamux1Ch3,
    DMA1,
    S3CR,
    S3FCR,
    S3M0AR,
    S3M1AR,
    S3NDTR,
    S3PAR,
    C3S,
    LIFCR,
    CDMEIF3,
    CFEIF3,
    CGIF3,
    CHTIF3,
    CTCIF3,
    CTEIF3,
    LISR,
    DMEIF3,
    FEIF3,
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
    Dmamux1Ch4,
    DMA1,
    S4CR,
    S4FCR,
    S4M0AR,
    S4M1AR,
    S4NDTR,
    S4PAR,
    C4S,
    HIFCR,
    CDMEIF4,
    CFEIF4,
    CGIF4,
    CHTIF4,
    CTCIF4,
    CTEIF4,
    HISR,
    DMEIF4,
    FEIF4,
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
    Dmamux1Ch5,
    DMA1,
    S5CR,
    S5FCR,
    S5M0AR,
    S5M1AR,
    S5NDTR,
    S5PAR,
    C5S,
    HIFCR,
    CDMEIF5,
    CFEIF5,
    CGIF5,
    CHTIF5,
    CTCIF5,
    CTEIF5,
    HISR,
    DMEIF5,
    FEIF5,
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
    Dmamux1Ch6,
    DMA1,
    S6CR,
    S6FCR,
    S6M0AR,
    S6M1AR,
    S6NDTR,
    S6PAR,
    C6S,
    HIFCR,
    CDMEIF6,
    CFEIF6,
    CGIF6,
    CHTIF6,
    CTCIF6,
    CTEIF6,
    HISR,
    DMEIF6,
    FEIF6,
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
    Dmamux1Ch7,
    DMA1,
    S7CR,
    S7FCR,
    S7M0AR,
    S7M1AR,
    S7NDTR,
    S7PAR,
    C7S,
    HIFCR,
    CDMEIF7,
    CFEIF7,
    CGIF7,
    CHTIF7,
    CTCIF7,
    CTEIF7,
    HISR,
    DMEIF7,
    FEIF7,
    GIF7,
    HTIF7,
    TCIF7,
    TEIF7,
}

map_dma_ch! {
    "Extracts DMA2 channel 0 register tokens.",
    periph_dma2_ch0,
    "DMA2 channel 0 peripheral variant.",
    Dma2Ch0,
    Dma2,
    Dmamux2Ch0,
    DMA2,
    S0CR,
    S0FCR,
    S0M0AR,
    S0M1AR,
    S0NDTR,
    S0PAR,
    C0S,
    LIFCR,
    CDMEIF0,
    CFEIF0,
    CGIF0,
    CHTIF0,
    CTCIF0,
    CTEIF0,
    LISR,
    DMEIF0,
    FEIF0,
    GIF0,
    HTIF0,
    TCIF0,
    TEIF0,
}

map_dma_ch! {
    "Extracts DMA2 channel 1 register tokens.",
    periph_dma2_ch1,
    "DMA2 channel 1 peripheral variant.",
    Dma2Ch1,
    Dma2,
    Dmamux2Ch1,
    DMA2,
    S1CR,
    S1FCR,
    S1M0AR,
    S1M1AR,
    S1NDTR,
    S1PAR,
    C1S,
    LIFCR,
    CDMEIF1,
    CFEIF1,
    CGIF1,
    CHTIF1,
    CTCIF1,
    CTEIF1,
    LISR,
    DMEIF1,
    FEIF1,
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
    Dmamux2Ch2,
    DMA2,
    S2CR,
    S2FCR,
    S2M0AR,
    S2M1AR,
    S2NDTR,
    S2PAR,
    C2S,
    LIFCR,
    CDMEIF2,
    CFEIF2,
    CGIF2,
    CHTIF2,
    CTCIF2,
    CTEIF2,
    LISR,
    DMEIF2,
    FEIF2,
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
    Dmamux2Ch3,
    DMA2,
    S3CR,
    S3FCR,
    S3M0AR,
    S3M1AR,
    S3NDTR,
    S3PAR,
    C3S,
    LIFCR,
    CDMEIF3,
    CFEIF3,
    CGIF3,
    CHTIF3,
    CTCIF3,
    CTEIF3,
    LISR,
    DMEIF3,
    FEIF3,
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
    Dmamux2Ch4,
    DMA2,
    S4CR,
    S4FCR,
    S4M0AR,
    S4M1AR,
    S4NDTR,
    S4PAR,
    C4S,
    HIFCR,
    CDMEIF4,
    CFEIF4,
    CGIF4,
    CHTIF4,
    CTCIF4,
    CTEIF4,
    HISR,
    DMEIF4,
    FEIF4,
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
    Dmamux2Ch5,
    DMA2,
    S5CR,
    S5FCR,
    S5M0AR,
    S5M1AR,
    S5NDTR,
    S5PAR,
    C5S,
    HIFCR,
    CDMEIF5,
    CFEIF5,
    CGIF5,
    CHTIF5,
    CTCIF5,
    CTEIF5,
    HISR,
    DMEIF5,
    FEIF5,
    GIF5,
    HTIF5,
    TCIF5,
    TEIF5,
}

map_dma_ch! {
    "Extracts DMA2 channel 6 register tokens.",
    periph_dma2_ch6,
    "DMA2 channel 6 peripheral variant.",
    Dma2Ch6,
    Dma2,
    Dmamux2Ch6,
    DMA2,
    S6CR,
    S6FCR,
    S6M0AR,
    S6M1AR,
    S6NDTR,
    S6PAR,
    C6S,
    HIFCR,
    CDMEIF6,
    CFEIF6,
    CGIF6,
    CHTIF6,
    CTCIF6,
    CTEIF6,
    HISR,
    DMEIF6,
    FEIF6,
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
    Dmamux2Ch7,
    DMA2,
    S7CR,
    S7FCR,
    S7M0AR,
    S7M1AR,
    S7NDTR,
    S7PAR,
    C7S,
    HIFCR,
    CDMEIF7,
    CFEIF7,
    CGIF7,
    CHTIF7,
    CTCIF7,
    CTEIF7,
    HISR,
    DMEIF7,
    FEIF7,
    GIF7,
    HTIF7,
    TCIF7,
    TEIF7,
}

//! DMA channels.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic DMA channel peripheral variant.
    pub trait DmaChMap {
        /// DMA head peripheral variant.
        type DmaMap: super::DmaMap;

        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        /// DMAMUX channel peripheral variant.
        type DmamuxChMap: super::mux::ch::DmamuxChMap;
    }

    /// Generic DMA channel peripheral.
    pub struct DmaChPeriph;

    DMA {
        CCR {
            0x20 RwRegBitBand;
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
            CHSEL { RwRwRegFieldBits }
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
            MBURST { RwRwRegFieldBits }
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
            PBURST { RwRwRegFieldBits }
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
            CT { RwRwRegFieldBitBand }
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
            DBM { RwRwRegFieldBitBand }
            #[cfg(any(
                stm32_mcu = "stm32f100",
                stm32_mcu = "stm32f101",
                stm32_mcu = "stm32f102",
                stm32_mcu = "stm32f103",
                stm32_mcu = "stm32f107",
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
            MEM2MEM { RwRwRegFieldBitBand }
            PL { RwRwRegFieldBits }
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
            PINCOS { RwRwRegFieldBitBand }
            MSIZE { RwRwRegFieldBits }
            PSIZE { RwRwRegFieldBits }
            MINC { RwRwRegFieldBitBand }
            PINC { RwRwRegFieldBitBand }
            CIRC { RwRwRegFieldBitBand }
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
            DIR { RwRwRegFieldBits }
            #[cfg(any(
                stm32_mcu = "stm32f100",
                stm32_mcu = "stm32f101",
                stm32_mcu = "stm32f102",
                stm32_mcu = "stm32f103",
                stm32_mcu = "stm32f107",
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
            DIR { RwRwRegFieldBitBand }
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
            PFCTRL { RwRwRegFieldBitBand }
            TEIE { RwRwRegFieldBitBand }
            HTIE { RwRwRegFieldBitBand }
            TCIE { RwRwRegFieldBitBand }
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
            DMEIE { RwRwRegFieldBitBand }
            EN { RwRwRegFieldBitBand }
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
        #[cfg(any(
            stm32_mcu = "stm32l4x1",
            stm32_mcu = "stm32l4x2",
            stm32_mcu = "stm32l4x3",
            stm32_mcu = "stm32l4x5",
            stm32_mcu = "stm32l4x6"
        ))]
        CSELR {
            0x20 RwRegBitBand Shared;
            CS { RwRwRegFieldBits }
        }
        IFCR {
            0x20 WoRegBitBand Shared;
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
            CDMEIF { WoWoRegFieldBitBand }
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
            CFEIF { WoWoRegFieldBitBand }
            #[cfg(any(
                stm32_mcu = "stm32f100",
                stm32_mcu = "stm32f101",
                stm32_mcu = "stm32f102",
                stm32_mcu = "stm32f103",
                stm32_mcu = "stm32f107",
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
            CGIF { WoWoRegFieldBitBand }
            CHTIF { WoWoRegFieldBitBand }
            CTCIF { WoWoRegFieldBitBand }
            CTEIF { WoWoRegFieldBitBand }
        }
        ISR {
            0x20 RoRegBitBand Shared;
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
            DMEIF { RoRoRegFieldBitBand }
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
            FEIF { RoRoRegFieldBitBand }
            #[cfg(any(
                stm32_mcu = "stm32f100",
                stm32_mcu = "stm32f101",
                stm32_mcu = "stm32f102",
                stm32_mcu = "stm32f103",
                stm32_mcu = "stm32f107",
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
            GIF { RoRoRegFieldBitBand }
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
                type DmaMap = super::$dma_ty;

                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                type DmamuxChMap = super::mux::ch::$dmamux_ch_ty;
            }

            drone_stm32_map_pieces::reg;
            crate::ch;

            DMA {
                $dma;
                CCR {
                    $ccr;
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
                    CHSEL { CHSEL }
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
                    MBURST { MBURST }
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
                    PBURST { PBURST }
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
                    CT { CT }
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
                    DBM { DBM }
                    #[cfg(any(
                        stm32_mcu = "stm32f100",
                        stm32_mcu = "stm32f101",
                        stm32_mcu = "stm32f102",
                        stm32_mcu = "stm32f103",
                        stm32_mcu = "stm32f107",
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
                    MEM2MEM { MEM2MEM }
                    PL { PL }
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
                    PINCOS { PINCOS }
                    MSIZE { MSIZE }
                    PSIZE { PSIZE }
                    MINC { MINC }
                    PINC { PINC }
                    CIRC { CIRC }
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
                    DIR { DIR }
                    #[cfg(any(
                        stm32_mcu = "stm32f100",
                        stm32_mcu = "stm32f101",
                        stm32_mcu = "stm32f102",
                        stm32_mcu = "stm32f103",
                        stm32_mcu = "stm32f107",
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
                    DIR { DIR }
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
                    PFCTRL { PFCTRL }
                    TEIE { TEIE }
                    HTIE { HTIE }
                    TCIE { TCIE }
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
                    DMEIE { DMEIE }
                    EN { EN }
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
                CFCR {
                    $cfcr;
                    FEIE { FEIE }
                    FS { FS }
                    DMDIS { DMDIS }
                    FTH { FTH }
                }
                CM0AR {
                    $cm0ar;
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
                    M0A { M0A }
                    #[cfg(any(
                        stm32_mcu = "stm32f100",
                        stm32_mcu = "stm32f101",
                        stm32_mcu = "stm32f102",
                        stm32_mcu = "stm32f103",
                        stm32_mcu = "stm32f107",
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
                    M0A { MA }
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
                #[cfg(any(
                    stm32_mcu = "stm32l4x1",
                    stm32_mcu = "stm32l4x2",
                    stm32_mcu = "stm32l4x3",
                    stm32_mcu = "stm32l4x5",
                    stm32_mcu = "stm32l4x6"
                ))]
                CSELR {
                    CSELR Shared;
                    CS { $cs }
                }
                IFCR {
                    $ifcr Shared;
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
                    CDMEIF { $cdmeif }
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
                    CFEIF { $cfeif }
                    #[cfg(any(
                        stm32_mcu = "stm32f100",
                        stm32_mcu = "stm32f101",
                        stm32_mcu = "stm32f102",
                        stm32_mcu = "stm32f103",
                        stm32_mcu = "stm32f107",
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
                    CGIF { $cgif }
                    CHTIF { $chtif }
                    CTCIF { $ctcif }
                    CTEIF { $cteif }
                }
                ISR {
                    $isr Shared;
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
                    DMEIF { $dmeif }
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
                    FEIF { $feif }
                    #[cfg(any(
                        stm32_mcu = "stm32f100",
                        stm32_mcu = "stm32f101",
                        stm32_mcu = "stm32f102",
                        stm32_mcu = "stm32f103",
                        stm32_mcu = "stm32f107",
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
                    GIF { $gif }
                    HTIF { $htif }
                    TCIF { $tcif }
                    TEIF { $teif }
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
map_dma_ch! {
    "Extracts DMA1 channel 1 register tokens.",
    periph_dma1_ch1,
    "DMA1 channel 1 peripheral variant.",
    Dma1Ch1,
    Dma1,
    Dmamux1Ch0,
    DMA1,
    CCR1,
    CFCR1,
    CMAR1,
    CMAR1,
    CNDTR1,
    CPAR1,
    C1S,
    IFCR,
    CDMEIF1,
    CFEIF1,
    CGIF1,
    CHTIF1,
    CTCIF1,
    CTEIF1,
    ISR,
    DMEIF1,
    FEIF1,
    GIF1,
    HTIF1,
    TCIF1,
    TEIF1,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA1 channel 2 register tokens.",
    periph_dma1_ch2,
    "DMA1 channel 2 peripheral variant.",
    Dma1Ch2,
    Dma1,
    Dmamux1Ch1,
    DMA1,
    CCR2,
    CFCR2,
    CMAR2,
    CMAR2,
    CNDTR2,
    CPAR2,
    C2S,
    IFCR,
    CDMEIF2,
    CFEIF2,
    CGIF2,
    CHTIF2,
    CTCIF2,
    CTEIF2,
    ISR,
    DMEIF2,
    FEIF2,
    GIF2,
    HTIF2,
    TCIF2,
    TEIF2,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA1 channel 3 register tokens.",
    periph_dma1_ch3,
    "DMA1 channel 3 peripheral variant.",
    Dma1Ch3,
    Dma1,
    Dmamux1Ch2,
    DMA1,
    CCR3,
    CFCR3,
    CMAR3,
    CMAR3,
    CNDTR3,
    CPAR3,
    C3S,
    IFCR,
    CDMEIF3,
    CFEIF3,
    CGIF3,
    CHTIF3,
    CTCIF3,
    CTEIF3,
    ISR,
    DMEIF3,
    FEIF3,
    GIF3,
    HTIF3,
    TCIF3,
    TEIF3,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA1 channel 4 register tokens.",
    periph_dma1_ch4,
    "DMA1 channel 4 peripheral variant.",
    Dma1Ch4,
    Dma1,
    Dmamux1Ch3,
    DMA1,
    CCR4,
    CFCR4,
    CMAR4,
    CMAR4,
    CNDTR4,
    CPAR4,
    C4S,
    IFCR,
    CDMEIF4,
    CFEIF4,
    CGIF4,
    CHTIF4,
    CTCIF4,
    CTEIF4,
    ISR,
    DMEIF4,
    FEIF4,
    GIF4,
    HTIF4,
    TCIF4,
    TEIF4,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA1 channel 5 register tokens.",
    periph_dma1_ch5,
    "DMA1 channel 5 peripheral variant.",
    Dma1Ch5,
    Dma1,
    Dmamux1Ch4,
    DMA1,
    CCR5,
    CFCR5,
    CMAR5,
    CMAR5,
    CNDTR5,
    CPAR5,
    C5S,
    IFCR,
    CDMEIF5,
    CFEIF5,
    CGIF5,
    CHTIF5,
    CTCIF5,
    CTEIF5,
    ISR,
    DMEIF5,
    FEIF5,
    GIF5,
    HTIF5,
    TCIF5,
    TEIF5,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA1 channel 6 register tokens.",
    periph_dma1_ch6,
    "DMA1 channel 6 peripheral variant.",
    Dma1Ch6,
    Dma1,
    Dmamux1Ch5,
    DMA1,
    CCR6,
    CFCR6,
    CMAR6,
    CMAR6,
    CNDTR6,
    CPAR6,
    C6S,
    IFCR,
    CDMEIF6,
    CFEIF6,
    CGIF6,
    CHTIF6,
    CTCIF6,
    CTEIF6,
    ISR,
    DMEIF6,
    FEIF6,
    GIF6,
    HTIF6,
    TCIF6,
    TEIF6,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA1 channel 7 register tokens.",
    periph_dma1_ch7,
    "DMA1 channel 7 peripheral variant.",
    Dma1Ch7,
    Dma1,
    Dmamux1Ch6,
    DMA1,
    CCR7,
    CFCR7,
    CMAR7,
    CMAR7,
    CNDTR7,
    CPAR7,
    C7S,
    IFCR,
    CDMEIF7,
    CFEIF7,
    CGIF7,
    CHTIF7,
    CTCIF7,
    CTEIF7,
    ISR,
    DMEIF7,
    FEIF7,
    GIF7,
    HTIF7,
    TCIF7,
    TEIF7,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA2 channel 1 register tokens.",
    periph_dma2_ch1,
    "DMA2 channel 1 peripheral variant.",
    Dma2Ch1,
    Dma2,
    Dmamux1Ch7,
    DMA2,
    CCR1,
    CFCR1,
    CMAR1,
    CMAR1,
    CNDTR1,
    CPAR1,
    C1S,
    IFCR,
    CDMEIF1,
    CFEIF1,
    CGIF1,
    CHTIF1,
    CTCIF1,
    CTEIF1,
    ISR,
    DMEIF1,
    FEIF1,
    GIF1,
    HTIF1,
    TCIF1,
    TEIF1,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA2 channel 2 register tokens.",
    periph_dma2_ch2,
    "DMA2 channel 2 peripheral variant.",
    Dma2Ch2,
    Dma2,
    Dmamux1Ch8,
    DMA2,
    CCR2,
    CFCR2,
    CMAR2,
    CMAR2,
    CNDTR2,
    CPAR2,
    C2S,
    IFCR,
    CDMEIF2,
    CFEIF2,
    CGIF2,
    CHTIF2,
    CTCIF2,
    CTEIF2,
    ISR,
    DMEIF2,
    FEIF2,
    GIF2,
    HTIF2,
    TCIF2,
    TEIF2,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA2 channel 3 register tokens.",
    periph_dma2_ch3,
    "DMA2 channel 3 peripheral variant.",
    Dma2Ch3,
    Dma2,
    Dmamux1Ch9,
    DMA2,
    CCR3,
    CFCR3,
    CMAR3,
    CMAR3,
    CNDTR3,
    CPAR3,
    C3S,
    IFCR,
    CDMEIF3,
    CFEIF3,
    CGIF3,
    CHTIF3,
    CTCIF3,
    CTEIF3,
    ISR,
    DMEIF3,
    FEIF3,
    GIF3,
    HTIF3,
    TCIF3,
    TEIF3,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA2 channel 4 register tokens.",
    periph_dma2_ch4,
    "DMA2 channel 4 peripheral variant.",
    Dma2Ch4,
    Dma2,
    Dmamux1Ch10,
    DMA2,
    CCR4,
    CFCR4,
    CMAR4,
    CMAR4,
    CNDTR4,
    CPAR4,
    C4S,
    IFCR,
    CDMEIF4,
    CFEIF4,
    CGIF4,
    CHTIF4,
    CTCIF4,
    CTEIF4,
    ISR,
    DMEIF4,
    FEIF4,
    GIF4,
    HTIF4,
    TCIF4,
    TEIF4,
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
map_dma_ch! {
    "Extracts DMA2 channel 5 register tokens.",
    periph_dma2_ch5,
    "DMA2 channel 5 peripheral variant.",
    Dma2Ch5,
    Dma2,
    Dmamux1Ch11,
    DMA2,
    CCR5,
    CFCR5,
    CMAR5,
    CMAR5,
    CNDTR5,
    CPAR5,
    C5S,
    IFCR,
    CDMEIF5,
    CFEIF5,
    CGIF5,
    CHTIF5,
    CTCIF5,
    CTEIF5,
    ISR,
    DMEIF5,
    FEIF5,
    GIF5,
    HTIF5,
    TCIF5,
    TEIF5,
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
map_dma_ch! {
    "Extracts DMA2 channel peripheral varian tokens.",
    periph_dma2_ch6,
    "DMA2 channel 6.",
    Dma2Ch6,
    Dma2,
    Dmamux1Ch12,
    DMA2,
    CCR6,
    CFCR6,
    CMAR6,
    CMAR6,
    CNDTR6,
    CPAR6,
    C6S,
    IFCR,
    CDMEIF6,
    CFEIF6,
    CGIF6,
    CHTIF6,
    CTCIF6,
    CTEIF6,
    ISR,
    DMEIF6,
    FEIF6,
    GIF6,
    HTIF6,
    TCIF6,
    TEIF6,
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
map_dma_ch! {
    "Extracts DMA2 channel 7 register tokens.",
    periph_dma2_ch7,
    "DMA2 channel 7 peripheral variant.",
    Dma2Ch7,
    Dma2,
    Dmamux1Ch13,
    DMA2,
    CCR7,
    CFCR7,
    CMAR7,
    CMAR7,
    CNDTR7,
    CPAR7,
    C7S,
    IFCR,
    CDMEIF7,
    CFEIF7,
    CGIF7,
    CHTIF7,
    CTCIF7,
    CTEIF7,
    ISR,
    DMEIF7,
    FEIF7,
    GIF7,
    HTIF7,
    TCIF7,
    TEIF7,
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

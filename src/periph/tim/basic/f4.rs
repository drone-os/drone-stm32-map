//! Basic timers.
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic basic timer peripheral variant.
    pub trait BasicTimMap {}

    /// Generic basic timer peripheral.
    pub struct BasicTimPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            TIMEN { RwRwRegFieldBitBand }
        }
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            TIMRST { RwRwRegFieldBitBand }
        }
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            TIMSMEN { RwRwRegFieldBitBand }
        }
    }

    DBG {
        DBGMCU {
            0x20 RwReg Shared;
            TIMSTOP { RwRwRegFieldBit }
        }
    }

    TIM {
        CR1 {
            0x20 RwRegBitBand;
            ARPE { RwRwRegFieldBitBand }
            CEN { RwRwRegFieldBitBand }
            OPM { RwRwRegFieldBitBand }
            UDIS { RwRwRegFieldBitBand }
            URS { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
            MMS { RwRwRegFieldBits }
        }
        DIER {
            0x20 RwRegBitBand;
            UDE { RwRwRegFieldBitBand }
            UIE { RwRwRegFieldBitBand }
        }
        SR {
            0x20 RwRegBitBand;
            UIF { RwRwRegFieldBitBand }
        }
        EGR {
            0x20 WoRegBitBand;
            UG { WoWoRegFieldBitBand }
        }
        CNT {
            0x20 RwRegBitBand;
            CNT { RwRwRegFieldBits }
        }
        PSC {
            0x20 RwRegBitBand;
            PSC { RwRwRegFieldBits }
        }
        ARR {
            0x20 RwRegBitBand;
            ARR { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_basic_tim {
    (
        $tim_macro_doc:expr,
        $tim_macro:ident,
        $tim_ty_doc:expr,
        $tim_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $timen:ident,
        $timrst:ident,
        $timsmen:ident,
        $dbgmcu:ident,
        $timstop:ident,
        $tim:ident,
    ) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl BasicTimMap for $tim_ty {}

            drone_stm32_map_pieces::reg;
            crate::basic;

            RCC {
                BUSENR {
                    $busenr Shared;
                    TIMEN { $timen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    TIMRST { $timrst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    TIMSMEN { $timsmen }
                }
            }

            DBG {
                DBGMCU {
                    $dbgmcu Shared;
                    TIMSTOP { $timstop }
                }
            }

            TIM {
                $tim;
                CR1 {
                    CR1;
                    ARPE { ARPE }
                    CEN { CEN }
                    OPM { OPM }
                    UDIS { UDIS }
                    URS { URS }
                }
                CR2 {
                    CR2;
                    MMS { MMS }
                }
                DIER {
                    DIER;
                    UDE { UDE }
                    UIE { UIE }
                }
                SR {
                    SR;
                    UIF { UIF }
                }
                EGR {
                    EGR;
                    UG { UG }
                }
                CNT {
                    CNT;
                    CNT { CNT }
                }
                PSC {
                    PSC;
                    PSC { PSC }
                }
                ARR {
                    ARR;
                    ARR { ARR }
                }
            }
        }
    };
}

map_basic_tim! {
    "Extracts TIM6 register tokens.",
    periph_tim6,
    "TIM6 peripheral variant.",
    Tim6,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM6EN,
    TIM6RST,
    TIM6LPEN,
    DBGMCU_APB1_FZ,
    DBG_TIM6_STOP,
    TIM6,
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469"
))]
map_basic_tim! {
    "Extracts TIM7 register tokens.",
    periph_tim7,
    "TIM7 peripheral variant.",
    Tim7,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM7EN,
    TIM7RST,
    TIM7LPEN,
    DBGMCU_APB1_FZ,
    DBG_TIM7_STOP,
    TIM7,
}

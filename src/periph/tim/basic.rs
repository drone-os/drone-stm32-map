//! Basic timers.

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

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
            TIMSMEN { RwRwRegFieldBitBand }
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

#[allow(unused_macros)]
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
                    TIMSMEN { $timsmen }
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

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_basic_tim! {
    "Extracts TIM6 register tokens.",
    periph_tim6,
    "TIM6 peripheral variant.",
    Tim6,
    APB1ENR,
    APB1RSTR,
    APB1SMENR,
    TIM6EN,
    TIM6RST,
    TIM6SMEN,
    TIM6,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_basic_tim! {
    "Extracts TIM7 register tokens.",
    periph_tim7,
    "TIM7 peripheral variant.",
    Tim7,
    APB1ENR,
    APB1RSTR,
    APB1SMENR,
    TIM7EN,
    TIM7RST,
    TIM7SMEN,
    TIM7,
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
map_basic_tim! {
    "Extracts TIM6 register tokens.",
    periph_tim6,
    "TIM6 peripheral variant.",
    Tim6,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    TIM6EN,
    TIM6RST,
    TIM6SMEN,
    TIM6,
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
map_basic_tim! {
    "Extracts TIM7 register tokens.",
    periph_tim7,
    "TIM7 peripheral variant.",
    Tim7,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    TIM7EN,
    TIM7RST,
    TIM7SMEN,
    TIM7,
}

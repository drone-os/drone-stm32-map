//! Low-power timers.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic low-power timer peripheral variant.
    pub trait LowPowerTimMap {}

    /// Generic low-power timer peripheral.
    pub struct LowPowerTimPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            LPTIMEN { RwRwRegFieldBitBand }
        }
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            LPTIMRST { RwRwRegFieldBitBand }
        }
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            LPTIMSMEN { RwRwRegFieldBitBand }
        }
        CCIPR {
            0x20 RwRegBitBand Shared;
            LPTIMSEL { RwRwRegFieldBits }
        }
    }
    LPTIM {
        ISR {
            0x20 RoRegBitBand;
            ARRM { RoRoRegFieldBitBand }
            ARROK { RoRoRegFieldBitBand }
            CMPM { RoRoRegFieldBitBand }
            CMPOK { RoRoRegFieldBitBand }
            DOWN { RoRoRegFieldBitBand }
            EXTTRIG { RoRoRegFieldBitBand }
            UP { RoRoRegFieldBitBand }
        }
        ICR {
            0x20 WoRegBitBand;
            ARRMCF { WoWoRegFieldBitBand }
            ARROKCF { WoWoRegFieldBitBand }
            CMPMCF { WoWoRegFieldBitBand }
            CMPOKCF { WoWoRegFieldBitBand }
            DOWNCF { WoWoRegFieldBitBand }
            EXTTRIGCF { WoWoRegFieldBitBand }
            UPCF { WoWoRegFieldBitBand }
        }
        IER {
            0x20 RwRegBitBand;
            ARRMIE { RwRwRegFieldBitBand }
            ARROKIE { RwRwRegFieldBitBand }
            CMPMIE { RwRwRegFieldBitBand }
            CMPOKIE { RwRwRegFieldBitBand }
            DOWNIE { RwRwRegFieldBitBand }
            EXTTRIGIE { RwRwRegFieldBitBand }
            UPIE { RwRwRegFieldBitBand }
        }
        CFGR {
            0x20 RwRegBitBand;
            CKFLT { RwRwRegFieldBits }
            CKPOL { RwRwRegFieldBits }
            CKSEL { RwRwRegFieldBitBand }
            COUNTMODE { RwRwRegFieldBitBand }
            ENC { RwRwRegFieldBitBand }
            PRELOAD { RwRwRegFieldBitBand }
            PRESC { RwRwRegFieldBits }
            TIMOUT { RwRwRegFieldBitBand }
            TRGFLT { RwRwRegFieldBits }
            TRIGEN { RwRwRegFieldBits }
            TRIGSEL { RwRwRegFieldBits }
            WAVE { RwRwRegFieldBitBand }
            WAVPOL { RwRwRegFieldBitBand }
        }
        CR {
            0x20 RwRegBitBand;
            CNTSTRT { RwRwRegFieldBitBand }
            ENABLE { RwRwRegFieldBitBand }
            SNGSTRT { RwRwRegFieldBitBand }
        }
        CMP {
            0x20 RwRegBitBand;
            CMP { RwRwRegFieldBits }
        }
        ARR {
            0x20 RwRegBitBand;
            ARR { RwRwRegFieldBits }
        }
        CNT {
            0x20 RoRegBitBand;
            CNT { RoRoRegFieldBits }
        }
        OR {
            0x20 RwRegBitBand;
            OR_0 { RwRwRegFieldBitBand }
            OR_1 { RwRwRegFieldBitBand }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_low_power_tim {
    (
        $tim_macro_doc:expr,
        $tim_macro:ident,
        $tim_ty_doc:expr,
        $tim_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $lptimen:ident,
        $lptimrst:ident,
        $lptimsmen:ident,
        $lptimsel:ident,
        $lptim:ident,
    ) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl LowPowerTimMap for $tim_ty {}

            drone_stm32_map_pieces::reg;
            crate::low_power;

            RCC {
                BUSENR {
                    $busenr Shared;
                    LPTIMEN { $lptimen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    LPTIMRST { $lptimrst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    LPTIMSMEN { $lptimsmen }
                }
                CCIPR {
                    CCIPR Shared;
                    LPTIMSEL { $lptimsel }
                }
            }
            LPTIM {
                $lptim;
                ISR {
                    ISR;
                    ARRM { ARRM }
                    ARROK { ARROK }
                    CMPM { CMPM }
                    CMPOK { CMPOK }
                    DOWN { DOWN }
                    EXTTRIG { EXTTRIG }
                    UP { UP }
                }
                ICR {
                    ICR;
                    ARRMCF { ARRMCF }
                    ARROKCF { ARROKCF }
                    CMPMCF { CMPMCF }
                    CMPOKCF { CMPOKCF }
                    DOWNCF { DOWNCF }
                    EXTTRIGCF { EXTTRIGCF }
                    UPCF { UPCF }
                }
                IER {
                    IER;
                    ARRMIE { ARRMIE }
                    ARROKIE { ARROKIE }
                    CMPMIE { CMPMIE }
                    CMPOKIE { CMPOKIE }
                    DOWNIE { DOWNIE }
                    EXTTRIGIE { EXTTRIGIE }
                    UPIE { UPIE }
                }
                CFGR {
                    CFGR;
                    CKFLT { CKFLT }
                    CKPOL { CKPOL }
                    CKSEL { CKSEL }
                    COUNTMODE { COUNTMODE }
                    ENC { ENC }
                    PRELOAD { PRELOAD }
                    PRESC { PRESC }
                    TIMOUT { TIMOUT }
                    TRGFLT { TRGFLT }
                    TRIGEN { TRIGEN }
                    TRIGSEL { TRIGSEL }
                    WAVE { WAVE }
                    WAVPOL { WAVPOL }
                }
                CR {
                    CR;
                    CNTSTRT { CNTSTRT }
                    ENABLE { ENABLE }
                    SNGSTRT { SNGSTRT }
                }
                CMP {
                    CMP;
                    CMP { CMP }
                }
                ARR {
                    ARR;
                    ARR { ARR }
                }
                CNT {
                    CNT;
                    CNT { CNT }
                }
                OR {
                    OR;
                    OR_0 { OR_0 }
                    OR_1 { OR_1 }
                }
            }
        }
    };
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
map_low_power_tim! {
    "Extracts LPTIM1 register tokens.",
    periph_lptim1,
    "LPTIM1 peripheral variant.",
    Lptim1,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    LPTIM1EN,
    LPTIM1RST,
    LPTIM1SMEN,
    LPTIM1SEL,
    LPTIM1,
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
map_low_power_tim! {
    "Extracts LPTIM2 register tokens.",
    periph_lptim2,
    "LPTIM2 peripheral variant.",
    Lptim2,
    APB1ENR2,
    APB1RSTR2,
    APB1SMENR2,
    LPTIM2EN,
    LPTIM2RST,
    LPTIM2SMEN,
    LPTIM2SEL,
    LPTIM2,
}

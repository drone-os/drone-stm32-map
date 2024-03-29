//! General-purpose timers.
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic general-purpose timer peripheral variant.
    pub trait GeneralTimMap {}

    /// Generic general-purpose timer peripheral.
    pub struct GeneralTimPeriph;

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
            CKD { RwRwRegFieldBits }
            CMS { RwRwRegFieldBits Option }
            DIR { RwRwRegFieldBitBand Option }
            #[cfg(any(
                drone_stm32_map = "stm32f401",
                drone_stm32_map = "stm32f405",
                drone_stm32_map = "stm32f407",
                drone_stm32_map = "stm32f411",
                drone_stm32_map = "stm32f427",
                drone_stm32_map = "stm32f429",
            ))]
            OPM { RwRwRegFieldBitBand }
            #[cfg(any(
                drone_stm32_map = "stm32f410",
                drone_stm32_map = "stm32f412",
                drone_stm32_map = "stm32f413",
                drone_stm32_map = "stm32f446",
                drone_stm32_map = "stm32f469"
            ))]
            OPM { RwRwRegFieldBitBand Option }
            UDIS { RwRwRegFieldBitBand }
            URS { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand Option;
            TI1S { RwRwRegFieldBitBand }
            MMS { RwRwRegFieldBits }
            CCDS { RwRwRegFieldBitBand }
        }
        SMCR {
            0x20 RwRegBitBand Option;
            #[cfg(any(
                drone_stm32_map = "stm32f401",
                drone_stm32_map = "stm32f405",
                drone_stm32_map = "stm32f407",
                drone_stm32_map = "stm32f411",
                drone_stm32_map = "stm32f412",
                drone_stm32_map = "stm32f413",
                drone_stm32_map = "stm32f427",
                drone_stm32_map = "stm32f429",
                drone_stm32_map = "stm32f446",
                drone_stm32_map = "stm32f469",
            ))]
            ETP { RwRwRegFieldBitBand Option }
            #[cfg(any(
                drone_stm32_map = "stm32f401",
                drone_stm32_map = "stm32f405",
                drone_stm32_map = "stm32f407",
                drone_stm32_map = "stm32f411",
                drone_stm32_map = "stm32f412",
                drone_stm32_map = "stm32f413",
                drone_stm32_map = "stm32f427",
                drone_stm32_map = "stm32f429",
                drone_stm32_map = "stm32f446",
                drone_stm32_map = "stm32f469",
            ))]
            ECE { RwRwRegFieldBitBand Option }
            #[cfg(any(
                drone_stm32_map = "stm32f401",
                drone_stm32_map = "stm32f405",
                drone_stm32_map = "stm32f407",
                drone_stm32_map = "stm32f411",
                drone_stm32_map = "stm32f412",
                drone_stm32_map = "stm32f413",
                drone_stm32_map = "stm32f427",
                drone_stm32_map = "stm32f429",
                drone_stm32_map = "stm32f446",
                drone_stm32_map = "stm32f469",
            ))]
            ETPS { RwRwRegFieldBits Option }
            #[cfg(any(
                drone_stm32_map = "stm32f401",
                drone_stm32_map = "stm32f405",
                drone_stm32_map = "stm32f407",
                drone_stm32_map = "stm32f411",
                drone_stm32_map = "stm32f412",
                drone_stm32_map = "stm32f413",
                drone_stm32_map = "stm32f427",
                drone_stm32_map = "stm32f429",
                drone_stm32_map = "stm32f446",
                drone_stm32_map = "stm32f469",
            ))]
            ETF { RwRwRegFieldBits Option }
            MSM { RwRwRegFieldBitBand }
            TS { RwRwRegFieldBits }
            SMS0_2 { RwRwRegFieldBits }
        }
        DIER {
            0x20 RwRegBitBand;
            CC1DE { RwRwRegFieldBitBand Option }
            CC1IE { RwRwRegFieldBitBand }
            CC2DE { RwRwRegFieldBitBand Option }
            CC2IE { RwRwRegFieldBitBand Option }
            CC3DE { RwRwRegFieldBitBand Option }
            CC3IE { RwRwRegFieldBitBand Option }
            CC4DE { RwRwRegFieldBitBand Option }
            CC4IE { RwRwRegFieldBitBand Option }
            TDE { RwRwRegFieldBitBand Option }
            TIE { RwRwRegFieldBitBand Option }
            UDE { RwRwRegFieldBitBand Option }
            UIE { RwRwRegFieldBitBand }
        }
        SR {
            0x20 RwRegBitBand;
            CC1IF { RwRwRegFieldBitBand }
            CC1OF { RwRwRegFieldBitBand }
            CC2IF { RwRwRegFieldBitBand Option }
            CC2OF { RwRwRegFieldBitBand Option }
            CC3IF { RwRwRegFieldBitBand Option }
            CC3OF { RwRwRegFieldBitBand Option }
            CC4IF { RwRwRegFieldBitBand Option }
            CC4OF { RwRwRegFieldBitBand Option }
            TIF { RwRwRegFieldBitBand Option }
            UIF { RwRwRegFieldBitBand }
        }
        EGR {
            0x20 WoRegBitBand;
            CC1G { WoWoRegFieldBitBand }
            CC2G { WoWoRegFieldBitBand Option }
            CC3G { WoWoRegFieldBitBand Option }
            CC4G { WoWoRegFieldBitBand Option }
            TG { WoWoRegFieldBitBand Option }
            UG { WoWoRegFieldBitBand }
        }
        CCMR1 {
            @Output 0x20 RwRegBitBand;
            CC1S { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits Option }
            #[cfg(any(
                drone_stm32_map = "stm32f401",
                drone_stm32_map = "stm32f405",
                drone_stm32_map = "stm32f407",
                drone_stm32_map = "stm32f411",
                drone_stm32_map = "stm32f412",
                drone_stm32_map = "stm32f413",
                drone_stm32_map = "stm32f427",
                drone_stm32_map = "stm32f429",
                drone_stm32_map = "stm32f446",
                drone_stm32_map = "stm32f469",
            ))]
            OC1CE { RwRwRegFieldBitBand Option }
            OC1FE { RwRwRegFieldBitBand }
            OC1M0_2 { RwRwRegFieldBits }
            OC1PE { RwRwRegFieldBitBand }
            #[cfg(any(
                drone_stm32_map = "stm32f401",
                drone_stm32_map = "stm32f405",
                drone_stm32_map = "stm32f407",
                drone_stm32_map = "stm32f411",
                drone_stm32_map = "stm32f412",
                drone_stm32_map = "stm32f413",
                drone_stm32_map = "stm32f427",
                drone_stm32_map = "stm32f429",
                drone_stm32_map = "stm32f446",
                drone_stm32_map = "stm32f469",
            ))]
            OC2CE { RwRwRegFieldBitBand Option }
            OC2FE { RwRwRegFieldBitBand Option }
            OC2M { RwRwRegFieldBits Option }
            OC2PE { RwRwRegFieldBitBand Option }
            @Input 0x20 RwRegBitBand;
            CC1S { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits Option }
            IC1F { RwRwRegFieldBits }
            IC1PSC { RwRwRegFieldBits }
            IC2F { RwRwRegFieldBits Option }
            IC2PSC { RwRwRegFieldBits Option }
        }
        CCMR2 {
            @Output 0x20 RwRegBitBand Option;
            CC3S { RwRwRegFieldBits }
            CC4S { RwRwRegFieldBits }
            #[cfg(any(
                drone_stm32_map = "stm32f401",
                drone_stm32_map = "stm32f405",
                drone_stm32_map = "stm32f407",
                drone_stm32_map = "stm32f411",
                drone_stm32_map = "stm32f412",
                drone_stm32_map = "stm32f413",
                drone_stm32_map = "stm32f427",
                drone_stm32_map = "stm32f429",
                drone_stm32_map = "stm32f446",
                drone_stm32_map = "stm32f469",
            ))]
            OC3CE { RwRwRegFieldBitBand }
            OC3FE { RwRwRegFieldBitBand }
            OC3M { RwRwRegFieldBits }
            OC3PE { RwRwRegFieldBitBand }
            #[cfg(any(
                drone_stm32_map = "stm32f401",
                drone_stm32_map = "stm32f405",
                drone_stm32_map = "stm32f407",
                drone_stm32_map = "stm32f411",
                drone_stm32_map = "stm32f412",
                drone_stm32_map = "stm32f413",
                drone_stm32_map = "stm32f427",
                drone_stm32_map = "stm32f429",
                drone_stm32_map = "stm32f446",
                drone_stm32_map = "stm32f469",
            ))]
            OC4CE { RwRwRegFieldBitBand }
            OC4FE { RwRwRegFieldBitBand }
            OC4M { RwRwRegFieldBits }
            OC4PE { RwRwRegFieldBitBand }
            @Input 0x20 RwRegBitBand Option;
            CC3S { RwRwRegFieldBits }
            CC4S { RwRwRegFieldBits }
            IC3F { RwRwRegFieldBits }
            IC3PSC { RwRwRegFieldBits }
            IC4F { RwRwRegFieldBits }
            IC4PSC { RwRwRegFieldBits }
        }
        CCER {
            0x20 RwRegBitBand;
            CC1E { RwRwRegFieldBitBand }
            CC1NP { RwRwRegFieldBitBand }
            CC1P { RwRwRegFieldBitBand }
            CC2E { RwRwRegFieldBitBand Option }
            CC2NP { RwRwRegFieldBitBand Option }
            CC2P { RwRwRegFieldBitBand Option }
            CC3E { RwRwRegFieldBitBand Option }
            CC3NP { RwRwRegFieldBitBand Option }
            CC3P { RwRwRegFieldBitBand Option }
            CC4E { RwRwRegFieldBitBand Option }
            CC4NP { RwRwRegFieldBitBand Option }
            CC4P { RwRwRegFieldBitBand Option }
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
        CCR1 {
            0x20 RwRegBitBand;
            CCR1 { RwRwRegFieldBits }
        }
        CCR2 {
            0x20 RwRegBitBand Option;
            CCR2 { RwRwRegFieldBits }
        }
        CCR3 {
            0x20 RwRegBitBand Option;
            CCR3 { RwRwRegFieldBits }
        }
        CCR4 {
            0x20 RwRegBitBand Option;
            CCR4 { RwRwRegFieldBits }
        }
        DCR {
            0x20 RwRegBitBand Option;
            DBA { RwRwRegFieldBits }
            DBL { RwRwRegFieldBits }
        }
        DMAR {
            0x20 RwRegBitBand Option;
            DMAB { RwRwRegFieldBits }
        }
        OR1 {
            0x20 RwRegBitBand Option;
            #[cfg(any(
                drone_stm32_map = "stm32f401",
                drone_stm32_map = "stm32f405",
                drone_stm32_map = "stm32f407",
                drone_stm32_map = "stm32f411",
                drone_stm32_map = "stm32f412",
                drone_stm32_map = "stm32f413",
                drone_stm32_map = "stm32f427",
                drone_stm32_map = "stm32f429",
                drone_stm32_map = "stm32f446",
                drone_stm32_map = "stm32f469"
            ))]
            ITR1_RMP { RwRwRegFieldBits Option }
            TI1_RMP { RwRwRegFieldBits Option }
            TI4_RMP { RwRwRegFieldBits Option }
        }
    }
}

macro_rules! map_general_tim {
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
        (
            $($cms:ident)?,
            $($dir:ident)?,
            $($opm:ident)?
        ),
        ($($cr2:ident)?),
        ($(
            $smcr:ident,
            $($etp:ident)?,
            $($ece:ident)?,
            $($etps:ident)?,
            $($etf:ident)?
        )?),
        (
            $($cc1de:ident)?,
            $($cc2de:ident)?,
            $($cc2ie:ident)?,
            $($cc3de:ident)?,
            $($cc3ie:ident)?,
            $($cc4de:ident)?,
            $($cc4ie:ident)?,
            $($tde:ident)?,
            $($tie:ident)?,
            $($ude:ident)?
        ),
        (
            $($cc2if:ident)?,
            $($cc2of:ident)?,
            $($cc3if:ident)?,
            $($cc3of:ident)?,
            $($cc4if:ident)?,
            $($cc4of:ident)?,
            $($tif:ident)?
        ),
        (
            $($cc2g:ident)?,
            $($cc3g:ident)?,
            $($cc4g:ident)?,
            $($tg:ident)?
        ),
        (
            $($cc2s:ident)?,
            $($oc1ce:ident)?,
            $($oc2ce:ident)?,
            $($oc2fe:ident)?,
            $($oc2m:ident)?,
            $($oc2pe:ident)?,
            $($ic2f:ident)?,
            $($ic2psc:ident)?
        ),
        ($($ccmr2_input:ident)?, $($ccmr2_output:ident)?),
        (
            $($cc2e:ident)?,
            $($cc2np:ident)?,
            $($cc2p:ident)?,
            $($cc3e:ident)?,
            $($cc3np:ident)?,
            $($cc3p:ident)?,
            $($cc4e:ident)?,
            $($cc4np:ident)?,
            $($cc4p:ident)?
        ),
        ($($ccr2:ident)?, $($ccr3:ident)?, $($ccr4:ident)?),
        ($($dcr:ident)?),
        ($($dmar:ident)?),
        ($(
            $or1:ident,
            $($itr1_rmp:ident)?,
            $($ti1_rmp:ident)?,
            $($ti4_rmp:ident)?
        )?),
    ) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl GeneralTimMap for $tim_ty {}

            drone_stm32_map_pieces::reg;
            crate::general;

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
                    CKD { CKD }
                    CMS { $($cms Option)* }
                    DIR { $($dir Option)* }
                    #[cfg(any(
                        drone_stm32_map = "stm32f401",
                        drone_stm32_map = "stm32f405",
                        drone_stm32_map = "stm32f407",
                        drone_stm32_map = "stm32f411",
                        drone_stm32_map = "stm32f427",
                        drone_stm32_map = "stm32f429",
                    ))]
                    OPM { OPM }
                    #[cfg(any(
                        drone_stm32_map = "stm32f410",
                        drone_stm32_map = "stm32f412",
                        drone_stm32_map = "stm32f413",
                        drone_stm32_map = "stm32f446",
                        drone_stm32_map = "stm32f469"
                    ))]
                    OPM { $($opm Option)* }
                    UDIS { UDIS }
                    URS { URS }
                }
                CR2 {
                    $(
                        $cr2 Option;
                        TI1S { TI1S }
                        MMS { MMS }
                        CCDS { CCDS }
                    )*
                }
                SMCR {
                    $(
                        $smcr Option;
                        #[cfg(any(
                            drone_stm32_map = "stm32f401",
                            drone_stm32_map = "stm32f405",
                            drone_stm32_map = "stm32f407",
                            drone_stm32_map = "stm32f411",
                            drone_stm32_map = "stm32f412",
                            drone_stm32_map = "stm32f413",
                            drone_stm32_map = "stm32f427",
                            drone_stm32_map = "stm32f429",
                            drone_stm32_map = "stm32f446",
                            drone_stm32_map = "stm32f469",
                        ))]
                        ETP { $($etp Option)* }
                        #[cfg(any(
                            drone_stm32_map = "stm32f401",
                            drone_stm32_map = "stm32f405",
                            drone_stm32_map = "stm32f407",
                            drone_stm32_map = "stm32f411",
                            drone_stm32_map = "stm32f412",
                            drone_stm32_map = "stm32f413",
                            drone_stm32_map = "stm32f427",
                            drone_stm32_map = "stm32f429",
                            drone_stm32_map = "stm32f446",
                            drone_stm32_map = "stm32f469",
                        ))]
                        ECE { $($ece Option)* }
                        #[cfg(any(
                            drone_stm32_map = "stm32f401",
                            drone_stm32_map = "stm32f405",
                            drone_stm32_map = "stm32f407",
                            drone_stm32_map = "stm32f411",
                            drone_stm32_map = "stm32f412",
                            drone_stm32_map = "stm32f413",
                            drone_stm32_map = "stm32f427",
                            drone_stm32_map = "stm32f429",
                            drone_stm32_map = "stm32f446",
                            drone_stm32_map = "stm32f469",
                        ))]
                        ETPS { $($etps Option)* }
                        #[cfg(any(
                            drone_stm32_map = "stm32f401",
                            drone_stm32_map = "stm32f405",
                            drone_stm32_map = "stm32f407",
                            drone_stm32_map = "stm32f411",
                            drone_stm32_map = "stm32f412",
                            drone_stm32_map = "stm32f413",
                            drone_stm32_map = "stm32f427",
                            drone_stm32_map = "stm32f429",
                            drone_stm32_map = "stm32f446",
                            drone_stm32_map = "stm32f469",
                        ))]
                        ETF { $($etf Option)* }
                        MSM { MSM }
                        TS { TS }
                        SMS0_2 { SMS }
                    )*
                }
                DIER {
                    DIER;
                    CC1DE { $($cc1de Option)* }
                    CC1IE { CC1IE }
                    CC2DE { $($cc2de Option)* }
                    CC2IE { $($cc2ie Option)* }
                    CC3DE { $($cc3de Option)* }
                    CC3IE { $($cc3ie Option)* }
                    CC4DE { $($cc4de Option)* }
                    CC4IE { $($cc4ie Option)* }
                    TDE { $($tde Option)* }
                    TIE { $($tie Option)* }
                    UDE { $($ude Option)* }
                    UIE { UIE }
                }
                SR {
                    SR;
                    CC1IF { CC1IF }
                    CC1OF { CC1OF }
                    CC2IF { $($cc2if Option)* }
                    CC2OF { $($cc2of Option)* }
                    CC3IF { $($cc3if Option)* }
                    CC3OF { $($cc3of Option)* }
                    CC4IF { $($cc4if Option)* }
                    CC4OF { $($cc4of Option)* }
                    TIF { $($tif Option)* }
                    UIF { UIF }
                }
                EGR {
                    EGR;
                    CC1G { CC1G }
                    CC2G { $($cc2g Option)* }
                    CC3G { $($cc3g Option)* }
                    CC4G { $($cc4g Option)* }
                    TG { $($tg Option)* }
                    UG { UG }
                }
                CCMR1 {
                    @Output CCMR1_Output;
                    CC1S { CC1S }
                    CC2S { $($cc2s Option)* }
                    #[cfg(any(
                        drone_stm32_map = "stm32f401",
                        drone_stm32_map = "stm32f405",
                        drone_stm32_map = "stm32f407",
                        drone_stm32_map = "stm32f411",
                        drone_stm32_map = "stm32f412",
                        drone_stm32_map = "stm32f413",
                        drone_stm32_map = "stm32f427",
                        drone_stm32_map = "stm32f429",
                        drone_stm32_map = "stm32f446",
                        drone_stm32_map = "stm32f469",
                    ))]
                    OC1CE { $($oc1ce Option)* }
                    OC1FE { OC1FE }
                    OC1M0_2 { OC1M }
                    OC1PE { OC1PE }
                    #[cfg(any(
                        drone_stm32_map = "stm32f401",
                        drone_stm32_map = "stm32f405",
                        drone_stm32_map = "stm32f407",
                        drone_stm32_map = "stm32f411",
                        drone_stm32_map = "stm32f412",
                        drone_stm32_map = "stm32f413",
                        drone_stm32_map = "stm32f427",
                        drone_stm32_map = "stm32f429",
                        drone_stm32_map = "stm32f446",
                        drone_stm32_map = "stm32f469",
                    ))]
                    OC2CE { $($oc2ce Option)* }
                    OC2FE { $($oc2fe Option)* }
                    OC2M { $($oc2m Option)* }
                    OC2PE { $($oc2pe Option)* }
                    @Input CCMR1_Input;
                    CC1S { CC1S }
                    CC2S { $($cc2s Option)* }
                    IC1F { IC1F }
                    IC1PSC { IC1PSC }
                    IC2F { $($ic2f Option)* }
                    IC2PSC { $($ic2psc Option)* }
                }
                CCMR2 {
                    @Output $(
                        $ccmr2_output Option;
                        CC3S { CC3S }
                        CC4S { CC4S }
                        #[cfg(any(
                            drone_stm32_map = "stm32f401",
                            drone_stm32_map = "stm32f405",
                            drone_stm32_map = "stm32f407",
                            drone_stm32_map = "stm32f411",
                            drone_stm32_map = "stm32f412",
                            drone_stm32_map = "stm32f413",
                            drone_stm32_map = "stm32f427",
                            drone_stm32_map = "stm32f429",
                            drone_stm32_map = "stm32f446",
                            drone_stm32_map = "stm32f469",
                        ))]
                        OC3CE { OC3CE }
                        OC3FE { OC3FE }
                        OC3M { OC3M }
                        OC3PE { OC3PE }
                        #[cfg(any(
                            drone_stm32_map = "stm32f401",
                            drone_stm32_map = "stm32f405",
                            drone_stm32_map = "stm32f407",
                            drone_stm32_map = "stm32f411",
                            drone_stm32_map = "stm32f412",
                            drone_stm32_map = "stm32f413",
                            drone_stm32_map = "stm32f427",
                            drone_stm32_map = "stm32f429",
                            drone_stm32_map = "stm32f446",
                            drone_stm32_map = "stm32f469",
                        ))]
                        OC4CE { OC4CE }
                        OC4FE { OC4FE }
                        OC4M { OC4M }
                        OC4PE { OC4PE }
                    )*
                    @Input $(
                        $ccmr2_input Option;
                        CC3S { CC3S }
                        CC4S { CC4S }
                        IC3F { IC3F }
                        IC3PSC { IC3PSC }
                        IC4F { IC4F }
                        IC4PSC { IC4PSC }
                    )*
                }
                CCER {
                    CCER;
                    CC1E { CC1E }
                    CC1NP { CC1NP }
                    CC1P { CC1P }
                    CC2E { $($cc2e Option)* }
                    CC2NP { $($cc2np Option)* }
                    CC2P { $($cc2p Option)* }
                    CC3E { $($cc3e Option)* }
                    CC3NP { $($cc3np Option)* }
                    CC3P { $($cc3p Option)* }
                    CC4E { $($cc4e Option)* }
                    CC4NP { $($cc4np Option)* }
                    CC4P { $($cc4p Option)* }
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
                CCR1 {
                    CCR1;
                    CCR1 { CCR1 }
                }
                CCR2 {
                    $(
                        $ccr2 Option;
                        CCR2 { CCR2 }
                    )*
                }
                CCR3 {
                    $(
                        $ccr3 Option;
                        CCR3 { CCR3 }
                    )*
                }
                CCR4 {
                    $(
                        $ccr4 Option;
                        CCR4 { CCR4 }
                    )*
                }
                DCR {
                    $(
                        $dcr Option;
                        DBA { DBA }
                        DBL { DBL }
                    )*
                }
                DMAR {
                    $(
                        $dmar Option;
                        DMAB { DMAB }
                    )*
                }
                OR1 {
                    $(
                        $or1 Option;
                        #[cfg(any(
                            drone_stm32_map = "stm32f401",
                            drone_stm32_map = "stm32f405",
                            drone_stm32_map = "stm32f407",
                            drone_stm32_map = "stm32f411",
                            drone_stm32_map = "stm32f412",
                            drone_stm32_map = "stm32f413",
                            drone_stm32_map = "stm32f427",
                            drone_stm32_map = "stm32f429",
                            drone_stm32_map = "stm32f446",
                            drone_stm32_map = "stm32f469",
                        ))]
                        ITR1_RMP { $($itr1_rmp Option)* }
                        TI1_RMP { $($ti1_rmp Option)* }
                        TI4_RMP { $($ti4_rmp Option)* }
                    )*
                }
            }
        }
    };
}

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM2 register tokens.",
    periph_tim2,
    "TIM2 peripheral variant.",
    Tim2,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM2EN,
    TIM2RST,
    TIM2LPEN,
    DBGMCU_APB1_FZ,
    DBG_TIM2_STOP,
    TIM2,
    (CMS, DIR, OPM),
    (CR2),
    (SMCR, ETP, ECE, ETPS, ETF),
    (CC1DE, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE, TDE, TIE, UDE),
    (CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF, TIF),
    (CC2G, CC3G, CC4G, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (CCR2, CCR3, CCR4),
    (DCR),
    (DMAR),
    (OR, ITR1_RMP,,),
}

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM3 register tokens.",
    periph_tim3,
    "TIM3 peripheral variant.",
    Tim3,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM3EN,
    TIM3RST,
    TIM3LPEN,
    DBGMCU_APB1_FZ,
    DBG_TIM3_STOP,
    TIM3,
    (CMS, DIR, OPM),
    (CR2),
    (SMCR, ETP, ECE, ETPS, ETF),
    (CC1DE, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE, TDE, TIE, UDE),
    (CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF, TIF),
    (CC2G, CC3G, CC4G, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (CCR2, CCR3, CCR4),
    (DCR),
    (DMAR),
    (),
}

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM4 register tokens.",
    periph_tim4,
    "TIM4 peripheral variant.",
    Tim4,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM4EN,
    TIM4RST,
    TIM4LPEN,
    DBGMCU_APB1_FZ,
    DBG_TIM4_STOP,
    TIM4,
    (CMS, DIR, OPM),
    (CR2),
    (SMCR, ETP, ECE, ETPS, ETF),
    (CC1DE, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE, TDE, TIE, UDE),
    (CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF, TIF),
    (CC2G, CC3G, CC4G, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (CCR2, CCR3, CCR4),
    (DCR),
    (DMAR),
    (),
}

map_general_tim! {
    "Extracts TIM5 register tokens.",
    periph_tim5,
    "TIM5 peripheral variant.",
    Tim5,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM5EN,
    TIM5RST,
    TIM5LPEN,
    DBGMCU_APB1_FZ,
    DBG_TIM5_STOP,
    TIM5,
    (CMS, DIR, OPM),
    (CR2),
    (SMCR, ETP, ECE, ETPS, ETF),
    (CC1DE, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE, TDE, TIE, UDE),
    (CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF, TIF),
    (CC2G, CC3G, CC4G, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (CCR2, CCR3, CCR4),
    (DCR),
    (DMAR),
    (OR,,, TI4_RMP),
}

map_general_tim! {
    "Extracts TIM9 register tokens.",
    periph_tim9,
    "TIM9 peripheral variant.",
    Tim9,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    TIM9EN,
    TIM9RST,
    TIM9LPEN,
    DBGMCU_APB2_FZ,
    DBG_TIM9_STOP,
    TIM9,
    (,, OPM),
    (),
    (SMCR,,,,),
    (,, CC2IE,,,,,, TIE,),
    (CC2IF, CC2OF,,,,, TIF),
    (CC2G,,, TG),
    (CC2S,,, OC2FE, OC2M, OC2PE, IC2F, IC2PSC),
    (,),
    (CC2E, CC2NP, CC2P,,,,,,),
    (CCR2,,),
    (),
    (),
    (),
}

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM10 register tokens.",
    periph_tim10,
    "TIM10 peripheral variant.",
    Tim10,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    TIM10EN,
    TIM10RST,
    TIM10LPEN,
    DBGMCU_APB2_FZ,
    DBG_TIM10_STOP,
    TIM10,
    (,,),
    (),
    (),
    (,,,,,,,,,),
    (,,,,,,),
    (,,,),
    (,,,,,,,),
    (,),
    (,,,,,,,,),
    (,,),
    (),
    (),
    (),
}

map_general_tim! {
    "Extracts TIM11 register tokens.",
    periph_tim11,
    "TIM11 peripheral variant.",
    Tim11,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    TIM11EN,
    TIM11RST,
    TIM11LPEN,
    DBGMCU_APB2_FZ,
    DBG_TIM11_STOP,
    TIM11,
    (,,),
    (),
    (),
    (,,,,,,,,,),
    (,,,,,,),
    (,,,),
    (,,,,,,,),
    (,),
    (,,,,,,,,),
    (,,),
    (),
    (),
    (OR,, TI1_RMP,),
}

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM12 register tokens.",
    periph_tim12,
    "TIM12 peripheral variant.",
    Tim12,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM12EN,
    TIM12RST,
    TIM12LPEN,
    DBGMCU_APB1_FZ,
    DBG_TIM12_STOP,
    TIM12,
    (,, OPM),
    (),
    (SMCR,,,,),
    (,, CC2IE,,,,,, TIE,),
    (CC2IF, CC2OF,,,,, TIF),
    (CC2G,,, TG),
    (CC2S,,, OC2FE, OC2M, OC2PE, IC2F, IC2PSC),
    (,),
    (CC2E, CC2NP, CC2P,,,,,,),
    (CCR2,,),
    (),
    (),
    (),
}

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM13 register tokens.",
    periph_tim13,
    "TIM13 peripheral variant.",
    Tim13,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM13EN,
    TIM13RST,
    TIM13LPEN,
    DBGMCU_APB1_FZ,
    DBG_TIM13_STOP,
    TIM13,
    (,,),
    (),
    (),
    (,,,,,,,,,),
    (,,,,,,),
    (,,,),
    (,,,,,,,),
    (,),
    (,,,,,,,,),
    (,,),
    (),
    (),
    (),
}

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM14 register tokens.",
    periph_tim14,
    "TIM14 peripheral variant.",
    Tim14,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM14EN,
    TIM14RST,
    TIM14LPEN,
    DBGMCU_APB1_FZ,
    DBG_TIM14_STOP,
    TIM14,
    (,,),
    (),
    (),
    (,,,,,,,,,),
    (,,,,,,),
    (,,,),
    (,,,,,,,),
    (,),
    (,,,,,,,,),
    (,,),
    (),
    (),
    (),
}

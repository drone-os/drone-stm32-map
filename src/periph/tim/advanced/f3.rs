//! Advanced-control timers.
//! for STM32F3 Series of mixed-signal MCUs with DSP and FPU instructions.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic advanced-control timer peripheral variant.
    pub trait AdvancedTimMap {}

    /// Generic advanced-control timer peripheral.
    pub struct AdvancedTimPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            TIMEN { RwRwRegFieldBitBand }
        }
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            TIMRST { RwRwRegFieldBitBand }
        }
    }
    TIM {
        CR1 {
            0x20 RwRegBitBand;
            CEN { RwRwRegFieldBitBand }
            UDIS { RwRwRegFieldBitBand }
            URS { RwRwRegFieldBitBand }
            OPM { RwRwRegFieldBitBand }
            DIR { RwRwRegFieldBitBand }
            CMS { RwRwRegFieldBits }
            ARPE { RwRwRegFieldBitBand }
            CKD { RwRwRegFieldBits }
            UIFREMAP { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
            CCPC { RwRwRegFieldBitBand }
            CCUS { RwRwRegFieldBitBand }
            CCDS { RwRwRegFieldBitBand }
            MMS { RwRwRegFieldBits }
            TI1S { RwRwRegFieldBitBand }
            OIS1 { RwRwRegFieldBitBand }
            OIS1N { RwRwRegFieldBitBand }
            OIS2 { RwRwRegFieldBitBand }
            OIS2N { RwRwRegFieldBitBand }
            OIS3 { RwRwRegFieldBitBand }
            OIS3N { RwRwRegFieldBitBand }
            OIS4 { RwRwRegFieldBitBand }
            OIS5 { RwRwRegFieldBitBand }
            OIS6 { RwRwRegFieldBitBand }
            MMS2 { RwRwRegFieldBits }
        }
        SMCR {
            0x20 RwRegBitBand;
            SMS { RwRwRegFieldBits }
            OCCS { RwRwRegFieldBitBand }
            TS { RwRwRegFieldBits }
            MSM { RwRwRegFieldBitBand }
            ETF { RwRwRegFieldBits }
            ETPS { RwRwRegFieldBits }
            ETP { RwRwRegFieldBitBand }
            ECE { RwRwRegFieldBitBand }
            SMS3 { RwRwRegFieldBitBand }
        }
        DIER {
            0x20 RwRegBitBand;
            UIE { RwRwRegFieldBitBand }
            CC1IE { RwRwRegFieldBitBand }
            CC2IE { RwRwRegFieldBitBand }
            CC3IE { RwRwRegFieldBitBand }
            CC4IE { RwRwRegFieldBitBand }
            COMIE { RwRwRegFieldBitBand }
            TIE { RwRwRegFieldBitBand }
            BIE { RwRwRegFieldBitBand }
            UDE { RwRwRegFieldBitBand }
            CC1DE { RwRwRegFieldBitBand }
            CC2DE { RwRwRegFieldBitBand }
            CC3DE { RwRwRegFieldBitBand }
            CC4DE { RwRwRegFieldBitBand }
            COMDE { RwRwRegFieldBitBand }
            TDE { RwRwRegFieldBitBand }
        }
        SR {
            0x20 RwRegBitBand;
            UIF { RwRwRegFieldBitBand }
            CC1IF { RwRwRegFieldBitBand }
            CC2IF { RwRwRegFieldBitBand }
            CC3IF { RwRwRegFieldBitBand }
            CC4IF { RwRwRegFieldBitBand }
            COMIF { RwRwRegFieldBitBand }
            TIF { RwRwRegFieldBitBand }
            BIF { RwRwRegFieldBitBand }
            B2IF { RwRwRegFieldBitBand }
            CC1OF { RwRwRegFieldBitBand }
            CC2OF { RwRwRegFieldBitBand }
            CC3OF { RwRwRegFieldBitBand }
            CC4OF { RwRwRegFieldBitBand }
            CC5IF { RwRwRegFieldBitBand }
            CC6IF { RwRwRegFieldBitBand }
        }
        EGR {
            0x20 WoRegBitBand;
            UG { WoWoRegFieldBitBand }
            CC1G { WoWoRegFieldBitBand }
            CC2G { WoWoRegFieldBitBand }
            CC3G { WoWoRegFieldBitBand }
            CC4G { WoWoRegFieldBitBand }
            COMG { WoWoRegFieldBitBand }
            TG { WoWoRegFieldBitBand }
            BG { WoWoRegFieldBitBand }
            B2G { WoWoRegFieldBitBand }
        }
        CCMR1 {
            @Output 0x20 RwRegBitBand;
            CC1S { RwRwRegFieldBits }
            OC1FE { RwRwRegFieldBitBand }
            OC1PE { RwRwRegFieldBitBand }
            OC1M { RwRwRegFieldBits }
            OC1CE { RwRwRegFieldBitBand }
            CC2S { RwRwRegFieldBits }
            OC2FE { RwRwRegFieldBitBand }
            OC2PE { RwRwRegFieldBitBand }
            OC2M { RwRwRegFieldBits }
            OC2CE { RwRwRegFieldBitBand }
            OC1M_3 { RwRwRegFieldBitBand }
            OC2M_3 { RwRwRegFieldBitBand }
            @Input 0x20 RwRegBitBand;
            CC1S { RwRwRegFieldBits }
            IC1PSC { RwRwRegFieldBits }
            IC1F { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits }
            IC2PSC { RwRwRegFieldBits }
            IC2F { RwRwRegFieldBits }
        }
        CCMR2 {
            @Output 0x20 RwRegBitBand;
            CC3S { RwRwRegFieldBits }
            OC3FE { RwRwRegFieldBitBand }
            OC3PE { RwRwRegFieldBitBand }
            OC3M { RwRwRegFieldBits }
            OC3CE { RwRwRegFieldBitBand }
            CC4S { RwRwRegFieldBits }
            OC4FE { RwRwRegFieldBitBand }
            OC4PE { RwRwRegFieldBitBand }
            OC4M { RwRwRegFieldBits }
            OC4CE { RwRwRegFieldBitBand }
            OC3M_3 { RwRwRegFieldBitBand }
            OC4M_3 { RwRwRegFieldBitBand }
            @Input 0x20 RwRegBitBand;
            CC3S { RwRwRegFieldBits }
            IC3PSC { RwRwRegFieldBits }
            IC3F { RwRwRegFieldBits }
            CC4S { RwRwRegFieldBits }
            IC4PSC { RwRwRegFieldBits }
            IC4F { RwRwRegFieldBits }
        }
        CCMR3 {
            @Output 0x20 RwRegBitBand;
            OC5FE { RwRwRegFieldBitBand }
            OC5PE { RwRwRegFieldBitBand }
            OC5M { RwRwRegFieldBits }
            OC5CE { RwRwRegFieldBitBand }
            OC6FE { RwRwRegFieldBitBand }
            OC6PE { RwRwRegFieldBitBand }
            OC6M { RwRwRegFieldBits }
            OC6CE { RwRwRegFieldBitBand }
            OC5M_3 { RwRwRegFieldBitBand }
            OC6M_3 { RwRwRegFieldBitBand }
        }
        CCER {
            0x20 RwRegBitBand;
            CC1E { RwRwRegFieldBitBand }
            CC1P { RwRwRegFieldBitBand }
            CC1NE { RwRwRegFieldBitBand }
            CC1NP { RwRwRegFieldBitBand }
            CC2E { RwRwRegFieldBitBand }
            CC2P { RwRwRegFieldBitBand }
            CC2NE { RwRwRegFieldBitBand }
            CC2NP { RwRwRegFieldBitBand }
            CC3E { RwRwRegFieldBitBand }
            CC3P { RwRwRegFieldBitBand }
            CC3NE { RwRwRegFieldBitBand }
            CC3NP { RwRwRegFieldBitBand }
            CC4E { RwRwRegFieldBitBand }
            CC4P { RwRwRegFieldBitBand }
            CC4NP { RwRwRegFieldBitBand }
            CC5E { RwRwRegFieldBitBand }
            CC5P { RwRwRegFieldBitBand }
            CC6E { RwRwRegFieldBitBand }
            CC6P { RwRwRegFieldBitBand }
        }
        CNT {
            0x20 RwRegBitBand;
            CNT { RwRwRegFieldBits }
            UIFCPY { RoRwRegFieldBitBand }
        }
        PSC {
            0x20 RwRegBitBand;
            PSC { RwRwRegFieldBits }
        }
        ARR {
            0x20 RwRegBitBand;
            ARR { RwRwRegFieldBits }
        }
        RCR {
            0x20 RwRegBitBand;
            REP { RwRwRegFieldBits }
        }
        CCR1 {
            0x20 RwRegBitBand;
            CCR1 { RwRwRegFieldBits }
        }
        CCR2 {
            0x20 RwRegBitBand;
            CCR2 { RwRwRegFieldBits }
        }
        CCR3 {
            0x20 RwRegBitBand;
            CCR3 { RwRwRegFieldBits }
        }
        CCR4 {
            0x20 RwRegBitBand;
            CCR4 { RwRwRegFieldBits }
        }
        CCR5 {
            0x20 RwRegBitBand;
            CCR5 { RwRwRegFieldBits }
            GC5C1 { RwRwRegFieldBitBand }
            GC5C2 { RwRwRegFieldBitBand }
            GC5C3 { RwRwRegFieldBitBand }
        }
        CCR6 {
            0x20 RwRegBitBand;
            CCR6 { RwRwRegFieldBits }
        }
        BDTR {
            0x20 RwRegBitBand;
            DTG { RwRwRegFieldBits }
            LOCK { RwRwRegFieldBits }
            OSSI { RwRwRegFieldBitBand }
            OSSR { RwRwRegFieldBitBand }
            BKE { RwRwRegFieldBitBand }
            BKP { RwRwRegFieldBitBand }
            AOE { RwRwRegFieldBitBand }
            MOE { RwRwRegFieldBitBand }
            BKF { RwRwRegFieldBits }
            BK2F { RwRwRegFieldBits }
            BK2E { RwRwRegFieldBitBand }
            BK2P { RwRwRegFieldBitBand }
        }
        DCR {
            0x20 RwRegBitBand;
            DBA { RwRwRegFieldBits }
            DBL { RwRwRegFieldBits }
        }
        DMAR {
            0x20 RwRegBitBand;
            DMAB { RwRwRegFieldBits }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_advanced_tim {
    (
        $tim_macro_doc:expr,
        $tim_macro:ident,
        $tim_ty_doc:expr,
        $tim_ty:ident,
        $timen:ident,
        $timrst:ident,
        $tim:ident,
        $busenr:ident,
        $busrstr:ident,
        $bkdfbke:ident,
        $bk2dfbke:ident,($($etr_adc1_rmp:ident)*),
    ) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl AdvancedTimMap for $tim_ty {}

            drone_stm32_map_pieces::reg;
            crate::advanced;

            RCC {
                BUSENR {
                    $busenr Shared;
                    TIMEN { $timen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    TIMRST { $timrst }
                }
            }
            TIM {
                $tim;
                CR1 {
                    CR1;
                    CEN { CEN }
                    UDIS { UDIS }
                    URS { URS }
                    OPM { OPM }
                    DIR { DIR }
                    CMS { CMS }
                    ARPE { ARPE }
                    CKD { CKD }
                    UIFREMAP { UIFREMAP }
                }
                CR2 {
                    CR2;
                    CCPC { CCPC }
                    CCUS { CCUS }
                    CCDS { CCDS }
                    MMS { MMS }
                    TI1S { TI1S }
                    OIS1 { OIS1 }
                    OIS1N { OIS1N }
                    OIS2 { OIS2 }
                    OIS2N { OIS2N }
                    OIS3 { OIS3 }
                    OIS3N { OIS3N }
                    OIS4 { OIS4 }
                    OIS5 { OIS5 }
                    OIS6 { OIS6 }
                    MMS2 { MMS2 }
                }
                SMCR {
                    SMCR;
                    SMS { SMS }
                    OCCS { OCCS }
                    TS { TS }
                    MSM { MSM }
                    ETF { ETF }
                    ETPS { ETPS }
                    ETP { ETP }
                    ECE { ECE }
                    SMS3 { SMS3 }
                }
                DIER {
                    DIER;
                    UIE { UIE }
                    CC1IE { CC1IE }
                    CC2IE { CC2IE }
                    CC3IE { CC3IE }
                    CC4IE { CC4IE }
                    COMIE { COMIE }
                    TIE { TIE }
                    BIE { BIE }
                    UDE { UDE }
                    CC1DE { CC1DE }
                    CC2DE { CC2DE }
                    CC3DE { CC3DE }
                    CC4DE { CC4DE }
                    COMDE { COMDE }
                    TDE { TDE }
                }
                SR {
                    SR;
                    UIF { UIF }
                    CC1IF { CC1IF }
                    CC2IF { CC2IF }
                    CC3IF { CC3IF }
                    CC4IF { CC4IF }
                    COMIF { COMIF }
                    TIF { TIF }
                    BIF { BIF }
                    B2IF { B2IF }
                    CC1OF { CC1OF }
                    CC2OF { CC2OF }
                    CC3OF { CC3OF }
                    CC4OF { CC4OF }
                    CC5IF { CC5IF }
                    CC6IF { CC6IF }
                }
                EGR {
                    EGR;
                    UG { UG }
                    CC1G { CC1G }
                    CC2G { CC2G }
                    CC3G { CC3G }
                    CC4G { CC4G }
                    COMG { COMG }
                    TG { TG }
                    BG { BG }
                    B2G { B2G }
                }
                CCMR1 {
                    @Output CCMR1_Output;
                    CC1S { CC1S }
                    OC1FE { OC1FE }
                    OC1PE { OC1PE }
                    OC1M { OC1M }
                    OC1CE { OC1CE }
                    CC2S { CC2S }
                    OC2FE { OC2FE }
                    OC2PE { OC2PE }
                    OC2M { OC2M }
                    OC2CE { OC2CE }
                    OC1M_3 { OC1M_3 }
                    OC2M_3 { OC2M_3 }
                    @Input CCMR1_Input;
                    CC1S { CC1S }
                    IC1PSC { IC1PSC }
                    IC1F { IC1F }
                    CC2S { CC2S }
                    IC2PSC { IC2PSC }
                    IC2F { IC2F }
                }
                CCMR2 {
                    @Output CCMR2_Output;
                    CC3S { CC3S }
                    OC3FE { OC3FE }
                    OC3PE { OC3PE }
                    OC3M { OC3M }
                    OC3CE { OC3CE }
                    CC4S { CC4S }
                    OC4FE { OC4FE }
                    OC4PE { OC4PE }
                    OC4M { OC4M }
                    OC4CE { OC4CE }
                    OC3M_3 { OC3M_3 }
                    OC4M_3 { OC4M_3 }
                    @Input CCMR2_Input;
                    CC3S { CC3S }
                    IC3PSC { IC3PSC }
                    IC3F { IC3F }
                    CC4S { CC4S }
                    IC4PSC { IC4PSC }
                    IC4F { IC4F }
                }
                CCMR3 {
                    @Output CCMR3_Output;
                    OC5FE { OC5FE }
                    OC5PE { OC5PE }
                    OC5M { OC5M }
                    OC5CE { OC5CE }
                    OC6FE { OC6FE }
                    OC6PE { OC6PE }
                    OC6M { OC6M }
                    OC6CE { OC6CE }
                    OC5M_3 { OC5M_3 }
                    OC6M_3 { OC6M_3 }
                }
                CCER {
                    CCER;
                    CC1E { CC1E }
                    CC1P { CC1P }
                    CC1NE { CC1NE }
                    CC1NP { CC1NP }
                    CC2E { CC2E }
                    CC2P { CC2P }
                    CC2NE { CC2NE }
                    CC2NP { CC2NP }
                    CC3E { CC3E }
                    CC3P { CC3P }
                    CC3NE { CC3NE }
                    CC3NP { CC3NP }
                    CC4E { CC4E }
                    CC4P { CC4P }
                    CC4NP { CC4NP }
                    CC5E { CC5E }
                    CC5P { CC5P }
                    CC6E { CC6E }
                    CC6P { CC6P }
                }
                CNT {
                    CNT;
                    CNT { CNT }
                    UIFCPY { UIFCPY }
                }
                PSC {
                    PSC;
                    PSC { PSC }
                }
                ARR {
                    ARR;
                    ARR { ARR }
                }
                RCR {
                    RCR;
                    REP { REP }
                }
                CCR1 {
                    CCR1;
                    CCR1 { CCR1 }
                }
                CCR2 {
                    CCR2;
                    CCR2 { CCR2 }
                }
                CCR3 {
                    CCR3;
                    CCR3 { CCR3 }
                }
                CCR4 {
                    CCR4;
                    CCR4 { CCR4 }
                }
                CCR5 {
                    CCR5;
                    CCR5 { CCR5 }
                    GC5C1 { GC5C1 }
                    GC5C2 { GC5C2 }
                    GC5C3 { GC5C3 }
                }
                CCR6 {
                    CCR6;
                    CCR6 { CCR6 }
                }
                BDTR {
                    BDTR;
                    DTG { DTG }
                    LOCK { LOCK }
                    OSSI { OSSI }
                    OSSR { OSSR }
                    BKE { BKE }
                    BKP { BKP }
                    AOE { AOE }
                    MOE { MOE }
                    BKF { BKF }
                    BK2F { BK2F }
                    BK2E { BK2E }
                    BK2P { BK2P }
                }
                DCR {
                    DCR;
                    DBA { DBA }
                    DBL { DBL }
                }
                DMAR {
                    DMAR;
                    DMAB { DMAB }
                }
            }
        }
    };
}

map_advanced_tim! {
    "Extracts TIM1 register tokens.",
    periph_tim1,
    "TIM1 peripheral variant",
    Tim1,
    TIM1EN,
    TIM1RST,
    TIM1,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    BK2DFBK0E,
    (),
}

map_advanced_tim! {
    "Extracts TIM8 register tokens.",
    periph_tim8,
    "TIM8 peripheral variant",
    Tim8,
    TIM8EN,
    TIM8RST,
    TIM8,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    BK2DFBK0E,
    (),
}

map_advanced_tim! {
    "Extracts TIM20 register tokens.",
    periph_tim20,
    "TIM20 peripheral variant",
    Tim2020,
    TIM20EN,
    TIM20RST,
    TIM20,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    BK2DFBK0E,
    (),
}


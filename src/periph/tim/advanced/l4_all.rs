//! Advanced-control timers.

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
            CKD { RwRwRegFieldBits }
            CMS { RwRwRegFieldBits }
            DIR { RwRwRegFieldBitBand }
            OPM { RwRwRegFieldBitBand }
            UDIS { RwRwRegFieldBitBand }
            URS { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
            CCDS { RwRwRegFieldBitBand }
            CCPC { RwRwRegFieldBitBand }
            CCUS { RwRwRegFieldBitBand }
            MMS { RwRwRegFieldBits }
            OIS1N { RwRwRegFieldBitBand }
            OIS1 { RwRwRegFieldBitBand }
            OIS2N { RwRwRegFieldBitBand }
            OIS2 { RwRwRegFieldBitBand }
            OIS3N { RwRwRegFieldBitBand }
            OIS3 { RwRwRegFieldBitBand }
            OIS4 { RwRwRegFieldBitBand }
            TI1S { RwRwRegFieldBitBand }
        }
        SMCR {
            0x20 RwRegBitBand;
            ECE { RwRwRegFieldBitBand }
            ETF { RwRwRegFieldBits }
            ETP { RwRwRegFieldBitBand }
            ETPS { RwRwRegFieldBits }
            MSM { RwRwRegFieldBitBand }
            SMS { RwRwRegFieldBits }
            TS { RwRwRegFieldBits }
        }
        DIER {
            0x20 RwRegBitBand;
            BIE { RwRwRegFieldBitBand }
            CC1DE { RwRwRegFieldBitBand }
            CC1IE { RwRwRegFieldBitBand }
            CC2DE { RwRwRegFieldBitBand }
            CC2IE { RwRwRegFieldBitBand }
            CC3DE { RwRwRegFieldBitBand }
            CC3IE { RwRwRegFieldBitBand }
            CC4DE { RwRwRegFieldBitBand }
            CC4IE { RwRwRegFieldBitBand }
            COMDE { RwRwRegFieldBitBand }
            COMIE { RwRwRegFieldBitBand }
            TDE { RwRwRegFieldBitBand }
            TIE { RwRwRegFieldBitBand }
            UDE { RwRwRegFieldBitBand }
            UIE { RwRwRegFieldBitBand }
        }
        SR {
            0x20 RwRegBitBand;
            BIF { RwRwRegFieldBitBand }
            CC1IF { RwRwRegFieldBitBand }
            CC1OF { RwRwRegFieldBitBand }
            CC2IF { RwRwRegFieldBitBand }
            CC2OF { RwRwRegFieldBitBand }
            CC3IF { RwRwRegFieldBitBand }
            CC3OF { RwRwRegFieldBitBand }
            CC4IF { RwRwRegFieldBitBand }
            CC4OF { RwRwRegFieldBitBand }
            COMIF { RwRwRegFieldBitBand }
            TIF { RwRwRegFieldBitBand }
            UIF { RwRwRegFieldBitBand }
        }
        EGR {
            0x20 WoRegBitBand;
            BG { WoWoRegFieldBitBand }
            CC1G { WoWoRegFieldBitBand }
            CC2G { WoWoRegFieldBitBand }
            CC3G { WoWoRegFieldBitBand }
            CC4G { WoWoRegFieldBitBand }
            COMG { WoWoRegFieldBitBand }
            TG { WoWoRegFieldBitBand }
            UG { WoWoRegFieldBitBand }
        }
        CCMR1 {
            @Output 0x20 RwRegBitBand;
            CC1S { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits }
            OC1CE { RwRwRegFieldBitBand }
            OC1FE { RwRwRegFieldBitBand }
            OC1M { RwRwRegFieldBits }
            OC1PE { RwRwRegFieldBitBand }
            OC2CE { RwRwRegFieldBitBand }
            OC2FE { RwRwRegFieldBitBand }
            OC2M { RwRwRegFieldBits }
            OC2PE { RwRwRegFieldBitBand }
            @Input 0x20 RwRegBitBand;
            CC1S { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits }
            IC1F { RwRwRegFieldBits }
            IC1PSC { RwRwRegFieldBits }
            IC2F { RwRwRegFieldBits }
            IC2PSC { RwRwRegFieldBits }
        }
        CCMR2 {
            @Output 0x20 RwRegBitBand;
            CC3S { RwRwRegFieldBits }
            CC4S { RwRwRegFieldBits }
            OC3CE { RwRwRegFieldBitBand }
            OC3FE { RwRwRegFieldBitBand }
            OC3M { RwRwRegFieldBits }
            OC3PE { RwRwRegFieldBitBand }
            OC4CE { RwRwRegFieldBitBand }
            OC4FE { RwRwRegFieldBitBand }
            OC4M { RwRwRegFieldBits }
            OC4PE { RwRwRegFieldBitBand }
            @Input 0x20 RwRegBitBand;
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
            CC1NE { RwRwRegFieldBitBand }
            CC1NP { RwRwRegFieldBitBand }
            CC1P { RwRwRegFieldBitBand }
            CC2E { RwRwRegFieldBitBand }
            CC2NE { RwRwRegFieldBitBand }
            CC2NP { RwRwRegFieldBitBand }
            CC2P { RwRwRegFieldBitBand }
            CC3E { RwRwRegFieldBitBand }
            CC3NE { RwRwRegFieldBitBand }
            CC3NP { RwRwRegFieldBitBand }
            CC3P { RwRwRegFieldBitBand }
            CC4E { RwRwRegFieldBitBand }
            CC4P { RwRwRegFieldBitBand }
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
        BDTR {
            0x20 RwRegBitBand;
            AOE { RwRwRegFieldBitBand }
            BKE { RwRwRegFieldBitBand }
            BKP { RwRwRegFieldBitBand }
            DTG { RwRwRegFieldBits }
            LOCK { RwRwRegFieldBits }
            MOE { RwRwRegFieldBitBand }
            OSSI { RwRwRegFieldBitBand }
            OSSR { RwRwRegFieldBitBand }
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
        OR1 {
            0x20 RwRegBitBand;
            ETR_ADC1_RMP { RwRwRegFieldBits Option }
            TI1_RMP { RwRwRegFieldBitBand }
        }
        CCMR3_Output {
            0x20 RwRegBitBand;
            OC5CE { RwRwRegFieldBitBand }
            OC5FE { RwRwRegFieldBitBand }
            OC5M_bit3 { RwRwRegFieldBits }
            OC5M { RwRwRegFieldBits }
            OC5PE { RwRwRegFieldBitBand }
            OC6CE { RwRwRegFieldBitBand }
            OC6FE { RwRwRegFieldBitBand }
            OC6M_bit3 { RwRwRegFieldBitBand }
            OC6M { RwRwRegFieldBits }
            OC6PE { RwRwRegFieldBitBand }
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
        OR2 {
            0x20 RwRegBitBand;
            BKCMP1E { RwRwRegFieldBitBand }
            BKCMP1P { RwRwRegFieldBitBand }
            BKCMP2E { RwRwRegFieldBitBand }
            BKCMP2P { RwRwRegFieldBitBand }
            BKDFBKE { RwRwRegFieldBitBand }
            BKINE { RwRwRegFieldBitBand }
            BKINP { RwRwRegFieldBitBand }
            ETRSEL { RwRwRegFieldBits }
        }
        OR3 {
            0x20 RwRegBitBand;
            BK2CMP1E { RwRwRegFieldBitBand }
            BK2CMP1P { RwRwRegFieldBitBand }
            BK2CMP2E { RwRwRegFieldBitBand }
            BK2CMP2P { RwRwRegFieldBitBand }
            BK2DFBKE { RwRwRegFieldBitBand }
            BK2INE { RwRwRegFieldBitBand }
            BK2INP { RwRwRegFieldBitBand }
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
        $timsmen:ident,
        $tim:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $bkdfbke:ident,
        $bk2dfbke:ident,
        ($($etr_adc1_rmp:ident)*),
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
                    CKD { CKD }
                    CMS { CMS }
                    DIR { DIR }
                    OPM { OPM }
                    UDIS { UDIS }
                    URS { URS }
                }
                CR2 {
                    CR2;
                    CCDS { CCDS }
                    CCPC { CCPC }
                    CCUS { CCUS }
                    MMS { MMS }
                    OIS1N { OIS1N }
                    OIS1 { OIS1 }
                    OIS2N { OIS2N }
                    OIS2 { OIS2 }
                    OIS3N { OIS3N }
                    OIS3 { OIS3 }
                    OIS4 { OIS4 }
                    TI1S { TI1S }
                }
                SMCR {
                    SMCR;
                    ECE { ECE }
                    ETF { ETF }
                    ETP { ETP }
                    ETPS { ETPS }
                    MSM { MSM }
                    SMS { SMS }
                    TS { TS }
                }
                DIER {
                    DIER;
                    BIE { BIE }
                    CC1DE { CC1DE }
                    CC1IE { CC1IE }
                    CC2DE { CC2DE }
                    CC2IE { CC2IE }
                    CC3DE { CC3DE }
                    CC3IE { CC3IE }
                    CC4DE { CC4DE }
                    CC4IE { CC4IE }
                    COMDE { COMDE }
                    COMIE { COMIE }
                    TDE { TDE }
                    TIE { TIE }
                    UDE { UDE }
                    UIE { UIE }
                }
                SR {
                    SR;
                    BIF { BIF }
                    CC1IF { CC1IF }
                    CC1OF { CC1OF }
                    CC2IF { CC2IF }
                    CC2OF { CC2OF }
                    CC3IF { CC3IF }
                    CC3OF { CC3OF }
                    CC4IF { CC4IF }
                    CC4OF { CC4OF }
                    COMIF { COMIF }
                    TIF { TIF }
                    UIF { UIF }
                }
                EGR {
                    EGR;
                    BG { BG }
                    CC1G { CC1G }
                    CC2G { CC2G }
                    CC3G { CC3G }
                    CC4G { CC4G }
                    COMG { COMG }
                    TG { TG }
                    UG { UG }
                }
                CCMR1 {
                    @Output CCMR1_Output;
                    CC1S { CC1S }
                    CC2S { CC2S }
                    OC1CE { OC1CE }
                    OC1FE { OC1FE }
                    OC1M { OC1M }
                    OC1PE { OC1PE }
                    OC2CE { OC2CE }
                    OC2FE { OC2FE }
                    OC2M { OC2M }
                    OC2PE { OC2PE }
                    @Input CCMR1_Input;
                    CC1S { CC1S }
                    CC2S { CC2S }
                    IC1F { IC1F }
                    IC1PSC { IC1PSC }
                    IC2F { IC2F }
                    IC2PSC { IC2PSC }
                }
                CCMR2 {
                    @Output CCMR2_Output;
                    CC3S { CC3S }
                    CC4S { CC4S }
                    OC3CE { OC3CE }
                    OC3FE { OC3FE }
                    OC3M { OC3M }
                    OC3PE { OC3PE }
                    OC4CE { OC4CE }
                    OC4FE { OC4FE }
                    OC4M { OC4M }
                    OC4PE { OC4PE }
                    @Input CCMR2_Input;
                    CC3S { CC3S }
                    CC4S { CC4S }
                    IC3F { IC3F }
                    IC3PSC { IC3PSC }
                    IC4F { IC4F }
                    IC4PSC { IC4PSC }
                }
                CCER {
                    CCER;
                    CC1E { CC1E }
                    CC1NE { CC1NE }
                    CC1NP { CC1NP }
                    CC1P { CC1P }
                    CC2E { CC2E }
                    CC2NE { CC2NE }
                    CC2NP { CC2NP }
                    CC2P { CC2P }
                    CC3E { CC3E }
                    CC3NE { CC3NE }
                    CC3NP { CC3NP }
                    CC3P { CC3P }
                    CC4E { CC4E }
                    CC4P { CC4P }
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
                BDTR {
                    BDTR;
                    AOE { AOE }
                    BKE { BKE }
                    BKP { BKP }
                    DTG { DTG }
                    LOCK { LOCK }
                    MOE { MOE }
                    OSSI { OSSI }
                    OSSR { OSSR }
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
                OR1 {
                    OR1;
                    ETR_ADC1_RMP { $($etr_adc1_rmp Option)* }
                    TI1_RMP { TI1_RMP }
                }
                CCMR3_Output {
                    CCMR3_Output;
                    OC5CE { OC5CE }
                    OC5FE { OC5FE }
                    OC5M_bit3 { OC5M_bit3 }
                    OC5M { OC5M }
                    OC5PE { OC5PE }
                    OC6CE { OC6CE }
                    OC6FE { OC6FE }
                    OC6M_bit3 { OC6M_bit3 }
                    OC6M { OC6M }
                    OC6PE { OC6PE }
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
                OR2 {
                    OR2;
                    BKCMP1E { BKCMP1E }
                    BKCMP1P { BKCMP1P }
                    BKCMP2E { BKCMP2E }
                    BKCMP2P { BKCMP2P }
                    BKDFBKE { $bkdfbke }
                    BKINE { BKINE }
                    BKINP { BKINP }
                    ETRSEL { ETRSEL }
                }
                OR3 {
                    OR3;
                    BK2CMP1E { BK2CMP1E }
                    BK2CMP1P { BK2CMP1P }
                    BK2CMP2E { BK2CMP2E }
                    BK2CMP2P { BK2CMP2P }
                    BK2DFBKE { $bk2dfbke }
                    BK2INE { BK2INE }
                    BK2INP { BK2INP }
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
    TIM1SMEN,
    TIM1,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    BKDFBK0E,
    BK2DFBK0E,
    (ETR_ADC1_RMP),
}

#[cfg(any(
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_advanced_tim! {
    "Extracts TIM8 register tokens.",
    periph_tim8,
    "TIM8 peripheral variant",
    Tim8,
    TIM8EN,
    TIM8RST,
    TIM8SMEN,
    TIM8,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    BKDFBK2E,
    BK2DFBK3E,
    (),
}

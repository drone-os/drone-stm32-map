//! General-purpose timers.

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
    /// General-purpose timer.
    pub trait GeneralTimMap {}

    RCC {
        APBENR {
            0x20 RwRegBitBand Shared;
            TIMEN { RwRwRegFieldBitBand }
        }
        APBRSTR {
            0x20 RwRegBitBand Shared;
            TIMRST { RwRwRegFieldBitBand }
        }
        APBSMENR {
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
            CMS { RwRwRegFieldBits Option }
            DIR { RwRwRegFieldBitBand Option }
            OPM { RwRwRegFieldBitBand }
            UDIS { RwRwRegFieldBitBand }
            UIFREMAP { RwRwRegFieldBitBand }
            URS { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
            OIS2 { RwRwRegFieldBitBand Option }
            OIS1N { RwRwRegFieldBitBand Option }
            OIS1 { RwRwRegFieldBitBand Option }
            TI1S { RwRwRegFieldBitBand Option }
            MMS { RwRwRegFieldBits Option }
            CCDS { RwRwRegFieldBitBand }
            CCUS { RwRwRegFieldBitBand Option }
            CCPC { RwRwRegFieldBitBand Option }
        }
        SMCR {
            0x20 RwRegBitBand Option;
            SMS3 { RwRwRegFieldBitBand }
            ETP { RwRwRegFieldBitBand Option }
            ECE { RwRwRegFieldBitBand Option }
            ETPS { RwRwRegFieldBits Option }
            ETF { RwRwRegFieldBits Option }
            MSM { RwRwRegFieldBitBand }
            TS { RwRwRegFieldBits }
            SMS0_2 { RwRwRegFieldBits }
        }
        DIER {
            0x20 RwRegBitBand;
            BIE { RwRwRegFieldBitBand Option }
            CC1DE { RwRwRegFieldBitBand }
            CC1IE { RwRwRegFieldBitBand }
            CC2DE { RwRwRegFieldBitBand Option }
            CC2IE { RwRwRegFieldBitBand Option }
            CC3DE { RwRwRegFieldBitBand Option }
            CC3IE { RwRwRegFieldBitBand Option }
            CC4DE { RwRwRegFieldBitBand Option }
            CC4IE { RwRwRegFieldBitBand Option }
            COMDE { RwRwRegFieldBitBand Option }
            COMIE { RwRwRegFieldBitBand Option }
            TDE { RwRwRegFieldBitBand Option }
            TIE { RwRwRegFieldBitBand Option }
            UDE { RwRwRegFieldBitBand }
            UIE { RwRwRegFieldBitBand }
        }
        SR {
            0x20 RwRegBitBand;
            BIF { RwRwRegFieldBitBand Option }
            CC1IF { RwRwRegFieldBitBand }
            CC1OF { RwRwRegFieldBitBand }
            CC2IF { RwRwRegFieldBitBand Option }
            CC2OF { RwRwRegFieldBitBand Option }
            CC3IF { RwRwRegFieldBitBand Option }
            CC3OF { RwRwRegFieldBitBand Option }
            CC4IF { RwRwRegFieldBitBand Option }
            CC4OF { RwRwRegFieldBitBand Option }
            COMIF { RwRwRegFieldBitBand Option }
            TIF { RwRwRegFieldBitBand Option }
            UIF { RwRwRegFieldBitBand }
        }
        EGR {
            0x20 WoRegBitBand;
            BG { WoWoRegFieldBitBand Option }
            CC1G { WoWoRegFieldBitBand }
            CC2G { WoWoRegFieldBitBand Option }
            CC3G { WoWoRegFieldBitBand Option }
            CC4G { WoWoRegFieldBitBand Option }
            COMG { WoWoRegFieldBitBand Option }
            TG { WoWoRegFieldBitBand Option }
            UG { WoWoRegFieldBitBand }
        }
        CCMR1_Output {
            0x20 RwRegBitBand;
            CC1S { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits Option }
            OC1CE { RwRwRegFieldBitBand Option }
            OC1FE { RwRwRegFieldBitBand }
            OC1M0_2 { RwRwRegFieldBits }
            OC1M3 { RwRwRegFieldBitBand }
            OC1PE { RwRwRegFieldBitBand }
            OC2CE { RwRwRegFieldBitBand Option }
            OC2FE { RwRwRegFieldBitBand Option }
            OC2M0_2 { RwRwRegFieldBits Option }
            OC2M3 { RwRwRegFieldBitBand Option }
            OC2PE { RwRwRegFieldBitBand Option }
        }
        CCMR1_Input {
            0x20 RwRegBitBand;
            CC1S { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits Option }
            IC1F { RwRwRegFieldBits }
            IC1PSC { RwRwRegFieldBits }
            IC2F { RwRwRegFieldBits Option }
            IC2PSC { RwRwRegFieldBits Option }
        }
        CCMR2_Output {
            0x20 RwRegBitBand Option;
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
        }
        CCMR2_Input {
            0x20 RwRegBitBand Option;
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
            CC1NE { RwRwRegFieldBitBand Option }
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
            UIFCPY_CNT31 { RwRwRegFieldBitBand Option }
            UIFCPY { RoRwRegFieldBitBand Option }
            CNT_H { RwRwRegFieldBits Option }
            CNT_L { RwRwRegFieldBits }
        }
        PSC {
            0x20 RwRegBitBand;
            PSC { RwRwRegFieldBits }
        }
        ARR {
            0x20 RwRegBitBand;
            ARR_H { RwRwRegFieldBits Option }
            ARR_L { RwRwRegFieldBits }
        }
        RCR {
            0x20 RwRegBitBand Option;
            REP { RwRwRegFieldBits }
        }
        CCR1 {
            0x20 RwRegBitBand;
            CCR1_H { RwRwRegFieldBits Option }
            CCR1_L { RwRwRegFieldBits }
        }
        CCR2 {
            0x20 RwRegBitBand Option;
            CCR2_H { RwRwRegFieldBits }
            CCR2_L { RwRwRegFieldBits }
        }
        CCR3 {
            0x20 RwRegBitBand Option;
            CCR3_H { RwRwRegFieldBits }
            CCR3_L { RwRwRegFieldBits }
        }
        CCR4 {
            0x20 RwRegBitBand Option;
            CCR4_H { RwRwRegFieldBits }
            CCR4_L { RwRwRegFieldBits }
        }
        BDTR {
            0x20 RwRegBitBand Option;
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
            0x20 RwRegBitBand Option;
            ENCODER_MODE { RwRwRegFieldBits Option }
            ETR1_RMP { RwRwRegFieldBitBand Option }
            ITR1_RMP { RwRwRegFieldBitBand Option }
            TI1_RMP_BIT { RwRwRegFieldBitBand Option }
            TI1_RMP { RwRwRegFieldBits Option }
            TI4_RMP { RwRwRegFieldBits Option }
        }
        OR2 {
            0x20 RwRegBitBand Option;
            BKCMP1E { RwRwRegFieldBitBand Option }
            BKCMP1P { RwRwRegFieldBitBand Option }
            BKCMP2E { RwRwRegFieldBitBand Option }
            BKCMP2P { RwRwRegFieldBitBand Option }
            BKDFBKE { RwRwRegFieldBitBand Option }
            BKINE { RwRwRegFieldBitBand Option }
            BKINP { RwRwRegFieldBitBand Option }
            ETRSEL { RwRwRegFieldBits Option }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_general_tim {
    (
        $tim_macro_doc:expr,
        $tim_macro:ident,
        $tim_ty_doc:expr,
        $tim_ty:ident,
        $apbenr:ident,
        $apbrstr:ident,
        $apbsmenr:ident,
        $timen:ident,
        $timrst:ident,
        $timsmen:ident,
        $tim:ident,
        ($($cms:ident)?, $($dir:ident)?),
        (
            $($ois2:ident)?,
            $($ois1n:ident)?,
            $($ois1:ident)?,
            $($ti1s:ident)?,
            $($mms:ident)?,
            $($ccus:ident)?,
            $($ccpc:ident)?
        ),
        ($(
            $smcr:ident,
            $($etp:ident)?,
            $($ece:ident)?,
            $($etps:ident)?,
            $($etf:ident)?
        )?),
        (
            $($bie:ident)?,
            $($cc2de:ident)?,
            $($cc2ie:ident)?,
            $($cc3de:ident)?,
            $($cc3ie:ident)?,
            $($cc4de:ident)?,
            $($cc4ie:ident)?,
            $($comde:ident)?,
            $($comie:ident)?,
            $($tde:ident)?,
            $($tie:ident)?
        ),
        (
            $($bif:ident)?,
            $($cc2if:ident)?,
            $($cc2of:ident)?,
            $($cc3if:ident)?,
            $($cc3of:ident)?,
            $($cc4if:ident)?,
            $($cc4of:ident)?,
            $($comif:ident)?,
            $($tif:ident)?
        ),
        (
            $($bg:ident)?,
            $($cc2g:ident)?,
            $($cc3g:ident)?,
            $($cc4g:ident)?,
            $($comg:ident)?,
            $($tg:ident)?
        ),
        (
            $($cc2s:ident)?,
            $($oc1ce:ident)?,
            $($oc2ce:ident)?,
            $($oc2fe:ident)?,
            $($oc2m0_2:ident)?,
            $($oc2m3:ident)?,
            $($oc2pe:ident)?,
            $($ic2f:ident)?,
            $($ic2psc:ident)?
        ),
        ($($ccmr2_input:ident)?, $($ccmr2_output:ident)?),
        (
            $($cc1ne:ident)?,
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
        (
            $cnt_l:ident,
            $($cnt_h:ident)?,
            $($uifcpy_cnt31:ident)?,
            $($uifcpy:ident)?
        ),
        ($arr_l:ident, $($arr_h:ident)?),
        ($($rcr:ident)?),
        ($ccr1_l:ident, $($ccr1_h:ident)?),
        ($($ccr2:ident)?, $($ccr3:ident)?, $($ccr4:ident)?),
        ($($bdtr:ident)?),
        ($(
            $or1:ident,
            $($encoder_mode:ident)?,
            $($etr1_rmp:ident)?,
            $($itr1_rmp:ident)?,
            $($ti1_rmp_bit:ident)?,
            $($ti1_rmp:ident)?,
            $($ti4_rmp:ident)?
        )?),
        ($(
            $or2:ident,
            $($bkcmp1e:ident)?,
            $($bkcmp1p:ident)?,
            $($bkcmp2e:ident)?,
            $($bkcmp2p:ident)?,
            $($bkdfbke:ident)?,
            $($bkine:ident)?,
            $($bkinp:ident)?,
            $($etrsel:ident)?
        )?),
    ) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl GeneralTimMap for $tim_ty {}

            ::drone_stm32_map_pieces::reg; general;

            RCC {
                APBENR {
                    $apbenr Shared;
                    TIMEN { $timen }
                }
                APBRSTR {
                    $apbrstr Shared;
                    TIMRST { $timrst }
                }
                APBSMENR {
                    $apbsmenr Shared;
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
                    CMS { $($cms Option)* }
                    DIR { $($dir Option)* }
                    OPM { OPM }
                    UDIS { UDIS }
                    UIFREMAP { UIFREMAP }
                    URS { URS }
                }
                CR2 {
                    CR2;
                    OIS2 { $($ois2 Option)* }
                    OIS1N { $($ois1n Option)* }
                    OIS1 { $($ois1 Option)* }
                    TI1S { $($ti1s Option)* }
                    MMS { $($mms Option)* }
                    CCDS { CCDS }
                    CCUS { $($ccus Option)* }
                    CCPC { $($ccpc Option)* }
                }
                SMCR {
                    $(
                        $smcr Option;
                        SMS3 { SMS3 }
                        ETP { $($etp Option)* }
                        ECE { $($ece Option)* }
                        ETPS { $($etps Option)* }
                        ETF { $($etf Option)* }
                        MSM { MSM }
                        TS { TS }
                        SMS0_2 { SMS0_2 }
                    )*
                }
                DIER {
                    DIER;
                    BIE { $($bie Option)* }
                    CC1DE { CC1DE }
                    CC1IE { CC1IE }
                    CC2DE { $($cc2de Option)* }
                    CC2IE { $($cc2ie Option)* }
                    CC3DE { $($cc3de Option)* }
                    CC3IE { $($cc3ie Option)* }
                    CC4DE { $($cc4de Option)* }
                    CC4IE { $($cc4ie Option)* }
                    COMDE { $($comde Option)* }
                    COMIE { $($comie Option)* }
                    TDE { $($tde Option)* }
                    TIE { $($tie Option)* }
                    UDE { UDE }
                    UIE { UIE }
                }
                SR {
                    SR;
                    BIF { $($bif Option)* }
                    CC1IF { CC1IF }
                    CC1OF { CC1OF }
                    CC2IF { $($cc2if Option)* }
                    CC2OF { $($cc2of Option)* }
                    CC3IF { $($cc3if Option)* }
                    CC3OF { $($cc3of Option)* }
                    CC4IF { $($cc4if Option)* }
                    CC4OF { $($cc4of Option)* }
                    COMIF { $($comif Option)* }
                    TIF { $($tif Option)* }
                    UIF { UIF }
                }
                EGR {
                    EGR;
                    BG { $($bg Option)* }
                    CC1G { CC1G }
                    CC2G { $($cc2g Option)* }
                    CC3G { $($cc3g Option)* }
                    CC4G { $($cc4g Option)* }
                    COMG { $($comg Option)* }
                    TG { $($tg Option)* }
                    UG { UG }
                }
                CCMR1_Output {
                    CCMR1_Output;
                    CC1S { CC1S }
                    CC2S { $($cc2s Option)* }
                    OC1CE { $($oc1ce Option)* }
                    OC1FE { OC1FE }
                    OC1M0_2 { OC1M0_2 }
                    OC1M3 { OC1M3 }
                    OC1PE { OC1PE }
                    OC2CE { $($oc2ce Option)* }
                    OC2FE { $($oc2fe Option)* }
                    OC2M0_2 { $($oc2m0_2 Option)* }
                    OC2M3 { $($oc2m3 Option)* }
                    OC2PE { $($oc2pe Option)* }
                }
                CCMR1_Input {
                    CCMR1_Input;
                    CC1S { CC1S }
                    CC2S { $($cc2s Option)* }
                    IC1F { IC1F }
                    IC1PSC { IC1PSC }
                    IC2F { $($ic2f Option)* }
                    IC2PSC { $($ic2psc Option)* }
                }
                CCMR2_Output {
                    $(
                        $ccmr2_output Option;
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
                    )*
                }
                CCMR2_Input {
                    $(
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
                    CC1NE { $($cc1ne Option)* }
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
                    UIFCPY_CNT31 { $($uifcpy_cnt31 Option)* }
                    UIFCPY { $($uifcpy Option)* }
                    CNT_H { $($cnt_h Option)* }
                    CNT_L { $cnt_l }
                }
                PSC {
                    PSC;
                    PSC { PSC }
                }
                ARR {
                    ARR;
                    ARR_H { $($arr_h Option)* }
                    ARR_L { $arr_l }
                }
                RCR {
                    $(
                        $rcr Option;
                        REP { REP }
                    )*
                }
                CCR1 {
                    CCR1;
                    CCR1_H { $($ccr1_h Option)* }
                    CCR1_L { $ccr1_l }
                }
                CCR2 {
                    $(
                        $ccr2 Option;
                        CCR2_H { CCR2_H }
                        CCR2_L { CCR2_L }
                    )*
                }
                CCR3 {
                    $(
                        $ccr3 Option;
                        CCR3_H { CCR3_H }
                        CCR3_L { CCR3_L }
                    )*
                }
                CCR4 {
                    $(
                        $ccr4 Option;
                        CCR4_H { CCR4_H }
                        CCR4_L { CCR4_L }
                    )*
                }
                BDTR {
                    $(
                        $bdtr Option;
                        AOE { AOE }
                        BKE { BKE }
                        BKP { BKP }
                        DTG { DTG }
                        LOCK { LOCK }
                        MOE { MOE }
                        OSSI { OSSI }
                        OSSR { OSSR }
                    )*
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
                    $(
                        $or1 Option;
                        ENCODER_MODE { $($encoder_mode Option)* }
                        ETR1_RMP { $($etr1_rmp Option)* }
                        ITR1_RMP { $($itr1_rmp Option)* }
                        TI1_RMP_BIT { $($ti1_rmp_bit Option)* }
                        TI1_RMP { $($ti1_rmp Option)* }
                        TI4_RMP { $($ti4_rmp Option)* }
                    )*
                }
                OR2 {
                    $(
                        $or2 Option;
                        BKCMP1E { $($bkcmp1e Option)* }
                        BKCMP1P { $($bkcmp1p Option)* }
                        BKCMP2E { $($bkcmp2e Option)* }
                        BKCMP2P { $($bkcmp2p Option)* }
                        BKDFBKE { $($bkdfbke Option)* }
                        BKINE { $($bkine Option)* }
                        BKINP { $($bkinp Option)* }
                        ETRSEL { $($etrsel Option)* }
                    )*
                }
            }
        }
    };
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
map_general_tim! {
    "Acquires TIM2",
    periph_tim2,
    "TIM2",
    Tim2,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    TIM2EN,
    TIM2RST,
    TIM2SMEN,
    TIM2,
    (CMS, DIR),
    (,,, TI1S, MMS,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M0_2, OC2M3, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (CNT_L, CNT_H, UIFCPY_CNT31,),
    (ARR_L, ARR_H),
    (),
    (CCR1_L, CCR1_H),
    (CCR2, CCR3, CCR4),
    (),
    (OR1,, ETR1_RMP, ITR1_RMP,,, TI4_RMP),
    (OR2,,,,,,,, ETRSEL),
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
map_general_tim! {
    "Acquires TIM3",
    periph_tim3,
    "TIM3",
    Tim3,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    TIM3EN,
    TIM3RST,
    TIM3SMEN,
    TIM3,
    (CMS, DIR),
    (,,, TI1S, MMS,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M0_2, OC2M3, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (CNT_L, CNT_H, UIFCPY_CNT31,),
    (ARR_L, ARR_H),
    (),
    (CCR1_L, CCR1_H),
    (CCR2, CCR3, CCR4),
    (),
    (OR1,,,,, TI1_RMP,),
    (OR2,,,,,,,, ETRSEL),
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
map_general_tim! {
    "Acquires TIM4",
    periph_tim4,
    "TIM4",
    Tim4,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    TIM4EN,
    TIM4RST,
    TIM4SMEN,
    TIM4,
    (CMS, DIR),
    (,,, TI1S, MMS,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M0_2, OC2M3, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (CNT_L, CNT_H, UIFCPY_CNT31,),
    (ARR_L, ARR_H),
    (),
    (CCR1_L, CCR1_H),
    (CCR2, CCR3, CCR4),
    (),
    (),
    (),
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
map_general_tim! {
    "Acquires TIM5",
    periph_tim5,
    "TIM5",
    Tim5,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    TIM5EN,
    TIM5RST,
    TIM5SMEN,
    TIM5,
    (CMS, DIR),
    (,,, TI1S, MMS,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M0_2, OC2M3, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (CNT_L, CNT_H, UIFCPY_CNT31,),
    (ARR_L, ARR_H),
    (),
    (CCR1_L, CCR1_H),
    (CCR2, CCR3, CCR4),
    (),
    (),
    (),
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
map_general_tim! {
    "Acquires TIM15",
    periph_tim15,
    "TIM15",
    Tim15,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    TIM15EN,
    TIM15RST,
    TIM15SMEN,
    TIM15,
    (,),
    (OIS2, OIS1N, OIS1, TI1S, MMS, CCUS, CCPC),
    (SMCR,,,,),
    (BIE, CC2DE, CC2IE,,,,, COMDE, COMIE, TDE, TIE),
    (BIF, CC2IF, CC2OF,,,,, COMIF, TIF),
    (BG,,,, COMG, TG),
    (CC2S,, OC2CE, OC2FE, OC2M0_2, OC2M3, OC2PE, IC2F, IC2PSC),
    (,),
    (CC1NE, CC2E, CC2NP, CC2P,,,,,,),
    (CNT,,, UIFCPY),
    (ARR,),
    (RCR),
    (CCR1,),
    (,,),
    (BDTR),
    (OR1, ENCODER_MODE,,, TI1_RMP,,),
    (OR2, BKCMP1E, BKCMP1P, BKCMP2E, BKCMP2P, BKDFBK1E, BKINE, BKINP,),
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
map_general_tim! {
    "Acquires TIM16",
    periph_tim16,
    "TIM16",
    Tim16,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    TIM16EN,
    TIM16RST,
    TIM16SMEN,
    TIM16,
    (,),
    (, OIS1N, OIS1,,, CCUS, CCPC),
    (),
    (BIE,,,,,,, COMDE, COMIE,,),
    (BIF,,,,,,, COMIF,),
    (BG,,,, COMG,),
    (,,,,,,,,),
    (,),
    (CC1NE,,,,,,,,,),
    (CNT,,, UIFCPY),
    (ARR,),
    (RCR),
    (CCR1,),
    (,,),
    (BDTR),
    (OR1,,,,, TI1_RMP,),
    (OR2, BKCMP1E, BKCMP1P, BKCMP2E, BKCMP2P, BKDFBK1E, BKINE, BKINP,),
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
map_general_tim! {
    "Acquires TIM17",
    periph_tim17,
    "TIM17",
    Tim17,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    TIM17EN,
    TIM17RST,
    TIM17SMEN,
    TIM17,
    (,),
    (, OIS1N, OIS1,,, CCUS, CCPC),
    (),
    (BIE,,,,,,, COMDE, COMIE,,),
    (BIF,,,,,,, COMIF,),
    (BG,,,, COMG,),
    (,,,,,,,,),
    (,),
    (CC1NE,,,,,,,,,),
    (CNT,,, UIFCPY),
    (ARR,),
    (RCR),
    (CCR1,),
    (,,),
    (BDTR),
    (OR1,,,,, TI1_RMP,),
    (OR2, BKCMP1E, BKCMP1P, BKCMP2E, BKCMP2P, BKDFBK1E, BKINE, BKINP,),
}

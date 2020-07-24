//! General-purpose timers.
//! for STM32F3 Series of mixed-signal MCUs with DSP and FPU instructions.

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
    }
    TIM {
        CR1 {
            0x20 RwRegBitBand;
            CEN { RwRwRegFieldBitBand }
            UDIS { RwRwRegFieldBitBand }
            URS { RwRwRegFieldBitBand }
            OPM { RwRwRegFieldBitBand }
            DIR { RwRwRegFieldBitBand Option }
            CMS { RwRwRegFieldBits Option }
            ARPE { RwRwRegFieldBitBand }
            CKD { RwRwRegFieldBits }
            UIFREMAP { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
            CCPC { RwRwRegFieldBitBand Option }
            CCUS { RwRwRegFieldBitBand Option }
            CCDS { RwRwRegFieldBitBand }
            MMS { RwRwRegFieldBits Option }
            TI1S { RwRwRegFieldBitBand Option }
            OIS1 { RwRwRegFieldBitBand Option }
            OIS1N { RwRwRegFieldBitBand Option }
            OIS2 { RwRwRegFieldBitBand Option }
            OIS2N { RwRwRegFieldBitBand Option }
            OIS3 { RwRwRegFieldBitBand Option }
            OIS3N { RwRwRegFieldBitBand Option }
            OIS4 { RwRwRegFieldBitBand Option }
            OIS5 { RwRwRegFieldBitBand Option }
            OIS6 { RwRwRegFieldBitBand Option }
            MMS2 { RwRwRegFieldBits Option}
        }
        SMCR {
            0x20 RwRegBitBand Option;
            SMS { RwRwRegFieldBits }
            OCCS { RwRwRegFieldBitBand Option }
            TS { RwRwRegFieldBits }
            MSM { RwRwRegFieldBitBand Option }
            ETF { RwRwRegFieldBits Option }
            ETPS { RwRwRegFieldBits Option }
            ECE { RwRwRegFieldBitBand Option }
            ETP { RwRwRegFieldBitBand Option }
            SMS_3 { RwRwRegFieldBitBand }
        }
        DIER {
            0x20 RwRegBitBand;
            TDE { RwRwRegFieldBitBand }
            COMDE { RwRwRegFieldBitBand Option }
            CC4DE { RwRwRegFieldBitBand Option }
            CC3DE { RwRwRegFieldBitBand Option }
            CC2DE { RwRwRegFieldBitBand Option }
            CC1DE { RwRwRegFieldBitBand Option }
            UDE { RwRwRegFieldBitBand }
            BIE { RwRwRegFieldBitBand Option }
            TIE { RwRwRegFieldBitBand Option }
            COMIE { RwRwRegFieldBitBand Option }
            CC4IE { RwRwRegFieldBitBand Option }
            CC3IE { RwRwRegFieldBitBand Option }
            CC2IE { RwRwRegFieldBitBand Option }
            CC1IE { RwRwRegFieldBitBand Option }
            UIE { RwRwRegFieldBitBand }
        }
        SR {
            0x20 RwRegBitBand;
            UIF { RwRwRegFieldBitBand }
            CC1IF { RwRwRegFieldBitBand }
            CC2IF { RwRwRegFieldBitBand Option }
            CC3IF { RwRwRegFieldBitBand Option }
            CC4IF { RwRwRegFieldBitBand Option }
            COMIF { RwRwRegFieldBitBand Option }
            TIF { RwRwRegFieldBitBand Option }
            BIF { RwRwRegFieldBitBand Option }
            B2IF { RwRwRegFieldBitBand Option }
            CC1OF { RwRwRegFieldBitBand }
            CC2OF { RwRwRegFieldBitBand Option }
            CC3OF { RwRwRegFieldBitBand Option }
            CC4OF { RwRwRegFieldBitBand Option }
            C5IF { RwRwRegFieldBitBand Option }
            C6IF { RwRwRegFieldBitBand Option }
        }
        EGR {
            0x20 WoRegBitBand;
            UG { WoWoRegFieldBitBand }
            CC1G { WoWoRegFieldBitBand }
            CC2G { WoWoRegFieldBitBand Option }
            CC3G { WoWoRegFieldBitBand Option }
            CC4G { WoWoRegFieldBitBand Option }
            COMG { WoWoRegFieldBitBand Option }
            TG { WoWoRegFieldBitBand }
            BG { WoWoRegFieldBitBand Option }
            B2G { WoWoRegFieldBitBand Option }
        }
        CCMR1 {
            @Output 0x20 RwRegBitBand Option;
            CC1S { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits Option}
            OC2CE { RwRwRegFieldBitBand Option }
            OC2M { RwRwRegFieldBits Option }
            OC2PE { RwRwRegFieldBitBand Option }
            OC2FE { RwRwRegFieldBitBand Option }
            OC1CE { RwRwRegFieldBitBand Option }
            OC1M { RwRwRegFieldBits }
            OC1PE { RwRwRegFieldBitBand }
            OC1FE { RwRwRegFieldBitBand }
            OC1M_3 { RwRwRegFieldBit }
            OC2M_3 { RwRwRegFieldBit Option }
            @Input 0x20 RwRegBitBand Option;
            CC1S { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits Option }
            IC2F { RwRwRegFieldBits Option}
            IC2PSC { RwRwRegFieldBits Option }
            IC1F { RwRwRegFieldBits }
            IC1PSC { RwRwRegFieldBits }
        }
        CCMR2 {
            @Output 0x20 RwRegBitBand Option;
            OC4M_3  { RwRwRegFieldBitBand }
            OC3M_3  { RwRwRegFieldBitBand }
            OC4CE { RwRwRegFieldBitBand }
            OC4M  { RwRwRegFieldBits }
            OC4PE { RwRwRegFieldBitBand }
            OC4FE { RwRwRegFieldBitBand }
            CC4S { RwRwRegFieldBits }
            OC3CE { RwRwRegFieldBitBand }
            OC3M { RwRwRegFieldBits }
            OC3PE { RwRwRegFieldBitBand }
            OC3FE { RwRwRegFieldBitBand }
            CC3S { RwRwRegFieldBits }
            @Input 0x20 RwRegBitBand Option;
            IC4F { RwRwRegFieldBits }
            IC4PSC { RwRwRegFieldBits }
            CC4S { RwRwRegFieldBits }
            IC3F { RwRwRegFieldBits }
            IC3PSC { RwRwRegFieldBits }
            CC3S { RwRwRegFieldBits }
        }
        CCER {
            0x20 RwRegBitBand;
            CC1E { RwRwRegFieldBitBand }
            CC1P { RwRwRegFieldBitBand }
            CC1NE { RwRwRegFieldBitBand Option }
            CC1NP { RwRwRegFieldBitBand }
            CC2E { RwRwRegFieldBitBand Option }
            CC2P { RwRwRegFieldBitBand Option }
            CC2NE { RwRwRegFieldBitBand Option }
            CC2NP { RwRwRegFieldBitBand Option }
            CC3E { RwRwRegFieldBitBand Option }
            CC3P { RwRwRegFieldBitBand Option }
            CC3NE { RwRwRegFieldBitBand Option }
            CC3NP { RwRwRegFieldBitBand Option }
            CC4E { RwRwRegFieldBitBand Option }
            CC4P { RwRwRegFieldBitBand Option }
            CC4NP { RwRwRegFieldBitBand Option }
            CC5E { RwRwRegFieldBitBand Option }
            CC5P { RwRwRegFieldBitBand Option }
            CC6E { RwRwRegFieldBitBand Option }
            CC6P { RwRwRegFieldBitBand Option }
        }
        CNT {
            0x20 RwRegBitBand;
            CNT { RwRwRegFieldBits Option }
            CNTL { RwRwRegFieldBits Option }
            CNTH { RwRwRegFieldBits Option }
            UIFCPY { RoRwRegFieldBitBand }
        }
        PSC {
            0x20 RwRegBitBand;
            PSC { RwRwRegFieldBits }
        }
        ARR {
            0x20 RwRegBitBand;
            ARR { RwRwRegFieldBits Option }
            ARRL { RwRwRegFieldBits Option }
            ARRH { RwRwRegFieldBits Option }
        }
        RCR {
            0x20 RwRegBitBand Option;
            REP  { RwRwRegFieldBits }
        }
        CCR1 {
            0x20 RwRegBitBand;
            CCR1 { RwRwRegFieldBits Option }
            CCR1L { RwRwRegFieldBits Option }
            CCR1H { RwRwRegFieldBits Option }
        }
        CCR2 {
            0x20 RwRegBitBand Option;
            CCR2 { RwRwRegFieldBits Option }
            CCR2L { RwRwRegFieldBits Option }
            CCR2H { RwRwRegFieldBits Option }
        }
        CCR3 {
            0x20 RwRegBitBand Option;
            CCR3L { RwRwRegFieldBits }
            CCR3H { RwRwRegFieldBits }
        }
        CCR4 {
            0x20 RwRegBitBand Option;
            CCR4L { RwRwRegFieldBits }
            CCR4H { RwRwRegFieldBits }
        }
        BDTR {
            0x20 RwRegBitBand Option;
            DTG { RwRwRegFieldBits }
            LOCK { RwRwRegFieldBits }
            OSSI{ RwRwRegFieldBitBand }
            OSSR{ RwRwRegFieldBitBand }
            BKE{ RwRwRegFieldBitBand }
            BKP{ RwRwRegFieldBitBand }
            AOE{ RwRwRegFieldBitBand }
            MOE{ RwRwRegFieldBitBand }
            BKF{ RwRwRegFieldBitBand }
            BK2F { RwRwRegFieldBits Option }
            BK2E { RwRwRegFieldBits Option }
            BK2P { RwRwRegFieldBits Option }
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
    }
}

#[allow(unused_macros)]
macro_rules! map_general_tim {
    (
        $tim_macro_doc:expr,
        $tim_macro:ident,
        $tim_ty_doc:expr,
        $tim_ty:ident,
        $busenr:ident,
        $timen:ident,
        $busrstr:ident,
        $timrst:ident,
        $tim:ident,
        (
            $($dir:ident)?,
            $($cms:ident)?
        ),
        (
            $($ccpc:ident)?,
            $($ccus:ident)?,
            $($mms:ident)?,
            $($ti1s:ident)?,
            $($ois1:ident)?,
            $($ois1n:ident)?,
            $($ois2:ident)?,
            $($ois2n:ident)?,
            $($ois3:ident)?,
            $($ois3n:ident)?,
            $($ois4:ident)?,
            $($ois5:ident)?,
            $($ois6:ident)?,
            $($mms2:ident)?
        ),
        ($(
            $smcr:ident,
            $($occs:ident)?,
            $($msm:ident)?,
            $($etf:ident)?,
            $($etps:ident)?,
            $($ece:ident)?,
            $($etp:ident)?
        )?),
        (
            $($comde:ident)?,
            $($cc4de:ident)?,
            $($cc3de:ident)?,
            $($cc2de:ident)?,
            $($cc1de:ident)?,
            $($bie:ident)?,
            $($tie:ident)?,
            $($comie:ident)?,
            $($cc4ie:ident)?,
            $($cc3ie:ident)?,
            $($cc2ie:ident)?,
            $($cc1ie:ident)?
        ),
        (
            $($cc2if:ident)?,
            $($cc3if:ident)?,
            $($cc4if:ident)?,
            $($comif:ident)?,
            $($tif:ident)?,
            $($bif:ident)?,
            $($b2if:ident)?,
            $($cc2of:ident)?,
            $($cc3of:ident)?,
            $($cc4of:ident)?,
            $($c5if:ident)?,
            $($c6if:ident)?
        ),
        (
            $($cc2g:ident)?,
            $($cc3g:ident)?,
            $($cc4g:ident)?,
            $($comg:ident)?,
            $($bg:ident)?,
            $($b2g:ident)?
        ),
        (
            $($ccmr1_output:ident)?,
            $($oc2ce:ident)?,
            $($oc2m:ident)?,
            $($oc2pe:ident)?,
            $($oc2fe:ident)?,
            $($cc2s:ident)?,
            $($oc1ce:ident)?,
            $($oc2m_3:ident)?,
            $($ccmr1_input:ident)?,
            $($cc2si:ident)?,
            $($ic2f:ident)?,
            $($ic2psc:ident)?
        ),
        ($($ccmr2_input:ident)?, $($ccmr2_output:ident)?),
        (
            $($cc1ne:ident)?,
            $($cc2e:ident)?,
            $($cc2p:ident)?,
            $($cc2ne:ident)?,
            $($cc2np:ident)?,
            $($cc3e:ident)?,
            $($cc3p:ident)?,
            $($cc3ne:ident)?,
            $($cc3np:ident)?,
            $($cc4e:ident)?,
            $($cc4p:ident)?,
            $($cc4np:ident)?,
            $($cc5e:ident)?,
            $($cc5p:ident)?,
            $($cc6e:ident)?,
            $($cc6p:ident)?
        ),
        (
            $($cnt:ident)?,
            $($cntl:ident)?,
            $($cnth:ident)?
        ),
        (
            $($arr:ident)?,
            $($arrl:ident)?,
            $($arrh:ident)?
        ),
        ($($rcr:ident)?),
        (
            $($ccr1:ident)?,
            $($ccr1l:ident)?,
            $($ccr1h:ident)?
        ),
        ($(
            $ccr2:ident,
            $($ccr2i:ident)?,
            $($ccr2l:ident)?,
            $($ccr2h:ident)?
        )?),
        ($($ccr3:ident)?),
        ($($ccr4:ident)?),
        ($(
            $bdtr:ident,
            $($bk2f:ident)?,
            $($bk2e:ident)?,
            $($bk2p:ident)?,
        )?),
        ($($dcr:ident)?),
        ($($dmar:ident)?),
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
            }
            TIM {
                $tim;
                CR1 {
                    CR1;
                    CEN { CEN }
                    UDIS { UDIS }
                    URS { URS }
                    OPM { OPM }
                    DIR { $($dir Option)* }
                    CMS { $($cms Option)* }
                    ARPE { ARPE }
                    CKD { CKD }
                    UIFREMAP { UIFREMAP }
                }
                CR2 {
                    CR2;
                    CCPC { $($ccpc Option)* }
                    CCUS { $($ccus Option)* }
                    CCDS { CCDS }
                    MMS { $($mms Option)* }
                    TI1S { $($ti1s Option)* }
                    OIS1 { $($ois1 Option)* }
                    OIS1N { $($ois1n Option)* }
                    OIS2 { $($ois2 Option)* }
                    OIS2N { $($ois2n Option)* }
                    OIS3 { $($ois3 Option)* }
                    OIS3N { $($ois3n Option)* }
                    OIS4 { $($ois4 Option)* }
                    OIS5 { $($ois5 Option)* }
                    OIS6 { $($ois6 Option)* }
                    MMS2 { $($mms2 Option)* }
                }
                SMCR {
                    $(
                        $smcr Option;
                        SMS { SMS }
                        OCCS { $($occs Option)* }
                        TS { TS }
                        MSM { $($msm Option)* }
                        ETF { $($etf Option)* }
                        ETPS { $($etps Option)* }
                        ECE { $($ece Option)* }
                        ETP { $($etp Option)* }
                        SMS_3 { SMS_3 }
                    )*
                }
                DIER {
                    DIER;
                    TDE { TDE }
                    COMDE { $($comde Option)* }
                    CC4DE { $($cc4de Option)* }
                    CC3DE { $($cc3de Option)* }
                    CC2DE { $($cc2de Option)* }
                    CC1DE { $($cc1de Option)* }
                    UDE { UDE }
                    BIE { $($bie Option)* }
                    TIE { $($tie Option)* }
                    COMIE { $($comie Option)* }
                    CC4IE { $($cc4ie Option)* }
                    CC3IE { $($cc3ie Option)* }
                    CC2IE { $($cc2ie Option)* }
                    CC1IE { $($cc1ie Option)* }
                    UIE { UIE }
                }
                SR {
                    SR;
                    UIF { UIF }
                    CC1IF { CC1IF }
                    CC2IF { $($cc2if Option)* }
                    CC3IF { $($cc3if Option)* }
                    CC4IF { $($cc4if Option)* }
                    COMIF { $($comif Option)* }
                    TIF { $($tif Option)* }
                    BIF { $($bif Option)* }
                    B2IF { $($b2if Option)* }
                    CC1OF { CC1OF }
                    CC2OF { $($cc2of Option)* }
                    CC3OF { $($cc3of Option)* }
                    CC4OF { $($cc4of Option)* }
                    C5IF { $($c5if Option)* }
                    C6IF { $($c6if Option)* }
                }
                EGR {
                    EGR;
                    UG { UG }
                    CC1G { CC1G }
                    CC2G { $($cc2g Option)* }
                    CC3G { $($cc3g Option)* }
                    CC4G { $($cc4g Option)* }
                    COMG { $($comg Option)* }
                    TG { TG }
                    BG { $($bg Option)* }
                    B2G { $($b2g Option)* }
                }
                CCMR1 {
                    @Output CCMR1_Output Option;
                    CC1S  { CC1S }
                    OC2CE { $($oc2ce Option)* }
                    OC2M { $($oc2m Option)* }
                    OC2PE { $($oc2pe Option)* }
                    OC2FE { $($oc2fe Option)* }
                    CC2S { $($cc2s Option)* }
                    OC1CE { $($oc1ce Option)* }
                    OC1M { OC1M }
                    OC1PE { OC1PE }
                    OC1FE { OC1FE }
                    OC1M_3 { OC1M_3 }
                    OC2M_3 { $($oc2m_3 Option)* }
                    @Input CCMR1_Input Option;
                    CC1S  { CC1S }
                    CC2S { $($cc2si Option)* }
                    IC2F { $($ic2f Option)* }
                    IC2PSC { $($ic2psc Option)* }
                    IC1F { IC1F }
                    IC1PSC { IC1PSC }
                }
                CCMR2 {
                   @Output $(
                        $ccmr2_output Option;
                        OC4M_3  { OC4M_3 }
                        OC3M_3  { OC3M_3 }
                        OC4CE { OC4CE }
                        OC4M  { OC4M }
                        OC4PE { OC4PE }
                        OC4FE { OC4FE }
                        CC4S { CC4S }
                        OC3CE { OC3CE }
                        OC3M { OC3M }
                        OC3PE { OC3PE }
                        OC3FE { OC3FE }
                        CC3S { CC3S }
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
                    CC1P { CC1P }
                    CC1NE { $($cc1ne Option)* }
                    CC1NP { CC1NP }
                    CC2E { $($cc2e Option)* }
                    CC2P { $($cc2p Option)* }
                    CC2NE { $($cc2ne Option)* }
                    CC2NP { $($cc2np Option)* }
                    CC3E { $($cc3e Option)* }
                    CC3P { $($cc3p Option)* }
                    CC3NE { $($cc3ne Option)* }
                    CC3NP { $($cc3np Option)* }
                    CC4E { $($cc4e Option)* }
                    CC4P { $($cc4p Option)* }
                    CC4NP { $($cc4np Option)* }
                    CC5E { $($cc5e Option)* }
                    CC5P { $($cc5p Option)* }
                    CC6E { $($cc6e Option)* }
                    CC6P { $($cc6p Option)* }
                }
                CNT {
                    CNT;
                    CNT { $($cnt Option)* }
                    CNTL { $($cntl Option)* }
                    CNTH { $($cnth Option)* }
                    UIFCPY { UIFCPY }
                }
                PSC {
                    PSC;
                    PSC { PSC }
                }
                ARR {
                    ARR;
                    ARR { $($arr Option)* }
                    ARRL { $($arrl Option)* }
                    ARRH { $($arrh Option)* }
                }
                RCR {
                    $(
                        $rcr Option;
                        REP { REP }
                    )*
                }
                CCR1 {
                    CCR1;
                    CCR1 { $($ccr1 Option)* }
                    CCR1L { $($ccr1l Option)* }
                    CCR1H { $($ccr1h Option)* }
                }
                CCR2 {
                    $(
                        $ccr2 Option;
                        CCR2 { $($ccr2i Option)* }
                        CCR2L { $($ccr2l Option)* }
                        CCR2H { $($ccr2h Option)* }
                    )*
                }
                CCR3 {
                    $(
                        $ccr3 Option;
                        CCR3L { CCR3L }
                        CCR3H { CCR3H }
                    )*
                }
                CCR4 {
                    $(
                        $ccr4 Option;
                        CCR4L { CCR4L }
                        CCR4H { CCR4H }
                    )*
                }
                BDTR {
                    $(
                        $bdtr Option;
                        DTG { DTG }
                        LOCK { LOCK }
                        OSSI { OSSI }
                        OSSR { OSSR }
                        BKE { BKE }
                        BKP { BKP }
                        AOE { AOE }
                        MOE { MOE }
                        BKF { BKF }
                        BK2F { $($bk2f Option)* }
                        BK2E { $($bk2e Option)* }
                        BK2P { $($bk2p Option)* }
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
            }
        }
    };
}

map_general_tim! {
    "Extracts TIM2 register tokens.",
    periph_tim2,
    "TIM2 peripheral variant.",
    Tim2,
    APB1ENR,
    TIM2EN,
    APB1RSTR,
    TIM2RST,
    TIM2,
    (DIR,CMS),
    (,,MMS,TI1S,,,,,,,,,,),
    (SMCR,OCCS,MSM,ETF,ETPS,ECE,ETP),
    (,CC4DE,CC3DE,CC2DE,CC1DE,,TIE,,CC4IE,CC3IE,CC2IE,CC1IE),
    (CC2IF,CC3IF,CC4IF,,TIF,,,CC2OF,CC3OF,CC4OF,,),
    (CC2G,CC3G,CC4G,,,),
    (CCMR1_Output,OC2CE,OC2M,OC2PE,OC2FE,CC2S,OC1CE,OC2M_3,CCMR1_Input,CC2S,IC2F,IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (,CC2E,CC2P,,CC2NP,CC3E,CC3P,CC3NP,CC4E,CC4P,CC4NP,,,,,),
    (,CNTL,CNTH),
    (,ARRL,ARRH),
    (),
    (,CCR1L,CCR1H),
    (CCR2,,CCR2L,CCR2H),
    (CCR3),
    (CCR4),
    (),
    (DCR),
    (DMAR),
}

map_general_tim! {
    "Extracts TIM3 register tokens.",
    periph_tim3,
    "TIM3 peripheral variant.",
    Tim3,
    APB1ENR,
    TIM3EN,
    APB1RSTR,
    TIM3RST,
    TIM3,
    (DIR,CMS),
    (,,MMS,TI1S,,,,,,,,,,),
    (SMCR,OCCS,MSM,ETF,ETPS,ECE,ETP),
    (,CC4DE,CC3DE,CC2DE,CC1DE,,TIE,,CC4IE,CC3IE,CC2IE,CC1IE),
    (CC2IF,CC3IF,CC4IF,,TIF,,,CC2OF,CC3OF,CC4OF,,),
    (CC2G,CC3G,CC4G,,,),
    (CCMR1_Output,OC2CE,OC2M,OC2PE,OC2FE,CC2S,OC1CE,OC2M_3,CCMR1_Input,CC2S,IC2F,IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (,CC2E,CC2P,,CC2NP,CC3E,CC3P,CC3NP,CC4E,CC4P,CC4NP,,,,,),
    (,CNTL,CNTH),
    (,ARRL,ARRH),
    (),
    (,CCR1L,CCR1H),
    (CCR2,,CCR2L,CCR2H),
    (CCR3),
    (CCR4),
    (),
    (DCR),
    (DMAR),
}

map_general_tim! {
    "Extracts TIM4 register tokens.",
    periph_tim4,
    "TIM4 peripheral variant.",
    Tim4,
    APB1ENR,
    TIM4EN,
    APB1RSTR,
    TIM4RST,
    TIM4,
    (DIR,CMS),
    (,,MMS,TI1S,,,,,,,,,,),
    (SMCR,OCCS,MSM,ETF,ETPS,ECE,ETP),
    (,CC4DE,CC3DE,CC2DE,CC1DE,,TIE,,CC4IE,CC3IE,CC2IE,CC1IE),
    (CC2IF,CC3IF,CC4IF,,TIF,,,CC2OF,CC3OF,CC4OF,,),
    (CC2G,CC3G,CC4G,,,),
    (CCMR1_Output,OC2CE,OC2M,OC2PE,OC2FE,CC2S,OC1CE,OC2M_3,CCMR1_Input,CC2S,IC2F,IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (,CC2E,CC2P,,CC2NP,CC3E,CC3P,CC3NP,CC4E,CC4P,CC4NP,,,,,),
    (,CNTL,CNTH),
    (,ARRL,ARRH),
    (),
    (,CCR1L,CCR1H),
    (CCR2,,CCR2L,CCR2H),
    (CCR3),
    (CCR4),
    (),
    (DCR),
    (DMAR),
}

map_general_tim! {
    "Extracts TIM15 register tokens.",
    periph_tim15,
    "TIM15 peripheral variant.",
    Tim15,
    APB2ENR,
    TIM15EN,
    APB2RSTR,
    TIM15RST,
    TIM15,
    (,),
    (CCPC,CCUS,MMS,TI1S,OIS1,OIS1N,OIS2,,,,,,,),
    (SMCR,,MSM,,,,),
    (COMDE,,,CC2DE,CC1DE,,BIE,TIE,COMIE,,CC2IE,CC1IE),
    (CC2IF,,,COMIF,TIF,BIF,,CC2OF,,,,),
    (CC2G,,,COMG,BG,),
    (CCMR1_Output,,OC2M,OC2PE,OC2FE,CC2S,,OC2M_3,CCMR1_Input,CC2S,IC2F,IC2PSC),
    (,),
    (CC1NE,CC2E,CC2P,,CC2NP,,,,,,,,,,,),
    (CNT,,),
    (ARR,,),
    (RCR),
    (CCR1,,),
    (CCR2,CCR2,,),
    (),
    (),
    (),
    (DCR),
    (DMAR),
}

map_general_tim! {
    "Extracts TIM16 register tokens.",
    periph_tim16,
    "TIM16 peripheral variant.",
    Tim16,
    APB2ENR,
    TIM16EN,
    APB2RSTR,
    TIM16RST,
    TIM16,
    (,),
    (CCPC,CCUS,,,OIS1,OIS1N,,,,,,,,),
    (),
    (COMDE,,,,CC1DE,,BIE,TIE,COMIE,,,CC1IE),
    (,,,COMIF,TIF,BIF,,,,,,),
    (,,,COMG,BG,),
    (CCMR1_Output,,,,,,,,CCMR1_Input,,,),
    (,),
    (CC1NE,,,,,,,,,,,,,,,),
    (CNT,,),
    (ARR,,),
    (RCR),
    (CCR1,,),
    (),
    (),
    (),
    (),
    (DCR),
    (DMAR),
}

map_general_tim! {
    "Extracts TIM17 register tokens.",
    periph_tim17,
    "TIM17 peripheral variant.",
    Tim17,
    APB2ENR,
    TIM17EN,
    APB2RSTR,
    TIM17RST,
    TIM17,
    (,),
    (CCPC,CCUS,,,OIS1,OIS1N,,,,,,,,),
    (),
    (COMDE,,,,CC1DE,,BIE,TIE,COMIE,,,CC1IE),
    (,,,COMIF,TIF,BIF,,,,,,),
    (,,,COMG,BG,),
    (CCMR1_Output,,,,,,,,CCMR1_Input,,,),
    (,),
    (CC1NE,,,,,,,,,,,,,,,),
    (CNT,,),
    (ARR,,),
    (RCR),
    (CCR1,,),
    (),
    (),
    (),
    (),
    (DCR),
    (DMAR),
}


use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic ADC peripheral variant.
    pub trait AdcMap {}

    /// Generic ADC peripheral.
    pub struct AdcPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            ADCEN { RwRwRegFieldBitBand }
        }
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            ADCSMEN { RwRwRegFieldBitBand }
        }
    }

    ADC {
        SR {
            0x20 RwReg;
            OVR { RwRwRegFieldBit }
            STRT { RwRwRegFieldBit }
            JSTRT { RwRwRegFieldBit }
            JEOC { RwRwRegFieldBit }
            EOC { RwRwRegFieldBit }
            AWD { RwRwRegFieldBit }
        }
        CR1 {
            0x20 RwReg;
            OVRIE { RwRwRegFieldBit }
            RES { RwRwRegFieldBits }
            AWDEN { RwRwRegFieldBit }
            JAWDEN { RwRwRegFieldBit }
            DISCNUM { RwRwRegFieldBits }
            JDISCEN { RwRwRegFieldBit }
            DISCEN { RwRwRegFieldBit }
            JAUTO { RwRwRegFieldBit }
            AWDSGL { RwRwRegFieldBit }
            SCAN { RwRwRegFieldBit }
            JEOCIE { RwRwRegFieldBit }
            AWDIE { RwRwRegFieldBit }
            EOCIE { RwRwRegFieldBit }
            AWDCH { RwRwRegFieldBits }
        }
        CR2 {
            0x20 RwReg;
            SWSTART { RwRwRegFieldBit }
            EXTEN { RwRwRegFieldBits }
            EXTSEL { RwRwRegFieldBits }
            JSWSTART { RwRwRegFieldBit }
            JEXTEN { RwRwRegFieldBits }
            JEXTSEL { RwRwRegFieldBits }
            ALIGN { RwRwRegFieldBit }
            EOCS { RwRwRegFieldBit }
            DDS { RwRwRegFieldBit }
            DMA { RwRwRegFieldBit }
            CONT { RwRwRegFieldBit }
            ADON { RwRwRegFieldBit }
        }
        SMPR1 {
            0x20 RwReg;
            SMP18 { RwRwRegFieldBits }
            SMP17 { RwRwRegFieldBits }
            SMP16 { RwRwRegFieldBits }
            SMP15 { RwRwRegFieldBits }
            SMP14 { RwRwRegFieldBits }
            SMP13 { RwRwRegFieldBits }
            SMP12 { RwRwRegFieldBits }
            SMP11 { RwRwRegFieldBits }
            SMP10 { RwRwRegFieldBits }
        }
        SMPR2 {
            0x20 RwReg;
            SMP9 { RwRwRegFieldBits }
            SMP8 { RwRwRegFieldBits }
            SMP7 { RwRwRegFieldBits }
            SMP6 { RwRwRegFieldBits }
            SMP5 { RwRwRegFieldBits }
            SMP4 { RwRwRegFieldBits }
            SMP3 { RwRwRegFieldBits }
            SMP2 { RwRwRegFieldBits }
            SMP1 { RwRwRegFieldBits }
            SMP0 { RwRwRegFieldBits }
        }
        JOFR1 {
            0x20 RwReg;
            JOFFSET1 { RwRwRegFieldBits }
        }
        JOFR2 {
            0x20 RwReg;
            JOFFSET2 { RwRwRegFieldBits }
        }
        JOFR3 {
            0x20 RwReg;
            JOFFSET3 { RwRwRegFieldBits }
        }
        JOFR4 {
            0x20 RwReg;
            JOFFSET4 { RwRwRegFieldBits }
        }
        HTR {
            0x20 RwReg;
            HT { RwRwRegFieldBits }
        }
        LTR {
            0x20 RwReg;
            LT { RwRwRegFieldBits }
        }
        SQR1 {
            0x20 RwReg;
            L { RwRwRegFieldBits }
            SQ16 { RwRwRegFieldBits }
            SQ15 { RwRwRegFieldBits }
            SQ14 { RwRwRegFieldBits }
            SQ13 { RwRwRegFieldBits }
        }
        SQR2 {
            0x20 RwReg;
            SQ12 { RwRwRegFieldBits }
            SQ11 { RwRwRegFieldBits }
            SQ10 { RwRwRegFieldBits }
            SQ9 { RwRwRegFieldBits }
            SQ8 { RwRwRegFieldBits }
            SQ7 { RwRwRegFieldBits }
        }
        SQR3 {
            0x20 RwReg;
            SQ6 { RwRwRegFieldBits }
            SQ5 { RwRwRegFieldBits }
            SQ4 { RwRwRegFieldBits }
            SQ3 { RwRwRegFieldBits }
            SQ2 { RwRwRegFieldBits }
            SQ1 { RwRwRegFieldBits }
        }
        DR {
            0x20 RoReg;
            DATA { RoRoRegFieldBits }
        }
        JSQR {
            0x20 RwReg;
            JL { RwRwRegFieldBits }
            JSQ1 { RwRwRegFieldBits }
            JSQ2 { RwRwRegFieldBits }
            JSQ3 { RwRwRegFieldBits }
            JSQ4 { RwRwRegFieldBits }
        }
        JDR1 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        JDR2 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        JDR3 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        JDR4 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
    }
}

macro_rules! map_adc {
    (
        $adc_macro_doc:expr,
        $adc_macro:ident,
        $adc_ty_doc:expr,
        $adc_ty:ident,
        $adc:ident,
        $busenr:ident,
        $bussmenr:ident,
        $adcen:ident,
        $adcsmen:ident,
    ) => {
        periph::map! {
            #[doc = $adc_macro_doc]
            pub macro $adc_macro;

            #[doc = $adc_ty_doc]
            pub struct $adc_ty;

            impl AdcMap for $adc_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    ADCEN { $adcen }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    ADCSMEN { $adcsmen }
                }
            }

            ADC {
                $adc;
                SR {
                    SR;
                    OVR { OVR }
                    STRT { STRT }
                    JSTRT { JSTRT }
                    JEOC { JEOC }
                    EOC { EOC }
                    AWD { AWD }
                }
                CR1 {
                    CR1;
                    OVRIE { OVRIE }
                    RES { RES }
                    AWDEN { AWDEN }
                    JAWDEN { JAWDEN }
                    DISCNUM { DISCNUM }
                    JDISCEN { JDISCEN }
                    DISCEN { DISCEN }
                    JAUTO { JAUTO }
                    AWDSGL { AWDSGL }
                    SCAN { SCAN }
                    JEOCIE { JEOCIE }
                    AWDIE { AWDIE }
                    EOCIE { EOCIE }
                    AWDCH { AWDCH }
                }
                CR2 {
                    CR2;
                    SWSTART { SWSTART }
                    EXTEN { EXTEN }
                    EXTSEL { EXTSEL }
                    JSWSTART { JSWSTART }
                    JEXTEN { JEXTEN }
                    JEXTSEL { JEXTSEL }
                    ALIGN { ALIGN }
                    EOCS { EOCS }
                    DDS { DDS }
                    DMA { DMA }
                    CONT { CONT }
                    ADON { ADON }
                }
                SMPR1 {
                    SMPR1;
                    SMP18 { SMP18 }
                    SMP17 { SMP17 }
                    SMP16 { SMP16 }
                    SMP15 { SMP15 }
                    SMP14 { SMP14 }
                    SMP13 { SMP13 }
                    SMP12 { SMP12 }
                    SMP11 { SMP11 }
                    SMP10 { SMP10 }
                }
                SMPR2 {
                    SMPR2;
                    SMP9 { SMP9 }
                    SMP8 { SMP8 }
                    SMP7 { SMP7 }
                    SMP6 { SMP6 }
                    SMP5 { SMP5 }
                    SMP4 { SMP4 }
                    SMP3 { SMP3 }
                    SMP2 { SMP2 }
                    SMP1 { SMP1 }
                    SMP0 { SMP0 }
                }
                JOFR1 {
                    JOFR1;
                    JOFFSET1 { JOFFSET1 }
                }
                JOFR2 {
                    JOFR2;
                    JOFFSET2 { JOFFSET2 }
                }
                JOFR3 {
                    JOFR3;
                    JOFFSET3 { JOFFSET3 }
                }
                JOFR4 {
                    JOFR4;
                    JOFFSET4 { JOFFSET4 }
                }
                HTR {
                    HTR;
                    HT { HT }
                }
                LTR {
                    LTR;
                    LT { LT }
                }
                SQR1 {
                    SQR1;
                    L { L }
                    SQ16 { SQ16 }
                    SQ15 { SQ15 }
                    SQ14 { SQ14 }
                    SQ13 { SQ13 }
                }
                SQR2 {
                    SQR2;
                    SQ12 { SQ12 }
                    SQ11 { SQ11 }
                    SQ10 { SQ10 }
                    SQ9 { SQ9 }
                    SQ8 { SQ8 }
                    SQ7 { SQ7 }
                }
                SQR3 {
                    SQR3;
                    SQ6 { SQ6 }
                    SQ5 { SQ5 }
                    SQ4 { SQ4 }
                    SQ3 { SQ3 }
                    SQ2 { SQ2 }
                    SQ1 { SQ1 }
                }
                DR {
                    DR;
                    DATA { DATA }
                }
                JSQR {
                    JSQR;
                    JL { JL }
                    JSQ1 { JSQ1 }
                    JSQ2 { JSQ2 }
                    JSQ3 { JSQ3 }
                    JSQ4 { JSQ4 }
                }
                JDR1 {
                    JDR1;
                    JDATA { JDATA }
                }
                JDR2 {
                    JDR2;
                    JDATA { JDATA }
                }
                JDR3 {
                    JDR3;
                    JDATA { JDATA }
                }
                JDR4 {
                    JDR4;
                    JDATA { JDATA }
                }
            }
        }
    };
}

map_adc! {
    "Extracts ADC1 register tokens.",
    periph_adc1,
    "ADC1 peripheral variant.",
    Adc1,
    ADC1,
    APB2ENR,
    APB2LPENR,
    ADC1EN,
    ADC1LPEN,
}

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469"
))]
map_adc! {
    "Extracts ADC2 register tokens.",
    periph_adc2,
    "ADC2 peripheral variant.",
    Adc2,
    ADC2,
    APB2ENR,
    APB2LPENR,
    ADC2EN,
    ADC2LPEN,
}

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469"
))]
map_adc! {
    "Extracts ADC3 register tokens.",
    periph_adc3,
    "ADC3 peripheral variant.",
    Adc3,
    ADC3,
    APB2ENR,
    APB2LPENR,
    ADC3EN,
    ADC3LPEN,
}

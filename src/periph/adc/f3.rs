use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic ADC peripheral variant.
    pub trait AdcMap {}

    /// Generic ADC peripheral.
    pub struct AdcPeriph;
    ADC {
        ISR {
            0x20 RwReg;
            JQOVF { RwRwRegFieldBit }
            AWD3 { RwRwRegFieldBit }
            AWD2 { RwRwRegFieldBit }
            AWD1 { RwRwRegFieldBit }
            JEOS { RwRwRegFieldBit }
            JEOC { RwRwRegFieldBit }
            OVR { RwRwRegFieldBit }
            EOS { RwRwRegFieldBit }
            EOC { RwRwRegFieldBit }
            EOSMP { RwRwRegFieldBit }
            ADRDY { RwRwRegFieldBit }
        }
        IER {
            0x20 RwReg;
            JQOVFIE { RwRwRegFieldBit }
            AWD3IE { RwRwRegFieldBit }
            AWD2IE { RwRwRegFieldBit }
            AWD1IE { RwRwRegFieldBit }
            JEOSIE { RwRwRegFieldBit }
            JEOCIE { RwRwRegFieldBit }
            OVRIE { RwRwRegFieldBit }
            EOSIE { RwRwRegFieldBit }
            EOCIE { RwRwRegFieldBit }
            EOSMPIE { RwRwRegFieldBit }
            ADRDYIE { RwRwRegFieldBit }
        }
        CR {
            0x20 RwReg;
            ADCAL { RwRwRegFieldBit }
            ADCALDIF { RwRwRegFieldBit }
            ADVREGEN { RwRwRegFieldBits }
            JADSTP { RwRwRegFieldBit }
            ADSTP { RwRwRegFieldBit }
            JADSTART { RwRwRegFieldBit }
            ADSTART { RwRwRegFieldBit }
            ADDIS { RwRwRegFieldBit }
            ADEN { RwRwRegFieldBit }
        }
        CFGR {
            0x20 RwReg;
            AWDCH1CH { RwRwRegFieldBits }
            JAUTO { RwRwRegFieldBit }
            JAWD1EN { RwRwRegFieldBit }
            AWD1EN { RwRwRegFieldBit }
            AWD1SGL { RwRwRegFieldBit }
            JQM { RwRwRegFieldBit }
            JDISCEN { RwRwRegFieldBit }
            DISCNUM { RwRwRegFieldBits }
            DISCEN { RwRwRegFieldBit }
            AUTDLY { RwRwRegFieldBit }
            CONT { RwRwRegFieldBit }
            OVRMOD { RwRwRegFieldBit }
            EXTEN { RwRwRegFieldBits }
            EXTSEL { RwRwRegFieldBits }
            ALIGN { RwRwRegFieldBit }
            RES { RwRwRegFieldBits }
            DMACFG { RwRwRegFieldBit }
            DMAEN { RwRwRegFieldBit }
        }
        SMPR1 {
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
        }
        SMPR2 {
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
        TR1 {
            0x20 RwReg;
            HT1 { RwRwRegFieldBits }
            LT1 { RwRwRegFieldBits }
        }
        TR2 {
            0x20 RwReg;
            HT2 { RwRwRegFieldBits }
            LT2 { RwRwRegFieldBits }
        }
        TR3 {
            0x20 RwReg;
            HT3 { RwRwRegFieldBits }
            LT3 { RwRwRegFieldBits }
        }
        SQR1 {
            0x20 RwReg;
            SQ4 { RwRwRegFieldBits }
            SQ3 { RwRwRegFieldBits }
            SQ2 { RwRwRegFieldBits }
            SQ1 { RwRwRegFieldBits }
            L3 { RwRwRegFieldBits }
        }
        SQR2 {
            0x20 RwReg;
            SQ9 { RwRwRegFieldBits }
            SQ8 { RwRwRegFieldBits }
            SQ7 { RwRwRegFieldBits }
            SQ6 { RwRwRegFieldBits }
            SQ5 { RwRwRegFieldBits }
        }
        SQR3 {
            0x20 RwReg;
            SQ14 { RwRwRegFieldBits }
            SQ13 { RwRwRegFieldBits }
            SQ12 { RwRwRegFieldBits }
            SQ11 { RwRwRegFieldBits }
            SQ10 { RwRwRegFieldBits }
        }
        SQR4 {
            0x20 RwReg;
            SQ16 { RwRwRegFieldBits }
            SQ15 { RwRwRegFieldBits }
        }
        DR {
            0x20 RoReg;
            RDATA { RoRoRegFieldBits }
        }
        JSQR {
            0x20 RwReg;
            JSQ4 { RwRwRegFieldBits }
            JSQ3 { RwRwRegFieldBits }
            JSQ2 { RwRwRegFieldBits }
            JSQ1 { RwRwRegFieldBits }
            JEXTEN { RwRwRegFieldBits }
            JEXTSEL { RwRwRegFieldBits }
            JL { RwRwRegFieldBits }
        }
        OFR1 {
            0x20 RwReg;
            OFFSET1_EN { RwRwRegFieldBit }
            OFFSET1_CH { RwRwRegFieldBits }
            OFFSET1 { RwRwRegFieldBits }
        }
        OFR2 {
            0x20 RwReg;
            OFFSET2_EN { RwRwRegFieldBit }
            OFFSET2_CH { RwRwRegFieldBits }
            OFFSET2 { RwRwRegFieldBits }
        }
        OFR3 {
            0x20 RwReg;
            OFFSET3_EN { RwRwRegFieldBit }
            OFFSET3_CH { RwRwRegFieldBits }
            OFFSET3 { RwRwRegFieldBits }
        }
        OFR4 {
            0x20 RwReg;
            OFFSET4_EN { RwRwRegFieldBit }
            OFFSET4_CH { RwRwRegFieldBits }
            OFFSET4 { RwRwRegFieldBits }
        }
        JDR1 {
            0x20 RoReg;
            JDATA1 { RoRoRegFieldBits }
        }
        JDR2 {
            0x20 RoReg;
            JDATA2 { RoRoRegFieldBits }
        }
        JDR3 {
            0x20 RoReg;
            JDATA3 { RoRoRegFieldBits }
        }
        JDR4 {
            0x20 RoReg;
            JDATA4 { RoRoRegFieldBits }
        }
        AWD2CR {
            0x20 RwReg;
            AWD2CH { RwRwRegFieldBits }
        }
        AWD3CR {
            0x20 RwReg;
            AWD3CH { RwRwRegFieldBits }
        }
        DIFSEL {
            0x20 RwReg;
            DIFSEL_1_15 { RwRwRegFieldBits }
            DIFSEL_16_18 { RwRwRegFieldBits }
        }
        CALFACT {
            0x20 RwReg;
            CALFACT_D { RwRwRegFieldBits }
            CALFACT_S { RwRwRegFieldBits }
        } 
    }
}

#[allow(unused_macros)]
macro_rules! map_adc {
    (
        $adc_macro_doc:expr, 
        $adc_macro:ident, 
        $adc_ty_doc:expr, 
        $adc_ty:ident, 
        $adc:ident,
    ) => {
        periph::map! {
            #[doc = $adc_macro_doc]
            pub macro $adc_macro;

            #[doc = $adc_ty_doc]
            pub struct $adc_ty;

            impl AdcMap for $adc_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            ADC {
                $adc;
                ISR {
                    ISR;
                    JQOVF { JQOVF }
                    AWD3 { AWD3 }
                    AWD2 { AWD2 }
                    AWD1 { AWD1 }
                    JEOS { JEOS }
                    JEOC { JEOC }
                    OVR { OVR }
                    EOS { EOS }
                    EOC { EOC }
                    EOSMP { EOSMP }
                    ADRDY { ADRDY }
                }
                IER {
                    IER;
                    JQOVFIE { JQOVFIE }
                    AWD3IE { AWD3IE }
                    AWD2IE { AWD2IE }
                    AWD1IE { AWD1IE }
                    JEOSIE { JEOSIE }
                    JEOCIE { JEOCIE }
                    OVRIE { OVRIE }
                    EOSIE { EOSIE }
                    EOCIE { EOCIE }
                    EOSMPIE { EOSMPIE }
                    ADRDYIE { ADRDYIE }
                }
                CR {
                    CR;
                    ADCAL { ADCAL }
                    ADCALDIF { ADCALDIF }
                    ADVREGEN { ADVREGEN }
                    JADSTP { JADSTP }
                    ADSTP { ADSTP }
                    JADSTART { JADSTART }
                    ADSTART { ADSTART }
                    ADDIS { ADDIS }
                    ADEN { ADEN }
                }
                CFGR {
                    CFGR;
                    AWDCH1CH { AWDCH1CH }
                    JAUTO { JAUTO }
                    JAWD1EN { JAWD1EN }
                    AWD1EN { AWD1EN }
                    AWD1SGL { AWD1SGL }
                    JQM { JQM }
                    JDISCEN { JDISCEN }
                    DISCNUM { DISCNUM }
                    DISCEN { DISCEN }
                    AUTDLY { AUTDLY }
                    CONT { CONT }
                    OVRMOD { OVRMOD }
                    EXTEN { EXTEN }
                    EXTSEL { EXTSEL }
                    ALIGN { ALIGN }
                    RES { RES }
                    DMACFG { DMACFG }
                    DMAEN { DMAEN }
                }
                SMPR1 {
                    SMPR1;
                    SMP9 { SMP9 }
                    SMP8 { SMP8 }
                    SMP7 { SMP7 }
                    SMP6 { SMP6 }
                    SMP5 { SMP5 }
                    SMP4 { SMP4 }
                    SMP3 { SMP3 }
                    SMP2 { SMP2 }
                    SMP1 { SMP1 }
                }
                SMPR2 {
                    SMPR2;
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
                TR1 {
                    TR1;
                    HT1 { HT1 }
                    LT1 { LT1 }
                }
                TR2 {
                    TR2;
                    HT2 { HT2 }
                    LT2 { LT2 }
                }
                TR3 {
                    TR3;
                    HT3 { HT3 }
                    LT3 { LT3 }
                }
                SQR1 {
                    SQR1;
                    SQ4 { SQ4 }
                    SQ3 { SQ3 }
                    SQ2 { SQ2 }
                    SQ1 { SQ1 }
                    L3 { L3 }
                }
                SQR2 {
                    SQR2;
                    SQ9 { SQ9 }
                    SQ8 { SQ8 }
                    SQ7 { SQ7 }
                    SQ6 { SQ6 }
                    SQ5 { SQ5 }
                }
                SQR3 {
                    SQR3;
                    SQ14 { SQ14 }
                    SQ13 { SQ13 }
                    SQ12 { SQ12 }
                    SQ11 { SQ11 }
                    SQ10 { SQ10 }
                }
                SQR4 {
                    SQR4;
                    SQ16 { SQ16 }
                    SQ15 { SQ15 }
                }
                DR {
                    DR;
                    RDATA { RDATA }
                }
                JSQR {
                    JSQR;
                    JSQ4 { JSQ4 }
                    JSQ3 { JSQ3 }
                    JSQ2 { JSQ2 }
                    JSQ1 { JSQ1 }
                    JEXTEN { JEXTEN }
                    JEXTSEL { JEXTSEL }
                    JL { JL }
                }
                OFR1 {
                    OFR1;
                    OFFSET1_EN { OFFSET1_EN }
                    OFFSET1_CH { OFFSET1_CH }
                    OFFSET1 { OFFSET1 }
                }
                OFR2 {
                    OFR2;
                    OFFSET2_EN { OFFSET2_EN }
                    OFFSET2_CH { OFFSET2_CH }
                    OFFSET2 { OFFSET2 }
                }
                OFR3 {
                    OFR3;
                    OFFSET3_EN { OFFSET3_EN }
                    OFFSET3_CH { OFFSET3_CH }
                    OFFSET3 { OFFSET3 }
                }
                OFR4 {
                    OFR4;
                    OFFSET4_EN { OFFSET4_EN }
                    OFFSET4_CH { OFFSET4_CH }
                    OFFSET4 { OFFSET4 }
                }
                JDR1 {
                    JDR1;
                    JDATA1 { JDATA1 }
                }
                JDR2 {
                    JDR2;
                    JDATA2 { JDATA2 }
                }
                JDR3 {
                    JDR3;
                    JDATA3 { JDATA3 }
                }
                JDR4 {
                    JDR4;
                    JDATA4 { JDATA4 }
                }
                AWD2CR {
                    AWD2CR;
                    AWD2CH { AWD2CH }
                }
                AWD3CR {
                    AWD3CR;
                    AWD3CH { AWD3CH }
                }
                DIFSEL {
                    DIFSEL;
                    DIFSEL_1_15 { DIFSEL_1_15 }
                    DIFSEL_16_18 { DIFSEL_16_18 }
                }
                CALFACT {
                    CALFACT;
                    CALFACT_D { CALFACT_D }
                    CALFACT_S { CALFACT_S }
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
}

map_adc! {
    "Extracts ADC2 register tokens.",
    periph_adc2,
    "ADC2 peripheral variant.",
    Adc2,
    ADC2,
}

map_adc! {
    "Extracts ADC3 register tokens.",
    periph_adc3,
    "ADC3 peripheral variant.",
    Adc3,
    ADC3,
}

map_adc! {
    "Extracts ADC4 register tokens.",
    periph_adc4,
    "ADC4 peripheral variant.",
    Adc4,
    ADC4,
}

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
        CCIPR {
            0x20 RwRegBitBand Shared;
            ADCSEL { RwRwRegFieldBits }
        }
    }
    ADC {
        ISR {
            0x20 RwReg;
            ADRDY { RwRwRegFieldBit }
            AWD1 { RwRwRegFieldBit }
            AWD2 { RwRwRegFieldBit }
            AWD3 { RwRwRegFieldBit }
            EOC { RwRwRegFieldBit }
            EOSMP { RwRwRegFieldBit }
            EOS { RwRwRegFieldBit }
            JEOC { RwRwRegFieldBit }
            JEOS { RwRwRegFieldBit }
            JQOVF { RwRwRegFieldBit }
            OVR { RwRwRegFieldBit }
        }
        IER {
            0x20 RwReg;
            ADRDYIE { RwRwRegFieldBit }
            AWD1IE { RwRwRegFieldBit }
            AWD2IE { RwRwRegFieldBit }
            AWD3IE { RwRwRegFieldBit }
            EOCIE { RwRwRegFieldBit }
            EOSIE { RwRwRegFieldBit }
            EOSMPIE { RwRwRegFieldBit }
            JEOCIE { RwRwRegFieldBit }
            JEOSIE { RwRwRegFieldBit }
            JQOVFIE { RwRwRegFieldBit }
            OVRIE { RwRwRegFieldBit }
        }
        CR {
            0x20 RwReg;
            ADCALDIF { RwRwRegFieldBit }
            ADCAL { RwRwRegFieldBit }
            ADDIS { RwRwRegFieldBit }
            ADEN { RwRwRegFieldBit }
            ADSTART { RwRwRegFieldBit }
            ADSTP { RwRwRegFieldBit }
            ADVREGEN { RwRwRegFieldBit }
            DEEPPWD { RwRwRegFieldBit }
            JADSTART { RwRwRegFieldBit }
            JADSTP { RwRwRegFieldBit }
        }
        CFGR {
            0x20 RwReg;
            ALIGN { RwRwRegFieldBit }
            AUTDLY { RwRwRegFieldBit }
            AWD1EN { RwRwRegFieldBit }
            AWD1SGL { RwRwRegFieldBit }
            AWDCH1CH { RwRwRegFieldBits }
            CONT { RwRwRegFieldBit }
            DISCEN { RwRwRegFieldBit }
            DISCNUM { RwRwRegFieldBits }
            DMACFG { RwRwRegFieldBit }
            DMAEN { RwRwRegFieldBit }
            EXTEN { RwRwRegFieldBits }
            EXTSEL { RwRwRegFieldBits }
            JAUTO { RwRwRegFieldBit }
            JAWD1EN { RwRwRegFieldBit }
            JDISCEN { RwRwRegFieldBit }
            JQDIS { RwRwRegFieldBit }
            JQM { RwRwRegFieldBit }
            OVRMOD { RwRwRegFieldBit }
            RES { RwRwRegFieldBits }
        }
        CFGR2 {
            0x20 RwReg;
            JOVSE { RwRwRegFieldBit }
            OVSR { RwRwRegFieldBits }
            OVSS { RwRwRegFieldBits }
            ROVSE { RwRwRegFieldBit }
            ROVSM { RwRwRegFieldBit }
            TROVS { RwRwRegFieldBit }
        }
        SMPR1 {
            0x20 RwReg;
            SMP0 { RwRwRegFieldBits }
            SMP1 { RwRwRegFieldBits }
            SMP2 { RwRwRegFieldBits }
            SMP3 { RwRwRegFieldBits }
            SMP4 { RwRwRegFieldBits }
            SMP5 { RwRwRegFieldBits }
            SMP6 { RwRwRegFieldBits }
            SMP7 { RwRwRegFieldBits }
            SMP8 { RwRwRegFieldBits }
            SMP9 { RwRwRegFieldBits }
        }
        SMPR2 {
            0x20 RwReg;
            SMP10 { RwRwRegFieldBits }
            SMP11 { RwRwRegFieldBits }
            SMP12 { RwRwRegFieldBits }
            SMP13 { RwRwRegFieldBits }
            SMP14 { RwRwRegFieldBits }
            SMP15 { RwRwRegFieldBits }
            SMP16 { RwRwRegFieldBits }
            SMP17 { RwRwRegFieldBits }
            SMP18 { RwRwRegFieldBits }
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
            L { RwRwRegFieldBits }
            SQ1 { RwRwRegFieldBits }
            SQ2 { RwRwRegFieldBits }
            SQ3 { RwRwRegFieldBits }
            SQ4 { RwRwRegFieldBits }
        }
        SQR2 {
            0x20 RwReg;
            SQ5 { RwRwRegFieldBits }
            SQ6 { RwRwRegFieldBits }
            SQ7 { RwRwRegFieldBits }
            SQ8 { RwRwRegFieldBits }
            SQ9 { RwRwRegFieldBits }
        }
        SQR3 {
            0x20 RwReg;
            SQ10 { RwRwRegFieldBits }
            SQ11 { RwRwRegFieldBits }
            SQ12 { RwRwRegFieldBits }
            SQ13 { RwRwRegFieldBits }
            SQ14 { RwRwRegFieldBits }
        }
        SQR4 {
            0x20 RwReg;
            SQ15 { RwRwRegFieldBits }
            SQ16 { RwRwRegFieldBits }
        }
        DR {
            0x20 RoReg;
            RDATA { RoRoRegFieldBits }
        }
        JSQR {
            0x20 RwReg;
            JEXTEN { RwRwRegFieldBits }
            JEXTSEL { RwRwRegFieldBits }
            JL { RwRwRegFieldBits }
            JSQ1 { RwRwRegFieldBits }
            JSQ2 { RwRwRegFieldBits }
            JSQ3 { RwRwRegFieldBits }
            JSQ4 { RwRwRegFieldBits }
        }
        OFR1 {
            0x20 RwReg;
            OFFSET1_CH { RwRwRegFieldBits }
            OFFSET1_EN { RwRwRegFieldBit }
            OFFSET1 { RwRwRegFieldBits }
        }
        OFR2 {
            0x20 RwReg;
            OFFSET2_CH { RwRwRegFieldBits }
            OFFSET2_EN { RwRwRegFieldBit }
            OFFSET2 { RwRwRegFieldBits }
        }
        OFR3 {
            0x20 RwReg;
            OFFSET3_CH { RwRwRegFieldBits }
            OFFSET3_EN { RwRwRegFieldBit }
            OFFSET3 { RwRwRegFieldBits }
        }
        OFR4 {
            0x20 RwReg;
            OFFSET4_CH { RwRwRegFieldBits }
            OFFSET4_EN { RwRwRegFieldBit }
            OFFSET4 { RwRwRegFieldBits }
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
            DIFSEL_16_18 { RoRwRegFieldBits }
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
                CCIPR {
                    CCIPR Shared;
                    ADCSEL { ADCSEL }
                }
            }
            ADC {
                $adc;
                ISR {
                    ISR;
                    ADRDY { ADRDY }
                    AWD1 { AWD1 }
                    AWD2 { AWD2 }
                    AWD3 { AWD3 }
                    EOC { EOC }
                    EOSMP { EOSMP }
                    EOS { EOS }
                    JEOC { JEOC }
                    JEOS { JEOS }
                    JQOVF { JQOVF }
                    OVR { OVR }
                }
                IER {
                    IER;
                    ADRDYIE { ADRDYIE }
                    AWD1IE { AWD1IE }
                    AWD2IE { AWD2IE }
                    AWD3IE { AWD3IE }
                    EOCIE { EOCIE }
                    EOSIE { EOSIE }
                    EOSMPIE { EOSMPIE }
                    JEOCIE { JEOCIE }
                    JEOSIE { JEOSIE }
                    JQOVFIE { JQOVFIE }
                    OVRIE { OVRIE }
                }
                CR {
                    CR;
                    ADCALDIF { ADCALDIF }
                    ADCAL { ADCAL }
                    ADDIS { ADDIS }
                    ADEN { ADEN }
                    ADSTART { ADSTART }
                    ADSTP { ADSTP }
                    ADVREGEN { ADVREGEN }
                    DEEPPWD { DEEPPWD }
                    JADSTART { JADSTART }
                    JADSTP { JADSTP }
                }
                CFGR {
                    CFGR;
                    ALIGN { ALIGN }
                    AUTDLY { AUTDLY }
                    AWD1EN { AWD1EN }
                    AWD1SGL { AWD1SGL }
                    AWDCH1CH { AWDCH1CH }
                    CONT { CONT }
                    DISCEN { DISCEN }
                    DISCNUM { DISCNUM }
                    DMACFG { DMACFG }
                    DMAEN { DMAEN }
                    EXTEN { EXTEN }
                    EXTSEL { EXTSEL }
                    JAUTO { JAUTO }
                    JAWD1EN { JAWD1EN }
                    JDISCEN { JDISCEN }
                    JQDIS { JQDIS }
                    JQM { JQM }
                    OVRMOD { OVRMOD }
                    RES { RES }
                }
                CFGR2 {
                    CFGR2;
                    JOVSE { JOVSE }
                    OVSR { OVSR }
                    OVSS { OVSS }
                    ROVSE { ROVSE }
                    ROVSM { ROVSM }
                    TROVS { TROVS }
                }
                SMPR1 {
                    SMPR1;
                    SMP0 { SMP0 }
                    SMP1 { SMP1 }
                    SMP2 { SMP2 }
                    SMP3 { SMP3 }
                    SMP4 { SMP4 }
                    SMP5 { SMP5 }
                    SMP6 { SMP6 }
                    SMP7 { SMP7 }
                    SMP8 { SMP8 }
                    SMP9 { SMP9 }
                }
                SMPR2 {
                    SMPR2;
                    SMP10 { SMP10 }
                    SMP11 { SMP11 }
                    SMP12 { SMP12 }
                    SMP13 { SMP13 }
                    SMP14 { SMP14 }
                    SMP15 { SMP15 }
                    SMP16 { SMP16 }
                    SMP17 { SMP17 }
                    SMP18 { SMP18 }
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
                    L { L }
                    SQ1 { SQ1 }
                    SQ2 { SQ2 }
                    SQ3 { SQ3 }
                    SQ4 { SQ4 }
                }
                SQR2 {
                    SQR2;
                    SQ5 { SQ5 }
                    SQ6 { SQ6 }
                    SQ7 { SQ7 }
                    SQ8 { SQ8 }
                    SQ9 { SQ9 }
                }
                SQR3 {
                    SQR3;
                    SQ10 { SQ10 }
                    SQ11 { SQ11 }
                    SQ12 { SQ12 }
                    SQ13 { SQ13 }
                    SQ14 { SQ14 }
                }
                SQR4 {
                    SQR4;
                    SQ15 { SQ15 }
                    SQ16 { SQ16 }
                }
                DR {
                    DR;
                    RDATA { RDATA }
                }
                JSQR {
                    JSQR;
                    JEXTEN { JEXTEN }
                    JEXTSEL { JEXTSEL }
                    JL { JL }
                    JSQ1 { JSQ1 }
                    JSQ2 { JSQ2 }
                    JSQ3 { JSQ3 }
                    JSQ4 { JSQ4 }
                }
                OFR1 {
                    OFR1;
                    OFFSET1_CH { OFFSET1_CH }
                    OFFSET1_EN { OFFSET1_EN }
                    OFFSET1 { OFFSET1 }
                }
                OFR2 {
                    OFR2;
                    OFFSET2_CH { OFFSET2_CH }
                    OFFSET2_EN { OFFSET2_EN }
                    OFFSET2 { OFFSET2 }
                }
                OFR3 {
                    OFR3;
                    OFFSET3_CH { OFFSET3_CH }
                    OFFSET3_EN { OFFSET3_EN }
                    OFFSET3 { OFFSET3 }
                }
                OFR4 {
                    OFR4;
                    OFFSET4_CH { OFFSET4_CH }
                    OFFSET4_EN { OFFSET4_EN }
                    OFFSET4 { OFFSET4 }
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
    "Extracts ADC register tokens.",
    periph_adc1,
    "ADC1 peripheral variant.",
    Adc1,
    ADC,
    AHB2ENR,
    AHB2SMENR,
    ADCEN,
    ADCSMEN,
}

//! Analog-to-digital converters dual mode common registers
//! for STM32F3 Series of mixed-signal MCUs with DSP and FPU instructions.

#[allow(unused_imports)]
use drone_core::periph;

periph::singular! {
    /// Extracts ADC Dual Mode register tokens.
    pub macro periph_adc_dual;

    /// ADC Dual Mode peripheral.
    pub struct AdcDualPeriph;

    drone_stm32_map_pieces::reg;
    crate::dual;

    ADC1_2 {
        CSR {
            ADDRDY_MST;
            EOSMP_MST;
            EOC_MST;
            EOS_MST;
            OVR_MST;
            JEOC_MST;
            JEOS_MST;
            AWD1_MST;
            AWD2_MST;
            AWD3_MST;
            JQOVF_MST;
            ADRDY_SLV;
            EOSMP_SLV;
            EOC_SLV;
            EOS_SLV;
            OVR_SLV;
            JEOC_SLV;
            JEOS_SLV;
            AWD1_SLV;
            AWD2_SLV;
            AWD3_SLV;
            JQOVF_SLV;
        }
        CCR {
            MULT;
            DELAY;
            DMACFG;
            MDMA;
            CKMODE;
            VREFEN;
            TSEN;
            VBATEN;
        }
        CDR {
            RDATA_SLV;
            RDATA_MST;
        }
    }

    ADC3_4 {
        CSR {
            ADDRDY_MST;
            EOSMP_MST;
            EOC_MST;
            EOS_MST;
            OVR_MST;
            JEOC_MST;
            JEOS_MST;
            AWD1_MST;
            AWD2_MST;
            AWD3_MST;
            JQOVF_MST;
            ADRDY_SLV;
            EOSMP_SLV;
            EOC_SLV;
            EOS_SLV;
            OVR_SLV;
            JEOC_SLV;
            JEOS_SLV;
            AWD1_SLV;
            AWD2_SLV;
            AWD3_SLV;
            JQOVF_SLV;
        }
        CCR {
            MULT;
            DELAY;
            DMACFG;
            MDMA;
            CKMODE;
            VREFEN;
            TSEN;
            VBATEN;
        }
        CDR {
            RDATA_SLV;
            RDATA_MST;
        }
    }
}

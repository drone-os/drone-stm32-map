//! Analog-to-digital converters dual mode common registers
//!
//! For STM32F3 Series of mixed-signal MCUs with DSP and FPU instructions.

use drone_core::periph;

periph::singular! {
    /// Extracts ADC Dual Mode register tokens.
    pub macro periph_adc_dual;

    /// ADC Dual Mode peripheral.
    pub struct AdcDualPeriph;

    drone_stm32_map_pieces::reg;
    crate::dual;

    ADC1_2 {
        CSR;
        CCR;
        CDR;
    }

    ADC3_4 {
        CSR;
        CCR;
        CDR;
    }
}

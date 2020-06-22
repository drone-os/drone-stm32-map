//! Analog-to-digital converters common registers.
//! for STM32L4+ series of ultra-low-power MCUs.

#[allow(unused_imports)]
use drone_core::periph;

periph::singular! {
    /// Extracts ADC Common register tokens.
    pub macro periph_adc_com;

    /// ADC Common peripheral.
    pub struct AdcComPeriph;

    drone_stm32_map_pieces::reg;
    crate::com;

    RCC {
        AHB2RSTR {
            ADCRST;
        }
    }
    ADC_Common {
        CSR;
        CCR;
    }
}

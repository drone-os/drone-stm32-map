//! Analog-to-digital converters common registers.
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

use drone_core::periph;

periph::singular! {
    /// Extracts ADC Common register tokens.
    pub macro periph_adc_com;

    /// ADC Common peripheral.
    pub struct AdcComPeriph;

    drone_stm32_map_pieces::reg;
    crate::com;

    RCC {
        APB2RSTR {
            ADCRST;
        }
    }

    ADC_Common {
        #[cfg(any(
            drone_stm32_map = "stm32f405",
            drone_stm32_map = "stm32f407",
            drone_stm32_map = "stm32f410",
            drone_stm32_map = "stm32f412",
            drone_stm32_map = "stm32f413",
            drone_stm32_map = "stm32f427",
            drone_stm32_map = "stm32f429",
            drone_stm32_map = "stm32f446",
            drone_stm32_map = "stm32f469"
        ))]
        CSR;
        CCR;
        #[cfg(any(
            drone_stm32_map = "stm32f446",
            drone_stm32_map = "stm32f469"
        ))]
        CDR;
    }
}

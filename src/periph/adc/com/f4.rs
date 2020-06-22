//! Analog-to-digital converters common registers.
//! for STM32F4 series of high-performance MCUs with DSP and FPU instructions.

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
        APB2RSTR {
            ADCRST;
        }
    }
    ADC_Common {
        #[cfg(any(
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469"
        ))]
        CSR;
        CCR;
        #[cfg(any(
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469"
        ))]
        CDR;
    }
}

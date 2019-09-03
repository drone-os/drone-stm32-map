//! Analog-to-digital converters common registers.

#[allow(unused_imports)]
use drone_core::periph;

#[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
))]
periph::singular! {
    /// Extracts ADC Common register tokens.
    pub macro periph_adc_com;

    /// ADC Common peripheral.
    pub struct AdcComPeriph;

    drone_stm32_map_pieces::reg;
    crate::com;

    ADC_Common {
        CSR;
        CCR;
    }
}

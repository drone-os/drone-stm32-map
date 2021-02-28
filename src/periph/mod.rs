//! STM32 peripheral mappings.

#[doc(no_inline)]
pub use drone_cortexm::map::periph::*;

#[cfg(feature = "adc")]
pub extern crate drone_stm32_map_periph_adc as adc;
#[cfg(feature = "dma")]
pub extern crate drone_stm32_map_periph_dma as dma;
#[cfg(feature = "exti")]
pub extern crate drone_stm32_map_periph_exti as exti;
#[cfg(feature = "flash")]
pub extern crate drone_stm32_map_periph_flash as flash;
#[cfg(feature = "gpio")]
pub extern crate drone_stm32_map_periph_gpio as gpio;
#[cfg(feature = "i2c")]
pub extern crate drone_stm32_map_periph_i2c as i2c;
#[cfg(feature = "rtc")]
pub extern crate drone_stm32_map_periph_rtc as rtc;
#[cfg(feature = "spi")]
pub extern crate drone_stm32_map_periph_spi as spi;
#[cfg(feature = "tim")]
pub extern crate drone_stm32_map_periph_tim as tim;
#[cfg(feature = "uart")]
pub extern crate drone_stm32_map_periph_uart as uart;

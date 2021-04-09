//! Mapping for Flash.
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

use drone_core::periph;

periph::singular! {
    /// Extracts Flash register tokens.
    pub macro periph_flash;

    /// FLASH peripheral.
    pub struct FlashPeriph;

    drone_stm32_map_pieces::reg;
    crate;

    FLASH {
        ACR;
        KEYR;
        OPTKEYR;
        SR;
        CR;
        OPTCR;
        #[cfg(any(
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f437",
            stm32_mcu = "stm32f439",
        ))]
        OPTCR1;
    }
}

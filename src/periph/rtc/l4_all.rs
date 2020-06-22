//! Real-time clock.
//! for STM32L4 series of ultra-low-power MCUs
//! and STM32L4+ technology with extended an additional features.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

#[cfg(any(
    stm32_mcu = "stm32l4x1",
    stm32_mcu = "stm32l4x2",
    stm32_mcu = "stm32l4x3",
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
periph::singular! {
    /// Extracts RTC register tokens.
    pub macro periph_rtc;

    /// RTC peripheral.
    pub struct RtcPeriph;

    drone_stm32_map_pieces::reg;
    crate;

    RCC {
        APB1ENR1 {
            RTCAPBEN;
        }
        APB1SMENR1 {
            RTCAPBSMEN;
        }
        BDCR {
            RTCEN;
            RTCSEL;
        }
    }
    RTC {
        TR;
        DR;
        CR;
        ISR;
        PRER;
        WUTR;
        ALRMAR;
        ALRMBR;
        WPR;
        SSR;
        SHIFTR;
        TSTR;
        TSDR;
        TSSSR;
        CALR;
        TAMPCR;
        ALRMASSR;
        ALRMBSSR;
        OR;
        BKP0R;
        BKP1R;
        BKP2R;
        BKP3R;
        BKP4R;
        BKP5R;
        BKP6R;
        BKP7R;
        BKP8R;
        BKP9R;
        BKP10R;
        BKP11R;
        BKP12R;
        BKP13R;
        BKP14R;
        BKP15R;
        BKP16R;
        BKP17R;
        BKP18R;
        BKP19R;
        BKP20R;
        BKP21R;
        BKP22R;
        BKP23R;
        BKP24R;
        BKP25R;
        BKP26R;
        BKP27R;
        BKP28R;
        BKP29R;
        BKP30R;
        BKP31R;
    }
}

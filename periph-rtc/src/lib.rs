//! Real-time clock.

#![feature(proc_macro_hygiene)]
#![no_std]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

#[allow(unused_imports)]
use drone_core::periph;

#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6",
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
))]
periph::one! {
    /// Acquires RTC.
    pub macro periph_rtc;

    /// RTC.
    pub struct RtcPeriph;

    ::drone_stm32_map_pieces::reg;;

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

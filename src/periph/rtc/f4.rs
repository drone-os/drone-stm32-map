//! Real-time clock.
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

use drone_core::periph;

periph::singular! {
    /// Extracts RTC register tokens.
    pub macro periph_rtc;

    /// RTC peripheral.
    pub struct RtcPeriph;

    drone_stm32_map_pieces::reg;
    crate;

    PWR {
        CR {
            DBP;
        }
    }

    RCC {
        APB1ENR {
            PWREN;
        }
        CFGR {
            RTCPRE;
        }
        CSR {
            LSIRDY;
            LSION;
        }
        BDCR {
            BDRST;
            RTCEN;
            RTCSEL1;
            RTCSEL0;
            LSEBYP;
            LSERDY;
            LSEON;
        }
    }

    RTC {
        TR;
        DR;
        CR;
        ISR;
        PRER;
        WUTR;
        CALIBR;
        ALRMBR;
        ALRMAR;
        WPR;
        SSR;
        SHIFTR;
        TSTR;
        TSDR;
        TSSSR;
        CALR;
        TAFCR;
        ALRMASSR;
        ALRMBSSR;
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
    }
}

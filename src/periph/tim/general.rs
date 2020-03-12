//! General-purpose timers.

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
    /// Generic general-purpose timer peripheral variant.
    pub trait GeneralTimMap {}

    /// Generic general-purpose timer peripheral.
    pub struct GeneralTimPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            TIMEN { RwRwRegFieldBitBand }
        }
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            TIMRST { RwRwRegFieldBitBand }
        }
        #[cfg(any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469",
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
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            TIMSMEN { RwRwRegFieldBitBand }
        }
    }
    TIM {
        CR1 {
            0x20 RwRegBitBand;
            ARPE { RwRwRegFieldBitBand }
            CEN { RwRwRegFieldBitBand }
            CKD { RwRwRegFieldBits }
            CMS { RwRwRegFieldBits Option }
            DIR { RwRwRegFieldBitBand Option }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
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
            OPM { RwRwRegFieldBitBand }
            #[cfg(any(
                stm32_mcu = "stm32f410",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469"
            ))]
            OPM { RwRwRegFieldBitBand Option }
            UDIS { RwRwRegFieldBitBand }
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
            UIFREMAP { RwRwRegFieldBitBand }
            URS { RwRwRegFieldBitBand }
        }
        #[cfg(any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469"
        ))]
        CR2 {
            0x20 RwRegBitBand Option;
            TI1S { RwRwRegFieldBitBand }
            MMS { RwRwRegFieldBits }
            CCDS { RwRwRegFieldBitBand }
        }
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
        CR2 {
            0x20 RwRegBitBand;
            OIS2 { RwRwRegFieldBitBand Option }
            OIS1N { RwRwRegFieldBitBand Option }
            OIS1 { RwRwRegFieldBitBand Option }
            TI1S { RwRwRegFieldBitBand Option }
            MMS { RwRwRegFieldBits Option }
            CCDS { RwRwRegFieldBitBand }
            CCUS { RwRwRegFieldBitBand Option }
            CCPC { RwRwRegFieldBitBand Option }
        }
        SMCR {
            0x20 RwRegBitBand Option;
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
            SMS3 { RwRwRegFieldBitBand }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469",
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
            ETP { RwRwRegFieldBitBand Option }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469",
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
            ECE { RwRwRegFieldBitBand Option }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469",
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
            ETPS { RwRwRegFieldBits Option }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469",
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
            ETF { RwRwRegFieldBits Option }
            MSM { RwRwRegFieldBitBand }
            TS { RwRwRegFieldBits }
            SMS0_2 { RwRwRegFieldBits }
        }
        DIER {
            0x20 RwRegBitBand;
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
            BIE { RwRwRegFieldBitBand Option }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f410",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469"
            ))]
            CC1DE { RwRwRegFieldBitBand Option }
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
            CC1DE { RwRwRegFieldBitBand }
            CC1IE { RwRwRegFieldBitBand }
            CC2DE { RwRwRegFieldBitBand Option }
            CC2IE { RwRwRegFieldBitBand Option }
            CC3DE { RwRwRegFieldBitBand Option }
            CC3IE { RwRwRegFieldBitBand Option }
            CC4DE { RwRwRegFieldBitBand Option }
            CC4IE { RwRwRegFieldBitBand Option }
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
            COMDE { RwRwRegFieldBitBand Option }
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
            COMIE { RwRwRegFieldBitBand Option }
            TDE { RwRwRegFieldBitBand Option }
            TIE { RwRwRegFieldBitBand Option }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f410",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469"
            ))]
            UDE { RwRwRegFieldBitBand Option }
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
            UDE { RwRwRegFieldBitBand }
            UIE { RwRwRegFieldBitBand }
        }
        SR {
            0x20 RwRegBitBand;
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
            BIF { RwRwRegFieldBitBand Option }
            CC1IF { RwRwRegFieldBitBand }
            CC1OF { RwRwRegFieldBitBand }
            CC2IF { RwRwRegFieldBitBand Option }
            CC2OF { RwRwRegFieldBitBand Option }
            CC3IF { RwRwRegFieldBitBand Option }
            CC3OF { RwRwRegFieldBitBand Option }
            CC4IF { RwRwRegFieldBitBand Option }
            CC4OF { RwRwRegFieldBitBand Option }
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
            COMIF { RwRwRegFieldBitBand Option }
            TIF { RwRwRegFieldBitBand Option }
            UIF { RwRwRegFieldBitBand }
        }
        EGR {
            0x20 WoRegBitBand;
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
            BG { WoWoRegFieldBitBand Option }
            CC1G { WoWoRegFieldBitBand }
            CC2G { WoWoRegFieldBitBand Option }
            CC3G { WoWoRegFieldBitBand Option }
            CC4G { WoWoRegFieldBitBand Option }
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
            COMG { WoWoRegFieldBitBand Option }
            TG { WoWoRegFieldBitBand Option }
            UG { WoWoRegFieldBitBand }
        }
        CCMR1 {
            @Output 0x20 RwRegBitBand;
            CC1S { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits Option }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469",
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
            OC1CE { RwRwRegFieldBitBand Option }
            OC1FE { RwRwRegFieldBitBand }
            OC1M0_2 { RwRwRegFieldBits }
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
            OC1M3 { RwRwRegFieldBitBand }
            OC1PE { RwRwRegFieldBitBand }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469",
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
            OC2CE { RwRwRegFieldBitBand Option }
            OC2FE { RwRwRegFieldBitBand Option }
            OC2M0_2 { RwRwRegFieldBits Option }
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
            OC2M3 { RwRwRegFieldBitBand Option }
            OC2PE { RwRwRegFieldBitBand Option }
            @Input 0x20 RwRegBitBand;
            CC1S { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits Option }
            IC1F { RwRwRegFieldBits }
            IC1PSC { RwRwRegFieldBits }
            IC2F { RwRwRegFieldBits Option }
            IC2PSC { RwRwRegFieldBits Option }
        }
        CCMR2 {
            @Output 0x20 RwRegBitBand Option;
            CC3S { RwRwRegFieldBits }
            CC4S { RwRwRegFieldBits }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469",
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
            OC3CE { RwRwRegFieldBitBand }
            OC3FE { RwRwRegFieldBitBand }
            OC3M { RwRwRegFieldBits }
            OC3PE { RwRwRegFieldBitBand }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469",
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
            OC4CE { RwRwRegFieldBitBand }
            OC4FE { RwRwRegFieldBitBand }
            OC4M { RwRwRegFieldBits }
            OC4PE { RwRwRegFieldBitBand }
            @Input 0x20 RwRegBitBand Option;
            CC3S { RwRwRegFieldBits }
            CC4S { RwRwRegFieldBits }
            IC3F { RwRwRegFieldBits }
            IC3PSC { RwRwRegFieldBits }
            IC4F { RwRwRegFieldBits }
            IC4PSC { RwRwRegFieldBits }
        }
        CCER {
            0x20 RwRegBitBand;
            CC1E { RwRwRegFieldBitBand }
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
            CC1NE { RwRwRegFieldBitBand Option }
            CC1NP { RwRwRegFieldBitBand }
            CC1P { RwRwRegFieldBitBand }
            CC2E { RwRwRegFieldBitBand Option }
            CC2NP { RwRwRegFieldBitBand Option }
            CC2P { RwRwRegFieldBitBand Option }
            CC3E { RwRwRegFieldBitBand Option }
            CC3NP { RwRwRegFieldBitBand Option }
            CC3P { RwRwRegFieldBitBand Option }
            CC4E { RwRwRegFieldBitBand Option }
            CC4NP { RwRwRegFieldBitBand Option }
            CC4P { RwRwRegFieldBitBand Option }
        }
        CNT {
            0x20 RwRegBitBand;
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
            UIFCPY_CNT31 { RwRwRegFieldBitBand Option }
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
            UIFCPY { RoRwRegFieldBitBand Option }
            CNT { RwRwRegFieldBits }
        }
        PSC {
            0x20 RwRegBitBand;
            PSC { RwRwRegFieldBits }
        }
        ARR {
            0x20 RwRegBitBand;
            ARR { RwRwRegFieldBits }
        }
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
        RCR {
            0x20 RwRegBitBand Option;
            REP { RwRwRegFieldBits }
        }
        CCR1 {
            0x20 RwRegBitBand;
            CCR1 { RwRwRegFieldBits }
        }
        CCR2 {
            0x20 RwRegBitBand Option;
            CCR2 { RwRwRegFieldBits }
        }
        CCR3 {
            0x20 RwRegBitBand Option;
            CCR3 { RwRwRegFieldBits }
        }
        CCR4 {
            0x20 RwRegBitBand Option;
            CCR4 { RwRwRegFieldBits }
        }
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
        BDTR {
            0x20 RwRegBitBand Option;
            AOE { RwRwRegFieldBitBand }
            BKE { RwRwRegFieldBitBand }
            BKP { RwRwRegFieldBitBand }
            DTG { RwRwRegFieldBits }
            LOCK { RwRwRegFieldBits }
            MOE { RwRwRegFieldBitBand }
            OSSI { RwRwRegFieldBitBand }
            OSSR { RwRwRegFieldBitBand }
        }
        #[cfg(any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469"
        ))]
        DCR {
            0x20 RwRegBitBand Option;
            DBA { RwRwRegFieldBits }
            DBL { RwRwRegFieldBits }
        }
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
        DCR {
            0x20 RwRegBitBand;
            DBA { RwRwRegFieldBits }
            DBL { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32f401",
            stm32_mcu = "stm32f405",
            stm32_mcu = "stm32f407",
            stm32_mcu = "stm32f410",
            stm32_mcu = "stm32f411",
            stm32_mcu = "stm32f412",
            stm32_mcu = "stm32f413",
            stm32_mcu = "stm32f427",
            stm32_mcu = "stm32f429",
            stm32_mcu = "stm32f446",
            stm32_mcu = "stm32f469"
        ))]
        DMAR {
            0x20 RwRegBitBand Option;
            DMAB { RwRwRegFieldBits }
        }
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
        DMAR {
            0x20 RwRegBitBand;
            DMAB { RwRwRegFieldBits }
        }
        OR1 {
            0x20 RwRegBitBand Option;
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
            ENCODER_MODE { RwRwRegFieldBits Option }
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
            ETR1_RMP { RwRwRegFieldBitBand Option }
            #[cfg(any(
                stm32_mcu = "stm32f401",
                stm32_mcu = "stm32f405",
                stm32_mcu = "stm32f407",
                stm32_mcu = "stm32f411",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f427",
                stm32_mcu = "stm32f429",
                stm32_mcu = "stm32f446",
                stm32_mcu = "stm32f469"
            ))]
            ITR1_RMP { RwRwRegFieldBits Option }
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
            ITR1_RMP { RwRwRegFieldBitBand Option }
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
            TI1_RMP_BIT { RwRwRegFieldBitBand Option }
            TI1_RMP { RwRwRegFieldBits Option }
            TI4_RMP { RwRwRegFieldBits Option }
        }
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
        OR2 {
            0x20 RwRegBitBand Option;
            BKCMP1E { RwRwRegFieldBitBand Option }
            BKCMP1P { RwRwRegFieldBitBand Option }
            BKCMP2E { RwRwRegFieldBitBand Option }
            BKCMP2P { RwRwRegFieldBitBand Option }
            BKDFBKE { RwRwRegFieldBitBand Option }
            BKINE { RwRwRegFieldBitBand Option }
            BKINP { RwRwRegFieldBitBand Option }
            ETRSEL { RwRwRegFieldBits Option }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_general_tim {
    (
        $tim_macro_doc:expr,
        $tim_macro:ident,
        $tim_ty_doc:expr,
        $tim_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $timen:ident,
        $timrst:ident,
        $timsmen:ident,
        $tim:ident,
        (
            $($cms:ident)?,
            $($dir:ident)?,
            $($opm:ident)?
        ),
        ($(
            $cr2:ident,
            $($ois2:ident)?,
            $($ois1n:ident)?,
            $($ois1:ident)?,
            $($ti1s:ident)?,
            $($mms:ident)?,
            $($ccus:ident)?,
            $($ccpc:ident)?
        )?),
        ($(
            $smcr:ident,
            $($etp:ident)?,
            $($ece:ident)?,
            $($etps:ident)?,
            $($etf:ident)?
        )?),
        (
            $($bie:ident)?,
            $($cc1de:ident)?,
            $($cc2de:ident)?,
            $($cc2ie:ident)?,
            $($cc3de:ident)?,
            $($cc3ie:ident)?,
            $($cc4de:ident)?,
            $($cc4ie:ident)?,
            $($comde:ident)?,
            $($comie:ident)?,
            $($tde:ident)?,
            $($tie:ident)?,
            $($ude:ident)?
        ),
        (
            $($bif:ident)?,
            $($cc2if:ident)?,
            $($cc2of:ident)?,
            $($cc3if:ident)?,
            $($cc3of:ident)?,
            $($cc4if:ident)?,
            $($cc4of:ident)?,
            $($comif:ident)?,
            $($tif:ident)?
        ),
        (
            $($bg:ident)?,
            $($cc2g:ident)?,
            $($cc3g:ident)?,
            $($cc4g:ident)?,
            $($comg:ident)?,
            $($tg:ident)?
        ),
        (
            $($cc2s:ident)?,
            $($oc1ce:ident)?,
            $($oc2ce:ident)?,
            $($oc2fe:ident)?,
            $($oc2m0_2:ident)?,
            $($oc2m3:ident)?,
            $($oc2pe:ident)?,
            $($ic2f:ident)?,
            $($ic2psc:ident)?
        ),
        ($($ccmr2_input:ident)?, $($ccmr2_output:ident)?),
        (
            $($cc1ne:ident)?,
            $($cc2e:ident)?,
            $($cc2np:ident)?,
            $($cc2p:ident)?,
            $($cc3e:ident)?,
            $($cc3np:ident)?,
            $($cc3p:ident)?,
            $($cc4e:ident)?,
            $($cc4np:ident)?,
            $($cc4p:ident)?
        ),
        ($($uifcpy_cnt31:ident)?, $($uifcpy:ident)?),
        ($($rcr:ident)?),
        ($($ccr2:ident)?, $($ccr3:ident)?, $($ccr4:ident)?),
        ($($bdtr:ident)?),
        ($($dcr:ident)?),
        ($($dmar:ident)?),
        ($(
            $or1:ident,
            $($encoder_mode:ident)?,
            $($etr1_rmp:ident)?,
            $($itr1_rmp:ident)?,
            $($ti1_rmp_bit:ident)?,
            $($ti1_rmp:ident)?,
            $($ti4_rmp:ident)?
        )?),
        ($(
            $or2:ident,
            $($bkcmp1e:ident)?,
            $($bkcmp1p:ident)?,
            $($bkcmp2e:ident)?,
            $($bkcmp2p:ident)?,
            $($bkdfbke:ident)?,
            $($bkine:ident)?,
            $($bkinp:ident)?,
            $($etrsel:ident)?
        )?),
    ) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl GeneralTimMap for $tim_ty {}

            drone_stm32_map_pieces::reg;
            crate::general;

            RCC {
                BUSENR {
                    $busenr Shared;
                    TIMEN { $timen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    TIMRST { $timrst }
                }
                #[cfg(any(
                    stm32_mcu = "stm32f401",
                    stm32_mcu = "stm32f405",
                    stm32_mcu = "stm32f407",
                    stm32_mcu = "stm32f410",
                    stm32_mcu = "stm32f411",
                    stm32_mcu = "stm32f412",
                    stm32_mcu = "stm32f413",
                    stm32_mcu = "stm32f427",
                    stm32_mcu = "stm32f429",
                    stm32_mcu = "stm32f446",
                    stm32_mcu = "stm32f469",
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
                BUSSMENR {
                    $bussmenr Shared;
                    TIMSMEN { $timsmen }
                }
            }
            TIM {
                $tim;
                CR1 {
                    CR1;
                    ARPE { ARPE }
                    CEN { CEN }
                    CKD { CKD }
                    CMS { $($cms Option)* }
                    DIR { $($dir Option)* }
                    #[cfg(any(
                        stm32_mcu = "stm32f401",
                        stm32_mcu = "stm32f405",
                        stm32_mcu = "stm32f407",
                        stm32_mcu = "stm32f411",
                        stm32_mcu = "stm32f427",
                        stm32_mcu = "stm32f429",
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
                    OPM { OPM }
                    #[cfg(any(
                        stm32_mcu = "stm32f410",
                        stm32_mcu = "stm32f412",
                        stm32_mcu = "stm32f413",
                        stm32_mcu = "stm32f446",
                        stm32_mcu = "stm32f469"
                    ))]
                    OPM { $($opm Option)* }
                    UDIS { UDIS }
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
                    UIFREMAP { UIFREMAP }
                    URS { URS }
                }
                #[cfg(any(
                    stm32_mcu = "stm32f401",
                    stm32_mcu = "stm32f405",
                    stm32_mcu = "stm32f407",
                    stm32_mcu = "stm32f410",
                    stm32_mcu = "stm32f411",
                    stm32_mcu = "stm32f412",
                    stm32_mcu = "stm32f413",
                    stm32_mcu = "stm32f427",
                    stm32_mcu = "stm32f429",
                    stm32_mcu = "stm32f446",
                    stm32_mcu = "stm32f469"
                ))]
                CR2 {
                    $(
                        $cr2 Option;
                        TI1S { TI1S }
                        MMS { MMS }
                        CCDS { CCDS }
                    )*
                }
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
                CR2 {
                    $(
                        $cr2;
                        OIS2 { $($ois2 Option)* }
                        OIS1N { $($ois1n Option)* }
                        OIS1 { $($ois1 Option)* }
                        TI1S { $($ti1s Option)* }
                        MMS { $($mms Option)* }
                        CCDS { CCDS }
                        CCUS { $($ccus Option)* }
                        CCPC { $($ccpc Option)* }
                    )*
                }
                SMCR {
                    $(
                        $smcr Option;
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
                        SMS3 { SMS3 }
                        #[cfg(any(
                            stm32_mcu = "stm32f401",
                            stm32_mcu = "stm32f405",
                            stm32_mcu = "stm32f407",
                            stm32_mcu = "stm32f411",
                            stm32_mcu = "stm32f412",
                            stm32_mcu = "stm32f413",
                            stm32_mcu = "stm32f427",
                            stm32_mcu = "stm32f429",
                            stm32_mcu = "stm32f446",
                            stm32_mcu = "stm32f469",
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
                        ETP { $($etp Option)* }
                        #[cfg(any(
                            stm32_mcu = "stm32f401",
                            stm32_mcu = "stm32f405",
                            stm32_mcu = "stm32f407",
                            stm32_mcu = "stm32f411",
                            stm32_mcu = "stm32f412",
                            stm32_mcu = "stm32f413",
                            stm32_mcu = "stm32f427",
                            stm32_mcu = "stm32f429",
                            stm32_mcu = "stm32f446",
                            stm32_mcu = "stm32f469",
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
                        ECE { $($ece Option)* }
                        #[cfg(any(
                            stm32_mcu = "stm32f401",
                            stm32_mcu = "stm32f405",
                            stm32_mcu = "stm32f407",
                            stm32_mcu = "stm32f411",
                            stm32_mcu = "stm32f412",
                            stm32_mcu = "stm32f413",
                            stm32_mcu = "stm32f427",
                            stm32_mcu = "stm32f429",
                            stm32_mcu = "stm32f446",
                            stm32_mcu = "stm32f469",
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
                        ETPS { $($etps Option)* }
                        #[cfg(any(
                            stm32_mcu = "stm32f401",
                            stm32_mcu = "stm32f405",
                            stm32_mcu = "stm32f407",
                            stm32_mcu = "stm32f411",
                            stm32_mcu = "stm32f412",
                            stm32_mcu = "stm32f413",
                            stm32_mcu = "stm32f427",
                            stm32_mcu = "stm32f429",
                            stm32_mcu = "stm32f446",
                            stm32_mcu = "stm32f469",
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
                        ETF { $($etf Option)* }
                        MSM { MSM }
                        TS { TS }
                        #[cfg(any(
                            stm32_mcu = "stm32f401",
                            stm32_mcu = "stm32f405",
                            stm32_mcu = "stm32f407",
                            stm32_mcu = "stm32f410",
                            stm32_mcu = "stm32f411",
                            stm32_mcu = "stm32f412",
                            stm32_mcu = "stm32f413",
                            stm32_mcu = "stm32f427",
                            stm32_mcu = "stm32f429",
                            stm32_mcu = "stm32f446",
                            stm32_mcu = "stm32f469"
                        ))]
                        SMS0_2 { SMS }
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
                        SMS0_2 { SMS0_2 }
                    )*
                }
                DIER {
                    DIER;
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
                    BIE { $($bie Option)* }
                    #[cfg(any(
                        stm32_mcu = "stm32f401",
                        stm32_mcu = "stm32f405",
                        stm32_mcu = "stm32f407",
                        stm32_mcu = "stm32f410",
                        stm32_mcu = "stm32f411",
                        stm32_mcu = "stm32f412",
                        stm32_mcu = "stm32f413",
                        stm32_mcu = "stm32f427",
                        stm32_mcu = "stm32f429",
                        stm32_mcu = "stm32f446",
                        stm32_mcu = "stm32f469"
                    ))]
                    CC1DE { $($cc1de Option)* }
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
                    CC1DE { CC1DE }
                    CC1IE { CC1IE }
                    CC2DE { $($cc2de Option)* }
                    CC2IE { $($cc2ie Option)* }
                    CC3DE { $($cc3de Option)* }
                    CC3IE { $($cc3ie Option)* }
                    CC4DE { $($cc4de Option)* }
                    CC4IE { $($cc4ie Option)* }
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
                    COMDE { $($comde Option)* }
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
                    COMIE { $($comie Option)* }
                    TDE { $($tde Option)* }
                    TIE { $($tie Option)* }
                    #[cfg(any(
                        stm32_mcu = "stm32f401",
                        stm32_mcu = "stm32f405",
                        stm32_mcu = "stm32f407",
                        stm32_mcu = "stm32f410",
                        stm32_mcu = "stm32f411",
                        stm32_mcu = "stm32f412",
                        stm32_mcu = "stm32f413",
                        stm32_mcu = "stm32f427",
                        stm32_mcu = "stm32f429",
                        stm32_mcu = "stm32f446",
                        stm32_mcu = "stm32f469"
                    ))]
                    UDE { $($ude Option)* }
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
                    UDE { UDE }
                    UIE { UIE }
                }
                SR {
                    SR;
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
                    BIF { $($bif Option)* }
                    CC1IF { CC1IF }
                    CC1OF { CC1OF }
                    CC2IF { $($cc2if Option)* }
                    CC2OF { $($cc2of Option)* }
                    CC3IF { $($cc3if Option)* }
                    CC3OF { $($cc3of Option)* }
                    CC4IF { $($cc4if Option)* }
                    CC4OF { $($cc4of Option)* }
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
                    COMIF { $($comif Option)* }
                    TIF { $($tif Option)* }
                    UIF { UIF }
                }
                EGR {
                    EGR;
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
                    BG { $($bg Option)* }
                    CC1G { CC1G }
                    CC2G { $($cc2g Option)* }
                    CC3G { $($cc3g Option)* }
                    CC4G { $($cc4g Option)* }
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
                    COMG { $($comg Option)* }
                    TG { $($tg Option)* }
                    UG { UG }
                }
                CCMR1 {
                    @Output CCMR1_Output;
                    CC1S { CC1S }
                    CC2S { $($cc2s Option)* }
                    #[cfg(any(
                        stm32_mcu = "stm32f401",
                        stm32_mcu = "stm32f405",
                        stm32_mcu = "stm32f407",
                        stm32_mcu = "stm32f411",
                        stm32_mcu = "stm32f412",
                        stm32_mcu = "stm32f413",
                        stm32_mcu = "stm32f427",
                        stm32_mcu = "stm32f429",
                        stm32_mcu = "stm32f446",
                        stm32_mcu = "stm32f469",
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
                    OC1CE { $($oc1ce Option)* }
                    OC1FE { OC1FE }
                    #[cfg(any(
                        stm32_mcu = "stm32f401",
                        stm32_mcu = "stm32f405",
                        stm32_mcu = "stm32f407",
                        stm32_mcu = "stm32f410",
                        stm32_mcu = "stm32f411",
                        stm32_mcu = "stm32f412",
                        stm32_mcu = "stm32f413",
                        stm32_mcu = "stm32f427",
                        stm32_mcu = "stm32f429",
                        stm32_mcu = "stm32f446",
                        stm32_mcu = "stm32f469"
                    ))]
                    OC1M0_2 { OC1M }
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
                    OC1M0_2 { OC1M0_2 }
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
                    OC1M3 { OC1M3 }
                    OC1PE { OC1PE }
                    #[cfg(any(
                        stm32_mcu = "stm32f401",
                        stm32_mcu = "stm32f405",
                        stm32_mcu = "stm32f407",
                        stm32_mcu = "stm32f411",
                        stm32_mcu = "stm32f412",
                        stm32_mcu = "stm32f413",
                        stm32_mcu = "stm32f427",
                        stm32_mcu = "stm32f429",
                        stm32_mcu = "stm32f446",
                        stm32_mcu = "stm32f469",
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
                    OC2CE { $($oc2ce Option)* }
                    OC2FE { $($oc2fe Option)* }
                    OC2M0_2 { $($oc2m0_2 Option)* }
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
                    OC2M3 { $($oc2m3 Option)* }
                    OC2PE { $($oc2pe Option)* }
                    @Input CCMR1_Input;
                    CC1S { CC1S }
                    CC2S { $($cc2s Option)* }
                    IC1F { IC1F }
                    IC1PSC { IC1PSC }
                    IC2F { $($ic2f Option)* }
                    IC2PSC { $($ic2psc Option)* }
                }
                CCMR2 {
                    @Output $(
                        $ccmr2_output Option;
                        CC3S { CC3S }
                        CC4S { CC4S }
                        #[cfg(any(
                            stm32_mcu = "stm32f401",
                            stm32_mcu = "stm32f405",
                            stm32_mcu = "stm32f407",
                            stm32_mcu = "stm32f411",
                            stm32_mcu = "stm32f412",
                            stm32_mcu = "stm32f413",
                            stm32_mcu = "stm32f427",
                            stm32_mcu = "stm32f429",
                            stm32_mcu = "stm32f446",
                            stm32_mcu = "stm32f469",
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
                        OC3CE { OC3CE }
                        OC3FE { OC3FE }
                        OC3M { OC3M }
                        OC3PE { OC3PE }
                        #[cfg(any(
                            stm32_mcu = "stm32f401",
                            stm32_mcu = "stm32f405",
                            stm32_mcu = "stm32f407",
                            stm32_mcu = "stm32f411",
                            stm32_mcu = "stm32f412",
                            stm32_mcu = "stm32f413",
                            stm32_mcu = "stm32f427",
                            stm32_mcu = "stm32f429",
                            stm32_mcu = "stm32f446",
                            stm32_mcu = "stm32f469",
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
                        OC4CE { OC4CE }
                        OC4FE { OC4FE }
                        OC4M { OC4M }
                        OC4PE { OC4PE }
                    )*
                    @Input $(
                        $ccmr2_input Option;
                        CC3S { CC3S }
                        CC4S { CC4S }
                        IC3F { IC3F }
                        IC3PSC { IC3PSC }
                        IC4F { IC4F }
                        IC4PSC { IC4PSC }
                    )*
                }
                CCER {
                    CCER;
                    CC1E { CC1E }
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
                    CC1NE { $($cc1ne Option)* }
                    CC1NP { CC1NP }
                    CC1P { CC1P }
                    CC2E { $($cc2e Option)* }
                    CC2NP { $($cc2np Option)* }
                    CC2P { $($cc2p Option)* }
                    CC3E { $($cc3e Option)* }
                    CC3NP { $($cc3np Option)* }
                    CC3P { $($cc3p Option)* }
                    CC4E { $($cc4e Option)* }
                    CC4NP { $($cc4np Option)* }
                    CC4P { $($cc4p Option)* }
                }
                CNT {
                    CNT;
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
                    UIFCPY_CNT31 { $($uifcpy_cnt31 Option)* }
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
                    UIFCPY { $($uifcpy Option)* }
                    CNT { CNT }
                }
                PSC {
                    PSC;
                    PSC { PSC }
                }
                ARR {
                    ARR;
                    ARR { ARR }
                }
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
                RCR {
                    $(
                        $rcr Option;
                        REP { REP }
                    )*
                }
                CCR1 {
                    CCR1;
                    CCR1 { CCR1 }
                }
                CCR2 {
                    $(
                        $ccr2 Option;
                        CCR2 { CCR2 }
                    )*
                }
                CCR3 {
                    $(
                        $ccr3 Option;
                        CCR3 { CCR3 }
                    )*
                }
                CCR4 {
                    $(
                        $ccr4 Option;
                        CCR4 { CCR4 }
                    )*
                }
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
                BDTR {
                    $(
                        $bdtr Option;
                        AOE { AOE }
                        BKE { BKE }
                        BKP { BKP }
                        DTG { DTG }
                        LOCK { LOCK }
                        MOE { MOE }
                        OSSI { OSSI }
                        OSSR { OSSR }
                    )*
                }
                #[cfg(any(
                    stm32_mcu = "stm32f401",
                    stm32_mcu = "stm32f405",
                    stm32_mcu = "stm32f407",
                    stm32_mcu = "stm32f410",
                    stm32_mcu = "stm32f411",
                    stm32_mcu = "stm32f412",
                    stm32_mcu = "stm32f413",
                    stm32_mcu = "stm32f427",
                    stm32_mcu = "stm32f429",
                    stm32_mcu = "stm32f446",
                    stm32_mcu = "stm32f469"
                ))]
                DCR {
                    $(
                        $dcr Option;
                        DBA { DBA }
                        DBL { DBL }
                    )*
                }
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
                DCR {
                    DCR;
                    DBA { DBA }
                    DBL { DBL }
                }
                #[cfg(any(
                    stm32_mcu = "stm32f401",
                    stm32_mcu = "stm32f405",
                    stm32_mcu = "stm32f407",
                    stm32_mcu = "stm32f410",
                    stm32_mcu = "stm32f411",
                    stm32_mcu = "stm32f412",
                    stm32_mcu = "stm32f413",
                    stm32_mcu = "stm32f427",
                    stm32_mcu = "stm32f429",
                    stm32_mcu = "stm32f446",
                    stm32_mcu = "stm32f469"
                ))]
                DMAR {
                    $(
                        $dmar Option;
                        DMAB { DMAB }
                    )*
                }
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
                DMAR {
                    DMAR;
                    DMAB { DMAB }
                }
                OR1 {
                    $(
                        $or1 Option;
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
                        ENCODER_MODE { $($encoder_mode Option)* }
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
                        ETR1_RMP { $($etr1_rmp Option)* }
                        #[cfg(any(
                            stm32_mcu = "stm32f401",
                            stm32_mcu = "stm32f405",
                            stm32_mcu = "stm32f407",
                            stm32_mcu = "stm32f411",
                            stm32_mcu = "stm32f412",
                            stm32_mcu = "stm32f413",
                            stm32_mcu = "stm32f427",
                            stm32_mcu = "stm32f429",
                            stm32_mcu = "stm32f446",
                            stm32_mcu = "stm32f469",
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
                        ITR1_RMP { $($itr1_rmp Option)* }
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
                        TI1_RMP_BIT { $($ti1_rmp_bit Option)* }
                        TI1_RMP { $($ti1_rmp Option)* }
                        TI4_RMP { $($ti4_rmp Option)* }
                    )*
                }
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
                OR2 {
                    $(
                        $or2 Option;
                        BKCMP1E { $($bkcmp1e Option)* }
                        BKCMP1P { $($bkcmp1p Option)* }
                        BKCMP2E { $($bkcmp2e Option)* }
                        BKCMP2P { $($bkcmp2p Option)* }
                        BKDFBKE { $($bkdfbke Option)* }
                        BKINE { $($bkine Option)* }
                        BKINP { $($bkinp Option)* }
                        ETRSEL { $($etrsel Option)* }
                    )*
                }
            }
        }
    };
}

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
map_general_tim! {
    "Extracts TIM2 register tokens.",
    periph_tim2,
    "TIM2 peripheral variant.",
    Tim2,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    TIM2EN,
    TIM2RST,
    TIM2SMEN,
    TIM2,
    (CMS, DIR,),
    (CR2,,,, TI1S, MMS,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (,, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE,),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M0_2, OC2M3, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (UIFCPY_CNT31,),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (),
    (),
    (OR1,, ETR1_RMP, ITR1_RMP,,, TI4_RMP),
    (OR2,,,,,,,, ETRSEL),
}

#[cfg(any(
    stm32_mcu = "stm32l4x1",
    stm32_mcu = "stm32l4x2",
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_general_tim! {
    "Extracts TIM3 register tokens.",
    periph_tim3,
    "TIM3 peripheral variant.",
    Tim3,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    TIM3EN,
    TIM3RST,
    TIM3SMEN,
    TIM3,
    (CMS, DIR,),
    (CR2,,,, TI1S, MMS,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (,, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE,),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M0_2, OC2M3, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (UIFCPY_CNT31,),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (),
    (),
    (OR1,,,,, TI1_RMP,),
    (OR2,,,,,,,, ETRSEL),
}

#[cfg(any(
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_general_tim! {
    "Extracts TIM4 register tokens.",
    periph_tim4,
    "TIM4 peripheral variant.",
    Tim4,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    TIM4EN,
    TIM4RST,
    TIM4SMEN,
    TIM4,
    (CMS, DIR,),
    (CR2,,,, TI1S, MMS,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (,, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE,),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M0_2, OC2M3, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (UIFCPY_CNT31,),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (),
    (),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_general_tim! {
    "Extracts TIM5 register tokens.",
    periph_tim5,
    "TIM5 peripheral variant.",
    Tim5,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    TIM5EN,
    TIM5RST,
    TIM5SMEN,
    TIM5,
    (CMS, DIR,),
    (CR2,,,, TI1S, MMS,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (,, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE,),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M0_2, OC2M3, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (UIFCPY_CNT31,),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (),
    (),
    (),
    (),
}

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
map_general_tim! {
    "Extracts TIM15 register tokens.",
    periph_tim15,
    "TIM15 peripheral variant.",
    Tim15,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    TIM15EN,
    TIM15RST,
    TIM15SMEN,
    TIM15,
    (,,),
    (CR2, OIS2, OIS1N, OIS1, TI1S, MMS, CCUS, CCPC),
    (SMCR,,,,),
    (BIE,, CC2DE, CC2IE,,,,, COMDE, COMIE, TDE, TIE,),
    (BIF, CC2IF, CC2OF,,,,, COMIF, TIF),
    (BG,,,, COMG, TG),
    (CC2S,, OC2CE, OC2FE, OC2M0_2, OC2M3, OC2PE, IC2F, IC2PSC),
    (,),
    (CC1NE, CC2E, CC2NP, CC2P,,,,,,),
    (,UIFCPY),
    (RCR),
    (,,),
    (BDTR),
    (),
    (),
    (OR1, ENCODER_MODE,,, TI1_RMP,,),
    (OR2, BKCMP1E, BKCMP1P, BKCMP2E, BKCMP2P, BKDFBK1E, BKINE, BKINP,),
}

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
map_general_tim! {
    "Extracts TIM16 register tokens.",
    periph_tim16,
    "TIM16 peripheral variant.",
    Tim16,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    TIM16EN,
    TIM16RST,
    TIM16SMEN,
    TIM16,
    (,,),
    (CR2,, OIS1N, OIS1,,, CCUS, CCPC),
    (),
    (BIE,,,,,,,, COMDE, COMIE,,,),
    (BIF,,,,,,, COMIF,),
    (BG,,,, COMG,),
    (,,,,,,,,),
    (,),
    (CC1NE,,,,,,,,,),
    (,UIFCPY),
    (RCR),
    (,,),
    (BDTR),
    (),
    (),
    (OR1,,,,, TI1_RMP,),
    (OR2, BKCMP1E, BKCMP1P, BKCMP2E, BKCMP2P, BKDFBK1E, BKINE, BKINP,),
}

#[cfg(any(
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_general_tim! {
    "Extracts TIM17 register tokens.",
    periph_tim17,
    "TIM17 peripheral variant.",
    Tim17,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    TIM17EN,
    TIM17RST,
    TIM17SMEN,
    TIM17,
    (,,),
    (CR2,, OIS1N, OIS1,,, CCUS, CCPC),
    (),
    (BIE,,,,,,,, COMDE, COMIE,,,),
    (BIF,,,,,,, COMIF,),
    (BG,,,, COMG,),
    (,,,,,,,,),
    (,),
    (CC1NE,,,,,,,,,),
    (,UIFCPY),
    (RCR),
    (,,),
    (BDTR),
    (),
    (),
    (OR1,,,,, TI1_RMP,),
    (OR2, BKCMP1E, BKCMP1P, BKCMP2E, BKCMP2P, BKDFBK1E, BKINE, BKINP,),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM2 register tokens.",
    periph_tim2,
    "TIM2 peripheral variant.",
    Tim2,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM2EN,
    TIM2RST,
    TIM2LPEN,
    TIM2,
    (CMS, DIR, OPM),
    (CR2,,,,,,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (, CC1DE, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE, UDE),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M,, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (,),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (DCR),
    (DMAR),
    (OR,,, ITR1_RMP,,,),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM3 register tokens.",
    periph_tim3,
    "TIM3 peripheral variant.",
    Tim3,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM3EN,
    TIM3RST,
    TIM3LPEN,
    TIM3,
    (CMS, DIR, OPM),
    (CR2,,,,,,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (, CC1DE, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE, UDE),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M,, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (,),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (DCR),
    (DMAR),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM4 register tokens.",
    periph_tim4,
    "TIM4 peripheral variant.",
    Tim4,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM4EN,
    TIM4RST,
    TIM4LPEN,
    TIM4,
    (CMS, DIR, OPM),
    (CR2,,,,,,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (, CC1DE, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE, UDE),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M,, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (,),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (DCR),
    (DMAR),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM5 register tokens.",
    periph_tim5,
    "TIM5 peripheral variant.",
    Tim5,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM5EN,
    TIM5RST,
    TIM5LPEN,
    TIM5,
    (CMS, DIR, OPM),
    (CR2,,,,,,,),
    (SMCR, ETP, ECE, ETPS, ETF),
    (, CC1DE, CC2DE, CC2IE, CC3DE, CC3IE, CC4DE, CC4IE,,, TDE, TIE, UDE),
    (, CC2IF, CC2OF, CC3IF, CC3OF, CC4IF, CC4OF,, TIF),
    (, CC2G, CC3G, CC4G,, TG),
    (CC2S, OC1CE, OC2CE, OC2FE, OC2M,, OC2PE, IC2F, IC2PSC),
    (CCMR2_Input, CCMR2_Output),
    (, CC2E, CC2NP, CC2P, CC3E, CC3NP, CC3P, CC4E, CC4NP, CC4P),
    (,),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (DCR),
    (DMAR),
    (OR,,,,,, TI4_RMP),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM9 register tokens.",
    periph_tim9,
    "TIM9 peripheral variant.",
    Tim9,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    TIM9EN,
    TIM9RST,
    TIM9LPEN,
    TIM9,
    (,, OPM),
    (),
    (SMCR,,,,),
    (,,, CC2IE,,,,,,,, TIE,),
    (, CC2IF, CC2OF,,,,,, TIF),
    (, CC2G,,,, TG),
    (CC2S,,, OC2FE, OC2M,, OC2PE, IC2F, IC2PSC),
    (,),
    (, CC2E, CC2NP, CC2P,,,,,,),
    (,),
    (),
    (CCR2,,),
    (),
    (),
    (),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM10 register tokens.",
    periph_tim10,
    "TIM10 peripheral variant.",
    Tim10,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    TIM10EN,
    TIM10RST,
    TIM10LPEN,
    TIM10,
    (,,),
    (),
    (),
    (,,,,,,,,,,,,),
    (,,,,,,,,),
    (,,,,,),
    (,,,,,,,,),
    (,),
    (,,,,,,,,,),
    (,),
    (),
    (,,),
    (),
    (),
    (),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM11 register tokens.",
    periph_tim11,
    "TIM11 peripheral variant.",
    Tim11,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    TIM11EN,
    TIM11RST,
    TIM11LPEN,
    TIM11,
    (,,),
    (),
    (),
    (,,,,,,,,,,,,),
    (,,,,,,,,),
    (,,,,,),
    (,,,,,,,,),
    (,),
    (,,,,,,,,,),
    (,),
    (),
    (,,),
    (),
    (),
    (),
    (OR,,,,, TI1_RMP,),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM12 register tokens.",
    periph_tim12,
    "TIM12 peripheral variant.",
    Tim12,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM12EN,
    TIM12RST,
    TIM12LPEN,
    TIM12,
    (,, OPM),
    (),
    (SMCR,,,,),
    (,,, CC2IE,,,,,,,, TIE,),
    (, CC2IF, CC2OF,,,,,, TIF),
    (, CC2G,,,, TG),
    (CC2S,,, OC2FE, OC2M,, OC2PE, IC2F, IC2PSC),
    (,),
    (, CC2E, CC2NP, CC2P,,,,,,),
    (,),
    (),
    (CCR2,,),
    (),
    (),
    (),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM13 register tokens.",
    periph_tim13,
    "TIM13 peripheral variant.",
    Tim13,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM13EN,
    TIM13RST,
    TIM13LPEN,
    TIM13,
    (,,),
    (),
    (),
    (,,,,,,,,,,,,),
    (,,,,,,,,),
    (,,,,,),
    (,,,,,,,,),
    (,),
    (,,,,,,,,,),
    (,),
    (),
    (,,),
    (),
    (),
    (),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_general_tim! {
    "Extracts TIM14 register tokens.",
    periph_tim14,
    "TIM14 peripheral variant.",
    Tim14,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    TIM14EN,
    TIM14RST,
    TIM14LPEN,
    TIM14,
    (,,),
    (),
    (),
    (,,,,,,,,,,,,),
    (,,,,,,,,),
    (,,,,,),
    (,,,,,,,,),
    (,),
    (,,,,,,,,,),
    (,),
    (),
    (,,),
    (),
    (),
    (),
    (),
    (),
}

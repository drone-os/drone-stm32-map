//! Mappings for General Purpose I/Os
//! for STM32F1 Series of mainstream MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic GPIO port peripheral variant.
    pub trait GpioPortMap {}

    /// Generic GPIO port peripheral.
    pub struct GpioPortPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            GPIOEN { RwRwRegFieldBitBand }
        }
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            GPIORST { RwRwRegFieldBitBand }
        }
    }

    GPIO {
        BRR {
            0x20 WoReg;
            BR0 { WoWoRegFieldBit }
            BR1 { WoWoRegFieldBit }
            BR2 { WoWoRegFieldBit }
            BR3 { WoWoRegFieldBit }
            BR4 { WoWoRegFieldBit }
            BR5 { WoWoRegFieldBit }
            BR6 { WoWoRegFieldBit }
            BR7 { WoWoRegFieldBit }
            BR8 { WoWoRegFieldBit }
            BR9 { WoWoRegFieldBit }
            BR10 { WoWoRegFieldBit }
            BR11 { WoWoRegFieldBit }
            BR12 { WoWoRegFieldBit }
            BR13 { WoWoRegFieldBit }
            BR14 { WoWoRegFieldBit }
            BR15 { WoWoRegFieldBit }
        }
        BSRR {
            0x20 WoReg;
            BR0 { WoWoRegFieldBit }
            BR1 { WoWoRegFieldBit }
            BR2 { WoWoRegFieldBit }
            BR3 { WoWoRegFieldBit }
            BR4 { WoWoRegFieldBit }
            BR5 { WoWoRegFieldBit }
            BR6 { WoWoRegFieldBit }
            BR7 { WoWoRegFieldBit }
            BR8 { WoWoRegFieldBit }
            BR9 { WoWoRegFieldBit }
            BR10 { WoWoRegFieldBit }
            BR11 { WoWoRegFieldBit }
            BR12 { WoWoRegFieldBit }
            BR13 { WoWoRegFieldBit }
            BR14 { WoWoRegFieldBit }
            BR15 { WoWoRegFieldBit }
            BS0 { WoWoRegFieldBit }
            BS1 { WoWoRegFieldBit }
            BS2 { WoWoRegFieldBit }
            BS3 { WoWoRegFieldBit }
            BS4 { WoWoRegFieldBit }
            BS5 { WoWoRegFieldBit }
            BS6 { WoWoRegFieldBit }
            BS7 { WoWoRegFieldBit }
            BS8 { WoWoRegFieldBit }
            BS9 { WoWoRegFieldBit }
            BS10 { WoWoRegFieldBit }
            BS11 { WoWoRegFieldBit }
            BS12 { WoWoRegFieldBit }
            BS13 { WoWoRegFieldBit }
            BS14 { WoWoRegFieldBit }
            BS15 { WoWoRegFieldBit }
        }
        CRL {
            0x20 RwReg;
            CNF0 { RwRwRegFieldBits }
            CNF1 { RwRwRegFieldBits }
            CNF2 { RwRwRegFieldBits }
            CNF3 { RwRwRegFieldBits }
            CNF4 { RwRwRegFieldBits }
            CNF5 { RwRwRegFieldBits }
            CNF6 { RwRwRegFieldBits }
            CNF7 { RwRwRegFieldBits }
            MODE0 { RwRwRegFieldBits }
            MODE1 { RwRwRegFieldBits }
            MODE2 { RwRwRegFieldBits }
            MODE3 { RwRwRegFieldBits }
            MODE4 { RwRwRegFieldBits }
            MODE5 { RwRwRegFieldBits }
            MODE6 { RwRwRegFieldBits }
            MODE7 { RwRwRegFieldBits }
        }
        CRH {
            0x20 RwReg;
            CNF8 { RwRwRegFieldBits }
            CNF9 { RwRwRegFieldBits }
            CNF10 { RwRwRegFieldBits }
            CNF11 { RwRwRegFieldBits }
            CNF12 { RwRwRegFieldBits }
            CNF13 { RwRwRegFieldBits }
            CNF14 { RwRwRegFieldBits }
            CNF15 { RwRwRegFieldBits }
            MODE8 { RwRwRegFieldBits }
            MODE9 { RwRwRegFieldBits }
            MODE10 { RwRwRegFieldBits }
            MODE11 { RwRwRegFieldBits }
            MODE12 { RwRwRegFieldBits }
            MODE13 { RwRwRegFieldBits }
            MODE14 { RwRwRegFieldBits }
            MODE15 { RwRwRegFieldBits }
        }
        IDR {
            0x20 RoReg;
            IDR0 { RoRoRegFieldBit }
            IDR1 { RoRoRegFieldBit }
            IDR2 { RoRoRegFieldBit }
            IDR3 { RoRoRegFieldBit }
            IDR4 { RoRoRegFieldBit }
            IDR5 { RoRoRegFieldBit }
            IDR6 { RoRoRegFieldBit }
            IDR7 { RoRoRegFieldBit }
            IDR8 { RoRoRegFieldBit }
            IDR9 { RoRoRegFieldBit }
            IDR10 { RoRoRegFieldBit }
            IDR11 { RoRoRegFieldBit }
            IDR12 { RoRoRegFieldBit }
            IDR13 { RoRoRegFieldBit }
            IDR14 { RoRoRegFieldBit }
            IDR15 { RoRoRegFieldBit }
        }
        LCKR {
            0x20 RwReg;
            LCK0 { RwRwRegFieldBit }
            LCK1 { RwRwRegFieldBit }
            LCK2 { RwRwRegFieldBit }
            LCK3 { RwRwRegFieldBit }
            LCK4 { RwRwRegFieldBit }
            LCK5 { RwRwRegFieldBit }
            LCK6 { RwRwRegFieldBit }
            LCK7 { RwRwRegFieldBit }
            LCK8 { RwRwRegFieldBit }
            LCK9 { RwRwRegFieldBit }
            LCK10 { RwRwRegFieldBit }
            LCK11 { RwRwRegFieldBit }
            LCK12 { RwRwRegFieldBit }
            LCK13 { RwRwRegFieldBit }
            LCK14 { RwRwRegFieldBit }
            LCK15 { RwRwRegFieldBit }
            LCKK { RwRwRegFieldBit }
        }
        ODR {
            0x20 RwReg;
            ODR0 { RwRwRegFieldBit }
            ODR1 { RwRwRegFieldBit }
            ODR2 { RwRwRegFieldBit }
            ODR3 { RwRwRegFieldBit }
            ODR4 { RwRwRegFieldBit }
            ODR5 { RwRwRegFieldBit }
            ODR6 { RwRwRegFieldBit }
            ODR7 { RwRwRegFieldBit }
            ODR8 { RwRwRegFieldBit }
            ODR9 { RwRwRegFieldBit }
            ODR10 { RwRwRegFieldBit }
            ODR11 { RwRwRegFieldBit }
            ODR12 { RwRwRegFieldBit }
            ODR13 { RwRwRegFieldBit }
            ODR14 { RwRwRegFieldBit }
            ODR15 { RwRwRegFieldBit }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_gpio_port {
    (
        $port_macro_doc:expr,
        $port_macro:ident,
        $port_ty_doc:expr,
        $port_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $gpio:ident,
        $gpioen:ident,
        $gpiorst:ident,
        $gpiosmen:ident,($($ascr:ident)*),
    ) => {
        periph::map! {
            #[doc = $port_macro_doc]
            pub macro $port_macro;

            #[doc = $port_ty_doc]
            pub struct $port_ty;

            impl GpioPortMap for $port_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    GPIOEN { $gpioen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    GPIORST { $gpiorst }
                }
            }

            GPIO {
                $gpio;
                BRR {
                    BRR;
                    BR0 { BR0 }
                    BR1 { BR1 }
                    BR2 { BR2 }
                    BR3 { BR3 }
                    BR4 { BR4 }
                    BR5 { BR5 }
                    BR6 { BR6 }
                    BR7 { BR7 }
                    BR8 { BR8 }
                    BR9 { BR9 }
                    BR10 { BR10 }
                    BR11 { BR11 }
                    BR12 { BR12 }
                    BR13 { BR13 }
                    BR14 { BR14 }
                    BR15 { BR15 }
                }
                BSRR {
                    BSRR;
                    BR0 { BR0 }
                    BR1 { BR1 }
                    BR2 { BR2 }
                    BR3 { BR3 }
                    BR4 { BR4 }
                    BR5 { BR5 }
                    BR6 { BR6 }
                    BR7 { BR7 }
                    BR8 { BR8 }
                    BR9 { BR9 }
                    BR10 { BR10 }
                    BR11 { BR11 }
                    BR12 { BR12 }
                    BR13 { BR13 }
                    BR14 { BR14 }
                    BR15 { BR15 }
                    BS0 { BS0 }
                    BS1 { BS1 }
                    BS2 { BS2 }
                    BS3 { BS3 }
                    BS4 { BS4 }
                    BS5 { BS5 }
                    BS6 { BS6 }
                    BS7 { BS7 }
                    BS8 { BS8 }
                    BS9 { BS9 }
                    BS10 { BS10 }
                    BS11 { BS11 }
                    BS12 { BS12 }
                    BS13 { BS13 }
                    BS14 { BS14 }
                    BS15 { BS15 }
                }
                CRL {
                    CRL;
                    CNF0 { CNF0 }
                    CNF1 { CNF1 }
                    CNF2 { CNF2 }
                    CNF3 { CNF3 }
                    CNF4 { CNF4 }
                    CNF5 { CNF5 }
                    CNF6 { CNF6 }
                    CNF7 { CNF7 }
                    MODE0 { MODE0 }
                    MODE1 { MODE1 }
                    MODE2 { MODE2 }
                    MODE3 { MODE3 }
                    MODE4 { MODE4 }
                    MODE5 { MODE5 }
                    MODE6 { MODE6 }
                    MODE7 { MODE7 }
                }
                CRH {
                    CRH;
                    CNF8 { CNF8 }
                    CNF9 { CNF9 }
                    CNF10 { CNF10 }
                    CNF11 { CNF11 }
                    CNF12 { CNF12 }
                    CNF13 { CNF13 }
                    CNF14 { CNF14 }
                    CNF15 { CNF15 }
                    MODE8 { MODE8 }
                    MODE9 { MODE9 }
                    MODE10 { MODE10 }
                    MODE11 { MODE11 }
                    MODE12 { MODE12 }
                    MODE13 { MODE13 }
                    MODE14 { MODE14 }
                    MODE15 { MODE15 }
                }
                IDR {
                    IDR;
                    IDR0 { IDR0 }
                    IDR1 { IDR1 }
                    IDR2 { IDR2 }
                    IDR3 { IDR3 }
                    IDR4 { IDR4 }
                    IDR5 { IDR5 }
                    IDR6 { IDR6 }
                    IDR7 { IDR7 }
                    IDR8 { IDR8 }
                    IDR9 { IDR9 }
                    IDR10 { IDR10 }
                    IDR11 { IDR11 }
                    IDR12 { IDR12 }
                    IDR13 { IDR13 }
                    IDR14 { IDR14 }
                    IDR15 { IDR15 }
                }
                LCKR {
                    LCKR;
                    LCK0 { LCK0 }
                    LCK1 { LCK1 }
                    LCK2 { LCK2 }
                    LCK3 { LCK3 }
                    LCK4 { LCK4 }
                    LCK5 { LCK5 }
                    LCK6 { LCK6 }
                    LCK7 { LCK7 }
                    LCK8 { LCK8 }
                    LCK9 { LCK9 }
                    LCK10 { LCK10 }
                    LCK11 { LCK11 }
                    LCK12 { LCK12 }
                    LCK13 { LCK13 }
                    LCK14 { LCK14 }
                    LCK15 { LCK15 }
                    LCKK { LCKK }
                }
                ODR {
                    ODR;
                    ODR0 { ODR0 }
                    ODR1 { ODR1 }
                    ODR2 { ODR2 }
                    ODR3 { ODR3 }
                    ODR4 { ODR4 }
                    ODR5 { ODR5 }
                    ODR6 { ODR6 }
                    ODR7 { ODR7 }
                    ODR8 { ODR8 }
                    ODR9 { ODR9 }
                    ODR10 { ODR10 }
                    ODR11 { ODR11 }
                    ODR12 { ODR12 }
                    ODR13 { ODR13 }
                    ODR14 { ODR14 }
                    ODR15 { ODR15 }
                }
            }
        }
    };
}

map_gpio_port! {
    "Extracts GPIO port A register tokens.",
    periph_gpio_a,
    "GPIO port A peripheral variant.",
    GpioA,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOA,
    IOPAEN,
    IOPARST,
    IOPASMEN,
    (),
}

map_gpio_port! {
    "Extracts GPIO port B register tokens.",
    periph_gpio_b,
    "GPIO port B peripheral variant.",
    GpioB,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOB,
    IOPBEN,
    IOPBRST,
    IOPBSMEN,
    (),
}

map_gpio_port! {
    "Extracts GPIO port C register tokens.",
    periph_gpio_c,
    "GPIO port C peripheral variant.",
    GpioC,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOC,
    IOPCEN,
    IOPCRST,
    IOPCSMEN,
    (),
}

map_gpio_port! {
    "Extracts GPIO port D register tokens.",
    periph_gpio_d,
    "GPIO port D peripheral variant.",
    GpioD,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOD,
    IOPDEN,
    IOPDRST,
    IOPDSMEN,
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
))]
map_gpio_port! {
    "Extracts GPIO port E register tokens.",
    periph_gpio_e,
    "GPIO port E peripheral variant.",
    GpioE,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOE,
    IOPEEN,
    IOPERST,
    IOPESMEN,
    (),
}

#[cfg(any(stm32_mcu = "stm32f100", stm32_mcu = "stm32f101", stm32_mcu = "stm32f103"))]
map_gpio_port! {
    "Extracts GPIO port F register tokens.",
    periph_gpio_f,
    "GPIO port F peripheral variant.",
    GpioF,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOF,
    IOPFEN,
    IOPFRST,
    IOPFSMEN,
    (),
}

#[cfg(any(stm32_mcu = "stm32f100", stm32_mcu = "stm32f101", stm32_mcu = "stm32f103"))]
map_gpio_port! {
    "Extracts GPIO port G register tokens.",
    periph_gpio_g,
    "GPIO port G peripheral variant.",
    GpioG,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOG,
    IOPGEN,
    IOPGRST,
    IOPGSMEN,
    (),
}

//! General-purpose I/O port heads.
//!
//! For STM32F1 Series of mainstream MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic GPIO port head peripheral variant.
    pub trait GpioHeadMap {}

    /// Generic GPIO port head peripheral.
    pub struct GpioHeadPeriph;

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
        LCKR {
            0x20 RwReg Shared;
            LCKK { RwRwRegFieldBit }
        }
    }
}

macro_rules! map_gpio_port_head {
    (
        $port_macro_doc:expr,
        $port_macro:ident,
        $port_ty_doc:expr,
        $port_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $gpio:ident,
        $gpioen:ident,
        $gpiorst:ident,
    ) => {
        periph::map! {
            #[doc = $port_macro_doc]
            pub macro $port_macro;

            #[doc = $port_ty_doc]
            pub struct $port_ty;

            impl GpioHeadMap for $port_ty {}

            drone_stm32_map_pieces::reg;
            crate::head;

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
                LCKR {
                    LCKR Shared;
                    LCKK { LCKK }
                }
            }
        }
    };
}

#[cfg(any(
    drone_stm32_map = "stm32f100",
    drone_stm32_map = "stm32f101",
    drone_stm32_map = "stm32f102",
    drone_stm32_map = "stm32f103",
    drone_stm32_map = "stm32f107",
))]
map_gpio_port_head! {
    "Extracts GPIO port A head register tokens.",
    periph_gpio_a_head,
    "GPIO port A head peripheral variant.",
    GpioAHead,
    APB2ENR,
    APB2RSTR,
    GPIOA,
    IOPAEN,
    IOPARST,
}

#[cfg(any(
    drone_stm32_map = "stm32f100",
    drone_stm32_map = "stm32f101",
    drone_stm32_map = "stm32f102",
    drone_stm32_map = "stm32f103",
    drone_stm32_map = "stm32f107",
))]
map_gpio_port_head! {
    "Extracts GPIO port B head register tokens.",
    periph_gpio_b_head,
    "GPIO port B head peripheral variant.",
    GpioBHead,
    APB2ENR,
    APB2RSTR,
    GPIOB,
    IOPBEN,
    IOPBRST,
}

#[cfg(any(
    drone_stm32_map = "stm32f100",
    drone_stm32_map = "stm32f101",
    drone_stm32_map = "stm32f102",
    drone_stm32_map = "stm32f103",
    drone_stm32_map = "stm32f107",
))]
map_gpio_port_head! {
    "Extracts GPIO port C head register tokens.",
    periph_gpio_c_head,
    "GPIO port C head peripheral variant.",
    GpioCHead,
    APB2ENR,
    APB2RSTR,
    GPIOC,
    IOPCEN,
    IOPCRST,
}

#[cfg(any(
    drone_stm32_map = "stm32f100",
    drone_stm32_map = "stm32f101",
    drone_stm32_map = "stm32f102",
    drone_stm32_map = "stm32f103",
    drone_stm32_map = "stm32f107",
))]
map_gpio_port_head! {
    "Extracts GPIO port D head register tokens.",
    periph_gpio_d_head,
    "GPIO port D head peripheral variant.",
    GpioDHead,
    APB2ENR,
    APB2RSTR,
    GPIOD,
    IOPDEN,
    IOPDRST,
}

#[cfg(any(
    drone_stm32_map = "stm32f100",
    drone_stm32_map = "stm32f101",
    drone_stm32_map = "stm32f103",
    drone_stm32_map = "stm32f107",
))]
map_gpio_port_head! {
    "Extracts GPIO port E head register tokens.",
    periph_gpio_e_head,
    "GPIO port E head peripheral variant.",
    GpioEHead,
    APB2ENR,
    APB2RSTR,
    GPIOE,
    IOPEEN,
    IOPERST,
}

#[cfg(any(
    drone_stm32_map = "stm32f100",
    drone_stm32_map = "stm32f101",
    drone_stm32_map = "stm32f103"
))]
map_gpio_port_head! {
    "Extracts GPIO port F head register tokens.",
    periph_gpio_f_head,
    "GPIO port F head peripheral variant.",
    GpioFHead,
    APB2ENR,
    APB2RSTR,
    GPIOF,
    IOPFEN,
    IOPFRST,
}

#[cfg(any(
    drone_stm32_map = "stm32f100",
    drone_stm32_map = "stm32f101",
    drone_stm32_map = "stm32f103"
))]
map_gpio_port_head! {
    "Extracts GPIO port G head register tokens.",
    periph_gpio_g_head,
    "GPIO port G head peripheral variant.",
    GpioGHead,
    APB2ENR,
    APB2RSTR,
    GPIOG,
    IOPGEN,
    IOPGRST,
}

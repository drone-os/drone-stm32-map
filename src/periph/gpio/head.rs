//! General-purpose I/O port heads.

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

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
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            GPIOSMEN { RwRwRegFieldBitBand }
        }
    }

    GPIO {
        LCKR {
            0x20 RwReg Shared;
            LCKK { RwRwRegFieldBit }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_gpio_port_head {
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
        $gpiosmen:ident,
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
                BUSSMENR {
                    $bussmenr Shared;
                    GPIOSMEN { $gpiosmen }
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
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_gpio_port_head! {
    "Extracts GPIO port A head register tokens.",
    periph_gpio_a_head,
    "GPIO port A head peripheral variant.",
    GpioAHead,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOA,
    IOPAEN,
    IOPARST,
    IOPASMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_gpio_port_head! {
    "Extracts GPIO port B head register tokens.",
    periph_gpio_b_head,
    "GPIO port B head peripheral variant.",
    GpioBHead,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOB,
    IOPBEN,
    IOPBRST,
    IOPBSMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_gpio_port_head! {
    "Extracts GPIO port C head register tokens.",
    periph_gpio_c_head,
    "GPIO port C head peripheral variant.",
    GpioCHead,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOC,
    IOPCEN,
    IOPCRST,
    IOPCSMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_gpio_port_head! {
    "Extracts GPIO port D head register tokens.",
    periph_gpio_d_head,
    "GPIO port D head peripheral variant.",
    GpioDHead,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOD,
    IOPDEN,
    IOPDRST,
    IOPDSMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_gpio_port_head! {
    "Extracts GPIO port E head register tokens.",
    periph_gpio_e_head,
    "GPIO port E head peripheral variant.",
    GpioEHead,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOE,
    IOPEEN,
    IOPERST,
    IOPESMEN,
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101", feature = "stm32f103"))]
map_gpio_port_head! {
    "Extracts GPIO port F head register tokens.",
    periph_gpio_f_head,
    "GPIO port F head peripheral variant.",
    GpioFHead,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOF,
    IOPFEN,
    IOPFRST,
    IOPFSMEN,
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101", feature = "stm32f103"))]
map_gpio_port_head! {
    "Extracts GPIO port G head register tokens.",
    periph_gpio_g_head,
    "GPIO port G head peripheral variant.",
    GpioGHead,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    GPIOG,
    IOPGEN,
    IOPGRST,
    IOPGSMEN,
}

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
map_gpio_port_head! {
    "Extracts GPIO port A head register tokens.",
    periph_gpio_a_head,
    "GPIO port A head peripheral variant.",
    GpioAHead,
    AHB2ENR,
    AHB2RSTR,
    AHB2SMENR,
    GPIOA,
    GPIOAEN,
    GPIOARST,
    GPIOASMEN,
}

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
map_gpio_port_head! {
    "Extracts GPIO port B head register tokens.",
    periph_gpio_b_head,
    "GPIO port B head peripheral variant.",
    GpioBHead,
    AHB2ENR,
    AHB2RSTR,
    AHB2SMENR,
    GPIOB,
    GPIOBEN,
    GPIOBRST,
    GPIOBSMEN,
}

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
map_gpio_port_head! {
    "Extracts GPIO port C head register tokens.",
    periph_gpio_c_head,
    "GPIO port C head peripheral variant.",
    GpioCHead,
    AHB2ENR,
    AHB2RSTR,
    AHB2SMENR,
    GPIOC,
    GPIOCEN,
    GPIOCRST,
    GPIOCSMEN,
}

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
map_gpio_port_head! {
    "Extracts GPIO port D head register tokens.",
    periph_gpio_d_head,
    "GPIO port D head peripheral variant.",
    GpioDHead,
    AHB2ENR,
    AHB2RSTR,
    AHB2SMENR,
    GPIOD,
    GPIODEN,
    GPIODRST,
    GPIODSMEN,
}

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
map_gpio_port_head! {
    "Extracts GPIO port E head register tokens.",
    periph_gpio_e_head,
    "GPIO port E head peripheral variant.",
    GpioEHead,
    AHB2ENR,
    AHB2RSTR,
    AHB2SMENR,
    GPIOE,
    GPIOEEN,
    GPIOERST,
    GPIOESMEN,
}

#[cfg(any(
    feature = "stm32l4x5",
    feature = "stm32l4x6",
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
))]
map_gpio_port_head! {
    "Extracts GPIO port F head register tokens.",
    periph_gpio_f_head,
    "GPIO port F head peripheral variant.",
    GpioFHead,
    AHB2ENR,
    AHB2RSTR,
    AHB2SMENR,
    GPIOF,
    GPIOFEN,
    GPIOFRST,
    GPIOFSMEN,
}

#[cfg(any(
    feature = "stm32l4x5",
    feature = "stm32l4x6",
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
))]
map_gpio_port_head! {
    "Extracts GPIO port G head register tokens.",
    periph_gpio_g_head,
    "GPIO port G head peripheral variant.",
    GpioGHead,
    AHB2ENR,
    AHB2RSTR,
    AHB2SMENR,
    GPIOG,
    GPIOGEN,
    GPIOGRST,
    GPIOGSMEN,
}

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
map_gpio_port_head! {
    "Extracts GPIO port H head register tokens.",
    periph_gpio_h_head,
    "GPIO port H head peripheral variant.",
    GpioHHead,
    AHB2ENR,
    AHB2RSTR,
    AHB2SMENR,
    GPIOH,
    GPIOHEN,
    GPIOHRST,
    GPIOHSMEN,
}

#[cfg(any(
    feature = "stm32l4x6",
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
))]
map_gpio_port_head! {
    "Extracts GPIO port I head register tokens.",
    periph_gpio_i_head,
    "GPIO port I head peripheral variant.",
    GpioIHead,
    AHB2ENR,
    AHB2RSTR,
    AHB2SMENR,
    GPIOI,
    GPIOIEN,
    GPIOIRST,
    GPIOISMEN,
}

//! General-purpose I/O port heads.

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
    /// GPIO port head.
    pub trait GpioHeadMap {}

    RCC {
        AHB2ENR {
            0x20 RwRegBitBand Shared;
            GPIOEN { RwRwRegFieldBitBand }
        }
        AHB2RSTR {
            0x20 RwRegBitBand Shared;
            GPIORST { RwRwRegFieldBitBand }
        }
        AHB2SMENR {
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

            ::drone_stm32_map_pieces::reg; head;

            RCC {
                AHB2ENR {
                    AHB2ENR Shared;
                    GPIOEN { $gpioen }
                }
                AHB2RSTR {
                    AHB2RSTR Shared;
                    GPIORST { $gpiorst }
                }
                AHB2SMENR {
                    AHB2SMENR Shared;
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
    "Acquires GPIO port A head.",
    periph_gpio_a_head,
    "GPIO port A head.",
    GpioAHead,
    GPIOA,
    GPIOAEN,
    GPIOARST,
    GPIOASMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
    "Acquires GPIO port B head.",
    periph_gpio_b_head,
    "GPIO port B head.",
    GpioBHead,
    GPIOB,
    GPIOBEN,
    GPIOBRST,
    GPIOBSMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
    "Acquires GPIO port C head.",
    periph_gpio_c_head,
    "GPIO port C head.",
    GpioCHead,
    GPIOC,
    GPIOCEN,
    GPIOCRST,
    GPIOCSMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
    "Acquires GPIO port D head.",
    periph_gpio_d_head,
    "GPIO port D head.",
    GpioDHead,
    GPIOD,
    GPIODEN,
    GPIODRST,
    GPIODSMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
    "Acquires GPIO port E head.",
    periph_gpio_e_head,
    "GPIO port E head.",
    GpioEHead,
    GPIOE,
    GPIOEEN,
    GPIOERST,
    GPIOESMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
    "Acquires GPIO port F head.",
    periph_gpio_f_head,
    "GPIO port F head.",
    GpioFHead,
    GPIOF,
    GPIOFEN,
    GPIOFRST,
    GPIOFSMEN,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
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
    "Acquires GPIO port G head.",
    periph_gpio_g_head,
    "GPIO port G head.",
    GpioGHead,
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
    "Acquires GPIO port H head.",
    periph_gpio_h_head,
    "GPIO port H head.",
    GpioHHead,
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
    "Acquires GPIO port I head.",
    periph_gpio_i_head,
    "GPIO port I head.",
    GpioIHead,
    GPIOI,
    GPIOIEN,
    GPIOIRST,
    GPIOISMEN,
}

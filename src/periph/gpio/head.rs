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
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f102",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f103",
    stm32_mcu = "stm32f107",
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

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f103"
))]
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

#[cfg(any(
    stm32_mcu = "stm32f100",
    stm32_mcu = "stm32f101",
    stm32_mcu = "stm32f103"
))]
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
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
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
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
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
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
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
map_gpio_port_head! {
    "Extracts GPIO port A head register tokens.",
    periph_gpio_a_head,
    "GPIO port A head peripheral variant.",
    GpioAHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOA,
    GPIOAEN,
    GPIOARST,
    GPIOALPEN,
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
map_gpio_port_head! {
    "Extracts GPIO port B head register tokens.",
    periph_gpio_b_head,
    "GPIO port B head peripheral variant.",
    GpioBHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOB,
    GPIOBEN,
    GPIOBRST,
    GPIOBLPEN,
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
map_gpio_port_head! {
    "Extracts GPIO port C head register tokens.",
    periph_gpio_c_head,
    "GPIO port C head peripheral variant.",
    GpioCHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOC,
    GPIOCEN,
    GPIOCRST,
    GPIOCLPEN,
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
map_gpio_port_head! {
    "Extracts GPIO port D head register tokens.",
    periph_gpio_d_head,
    "GPIO port D head peripheral variant.",
    GpioDHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOD,
    GPIODEN,
    GPIODRST,
    GPIODLPEN,
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
map_gpio_port_head! {
    "Extracts GPIO port E head register tokens.",
    periph_gpio_e_head,
    "GPIO port E head peripheral variant.",
    GpioEHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOE,
    GPIOEEN,
    GPIOERST,
    GPIOELPEN,
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
map_gpio_port_head! {
    "Extracts GPIO port F head register tokens.",
    periph_gpio_f_head,
    "GPIO port F head peripheral variant.",
    GpioFHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOF,
    GPIOFEN,
    GPIOFRST,
    GPIOFLPEN,
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
map_gpio_port_head! {
    "Extracts GPIO port G head register tokens.",
    periph_gpio_g_head,
    "GPIO port G head peripheral variant.",
    GpioGHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOG,
    GPIOGEN,
    GPIOGRST,
    GPIOGLPEN,
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
map_gpio_port_head! {
    "Extracts GPIO port H head register tokens.",
    periph_gpio_h_head,
    "GPIO port H head peripheral variant.",
    GpioHHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOH,
    GPIOHEN,
    GPIOHRST,
    GPIOHLPEN,
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
map_gpio_port_head! {
    "Extracts GPIO port I head register tokens.",
    periph_gpio_i_head,
    "GPIO port I head peripheral variant.",
    GpioIHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOI,
    GPIOIEN,
    GPIOIRST,
    GPIOILPEN,
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
map_gpio_port_head! {
    "Extracts GPIO port J head register tokens.",
    periph_gpio_j_head,
    "GPIO port J head peripheral variant.",
    GpioJHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOJ,
    GPIOJEN,
    GPIOJRST,
    GPIOJLPEN,
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
map_gpio_port_head! {
    "Extracts GPIO port K head register tokens.",
    periph_gpio_k_head,
    "GPIO port K head peripheral variant.",
    GpioKHead,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    GPIOK,
    GPIOKEN,
    GPIOKRST,
    GPIOKLPEN,
}

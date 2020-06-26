//! General-purpose I/O port heads.
//! for STM32F3 Series of mixed-signal MCUs with DSP and FPU instructions.

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

#[allow(unused_macros)]
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
map_gpio_port_head! {
    "Extracts GPIO port A head register tokens.",
    periph_gpio_a_head,
    "GPIO port A head peripheral variant.",
    GpioAHead,
    AHBENR,
    AHBRSTR,
    GPIOA,
    IOPAEN,
    IOPARST,
}

map_gpio_port_head! {
    "Extracts GPIO port B head register tokens.",
    periph_gpio_b_head,
    "GPIO port B head peripheral variant.",
    GpioBHead,
    AHBENR,
    AHBRSTR,
    GPIOB,
    IOPAEN,
    IOPARST,
}

map_gpio_port_head! {
    "Extracts GPIO port C head register tokens.",
    periph_gpio_c_head,
    "GPIO port C head peripheral variant.",
    GpioCHead,
    AHBENR,
    AHBRSTR,
    GPIOC,
    IOPAEN,
    IOPARST,
}

map_gpio_port_head! {
    "Extracts GPIO port D head register tokens.",
    periph_gpio_d_head,
    "GPIO port D head peripheral variant.",
    GpioDHead,
    AHBENR,
    AHBRSTR,
    GPIOD,
    IOPAEN,
    IOPARST,
}

map_gpio_port_head! {
    "Extracts GPIO port E head register tokens.",
    periph_gpio_e_head,
    "GPIO port E head peripheral variant.",
    GpioEHead,
    AHBENR,
    AHBRSTR,
    GPIOE,
    IOPAEN,
    IOPARST,
}

map_gpio_port_head! {
    "Extracts GPIO port F head register tokens.",
    periph_gpio_f_head,
    "GPIO port F head peripheral variant.",
    GpioFHead,
    AHBENR,
    AHBRSTR,
    GPIOF,
    IOPAEN,
    IOPARST,
}

map_gpio_port_head! {
    "Extracts GPIO port G head register tokens.",
    periph_gpio_g_head,
    "GPIO port G head peripheral variant.",
    GpioGHead,
    AHBENR,
    AHBRSTR,
    GPIOG,
    IOPAEN,
    IOPARST,
}

map_gpio_port_head! {
    "Extracts GPIO port H head register tokens.",
    periph_gpio_h_head,
    "GPIO port H head peripheral variant.",
    GpioHHead,
    AHBENR,
    AHBRSTR,
    GPIOH,
    IOPAEN,
    IOPARST,
}

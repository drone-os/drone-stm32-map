//! General-purpose I/O ports without pins.

use drone_core::res;
use drone_cortex_m::reg::marker::*;

res! {
  /// GPIO port without pins.
  pub trait GpioPortPinless {}

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
macro_rules! map_gpio_port_pinless {
  (
    $port_doc:expr,
    $port_ty:ident,
    $gpio:ident,
    $gpioen:ident,
    $gpiorst:ident,
    $gpiosmen:ident,
  ) => {
    res::map! {
      #[doc = $port_doc]
      pub struct $port_ty;

      impl GpioPortPinless for $port_ty {}

      ::drone_stm32_map_pieces::reg; pinless;

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
map_gpio_port_pinless! {
  "GPIO port A without pins.",
  GpioAPinless,
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
map_gpio_port_pinless! {
  "GPIO port B without pins.",
  GpioBPinless,
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
map_gpio_port_pinless! {
  "GPIO port C without pins.",
  GpioCPinless,
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
map_gpio_port_pinless! {
  "GPIO port D without pins.",
  GpioDPinless,
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
map_gpio_port_pinless! {
  "GPIO port E without pins.",
  GpioEPinless,
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
map_gpio_port_pinless! {
  "GPIO port F without pins.",
  GpioFPinless,
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
map_gpio_port_pinless! {
  "GPIO port G without pins.",
  GpioGPinless,
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
map_gpio_port_pinless! {
  "GPIO port H without pins.",
  GpioHPinless,
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
map_gpio_port_pinless! {
  "GPIO port I without pins.",
  GpioIPinless,
  GPIOI,
  GPIOIEN,
  GPIOIRST,
  GPIOISMEN,
}

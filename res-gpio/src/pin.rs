//! General-purpose I/O pins.

use drone_core::res;
use drone_cortex_m::reg::marker::*;

res! {
  /// GPIO pin.
  pub trait GpioPin {
    /// GPIO port without pins.
    type GpioPortPinless: super::pinless::GpioPortPinless;
  }

  GPIO {
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
    AFR {
      0x20 RwReg Shared;
      AFR { RwRwRegFieldBits }
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
    ASCR {
      0x20 RwReg Shared;
      ASC { RwRwRegFieldBit Option }
    }
    #[cfg(any(
      feature = "stm32f100",
      feature = "stm32f101",
      feature = "stm32f102",
      feature = "stm32f103",
      feature = "stm32f107",
      feature = "stm32l4x6",
      feature = "stm32l4r5",
      feature = "stm32l4r7",
      feature = "stm32l4r9",
      feature = "stm32l4s5",
      feature = "stm32l4s7",
      feature = "stm32l4s9"
    ))]
    BRR {
      0x20 WoReg Shared;
      BR { WoWoRegFieldBit }
    }
    BSRR {
      0x20 WoReg Shared;
      BR { WoWoRegFieldBit }
      BS { WoWoRegFieldBit }
    }
    #[cfg(any(
      feature = "stm32f100",
      feature = "stm32f101",
      feature = "stm32f102",
      feature = "stm32f103",
      feature = "stm32f107"
    ))]
    CR {
      0x20 RwReg Shared;
      CNF { RwRwRegFieldBits }
      MODE { RwRwRegFieldBits }
    }
    IDR {
      0x20 RoReg Shared;
      IDR { RoRoRegFieldBit }
    }
    LCKR {
      0x20 RwReg Shared;
      LCK { RwRwRegFieldBit }
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
    MODER {
      0x20 RwReg Shared;
      MODER { RwRwRegFieldBits }
    }
    ODR {
      0x20 RwReg Shared;
      ODR { RwRwRegFieldBit }
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
    OSPEEDR {
      0x20 RwReg Shared;
      OSPEEDR { RwRwRegFieldBits }
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
    OTYPER {
      0x20 RwReg Shared;
      OT { RwRwRegFieldBit }
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
    PUPDR {
      0x20 RwReg Shared;
      PUPDR { RwRwRegFieldBits }
    }
  }
}

#[allow(unused_macros)]
macro_rules! map_gpio_pin {
  (
    $port_ty:ident,
    $pin_doc:expr,
    $pin_ty:ident,
    $gpio:ident,
    $afr_ty:ident,
    $asc_ty:ident,
    $br_ty:ident,
    $bs_ty:ident,
    $cnf_ty:ident,
    $mode_ty:ident,
    $idr_ty:ident,
    $lck_ty:ident,
    $moder_ty:ident,
    $odr_ty:ident,
    $ospeedr_ty:ident,
    $ot_ty:ident,
    $pupdr_ty:ident,
    $afr_path:ident,
    $cr_path:ident,
    ($($ascr_option:ident)*),
  ) => {
    res::map! {
      #[doc = $pin_doc]
      pub struct $pin_ty;

      impl GpioPin for $pin_ty {
        type GpioPortPinless = super::pinless::$port_ty;
      }

      ::drone_stm32_map_pieces::reg; pin;

      GPIO {
        $gpio;
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
        AFR {
          $afr_path Shared;
          AFR { $afr_ty }
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
        ASCR {
          ASCR Shared;
          ASC { $($asc_ty $ascr_option)* }
        }
        #[cfg(any(
          feature = "stm32f100",
          feature = "stm32f101",
          feature = "stm32f102",
          feature = "stm32f103",
          feature = "stm32f107",
          feature = "stm32l4x6",
          feature = "stm32l4r5",
          feature = "stm32l4r7",
          feature = "stm32l4r9",
          feature = "stm32l4s5",
          feature = "stm32l4s7",
          feature = "stm32l4s9"
        ))]
        BRR {
          BRR Shared;
          BR { $br_ty }
        }
        BSRR {
          BSRR Shared;
          BR { $br_ty }
          BS { $bs_ty }
        }
        #[cfg(any(
          feature = "stm32f100",
          feature = "stm32f101",
          feature = "stm32f102",
          feature = "stm32f103",
          feature = "stm32f107"
        ))]
        CR {
          $cr_path Shared;
          CNF { $cnf_ty }
          MODE { $mode_ty }
        }
        IDR {
          IDR Shared;
          IDR { $idr_ty }
        }
        LCKR {
          LCKR Shared;
          LCK { $lck_ty }
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
        MODER {
          MODER Shared;
          MODER { $moder_ty }
        }
        ODR {
          ODR Shared;
          ODR { $odr_ty }
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
        OSPEEDR {
          OSPEEDR Shared;
          OSPEEDR { $ospeedr_ty }
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
        OTYPER {
          OTYPER Shared;
          OT { $ot_ty }
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
        PUPDR {
          PUPDR Shared;
          PUPDR { $pupdr_ty }
        }
      }
    }
  };
}

#[allow(unused_macros)]
macro_rules! map_gpio_pins {
  (
    $port_ty:ident,
    $pin0_doc:expr,
    $pin0_ty:ident,
    $pin1_doc:expr,
    $pin1_ty:ident,
    $pin2_doc:expr,
    $pin2_ty:ident,
    $pin3_doc:expr,
    $pin3_ty:ident,
    $pin4_doc:expr,
    $pin4_ty:ident,
    $pin5_doc:expr,
    $pin5_ty:ident,
    $pin6_doc:expr,
    $pin6_ty:ident,
    $pin7_doc:expr,
    $pin7_ty:ident,
    $pin8_doc:expr,
    $pin8_ty:ident,
    $pin9_doc:expr,
    $pin9_ty:ident,
    $pin10_doc:expr,
    $pin10_ty:ident,
    $pin11_doc:expr,
    $pin11_ty:ident,
    $pin12_doc:expr,
    $pin12_ty:ident,
    $pin13_doc:expr,
    $pin13_ty:ident,
    $pin14_doc:expr,
    $pin14_ty:ident,
    $pin15_doc:expr,
    $pin15_ty:ident,
    $gpio:ident,
    $gpioen:ident,
    ($($ascr_option:ident)*),
  ) => {
    map_gpio_pin! {
      $port_ty,
      $pin0_doc,
      $pin0_ty,
      $gpio,
      AFRL0,
      ASC0,
      BR0,
      BS0,
      CNF0,
      MODE0,
      IDR0,
      LCK0,
      MODER0,
      ODR0,
      OSPEEDR0,
      OT0,
      PUPDR0,
      AFRL,
      CRL,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin1_doc,
      $pin1_ty,
      $gpio,
      AFRL1,
      ASC1,
      BR1,
      BS1,
      CNF1,
      MODE1,
      IDR1,
      LCK1,
      MODER1,
      ODR1,
      OSPEEDR1,
      OT1,
      PUPDR1,
      AFRL,
      CRL,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin2_doc,
      $pin2_ty,
      $gpio,
      AFRL2,
      ASC2,
      BR2,
      BS2,
      CNF2,
      MODE2,
      IDR2,
      LCK2,
      MODER2,
      ODR2,
      OSPEEDR2,
      OT2,
      PUPDR2,
      AFRL,
      CRL,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin3_doc,
      $pin3_ty,
      $gpio,
      AFRL3,
      ASC3,
      BR3,
      BS3,
      CNF3,
      MODE3,
      IDR3,
      LCK3,
      MODER3,
      ODR3,
      OSPEEDR3,
      OT3,
      PUPDR3,
      AFRL,
      CRL,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin4_doc,
      $pin4_ty,
      $gpio,
      AFRL4,
      ASC4,
      BR4,
      BS4,
      CNF4,
      MODE4,
      IDR4,
      LCK4,
      MODER4,
      ODR4,
      OSPEEDR4,
      OT4,
      PUPDR4,
      AFRL,
      CRL,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin5_doc,
      $pin5_ty,
      $gpio,
      AFRL5,
      ASC5,
      BR5,
      BS5,
      CNF5,
      MODE5,
      IDR5,
      LCK5,
      MODER5,
      ODR5,
      OSPEEDR5,
      OT5,
      PUPDR5,
      AFRL,
      CRL,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin6_doc,
      $pin6_ty,
      $gpio,
      AFRL6,
      ASC6,
      BR6,
      BS6,
      CNF6,
      MODE6,
      IDR6,
      LCK6,
      MODER6,
      ODR6,
      OSPEEDR6,
      OT6,
      PUPDR6,
      AFRL,
      CRL,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin7_doc,
      $pin7_ty,
      $gpio,
      AFRL7,
      ASC7,
      BR7,
      BS7,
      CNF7,
      MODE7,
      IDR7,
      LCK7,
      MODER7,
      ODR7,
      OSPEEDR7,
      OT7,
      PUPDR7,
      AFRL,
      CRL,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin8_doc,
      $pin8_ty,
      $gpio,
      AFRH8,
      ASC8,
      BR8,
      BS8,
      CNF8,
      MODE8,
      IDR8,
      LCK8,
      MODER8,
      ODR8,
      OSPEEDR8,
      OT8,
      PUPDR8,
      AFRH,
      CRH,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin9_doc,
      $pin9_ty,
      $gpio,
      AFRH9,
      ASC9,
      BR9,
      BS9,
      CNF9,
      MODE9,
      IDR9,
      LCK9,
      MODER9,
      ODR9,
      OSPEEDR9,
      OT9,
      PUPDR9,
      AFRH,
      CRH,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin10_doc,
      $pin10_ty,
      $gpio,
      AFRH10,
      ASC10,
      BR10,
      BS10,
      CNF10,
      MODE10,
      IDR10,
      LCK10,
      MODER10,
      ODR10,
      OSPEEDR10,
      OT10,
      PUPDR10,
      AFRH,
      CRH,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin11_doc,
      $pin11_ty,
      $gpio,
      AFRH11,
      ASC11,
      BR11,
      BS11,
      CNF11,
      MODE11,
      IDR11,
      LCK11,
      MODER11,
      ODR11,
      OSPEEDR11,
      OT11,
      PUPDR11,
      AFRH,
      CRH,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin12_doc,
      $pin12_ty,
      $gpio,
      AFRH12,
      ASC12,
      BR12,
      BS12,
      CNF12,
      MODE12,
      IDR12,
      LCK12,
      MODER12,
      ODR12,
      OSPEEDR12,
      OT12,
      PUPDR12,
      AFRH,
      CRH,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin13_doc,
      $pin13_ty,
      $gpio,
      AFRH13,
      ASC13,
      BR13,
      BS13,
      CNF13,
      MODE13,
      IDR13,
      LCK13,
      MODER13,
      ODR13,
      OSPEEDR13,
      OT13,
      PUPDR13,
      AFRH,
      CRH,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin14_doc,
      $pin14_ty,
      $gpio,
      AFRH14,
      ASC14,
      BR14,
      BS14,
      CNF14,
      MODE14,
      IDR14,
      LCK14,
      MODER14,
      ODR14,
      OSPEEDR14,
      OT14,
      PUPDR14,
      AFRH,
      CRH,
      ($($ascr_option)*),
    }
    map_gpio_pin! {
      $port_ty,
      $pin15_doc,
      $pin15_ty,
      $gpio,
      AFRH15,
      ASC15,
      BR15,
      BS15,
      CNF15,
      MODE15,
      IDR15,
      LCK15,
      MODER15,
      ODR15,
      OSPEEDR15,
      OT15,
      PUPDR15,
      AFRH,
      CRH,
      ($($ascr_option)*),
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
map_gpio_pins! {
  GpioAPinless,
  "GPIO port A pin 0.",
  GpioA0,
  "GPIO port A pin 1.",
  GpioA1,
  "GPIO port A pin 2.",
  GpioA2,
  "GPIO port A pin 3.",
  GpioA3,
  "GPIO port A pin 4.",
  GpioA4,
  "GPIO port A pin 5.",
  GpioA5,
  "GPIO port A pin 6.",
  GpioA6,
  "GPIO port A pin 7.",
  GpioA7,
  "GPIO port A pin 8.",
  GpioA8,
  "GPIO port A pin 9.",
  GpioA9,
  "GPIO port A pin 10.",
  GpioA10,
  "GPIO port A pin 11.",
  GpioA11,
  "GPIO port A pin 12.",
  GpioA12,
  "GPIO port A pin 13.",
  GpioA13,
  "GPIO port A pin 14.",
  GpioA14,
  "GPIO port A pin 15.",
  GpioA15,
  GPIOA,
  GPIOAEN,
  (Option),
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
map_gpio_pins! {
  GpioBPinless,
  "GPIO port B pin 0.",
  GpioB0,
  "GPIO port B pin 1.",
  GpioB1,
  "GPIO port B pin 2.",
  GpioB2,
  "GPIO port B pin 3.",
  GpioB3,
  "GPIO port B pin 4.",
  GpioB4,
  "GPIO port B pin 5.",
  GpioB5,
  "GPIO port B pin 6.",
  GpioB6,
  "GPIO port B pin 7.",
  GpioB7,
  "GPIO port B pin 8.",
  GpioB8,
  "GPIO port B pin 9.",
  GpioB9,
  "GPIO port B pin 10.",
  GpioB10,
  "GPIO port B pin 11.",
  GpioB11,
  "GPIO port B pin 12.",
  GpioB12,
  "GPIO port B pin 13.",
  GpioB13,
  "GPIO port B pin 14.",
  GpioB14,
  "GPIO port B pin 15.",
  GpioB15,
  GPIOB,
  GPIOBEN,
  (Option),
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
map_gpio_pins! {
  GpioCPinless,
  "GPIO port C pin 0.",
  GpioC0,
  "GPIO port C pin 1.",
  GpioC1,
  "GPIO port C pin 2.",
  GpioC2,
  "GPIO port C pin 3.",
  GpioC3,
  "GPIO port C pin 4.",
  GpioC4,
  "GPIO port C pin 5.",
  GpioC5,
  "GPIO port C pin 6.",
  GpioC6,
  "GPIO port C pin 7.",
  GpioC7,
  "GPIO port C pin 8.",
  GpioC8,
  "GPIO port C pin 9.",
  GpioC9,
  "GPIO port C pin 10.",
  GpioC10,
  "GPIO port C pin 11.",
  GpioC11,
  "GPIO port C pin 12.",
  GpioC12,
  "GPIO port C pin 13.",
  GpioC13,
  "GPIO port C pin 14.",
  GpioC14,
  "GPIO port C pin 15.",
  GpioC15,
  GPIOC,
  GPIOCEN,
  (Option),
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
map_gpio_pins! {
  GpioDPinless,
  "GPIO port D pin 0.",
  GpioD0,
  "GPIO port D pin 1.",
  GpioD1,
  "GPIO port D pin 2.",
  GpioD2,
  "GPIO port D pin 3.",
  GpioD3,
  "GPIO port D pin 4.",
  GpioD4,
  "GPIO port D pin 5.",
  GpioD5,
  "GPIO port D pin 6.",
  GpioD6,
  "GPIO port D pin 7.",
  GpioD7,
  "GPIO port D pin 8.",
  GpioD8,
  "GPIO port D pin 9.",
  GpioD9,
  "GPIO port D pin 10.",
  GpioD10,
  "GPIO port D pin 11.",
  GpioD11,
  "GPIO port D pin 12.",
  GpioD12,
  "GPIO port D pin 13.",
  GpioD13,
  "GPIO port D pin 14.",
  GpioD14,
  "GPIO port D pin 15.",
  GpioD15,
  GPIOD,
  GPIODEN,
  (Option),
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
map_gpio_pins! {
  GpioEPinless,
  "GPIO port E pin 0.",
  GpioE0,
  "GPIO port E pin 1.",
  GpioE1,
  "GPIO port E pin 2.",
  GpioE2,
  "GPIO port E pin 3.",
  GpioE3,
  "GPIO port E pin 4.",
  GpioE4,
  "GPIO port E pin 5.",
  GpioE5,
  "GPIO port E pin 6.",
  GpioE6,
  "GPIO port E pin 7.",
  GpioE7,
  "GPIO port E pin 8.",
  GpioE8,
  "GPIO port E pin 9.",
  GpioE9,
  "GPIO port E pin 10.",
  GpioE10,
  "GPIO port E pin 11.",
  GpioE11,
  "GPIO port E pin 12.",
  GpioE12,
  "GPIO port E pin 13.",
  GpioE13,
  "GPIO port E pin 14.",
  GpioE14,
  "GPIO port E pin 15.",
  GpioE15,
  GPIOE,
  GPIOEEN,
  (Option),
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
map_gpio_pins! {
  GpioFPinless,
  "GPIO port F pin 0.",
  GpioF0,
  "GPIO port F pin 1.",
  GpioF1,
  "GPIO port F pin 2.",
  GpioF2,
  "GPIO port F pin 3.",
  GpioF3,
  "GPIO port F pin 4.",
  GpioF4,
  "GPIO port F pin 5.",
  GpioF5,
  "GPIO port F pin 6.",
  GpioF6,
  "GPIO port F pin 7.",
  GpioF7,
  "GPIO port F pin 8.",
  GpioF8,
  "GPIO port F pin 9.",
  GpioF9,
  "GPIO port F pin 10.",
  GpioF10,
  "GPIO port F pin 11.",
  GpioF11,
  "GPIO port F pin 12.",
  GpioF12,
  "GPIO port F pin 13.",
  GpioF13,
  "GPIO port F pin 14.",
  GpioF14,
  "GPIO port F pin 15.",
  GpioF15,
  GPIOF,
  GPIOFEN,
  (Option),
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
map_gpio_pins! {
  GpioGPinless,
  "GPIO port G pin 0.",
  GpioG0,
  "GPIO port G pin 1.",
  GpioG1,
  "GPIO port G pin 2.",
  GpioG2,
  "GPIO port G pin 3.",
  GpioG3,
  "GPIO port G pin 4.",
  GpioG4,
  "GPIO port G pin 5.",
  GpioG5,
  "GPIO port G pin 6.",
  GpioG6,
  "GPIO port G pin 7.",
  GpioG7,
  "GPIO port G pin 8.",
  GpioG8,
  "GPIO port G pin 9.",
  GpioG9,
  "GPIO port G pin 10.",
  GpioG10,
  "GPIO port G pin 11.",
  GpioG11,
  "GPIO port G pin 12.",
  GpioG12,
  "GPIO port G pin 13.",
  GpioG13,
  "GPIO port G pin 14.",
  GpioG14,
  "GPIO port G pin 15.",
  GpioG15,
  GPIOG,
  GPIOGEN,
  (Option),
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
map_gpio_pins! {
  GpioHPinless,
  "GPIO port H pin 0.",
  GpioH0,
  "GPIO port H pin 1.",
  GpioH1,
  "GPIO port H pin 2.",
  GpioH2,
  "GPIO port H pin 3.",
  GpioH3,
  "GPIO port H pin 4.",
  GpioH4,
  "GPIO port H pin 5.",
  GpioH5,
  "GPIO port H pin 6.",
  GpioH6,
  "GPIO port H pin 7.",
  GpioH7,
  "GPIO port H pin 8.",
  GpioH8,
  "GPIO port H pin 9.",
  GpioH9,
  "GPIO port H pin 10.",
  GpioH10,
  "GPIO port H pin 11.",
  GpioH11,
  "GPIO port H pin 12.",
  GpioH12,
  "GPIO port H pin 13.",
  GpioH13,
  "GPIO port H pin 14.",
  GpioH14,
  "GPIO port H pin 15.",
  GpioH15,
  GPIOH,
  GPIOHEN,
  (Option),
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
map_gpio_pins! {
  GpioIPinless,
  "GPIO port I pin 0.",
  GpioI0,
  "GPIO port I pin 1.",
  GpioI1,
  "GPIO port I pin 2.",
  GpioI2,
  "GPIO port I pin 3.",
  GpioI3,
  "GPIO port I pin 4.",
  GpioI4,
  "GPIO port I pin 5.",
  GpioI5,
  "GPIO port I pin 6.",
  GpioI6,
  "GPIO port I pin 7.",
  GpioI7,
  "GPIO port I pin 8.",
  GpioI8,
  "GPIO port I pin 9.",
  GpioI9,
  "GPIO port I pin 10.",
  GpioI10,
  "GPIO port I pin 11.",
  GpioI11,
  "GPIO port I pin 12.",
  GpioI12,
  "GPIO port I pin 13.",
  GpioI13,
  "GPIO port I pin 14.",
  GpioI14,
  "GPIO port I pin 15.",
  GpioI15,
  GPIOI,
  GPIOIEN,
  (),
}

//! General Purpose I/Os.

#![feature(proc_macro_hygiene)]
#![no_std]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

pub mod head;
pub mod pin;

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
  /// GPIO port.
  pub trait GpioPortMap {}

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
    AFRL {
      0x20 RwReg;
      AFRL0 { RwRwRegFieldBits }
      AFRL1 { RwRwRegFieldBits }
      AFRL2 { RwRwRegFieldBits }
      AFRL3 { RwRwRegFieldBits }
      AFRL4 { RwRwRegFieldBits }
      AFRL5 { RwRwRegFieldBits }
      AFRL6 { RwRwRegFieldBits }
      AFRL7 { RwRwRegFieldBits }
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
    AFRH {
      0x20 RwReg;
      AFRH8 { RwRwRegFieldBits }
      AFRH9 { RwRwRegFieldBits }
      AFRH10 { RwRwRegFieldBits }
      AFRH11 { RwRwRegFieldBits }
      AFRH12 { RwRwRegFieldBits }
      AFRH13 { RwRwRegFieldBits }
      AFRH14 { RwRwRegFieldBits }
      AFRH15 { RwRwRegFieldBits }
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
      0x20 RwReg Option;
      ASC0 { RwRwRegFieldBit }
      ASC1 { RwRwRegFieldBit }
      ASC2 { RwRwRegFieldBit }
      ASC3 { RwRwRegFieldBit }
      ASC4 { RwRwRegFieldBit }
      ASC5 { RwRwRegFieldBit }
      ASC6 { RwRwRegFieldBit }
      ASC7 { RwRwRegFieldBit }
      ASC8 { RwRwRegFieldBit }
      ASC9 { RwRwRegFieldBit }
      ASC10 { RwRwRegFieldBit }
      ASC11 { RwRwRegFieldBit }
      ASC12 { RwRwRegFieldBit }
      ASC13 { RwRwRegFieldBit }
      ASC14 { RwRwRegFieldBit }
      ASC15 { RwRwRegFieldBit }
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
    #[cfg(any(
      feature = "stm32f100",
      feature = "stm32f101",
      feature = "stm32f102",
      feature = "stm32f103",
      feature = "stm32f107"
    ))]
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
    #[cfg(any(
      feature = "stm32f100",
      feature = "stm32f101",
      feature = "stm32f102",
      feature = "stm32f103",
      feature = "stm32f107"
    ))]
    CRH {
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
      0x20 RwReg;
      MODER0 { RwRwRegFieldBits }
      MODER1 { RwRwRegFieldBits }
      MODER2 { RwRwRegFieldBits }
      MODER3 { RwRwRegFieldBits }
      MODER4 { RwRwRegFieldBits }
      MODER5 { RwRwRegFieldBits }
      MODER6 { RwRwRegFieldBits }
      MODER7 { RwRwRegFieldBits }
      MODER8 { RwRwRegFieldBits }
      MODER9 { RwRwRegFieldBits }
      MODER10 { RwRwRegFieldBits }
      MODER11 { RwRwRegFieldBits }
      MODER12 { RwRwRegFieldBits }
      MODER13 { RwRwRegFieldBits }
      MODER14 { RwRwRegFieldBits }
      MODER15 { RwRwRegFieldBits }
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
      0x20 RwReg;
      OSPEEDR0 { RwRwRegFieldBits }
      OSPEEDR1 { RwRwRegFieldBits }
      OSPEEDR2 { RwRwRegFieldBits }
      OSPEEDR3 { RwRwRegFieldBits }
      OSPEEDR4 { RwRwRegFieldBits }
      OSPEEDR5 { RwRwRegFieldBits }
      OSPEEDR6 { RwRwRegFieldBits }
      OSPEEDR7 { RwRwRegFieldBits }
      OSPEEDR8 { RwRwRegFieldBits }
      OSPEEDR9 { RwRwRegFieldBits }
      OSPEEDR10 { RwRwRegFieldBits }
      OSPEEDR11 { RwRwRegFieldBits }
      OSPEEDR12 { RwRwRegFieldBits }
      OSPEEDR13 { RwRwRegFieldBits }
      OSPEEDR14 { RwRwRegFieldBits }
      OSPEEDR15 { RwRwRegFieldBits }
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
      0x20 RwReg;
      OT0 { RwRwRegFieldBit }
      OT1 { RwRwRegFieldBit }
      OT2 { RwRwRegFieldBit }
      OT3 { RwRwRegFieldBit }
      OT4 { RwRwRegFieldBit }
      OT5 { RwRwRegFieldBit }
      OT6 { RwRwRegFieldBit }
      OT7 { RwRwRegFieldBit }
      OT8 { RwRwRegFieldBit }
      OT9 { RwRwRegFieldBit }
      OT10 { RwRwRegFieldBit }
      OT11 { RwRwRegFieldBit }
      OT12 { RwRwRegFieldBit }
      OT13 { RwRwRegFieldBit }
      OT14 { RwRwRegFieldBit }
      OT15 { RwRwRegFieldBit }
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
      0x20 RwReg;
      PUPDR0 { RwRwRegFieldBits }
      PUPDR1 { RwRwRegFieldBits }
      PUPDR2 { RwRwRegFieldBits }
      PUPDR3 { RwRwRegFieldBits }
      PUPDR4 { RwRwRegFieldBits }
      PUPDR5 { RwRwRegFieldBits }
      PUPDR6 { RwRwRegFieldBits }
      PUPDR7 { RwRwRegFieldBits }
      PUPDR8 { RwRwRegFieldBits }
      PUPDR9 { RwRwRegFieldBits }
      PUPDR10 { RwRwRegFieldBits }
      PUPDR11 { RwRwRegFieldBits }
      PUPDR12 { RwRwRegFieldBits }
      PUPDR13 { RwRwRegFieldBits }
      PUPDR14 { RwRwRegFieldBits }
      PUPDR15 { RwRwRegFieldBits }
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
    $gpio:ident,
    $gpioen:ident,
    $gpiorst:ident,
    $gpiosmen:ident,
    ($($ascr:ident)*),
  ) => {
    periph::map! {
      #[doc = $port_macro_doc]
      pub macro $port_macro;

      #[doc = $port_ty_doc]
      pub struct $port_ty;

      impl GpioPortMap for $port_ty {}

      ::drone_stm32_map_pieces::reg;;

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
        AFRL {
          AFRL;
          AFRL0 { AFRL0 }
          AFRL1 { AFRL1 }
          AFRL2 { AFRL2 }
          AFRL3 { AFRL3 }
          AFRL4 { AFRL4 }
          AFRL5 { AFRL5 }
          AFRL6 { AFRL6 }
          AFRL7 { AFRL7 }
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
        AFRH {
          AFRH;
          AFRH8 { AFRH8 }
          AFRH9 { AFRH9 }
          AFRH10 { AFRH10 }
          AFRH11 { AFRH11 }
          AFRH12 { AFRH12 }
          AFRH13 { AFRH13 }
          AFRH14 { AFRH14 }
          AFRH15 { AFRH15 }
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
          $(
            $ascr Option;
            ASC0 { ASC0 }
            ASC1 { ASC1 }
            ASC2 { ASC2 }
            ASC3 { ASC3 }
            ASC4 { ASC4 }
            ASC5 { ASC5 }
            ASC6 { ASC6 }
            ASC7 { ASC7 }
            ASC8 { ASC8 }
            ASC9 { ASC9 }
            ASC10 { ASC10 }
            ASC11 { ASC11 }
            ASC12 { ASC12 }
            ASC13 { ASC13 }
            ASC14 { ASC14 }
            ASC15 { ASC15 }
          )*
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
        #[cfg(any(
          feature = "stm32f100",
          feature = "stm32f101",
          feature = "stm32f102",
          feature = "stm32f103",
          feature = "stm32f107"
        ))]
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
        #[cfg(any(
          feature = "stm32f100",
          feature = "stm32f101",
          feature = "stm32f102",
          feature = "stm32f103",
          feature = "stm32f107"
        ))]
        CRH {
          CRH;
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
          MODER;
          MODER0 { MODER0 }
          MODER1 { MODER1 }
          MODER2 { MODER2 }
          MODER3 { MODER3 }
          MODER4 { MODER4 }
          MODER5 { MODER5 }
          MODER6 { MODER6 }
          MODER7 { MODER7 }
          MODER8 { MODER8 }
          MODER9 { MODER9 }
          MODER10 { MODER10 }
          MODER11 { MODER11 }
          MODER12 { MODER12 }
          MODER13 { MODER13 }
          MODER14 { MODER14 }
          MODER15 { MODER15 }
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
          OSPEEDR;
          OSPEEDR0 { OSPEEDR0 }
          OSPEEDR1 { OSPEEDR1 }
          OSPEEDR2 { OSPEEDR2 }
          OSPEEDR3 { OSPEEDR3 }
          OSPEEDR4 { OSPEEDR4 }
          OSPEEDR5 { OSPEEDR5 }
          OSPEEDR6 { OSPEEDR6 }
          OSPEEDR7 { OSPEEDR7 }
          OSPEEDR8 { OSPEEDR8 }
          OSPEEDR9 { OSPEEDR9 }
          OSPEEDR10 { OSPEEDR10 }
          OSPEEDR11 { OSPEEDR11 }
          OSPEEDR12 { OSPEEDR12 }
          OSPEEDR13 { OSPEEDR13 }
          OSPEEDR14 { OSPEEDR14 }
          OSPEEDR15 { OSPEEDR15 }
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
          OTYPER;
          OT0 { OT0 }
          OT1 { OT1 }
          OT2 { OT2 }
          OT3 { OT3 }
          OT4 { OT4 }
          OT5 { OT5 }
          OT6 { OT6 }
          OT7 { OT7 }
          OT8 { OT8 }
          OT9 { OT9 }
          OT10 { OT10 }
          OT11 { OT11 }
          OT12 { OT12 }
          OT13 { OT13 }
          OT14 { OT14 }
          OT15 { OT15 }
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
          PUPDR;
          PUPDR0 { PUPDR0 }
          PUPDR1 { PUPDR1 }
          PUPDR2 { PUPDR2 }
          PUPDR3 { PUPDR3 }
          PUPDR4 { PUPDR4 }
          PUPDR5 { PUPDR5 }
          PUPDR6 { PUPDR6 }
          PUPDR7 { PUPDR7 }
          PUPDR8 { PUPDR8 }
          PUPDR9 { PUPDR9 }
          PUPDR10 { PUPDR10 }
          PUPDR11 { PUPDR11 }
          PUPDR12 { PUPDR12 }
          PUPDR13 { PUPDR13 }
          PUPDR14 { PUPDR14 }
          PUPDR15 { PUPDR15 }
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
map_gpio_port! {
  "Acquires GPIO port A.",
  periph_gpio_a,
  "GPIO port A.",
  GpioA,
  GPIOA,
  GPIOAEN,
  GPIOARST,
  GPIOASMEN,
  (ASCR),
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
map_gpio_port! {
  "Acquires GPIO port B.",
  periph_gpio_b,
  "GPIO port B.",
  GpioB,
  GPIOB,
  GPIOBEN,
  GPIOBRST,
  GPIOBSMEN,
  (ASCR),
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
map_gpio_port! {
  "Acquires GPIO port C.",
  periph_gpio_c,
  "GPIO port C.",
  GpioC,
  GPIOC,
  GPIOCEN,
  GPIOCRST,
  GPIOCSMEN,
  (ASCR),
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
map_gpio_port! {
  "Acquires GPIO port D.",
  periph_gpio_d,
  "GPIO port D.",
  GpioD,
  GPIOD,
  GPIODEN,
  GPIODRST,
  GPIODSMEN,
  (ASCR),
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
map_gpio_port! {
  "Acquires GPIO port E.",
  periph_gpio_e,
  "GPIO port E.",
  GpioE,
  GPIOE,
  GPIOEEN,
  GPIOERST,
  GPIOESMEN,
  (ASCR),
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
map_gpio_port! {
  "Acquires GPIO port F.",
  periph_gpio_f,
  "GPIO port F.",
  GpioF,
  GPIOF,
  GPIOFEN,
  GPIOFRST,
  GPIOFSMEN,
  (ASCR),
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
map_gpio_port! {
  "Acquires GPIO port G.",
  periph_gpio_g,
  "GPIO port G.",
  GpioG,
  GPIOG,
  GPIOGEN,
  GPIOGRST,
  GPIOGSMEN,
  (ASCR),
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
map_gpio_port! {
  "Acquires GPIO port H.",
  periph_gpio_h,
  "GPIO port H.",
  GpioH,
  GPIOH,
  GPIOHEN,
  GPIOHRST,
  GPIOHSMEN,
  (ASCR),
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
map_gpio_port! {
  "Acquires GPIO port I.",
  periph_gpio_i,
  "GPIO port I.",
  GpioI,
  GPIOI,
  GPIOIEN,
  GPIOIRST,
  GPIOISMEN,
  (),
}

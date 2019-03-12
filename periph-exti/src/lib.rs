//! Extended interrupts and events controller.

#![feature(proc_macro_hygiene)]
#![no_std]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
  /// EXTI.
  pub trait ExtiMap {}

  SYSCFG {
    EXTICR {
      0x20 RwRegBitBand Shared;
      EXTI { RwRwRegFieldBits Option }
    }
  }
  EXTI {
    IMR {
      0x20 RwRegBitBand Shared;
      IM { RwRwRegFieldBitBand }
    }
    EMR {
      0x20 RwRegBitBand Shared;
      EM { RwRwRegFieldBitBand }
    }
    RTSR {
      0x20 RwRegBitBand Shared;
      RT { RwRwRegFieldBitBand Option }
    }
    FTSR {
      0x20 RwRegBitBand Shared;
      FT { RwRwRegFieldBitBand Option }
    }
    SWIER {
      0x20 RwRegBitBand Shared;
      SWI { RwRwRegFieldBitBand Option }
    }
    PR {
      0x20 RwRegBitBand Shared;
      PIF { RwRwRegFieldBitBand Option }
    }
  }
}

#[allow(unused_macros)]
macro_rules! map_exti {
  (
    $exti_macro_doc:expr,
    $exti_macro:ident,
    $exti_ty_doc:expr,
    $exti_ty:ident,
    $exticr:ident,
    $imr:ident,
    $emr:ident,
    $rtsr:ident,
    $ftsr:ident,
    $swier:ident,
    $pr:ident,
    $im:ident,
    $em:ident,
    ($($exti:ident)?),
    ($($rt:ident)?),
    ($($ft:ident)?),
    ($($swi:ident)?),
    ($($pif:ident)?),
  ) => {
    periph::map! {
      #[doc = $exti_macro_doc]
      pub macro $exti_macro;

      #[doc = $exti_ty_doc]
      pub struct $exti_ty;

      impl ExtiMap for $exti_ty {}

      ::drone_stm32_map_pieces::reg;;

      SYSCFG {
        EXTICR {
          $exticr Shared;
          EXTI { $($exti Option)* }
        }
      }
      EXTI {
        IMR {
          $imr Shared;
          IM { $im }
        }
        EMR {
          $emr Shared;
          EM { $em }
        }
        RTSR {
          $rtsr Shared;
          RT { $($rt Option)* }
        }
        FTSR {
          $ftsr Shared;
          FT { $($ft Option)* }
        }
        SWIER {
          $swier Shared;
          SWI { $($swi Option)* }
        }
        PR {
          $pr Shared;
          PIF { $($pif Option)* }
        }
      }
    }
  };
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
map_exti! {
  "Acquires EXTI Line 0.",
  periph_exti0,
  "EXTI Line 0.",
  Exti0,
  EXTICR1,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR0,
  MR0,
  (EXTI0),
  (TR0),
  (TR0),
  (SWIER0),
  (PR0),
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
map_exti! {
  "Acquires EXTI Line 1.",
  periph_exti1,
  "EXTI Line 1.",
  Exti1,
  EXTICR1,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR1,
  MR1,
  (EXTI1),
  (TR1),
  (TR1),
  (SWIER1),
  (PR1),
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
map_exti! {
  "Acquires EXTI Line 2.",
  periph_exti2,
  "EXTI Line 2.",
  Exti2,
  EXTICR1,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR2,
  MR2,
  (EXTI2),
  (TR2),
  (TR2),
  (SWIER2),
  (PR2),
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
map_exti! {
  "Acquires EXTI Line 3.",
  periph_exti3,
  "EXTI Line 3.",
  Exti3,
  EXTICR1,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR3,
  MR3,
  (EXTI3),
  (TR3),
  (TR3),
  (SWIER3),
  (PR3),
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
map_exti! {
  "Acquires EXTI Line 4.",
  periph_exti4,
  "EXTI Line 4.",
  Exti4,
  EXTICR2,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR4,
  MR4,
  (EXTI4),
  (TR4),
  (TR4),
  (SWIER4),
  (PR4),
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
map_exti! {
  "Acquires EXTI Line 5.",
  periph_exti5,
  "EXTI Line 5.",
  Exti5,
  EXTICR2,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR5,
  MR5,
  (EXTI5),
  (TR5),
  (TR5),
  (SWIER5),
  (PR5),
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
map_exti! {
  "Acquires EXTI Line 6.",
  periph_exti6,
  "EXTI Line 6.",
  Exti6,
  EXTICR2,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR6,
  MR6,
  (EXTI6),
  (TR6),
  (TR6),
  (SWIER6),
  (PR6),
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
map_exti! {
  "Acquires EXTI Line 7.",
  periph_exti7,
  "EXTI Line 7.",
  Exti7,
  EXTICR2,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR7,
  MR7,
  (EXTI7),
  (TR7),
  (TR7),
  (SWIER7),
  (PR7),
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
map_exti! {
  "Acquires EXTI Line 8.",
  periph_exti8,
  "EXTI Line 8.",
  Exti8,
  EXTICR3,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR8,
  MR8,
  (EXTI8),
  (TR8),
  (TR8),
  (SWIER8),
  (PR8),
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
map_exti! {
  "Acquires EXTI Line 9.",
  periph_exti9,
  "EXTI Line 9.",
  Exti9,
  EXTICR3,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR9,
  MR9,
  (EXTI9),
  (TR9),
  (TR9),
  (SWIER9),
  (PR9),
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
map_exti! {
  "Acquires EXTI Line 10.",
  periph_exti10,
  "EXTI Line 10.",
  Exti10,
  EXTICR3,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR10,
  MR10,
  (EXTI10),
  (TR10),
  (TR10),
  (SWIER10),
  (PR10),
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
map_exti! {
  "Acquires EXTI Line 11.",
  periph_exti11,
  "EXTI Line 11.",
  Exti11,
  EXTICR3,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR11,
  MR11,
  (EXTI11),
  (TR11),
  (TR11),
  (SWIER11),
  (PR11),
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
map_exti! {
  "Acquires EXTI Line 12.",
  periph_exti12,
  "EXTI Line 12.",
  Exti12,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR12,
  MR12,
  (EXTI12),
  (TR12),
  (TR12),
  (SWIER12),
  (PR12),
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
map_exti! {
  "Acquires EXTI Line 13.",
  periph_exti13,
  "EXTI Line 13.",
  Exti13,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR13,
  MR13,
  (EXTI13),
  (TR13),
  (TR13),
  (SWIER13),
  (PR13),
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
map_exti! {
  "Acquires EXTI Line 14.",
  periph_exti14,
  "EXTI Line 14.",
  Exti14,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR14,
  MR14,
  (EXTI14),
  (TR14),
  (TR14),
  (SWIER14),
  (PR14),
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
map_exti! {
  "Acquires EXTI Line 15.",
  periph_exti15,
  "EXTI Line 15.",
  Exti15,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR15,
  MR15,
  (EXTI15),
  (TR15),
  (TR15),
  (SWIER15),
  (PR15),
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
map_exti! {
  "Acquires EXTI Line 16.",
  periph_exti16,
  "EXTI Line 16.",
  Exti16,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR16,
  MR16,
  (),
  (TR16),
  (TR16),
  (SWIER16),
  (PR16),
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
map_exti! {
  "Acquires EXTI Line 17.",
  periph_exti17,
  "EXTI Line 17.",
  Exti17,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR17,
  MR17,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 18.",
  periph_exti18,
  "EXTI Line 18.",
  Exti18,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR18,
  MR18,
  (),
  (TR18),
  (TR18),
  (SWIER18),
  (PR18),
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
map_exti! {
  "Acquires EXTI Line 19.",
  periph_exti19,
  "EXTI Line 19.",
  Exti19,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR19,
  MR19,
  (),
  (TR19),
  (TR19),
  (SWIER19),
  (PR19),
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
map_exti! {
  "Acquires EXTI Line 20.",
  periph_exti20,
  "EXTI Line 20.",
  Exti20,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR20,
  MR20,
  (),
  (TR20),
  (TR20),
  (SWIER20),
  (PR20),
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
map_exti! {
  "Acquires EXTI Line 21.",
  periph_exti21,
  "EXTI Line 21.",
  Exti21,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR21,
  MR21,
  (),
  (TR21),
  (TR21),
  (SWIER21),
  (PR21),
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
map_exti! {
  "Acquires EXTI Line 22.",
  periph_exti22,
  "EXTI Line 22.",
  Exti22,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR22,
  MR22,
  (),
  (TR22),
  (TR22),
  (SWIER22),
  (PR22),
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
map_exti! {
  "Acquires EXTI Line 23.",
  periph_exti23,
  "EXTI Line 23.",
  Exti23,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR23,
  MR23,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 24.",
  periph_exti24,
  "EXTI Line 24.",
  Exti24,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR24,
  MR24,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 25.",
  periph_exti25,
  "EXTI Line 25.",
  Exti25,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR25,
  MR25,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 26.",
  periph_exti26,
  "EXTI Line 26.",
  Exti26,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR26,
  MR26,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 27.",
  periph_exti27,
  "EXTI Line 27.",
  Exti27,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR27,
  MR27,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 28.",
  periph_exti28,
  "EXTI Line 28.",
  Exti28,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR28,
  MR28,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 29.",
  periph_exti29,
  "EXTI Line 29.",
  Exti29,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR29,
  MR29,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 30.",
  periph_exti30,
  "EXTI Line 30.",
  Exti30,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR30,
  MR30,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 31.",
  periph_exti31,
  "EXTI Line 31.",
  Exti31,
  EXTICR4,
  IMR1,
  EMR1,
  RTSR1,
  FTSR1,
  SWIER1,
  PR1,
  MR31,
  MR31,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 32.",
  periph_exti32,
  "EXTI Line 32.",
  Exti32,
  EXTICR2,
  IMR2,
  EMR2,
  RTSR2,
  FTSR2,
  SWIER2,
  PR2,
  MR32,
  MR32,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 33.",
  periph_exti33,
  "EXTI Line 33.",
  Exti33,
  EXTICR2,
  IMR2,
  EMR2,
  RTSR2,
  FTSR2,
  SWIER2,
  PR2,
  MR33,
  MR33,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 34.",
  periph_exti34,
  "EXTI Line 34.",
  Exti34,
  EXTICR2,
  IMR2,
  EMR2,
  RTSR2,
  FTSR2,
  SWIER2,
  PR2,
  MR34,
  MR34,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 35.",
  periph_exti35,
  "EXTI Line 35.",
  Exti35,
  EXTICR2,
  IMR2,
  EMR2,
  RTSR2,
  FTSR2,
  SWIER2,
  PR2,
  MR35,
  MR35,
  (),
  (RT35),
  (FT35),
  (SWI35),
  (PIF35),
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
map_exti! {
  "Acquires EXTI Line 36.",
  periph_exti36,
  "EXTI Line 36.",
  Exti36,
  EXTICR2,
  IMR2,
  EMR2,
  RTSR2,
  FTSR2,
  SWIER2,
  PR2,
  MR36,
  MR36,
  (),
  (RT36),
  (FT36),
  (SWI36),
  (PIF36),
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
map_exti! {
  "Acquires EXTI Line 37.",
  periph_exti37,
  "EXTI Line 37.",
  Exti37,
  EXTICR2,
  IMR2,
  EMR2,
  RTSR2,
  FTSR2,
  SWIER2,
  PR2,
  MR37,
  MR37,
  (),
  (RT37),
  (FT37),
  (SWI37),
  (PIF37),
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
map_exti! {
  "Acquires EXTI Line 38.",
  periph_exti38,
  "EXTI Line 38.",
  Exti38,
  EXTICR2,
  IMR2,
  EMR2,
  RTSR2,
  FTSR2,
  SWIER2,
  PR2,
  MR38,
  MR38,
  (),
  (RT38),
  (FT38),
  (SWI38),
  (PIF38),
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
map_exti! {
  "Acquires EXTI Line 39.",
  periph_exti39,
  "EXTI Line 39.",
  Exti39,
  EXTICR2,
  IMR2,
  EMR2,
  RTSR2,
  FTSR2,
  SWIER2,
  PR2,
  MR39,
  MR39,
  (),
  (),
  (),
  (),
  (),
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
map_exti! {
  "Acquires EXTI Line 40.",
  periph_exti40,
  "EXTI Line 40.",
  Exti40,
  EXTICR2,
  IMR2,
  EMR2,
  RTSR2,
  FTSR2,
  SWIER2,
  PR2,
  MR40,
  MR40,
  (),
  (),
  (),
  (),
  (),
}

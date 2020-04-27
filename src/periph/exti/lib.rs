//! Extended interrupts and events controller.

#![feature(proc_macro_hygiene)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds, clippy::wildcard_imports)]
#![no_std]

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic EXTI peripheral variant.
    pub trait ExtiMap {}

    /// Generic EXTI peripheral.
    pub struct ExtiPeriph;

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

            drone_stm32_map_pieces::reg;
            crate;

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
map_exti! {
    "Extracts EXTI Line 0 register tokens.",
    periph_exti0,
    "EXTI Line 0 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 1 register tokens.",
    periph_exti1,
    "EXTI Line 1 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 2 register tokens.",
    periph_exti2,
    "EXTI Line 2 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 3 register tokens.",
    periph_exti3,
    "EXTI Line 3 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 4 register tokens.",
    periph_exti4,
    "EXTI Line 4 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 5 register tokens.",
    periph_exti5,
    "EXTI Line 5 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 6 register tokens.",
    periph_exti6,
    "EXTI Line 6 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 7 register tokens.",
    periph_exti7,
    "EXTI Line 7 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 8 register tokens.",
    periph_exti8,
    "EXTI Line 8 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 9 register tokens.",
    periph_exti9,
    "EXTI Line 9 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 10 register tokens.",
    periph_exti10,
    "EXTI Line 10 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 11 register tokens.",
    periph_exti11,
    "EXTI Line 11 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line peripheral variant 12 register tokens.",
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
map_exti! {
    "Extracts EXTI Line 13 register tokens.",
    periph_exti13,
    "EXTI Line 13 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 14 register tokens.",
    periph_exti14,
    "EXTI Line 14 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 15 register tokens.",
    periph_exti15,
    "EXTI Line 15 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 16 register tokens.",
    periph_exti16,
    "EXTI Line 16 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 17 register tokens.",
    periph_exti17,
    "EXTI Line 17 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 18 register tokens.",
    periph_exti18,
    "EXTI Line 18 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 19 register tokens.",
    periph_exti19,
    "EXTI Line 19 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 20 register tokens.",
    periph_exti20,
    "EXTI Line 20 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 21 register tokens.",
    periph_exti21,
    "EXTI Line 21 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 22 register tokens.",
    periph_exti22,
    "EXTI Line 22 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 23 register tokens.",
    periph_exti23,
    "EXTI Line 23 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 24 register tokens.",
    periph_exti24,
    "EXTI Line 24 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 25 register tokens.",
    periph_exti25,
    "EXTI Line 25 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 26 register tokens.",
    periph_exti26,
    "EXTI Line 26 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 27 register tokens.",
    periph_exti27,
    "EXTI Line 27 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 28 register tokens.",
    periph_exti28,
    "EXTI Line 28 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 29 register tokens.",
    periph_exti29,
    "EXTI Line 29 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 30 register tokens.",
    periph_exti30,
    "EXTI Line 30 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 31 register tokens.",
    periph_exti31,
    "EXTI Line 31 peripheral variant.",
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
map_exti! {
    "Extracts EXTI Line 32 register tokens.",
    periph_exti32,
    "EXTI Line 32 peripheral variant.",
    Exti32,
    EXTICR4,
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
map_exti! {
    "Extracts EXTI Line 33 register tokens.",
    periph_exti33,
    "EXTI Line 33 peripheral variant.",
    Exti33,
    EXTICR4,
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
map_exti! {
    "Extracts EXTI Line 34 register tokens.",
    periph_exti34,
    "EXTI Line 34 peripheral variant.",
    Exti34,
    EXTICR4,
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
map_exti! {
    "Extracts EXTI Line 35 register tokens.",
    periph_exti35,
    "EXTI Line 35 peripheral variant.",
    Exti35,
    EXTICR4,
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
map_exti! {
    "Extracts EXTI Line 36 register tokens.",
    periph_exti36,
    "EXTI Line 36 peripheral variant.",
    Exti36,
    EXTICR4,
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
map_exti! {
    "Extracts EXTI Line 37 register tokens.",
    periph_exti37,
    "EXTI Line 37 peripheral variant.",
    Exti37,
    EXTICR4,
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
map_exti! {
    "Extracts EXTI Line 38 register tokens.",
    periph_exti38,
    "EXTI Line 38 peripheral variant.",
    Exti38,
    EXTICR4,
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
map_exti! {
    "Extracts EXTI Line 39 register tokens.",
    periph_exti39,
    "EXTI Line 39 peripheral variant.",
    Exti39,
    EXTICR4,
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
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_exti! {
    "Extracts EXTI Line 40 register tokens.",
    periph_exti40,
    "EXTI Line 40 peripheral variant.",
    Exti40,
    EXTICR4,
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
map_exti! {
    "Extracts EXTI Line 0 register tokens.",
    periph_exti0,
    "EXTI Line 0 peripheral variant.",
    Exti0,
    EXTICR1,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR0,
    MR0,
    (EXTI0),
    (TR0),
    (TR0),
    (SWIER0),
    (PR0),
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
map_exti! {
    "Extracts EXTI Line 1 register tokens.",
    periph_exti1,
    "EXTI Line 1 peripheral variant.",
    Exti1,
    EXTICR1,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR1,
    MR1,
    (EXTI1),
    (TR1),
    (TR1),
    (SWIER1),
    (PR1),
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
map_exti! {
    "Extracts EXTI Line 2 register tokens.",
    periph_exti2,
    "EXTI Line 2 peripheral variant.",
    Exti2,
    EXTICR1,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR2,
    MR2,
    (EXTI2),
    (TR2),
    (TR2),
    (SWIER2),
    (PR2),
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
map_exti! {
    "Extracts EXTI Line 3 register tokens.",
    periph_exti3,
    "EXTI Line 3 peripheral variant.",
    Exti3,
    EXTICR1,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR3,
    MR3,
    (EXTI3),
    (TR3),
    (TR3),
    (SWIER3),
    (PR3),
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
map_exti! {
    "Extracts EXTI Line 4 register tokens.",
    periph_exti4,
    "EXTI Line 4 peripheral variant.",
    Exti4,
    EXTICR2,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR4,
    MR4,
    (EXTI4),
    (TR4),
    (TR4),
    (SWIER4),
    (PR4),
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
map_exti! {
    "Extracts EXTI Line 5 register tokens.",
    periph_exti5,
    "EXTI Line 5 peripheral variant.",
    Exti5,
    EXTICR2,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR5,
    MR5,
    (EXTI5),
    (TR5),
    (TR5),
    (SWIER5),
    (PR5),
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
map_exti! {
    "Extracts EXTI Line 6 register tokens.",
    periph_exti6,
    "EXTI Line 6 peripheral variant.",
    Exti6,
    EXTICR2,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR6,
    MR6,
    (EXTI6),
    (TR6),
    (TR6),
    (SWIER6),
    (PR6),
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
map_exti! {
    "Extracts EXTI Line 7 register tokens.",
    periph_exti7,
    "EXTI Line 7 peripheral variant.",
    Exti7,
    EXTICR2,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR7,
    MR7,
    (EXTI7),
    (TR7),
    (TR7),
    (SWIER7),
    (PR7),
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
map_exti! {
    "Extracts EXTI Line 8 register tokens.",
    periph_exti8,
    "EXTI Line 8 peripheral variant.",
    Exti8,
    EXTICR3,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR8,
    MR8,
    (EXTI8),
    (TR8),
    (TR8),
    (SWIER8),
    (PR8),
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
map_exti! {
    "Extracts EXTI Line 9 register tokens.",
    periph_exti9,
    "EXTI Line 9 peripheral variant.",
    Exti9,
    EXTICR3,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR9,
    MR9,
    (EXTI9),
    (TR9),
    (TR9),
    (SWIER9),
    (PR9),
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
map_exti! {
    "Extracts EXTI Line 10 register tokens.",
    periph_exti10,
    "EXTI Line 10 peripheral variant.",
    Exti10,
    EXTICR3,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR10,
    MR10,
    (EXTI10),
    (TR10),
    (TR10),
    (SWIER10),
    (PR10),
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
map_exti! {
    "Extracts EXTI Line 11 register tokens.",
    periph_exti11,
    "EXTI Line 11 peripheral variant.",
    Exti11,
    EXTICR3,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR11,
    MR11,
    (EXTI11),
    (TR11),
    (TR11),
    (SWIER11),
    (PR11),
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
map_exti! {
    "Extracts EXTI Line 12 register tokens.",
    periph_exti12,
    "EXTI Line 12 peripheral variant.",
    Exti12,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR12,
    MR12,
    (EXTI12),
    (TR12),
    (TR12),
    (SWIER12),
    (PR12),
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
map_exti! {
    "Extracts EXTI Line 13 register tokens.",
    periph_exti13,
    "EXTI Line 13 peripheral variant.",
    Exti13,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR13,
    MR13,
    (EXTI13),
    (TR13),
    (TR13),
    (SWIER13),
    (PR13),
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
map_exti! {
    "Extracts EXTI Line 14 register tokens.",
    periph_exti14,
    "EXTI Line 14 peripheral variant.",
    Exti14,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR14,
    MR14,
    (EXTI14),
    (TR14),
    (TR14),
    (SWIER14),
    (PR14),
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
map_exti! {
    "Extracts EXTI Line 15 register tokens.",
    periph_exti15,
    "EXTI Line 15 peripheral variant.",
    Exti15,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR15,
    MR15,
    (EXTI15),
    (TR15),
    (TR15),
    (SWIER15),
    (PR15),
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
map_exti! {
    "Extracts EXTI Line 16 register tokens.",
    periph_exti16,
    "EXTI Line 16 peripheral variant.",
    Exti16,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR16,
    MR16,
    (),
    (TR16),
    (TR16),
    (SWIER16),
    (PR16),
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
map_exti! {
    "Extracts EXTI Line 17 register tokens.",
    periph_exti17,
    "EXTI Line 17 peripheral variant.",
    Exti17,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR17,
    MR17,
    (),
    (TR17),
    (TR17),
    (SWIER17),
    (PR17),
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
map_exti! {
    "Extracts EXTI Line 18 register tokens.",
    periph_exti18,
    "EXTI Line 18 peripheral variant.",
    Exti18,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR18,
    MR18,
    (),
    (TR18),
    (TR18),
    (SWIER18),
    (PR18),
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
))]
map_exti! {
    "Extracts EXTI Line 19 register tokens.",
    periph_exti19,
    "EXTI Line 19 peripheral variant.",
    Exti19,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR19,
    MR19,
    (),
    (TR19),
    (TR19),
    (SWIER19),
    (PR19),
}

#[cfg(any(stm32_mcu = "stm32f446", stm32_mcu = "stm32f469",))]
map_exti! {
    "Extracts EXTI Line 19 register tokens.",
    periph_exti19,
    "EXTI Line 19 peripheral variant.",
    Exti19,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR19,
    MR19,
    (),
    (),
    (),
    (SWIER19),
    (PR19),
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_exti! {
    "Extracts EXTI Line 20 register tokens.",
    periph_exti20,
    "EXTI Line 20 peripheral variant.",
    Exti20,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR20,
    MR20,
    (),
    (TR20),
    (TR20),
    (SWIER20),
    (PR20),
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
map_exti! {
    "Extracts EXTI Line 21 register tokens.",
    periph_exti21,
    "EXTI Line 21 peripheral variant.",
    Exti21,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR21,
    MR21,
    (),
    (TR21),
    (TR21),
    (SWIER21),
    (PR21),
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
map_exti! {
    "Extracts EXTI Line 22 register tokens.",
    periph_exti22,
    "EXTI Line 22 peripheral variant.",
    Exti22,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR22,
    MR22,
    (),
    (TR22),
    (TR22),
    (SWIER22),
    (PR22),
}

#[cfg(any(stm32_mcu = "stm32f413",))]
map_exti! {
    "Extracts EXTI Line 23 register tokens.",
    periph_exti23,
    "EXTI Line 23 peripheral variant.",
    Exti23,
    EXTICR4,
    IMR,
    EMR,
    RTSR,
    FTSR,
    SWIER,
    PR,
    MR23,
    MR23,
    (),
    (TR23),
    (TR23),
    (SWIER23),
    (PR23),
}

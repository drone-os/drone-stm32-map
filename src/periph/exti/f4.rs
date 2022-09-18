//! Extended interrupts and events controller.
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

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
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
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

#[cfg(any(drone_stm32_map = "stm32f446", drone_stm32_map = "stm32f469"))]
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
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
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

#[cfg(any(drone_stm32_map = "stm32f413"))]
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

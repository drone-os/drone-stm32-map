//! USB on the go full speed (OTG_FS) pwrclk peripherals. 
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic USB-OTGFS peripheral variant.
    pub trait PwrclkOtgfsMap {}

    /// Generic USB-OTGFS peripheral.
    pub struct PwrclkOtgfsPeriph;

    OTG_FS_PWRCLK {
        FS_PCGCCTL {
            0x20 RwReg;
            PHYSUSP { RwRwRegFieldBit }
            GATEHCLK { RwRwRegFieldBit }
            STPPCLK { RwRwRegFieldBit }
        }
    }
}

macro_rules! map_otgfs_pwrclk {
    (
        $otgfs_macro_doc:expr,
        $otgfs_macro:ident,
        $otgfs_ty_doc:expr,
        $otgfs_ty:ident,
        $otgfs:ident,
    ) => {
        periph::map! {
            #[doc = $otgfs_macro_doc]
            pub macro $otgfs_macro;

            #[doc = $otgfs_ty_doc]
            pub struct $otgfs_ty;

            impl PwrclkOtgfsMap for $otgfs_ty {}

            drone_stm32_map_pieces::reg;
            crate::pwrclk;

            OTG_FS_PWRCLK {
                 FS_PCGCCTL {
                    FS_PCGCCTL;
                    PHYSUSP { PHYSUSP }
                    GATEHCLK { GATEHCLK }
                    STPPCLK { STPPCLK }
                }
            }
        }
    }
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_pwrclk! {
    "Extracts USB-OTGFS register tokens.",
    periph_otgfs_pwrclk,
    "USB-OTGFS peripheral variant.",
    Otgfs,
    OTGFS,
}


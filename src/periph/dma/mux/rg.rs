//! DMAMUX request generators.

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
    /// Generic DMAMUX request generator peripheral variant.
    pub trait DmamuxRgMap {
        /// DMAMUX head peripheral variant.
        type DmamuxMap: super::DmamuxMap;
    }

    /// Generic DMAMUX request generator peripheral.
    pub struct DmamuxRgPeriph;

    DMAMUX {
        RGCR {
            0x20 RwRegBitBand;
            GNBREQ { RwRwRegFieldBits }
            GPOL { RwRwRegFieldBits }
            GE { RwRwRegFieldBitBand }
            OIE { RwRwRegFieldBitBand }
            SIG_ID { RwRwRegFieldBits }
        }
        RGSR {
            0x20 RoRegBitBand Shared;
            OF { RoRoRegFieldBitBand }
        }
        RGCFR {
            0x20 WoRegBitBand Shared;
            COF { WoWoRegFieldBitBand }
        }
    }
}

macro_rules! map_dmamux_rg {
    (
        $dmamux_rg_macro_doc:expr,
        $dmamux_rg_macro:ident,
        $dmamux_rg_ty_doc:expr,
        $dmamux_rg_ty:ident,
        $rgcr:ident,
        $of:ident,
        $cof:ident,
    ) => {
        periph::map! {
            #[doc = $dmamux_rg_macro_doc]
            pub macro $dmamux_rg_macro;

            #[doc = $dmamux_rg_ty_doc]
            pub struct $dmamux_rg_ty;

            impl DmamuxRgMap for $dmamux_rg_ty {
                type DmamuxMap = super::Dmamux1;
            }

            drone_stm32_map_pieces::reg;
            crate::mux::rg;

            DMAMUX {
                DMAMUX1;
                RGCR {
                    $rgcr;
                    GNBREQ { GNBREQ }
                    GPOL { GPOL }
                    GE { GE }
                    OIE { OIE }
                    SIG_ID { SIG_ID }
                }
                RGSR {
                    RGSR Shared;
                    OF { $of }
                }
                RGCFR {
                    RGCFR Shared;
                    COF { $cof }
                }
            }
        }
    };
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 0 register tokens.",
    periph_dmamux1_rg0,
    "DMAMUX1 request generator 0 peripheral.",
    Dmamux1Rg0,
    RG0CR,
    OF0,
    COF0,
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 1 register tokens.",
    periph_dmamux1_rg1,
    "DMAMUX1 request generator 1 peripheral.",
    Dmamux1Rg1,
    RG1CR,
    OF1,
    COF1,
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 2 register tokens.",
    periph_dmamux1_rg2,
    "DMAMUX1 request generator 2 peripheral.",
    Dmamux1Rg2,
    RG2CR,
    OF2,
    COF2,
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 3 register tokens.",
    periph_dmamux1_rg3,
    "DMAMUX1 request generator 3 peripheral.",
    Dmamux1Rg3,
    RG3CR,
    OF3,
    COF3,
}

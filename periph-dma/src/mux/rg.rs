//! DMAMUX request generators.

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
  /// DMAMUX request generator.
  pub trait DmamuxRgMap {
    /// DMAMUX head.
    type DmamuxMap: super::DmamuxMap;
  }

  DMAMUX {
    RGCR {
      0x20 RwReg;
      GNBREQ { RwRwRegFieldBits }
      GPOL { RwRwRegFieldBits }
      GE { RwRwRegFieldBit }
      OIE { RwRwRegFieldBit }
      SIG_ID { RwRwRegFieldBits }
    }
    RGSR {
      0x20 RoReg Shared;
      OF { RoRoRegFieldBit }
    }
    RGCFR {
      0x20 WoReg Shared;
      COF { WoWoRegFieldBit }
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

      ::drone_stm32_map_pieces::reg; mux::rg;

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
  "Acquires DMAMUX1 request generator 0.",
  periph_dmamux1_rg0,
  "DMAMUX1 request generator 0.",
  Dmamux1Rg0,
  RG0CR,
  OF0,
  COF0,
}

map_dmamux_rg! {
  "Acquires DMAMUX1 request generator 1.",
  periph_dmamux1_rg1,
  "DMAMUX1 request generator 1.",
  Dmamux1Rg1,
  RG1CR,
  OF1,
  COF1,
}

map_dmamux_rg! {
  "Acquires DMAMUX1 request generator 2.",
  periph_dmamux1_rg2,
  "DMAMUX1 request generator 2.",
  Dmamux1Rg2,
  RG2CR,
  OF2,
  COF2,
}

map_dmamux_rg! {
  "Acquires DMAMUX1 request generator 3.",
  periph_dmamux1_rg3,
  "DMAMUX1 request generator 3.",
  Dmamux1Rg3,
  RG3CR,
  OF3,
  COF3,
}

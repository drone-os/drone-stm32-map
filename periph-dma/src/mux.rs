//! DMA request multiplexer.

pub mod ch;
pub mod rg;

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
    /// DMAMUX head.
    pub trait DmamuxMap {}

    RCC {
        AHB1ENR {
            0x20 RwRegBitBand Shared;
            DMAMUXEN { RwRwRegFieldBitBand }
        }
        AHB1RSTR {
            0x20 RwRegBitBand Shared;
            DMAMUXRST { RwRwRegFieldBitBand }
        }
        AHB1SMENR {
            0x20 RwRegBitBand Shared;
            DMAMUXSMEN { RwRwRegFieldBitBand }
        }
    }
}

periph::map! {
    /// Acquires DMAMUX1 head.
    pub macro periph_dmamux1;

    /// DMAMUX1 head.
    pub struct Dmamux1;

    impl DmamuxMap for Dmamux1 {}

    ::drone_stm32_map_pieces::reg; mux;

    RCC {
        AHB1ENR {
            AHB1ENR Shared;
            DMAMUXEN { DMAMUX1EN }
        }
        AHB1RSTR {
            AHB1RSTR Shared;
            DMAMUXRST { DMAMUX1RST }
        }
        AHB1SMENR {
            AHB1SMENR Shared;
            DMAMUXSMEN { DMAMUX1SMEN }
        }
    }
}

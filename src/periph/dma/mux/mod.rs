//! DMA request multiplexer.

pub mod ch;
pub mod rg;

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
    /// Generic DMAMUX head peripheral variant.
    pub trait DmamuxMap {}

    /// Generic DMAMUX head peripheral.
    pub struct DmamuxPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            DMAMUXEN { RwRwRegFieldBitBand }
        }
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            DMAMUXRST { RwRwRegFieldBitBand }
        }
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            DMAMUXSMEN { RwRwRegFieldBitBand }
        }
    }
}

periph::map! {
    /// Extracts DMAMUX1 head register tokens.
    pub macro periph_dmamux1;

    /// DMAMUX1 head peripheral variant.
    pub struct Dmamux1;

    impl DmamuxMap for Dmamux1 {}

    drone_stm32_map_pieces::reg;
    crate::mux;

    RCC {
        BUSENR {
            AHB1ENR Shared;
            DMAMUXEN { DMAMUX1EN }
        }
        BUSRSTR {
            AHB1RSTR Shared;
            DMAMUXRST { DMAMUX1RST }
        }
        BUSSMENR {
            AHB1SMENR Shared;
            DMAMUXSMEN { DMAMUX1SMEN }
        }
    }
}

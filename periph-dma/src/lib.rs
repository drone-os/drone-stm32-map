//! Direct Memory Access.

#![feature(proc_macro_hygiene)]
#![no_std]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

pub mod ch;
#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
pub mod mux;

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
  /// DMA head.
  pub trait DmaMap {}

  RCC {
    AHB1ENR {
      0x20 RwRegBitBand Shared;
      DMAEN { RwRwRegFieldBitBand }
    }
    AHB1RSTR {
      0x20 RwRegBitBand Shared;
      DMARST { RwRwRegFieldBitBand }
    }
    AHB1SMENR {
      0x20 RwRegBitBand Shared;
      DMASMEN { RwRwRegFieldBitBand }
    }
  }
}

#[allow(unused_macros)]
macro_rules! map_dma {
  (
    $dma_macro_doc:expr,
    $dma_macro:ident,
    $dma_ty_doc:expr,
    $dma_ty:ident,
    $dmaen:ident,
    $dmarst:ident,
    $dmasmen:ident,
  ) => {
    periph::map! {
      #[doc = $dma_macro_doc]
      pub macro $dma_macro;

      #[doc = $dma_ty_doc]
      pub struct $dma_ty;

      impl DmaMap for $dma_ty {}

      ::drone_stm32_map_pieces::reg;;

      RCC {
        AHB1ENR {
          AHB1ENR Shared;
          DMAEN { $dmaen }
        }
        AHB1RSTR {
          AHB1RSTR Shared;
          DMARST { $dmarst }
        }
        AHB1SMENR {
          AHB1SMENR Shared;
          DMASMEN { $dmasmen }
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
map_dma! {
  "Acquires DMA1 head.",
  periph_dma1,
  "DMA1 head.",
  Dma1,
  DMA1EN,
  DMA1RST,
  DMA1SMEN,
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
map_dma! {
  "Acquires DMA2 head.",
  periph_dma2,
  "DMA2 head.",
  Dma2,
  DMA2EN,
  DMA2RST,
  DMA2SMEN,
}

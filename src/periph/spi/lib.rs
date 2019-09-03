//! Serial Peripheral Interface.

#![feature(proc_macro_hygiene)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds)]
#![no_std]

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
    /// Generic SPI peripheral variant.
    pub trait SpiMap {}

    /// Generic SPI peripheral.
    pub struct SpiPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            SPIEN { RwRwRegFieldBitBand }
        }
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            SPIRST { RwRwRegFieldBitBand }
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
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            SPISMEN { RwRwRegFieldBitBand }
        }
    }
    SPI {
        CR1 {
            0x20 RwRegBitBand;
            BIDIMODE { RwRwRegFieldBitBand }
            BIDIOE { RwRwRegFieldBitBand }
            BR { RwRwRegFieldBits }
            CPHA { RwRwRegFieldBitBand }
            CPOL { RwRwRegFieldBitBand }
            CRCEN { RwRwRegFieldBitBand }
            CRCNEXT { RwRwRegFieldBitBand }
            DFF { RwRwRegFieldBitBand }
            LSBFIRST { RwRwRegFieldBitBand }
            MSTR { RwRwRegFieldBitBand }
            RXONLY { RwRwRegFieldBitBand }
            SPE { RwRwRegFieldBitBand }
            SSI { RwRwRegFieldBitBand }
            SSM { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
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
            DS { RwRwRegFieldBits }
            ERRIE { RwRwRegFieldBitBand }
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
            FRF { RwRwRegFieldBitBand }
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
            FRXTH { RwRwRegFieldBitBand }
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
            LDMA_RX { RwRwRegFieldBitBand }
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
            LDMA_TX { RwRwRegFieldBitBand }
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
            NSSP { RwRwRegFieldBitBand }
            RXDMAEN { RwRwRegFieldBitBand }
            RXNEIE { RwRwRegFieldBitBand }
            SSOE { RwRwRegFieldBitBand }
            TXDMAEN { RwRwRegFieldBitBand }
            TXEIE { RwRwRegFieldBitBand }
        }
        SR {
            0x20 RwRegBitBand;
            BSY { RoRwRegFieldBitBand }
            #[cfg(any(
                feature = "stm32f101",
                feature = "stm32f102",
                feature = "stm32f103",
                feature = "stm32f107",
            ))]
            CHSIDE { RoRwRegFieldBitBand }
            CRCERR { RwRwRegFieldBitBand }
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
            FRLVL { RoRwRegFieldBits }
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
            FTLVL { RoRwRegFieldBits }
            MODF { RoRwRegFieldBitBand }
            OVR { RoRwRegFieldBitBand }
            RXNE { RoRwRegFieldBitBand }
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
            TIFRFE { RoRwRegFieldBitBand }
            TXE { RoRwRegFieldBitBand }
            #[cfg(any(
                feature = "stm32f101",
                feature = "stm32f102",
                feature = "stm32f103",
                feature = "stm32f107",
            ))]
            UDR { RoRwRegFieldBitBand }
        }
        DR {
            0x20 RwRegBitBand;
            DR { RwRwRegFieldBits }
        }
        CRCPR {
            0x20 RwRegBitBand;
            CRCPOLY { RwRwRegFieldBits }
        }
        RXCRCR {
            0x20 RoRegBitBand;
            RxCRC { RoRoRegFieldBits }
        }
        TXCRCR {
            0x20 RoRegBitBand;
            TxCRC { RoRoRegFieldBits }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_spi {
    (
        $spi_macro_doc:expr,
        $spi_macro:ident,
        $spi_ty_doc:expr,
        $spi_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $spien:ident,
        $spirst:ident,
        $spismen:ident,
        $spi:ident,
    ) => {
        periph::map! {
            #[doc = $spi_macro_doc]
            pub macro $spi_macro;

            #[doc = $spi_ty_doc]
            pub struct $spi_ty;

            impl SpiMap for $spi_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    SPIEN { $spien }
                }
                BUSRSTR {
                    $busrstr Shared;
                    SPIRST { $spirst }
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
                BUSSMENR {
                    $bussmenr Shared;
                    SPISMEN { $spismen }
                }
            }
            SPI {
                $spi;
                CR1 {
                    CR1;
                    BIDIMODE { BIDIMODE }
                    BIDIOE { BIDIOE }
                    BR { BR }
                    CPHA { CPHA }
                    CPOL { CPOL }
                    CRCEN { CRCEN }
                    CRCNEXT { CRCNEXT }
                    DFF { DFF }
                    LSBFIRST { LSBFIRST }
                    MSTR { MSTR }
                    RXONLY { RXONLY }
                    SPE { SPE }
                    SSI { SSI }
                    SSM { SSM }
                }
                CR2 {
                    CR2;
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
                    DS { DS }
                    ERRIE { ERRIE }
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
                    FRF { FRF }
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
                    FRXTH { FRXTH }
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
                    LDMA_RX { LDMA_RX }
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
                    LDMA_TX { LDMA_TX }
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
                    NSSP { NSSP }
                    RXDMAEN { RXDMAEN }
                    RXNEIE { RXNEIE }
                    SSOE { SSOE }
                    TXDMAEN { TXDMAEN }
                    TXEIE { TXEIE }
                }
                SR {
                    SR;
                    BSY { BSY }
                    #[cfg(any(
                        feature = "stm32f101",
                        feature = "stm32f102",
                        feature = "stm32f103",
                        feature = "stm32f107",
                    ))]
                    CHSIDE { CHSIDE }
                    CRCERR { CRCERR }
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
                    FRLVL { FRLVL }
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
                    FTLVL { FTLVL }
                    MODF { MODF }
                    OVR { OVR }
                    RXNE { RXNE }
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
                    TIFRFE { TIFRFE }
                    TXE { TXE }
                    #[cfg(any(
                        feature = "stm32f101",
                        feature = "stm32f102",
                        feature = "stm32f103",
                        feature = "stm32f107",
                    ))]
                    UDR { UDR }
                }
                DR {
                    DR;
                    DR { DR }
                }
                CRCPR {
                    CRCPR;
                    CRCPOLY { CRCPOLY }
                }
                RXCRCR {
                    RXCRCR;
                    RxCRC { RxCRC }
                }
                TXCRCR {
                    TXCRCR;
                    TxCRC { TxCRC }
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
map_spi! {
    "Extracts SPI1 register tokens.",
    periph_spi1,
    "SPI1 peripheral variant.",
    Spi1,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    SPI1EN,
    SPI1RST,
    SPI1SMEN,
    SPI1,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_spi! {
    "Extracts SPI2 register tokens.",
    periph_spi2,
    "SPI2 peripheral variant.",
    Spi2,
    APB1ENR,
    APB1RSTR,
    APB1SMENR,
    SPI2EN,
    SPI2RST,
    SPI2SMEN,
    SPI2,
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f107",
))]
map_spi! {
    "Extracts SPI3 register tokens.",
    periph_spi3,
    "SPI3 peripheral variant.",
    Spi3,
    APB1ENR,
    APB1RSTR,
    APB1SMENR,
    SPI3EN,
    SPI3RST,
    SPI3SMEN,
    SPI3,
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
map_spi! {
    "Extracts SPI2 register tokens.",
    periph_spi2,
    "SPI2 peripheral variant.",
    Spi2,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    SPI2EN,
    SPI2RST,
    SPI2SMEN,
    SPI2,
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
map_spi! {
    "Extracts SPI3 register tokens.",
    periph_spi3,
    "SPI3 peripheral variant.",
    Spi3,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    SPI3EN,
    SPI3RST,
    SPI3SMEN,
    SPI3,
}

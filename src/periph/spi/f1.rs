//! Serial Peripheral Interface.
//!
//! For STM32F1 Series of mainstream MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

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
            ERRIE { RwRwRegFieldBitBand }
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
                drone_stm32_map = "stm32f101",
                drone_stm32_map = "stm32f102",
                drone_stm32_map = "stm32f103",
                drone_stm32_map = "stm32f107",
            ))]
            CHSIDE { RoRwRegFieldBitBand }
            CRCERR { RwRwRegFieldBitBand }
            MODF { RoRwRegFieldBitBand }
            OVR { RoRwRegFieldBitBand }
            RXNE { RoRwRegFieldBitBand }
            TXE { RoRwRegFieldBitBand }
            #[cfg(any(
                drone_stm32_map = "stm32f101",
                drone_stm32_map = "stm32f102",
                drone_stm32_map = "stm32f103",
                drone_stm32_map = "stm32f107",
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

macro_rules! map_spi {
    (
        $spi_macro_doc:expr,
        $spi_macro:ident,
        $spi_ty_doc:expr,
        $spi_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $spien:ident,
        $spirst:ident,
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
                    ERRIE { ERRIE }
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
                        drone_stm32_map = "stm32f101",
                        drone_stm32_map = "stm32f102",
                        drone_stm32_map = "stm32f103",
                        drone_stm32_map = "stm32f107",
                    ))]
                    CHSIDE { CHSIDE }
                    CRCERR { CRCERR }
                    MODF { MODF }
                    OVR { OVR }
                    RXNE { RXNE }
                    TXE { TXE }
                    #[cfg(any(
                        drone_stm32_map = "stm32f101",
                        drone_stm32_map = "stm32f102",
                        drone_stm32_map = "stm32f103",
                        drone_stm32_map = "stm32f107",
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

map_spi! {
    "Extracts SPI1 register tokens.",
    periph_spi1,
    "SPI1 peripheral variant.",
    Spi1,
    APB2ENR,
    APB2RSTR,
    SPI1EN,
    SPI1RST,
    SPI1,
}

map_spi! {
    "Extracts SPI2 register tokens.",
    periph_spi2,
    "SPI2 peripheral variant.",
    Spi2,
    APB1ENR,
    APB1RSTR,
    SPI2EN,
    SPI2RST,
    SPI2,
}

#[cfg(any(
    drone_stm32_map = "stm32f100",
    drone_stm32_map = "stm32f101",
    drone_stm32_map = "stm32f103",
    drone_stm32_map = "stm32f107",
))]
map_spi! {
    "Extracts SPI3 register tokens.",
    periph_spi3,
    "SPI3 peripheral variant.",
    Spi3,
    APB1ENR,
    APB1RSTR,
    SPI3EN,
    SPI3RST,
    SPI3,
}

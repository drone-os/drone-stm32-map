//! Serial Peripheral Interface.
//! for STM32F4 series of high-performance MCUs with DSP and FPU instructions.

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
                stm32_mcu = "stm32f101",
                stm32_mcu = "stm32f102",
                stm32_mcu = "stm32f103",
                stm32_mcu = "stm32f107",
            ))]
            CHSIDE { RoRwRegFieldBitBand }
            CRCERR { RwRwRegFieldBitBand }
            MODF { RoRwRegFieldBitBand }
            OVR { RoRwRegFieldBitBand }
            RXNE { RoRwRegFieldBitBand }
            TXE { RoRwRegFieldBitBand }
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
                    DS { DS }
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
                    CHSIDE { CHSIDE }
                    CRCERR { CRCERR }
                    MODF { MODF }
                    OVR { OVR }
                    RXNE { RXNE }
                    TXE { TXE }
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

//! Serial Peripheral Interface.
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

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
            CRCEN { RwRwRegFieldBitBand }
            CRCNEXT { RwRwRegFieldBitBand }
            DFF { RwRwRegFieldBitBand }
            RXONLY { RwRwRegFieldBitBand }
            SSM { RwRwRegFieldBitBand }
            SSI { RwRwRegFieldBitBand }
            LSBFIRST { RwRwRegFieldBitBand }
            SPE { RwRwRegFieldBitBand }
            BR { RwRwRegFieldBits }
            MSTR { RwRwRegFieldBitBand }
            CPOL { RwRwRegFieldBitBand }
            CPHA { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
            TXEIE { RwRwRegFieldBitBand }
            RXNEIE { RwRwRegFieldBitBand }
            ERRIE { RwRwRegFieldBitBand }
            FRF { RwRwRegFieldBitBand }
            SSOE { RwRwRegFieldBitBand }
            TXDMAEN { RwRwRegFieldBitBand }
            RXDMAEN { RwRwRegFieldBitBand }
        }
        SR {
            0x20 RwRegBitBand;
            FRE { RoRwRegFieldBitBand }
            BSY { RoRwRegFieldBitBand }
            OVR { RoRwRegFieldBitBand }
            MODF { RoRwRegFieldBitBand }
            CRCERR { RwRwRegFieldBitBand }
            UDR { RoRwRegFieldBitBand }
            CHSIDE { RoRwRegFieldBitBand }
            TXE { RoRwRegFieldBitBand }
            RXNE { RoRwRegFieldBitBand }
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
        I2SCFGR {
            0x20 RwRegBitBand Option;
            #[cfg(any(
                stm32_mcu = "stm32f410",
                stm32_mcu = "stm32f412",
                stm32_mcu = "stm32f413",
                stm32_mcu = "stm32f446",
            ))]
            ASTREN { RwRwRegFieldBitBand }
            I2SMOD { RwRwRegFieldBitBand }
            I2SE { RwRwRegFieldBitBand }
            I2SCFG { RwRwRegFieldBits }
            PCMSYNC { RwRwRegFieldBitBand }
            I2SSTD { RwRwRegFieldBits }
            CKPOL { RwRwRegFieldBitBand }
            DATLEN { RwRwRegFieldBits }
            CHLEN { RwRwRegFieldBitBand }
        }
        I2SPR {
            0x20 RwRegBitBand Option;
            MCKOE { RwRwRegFieldBitBand }
            ODD { RwRwRegFieldBitBand }
            I2SDIV { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_spi {
    (
        $spi_macro_doc:expr,
        $spi_macro:ident,
        $spi_ty_doc:expr,
        $spi_ty:ident,
        $spien:ident,
        $spirst:ident,
        $spismen:ident,
        $spi:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        ($($i2scfgr:ident)?),
        ($($i2spr:ident)?),
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
                    ERRIE { ERRIE }
                    RXDMAEN { RXDMAEN }
                    RXNEIE { RXNEIE }
                    FRF { FRF }
                    SSOE { SSOE }
                    TXDMAEN { TXDMAEN }
                    TXEIE { TXEIE }
                }
                SR {
                    SR;
                    FRE { TIFRFE }
                    BSY { BSY }
                    CHSIDE { CHSIDE }
                    CRCERR { CRCERR }
                    MODF { MODF }
                    OVR { OVR }
                    RXNE { RXNE }
                    TXE { TXE }
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
                I2SCFGR {
                    $(
                        $i2scfgr Option;
                        #[cfg(any(
                            stm32_mcu = "stm32f410",
                            stm32_mcu = "stm32f412",
                            stm32_mcu = "stm32f413",
                            stm32_mcu = "stm32f446",
                        ))]
                        ASTREN { ASTREN }
                        I2SMOD { I2SMOD }
                        I2SE { I2SE }
                        I2SCFG { I2SCFG }
                        PCMSYNC { PCMSYNC }
                        I2SSTD { I2SSTD }
                        CKPOL { CKPOL }
                        DATLEN { DATLEN }
                        CHLEN { CHLEN }
                    )*
                }
                I2SPR {
                    $(
                        $i2spr Option;
                        MCKOE { MCKOE }
                        ODD { ODD }
                        I2SDIV { I2SDIV }
                    )*
                }
            }
        }
    };
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts SPI1 register tokens.",
    periph_spi1,
    "SPI1 peripheral variant.",
    Spi1,
    SPI1EN,
    SPI1RST,
    SPI1LPEN,
    SPI1,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
))]
map_spi! {
    "Extracts SPI1 register tokens.",
    periph_spi1,
    "SPI1 peripheral variant.",
    Spi1,
    SPI1EN,
    SPI1RST,
    SPI1LPEN,
    SPI1,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    (I2SCFGR),
    (I2SPR),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts SPI2 register tokens.",
    periph_spi2,
    "SPI2 peripheral variant.",
    Spi2,
    SPI2EN,
    SPI2RST,
    SPI2LPEN,
    SPI2,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    (I2SCFGR),
    (I2SPR),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts SPI3 register tokens.",
    periph_spi3,
    "SPI3 peripheral variant.",
    Spi3,
    SPI3EN,
    SPI3RST,
    SPI3LPEN,
    SPI3,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    (I2SCFGR),
    (I2SPR),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts SPI4 register tokens.",
    periph_spi4,
    "SPI4 peripheral variant.",
    Spi4,
    SPI4EN,
    SPI4RST,
    SPI4LPEN,
    SPI4,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    (),
    (),
}

#[cfg(any(stm32_mcu = "stm32f411", stm32_mcu = "stm32f412", stm32_mcu = "stm32f413",))]
map_spi! {
    "Extracts SPI4 register tokens.",
    periph_spi4,
    "SPI4 peripheral variant.",
    Spi4,
    SPI4EN,
    SPI4RST,
    SPI4LPEN,
    SPI4,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    (I2SCFGR),
    (I2SPR),
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts SPI5 register tokens.",
    periph_spi5,
    "SPI5 peripheral variant.",
    Spi5,
    SPI5EN,
    SPI5RST,
    SPI5LPEN,
    SPI5,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
))]
map_spi! {
    "Extracts SPI5 register tokens.",
    periph_spi5,
    "SPI5 peripheral variant.",
    Spi5,
    SPI5EN,
    SPI5RST,
    SPI5LPEN,
    SPI5,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    (I2SCFGR),
    (I2SPR),
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts SPI6 register tokens.",
    periph_spi6,
    "SPI6 peripheral variant.",
    Spi6,
    SPI6EN,
    SPI6RST,
    SPI6LPEN,
    SPI6,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    (),
    (),
}

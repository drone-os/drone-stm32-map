//! Serial Peripheral Interface
//!
//! For STM32F4 Series of mainstream MCUs

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph!{
    /// Generic SPI peripheral variant.
    pub trait SpiMap{}

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
            SPILPEN { RwRwRegFieldBitBand }
        }
    }

    SPI {
        CR1 {
            0x20 RwRegBitBand Option;
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
        CR2{
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
            0x20 RwRegBitBand Option;
            CRCPOLY { RwRwRegFieldBits }
        }
        RXCRCR {
            0x20 RoRegBitBand Option;
            RxCRC { RoRoRegFieldBits }
        }
        TXCRCR {
            0x20 RoRegBitBand Option;
            TxCRC { RoRoRegFieldBits }
        }
        I2SCFGR {
            0x20 RwRegBitBand Option;
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
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $spien:ident,
        $spirst:ident,
        $spilpen:ident,      
        $spi:ident,
        ($($cr1:ident)?),
        ($($crcpr:ident)?),
        ($($rxcrcr:ident)?),
        ($($txcrcr:ident)?),
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
                    SPILPEN{ $spilpen }
                }
            }
            SPI {
                $spi;
                CR1 {
                    $(
                        $cr1 Option;
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
                    )*
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
                    $(
                        $crcpr Option;
                        CRCPOLY { CRCPOLY }
                    )*
                }
                RXCRCR {
                    $(
                        $rxcrcr Option;
                        RxCRC { RxCRC }
                    )*
                }
                TXCRCR {
                    $(
                        $txcrcr Option;
                        TxCRC { TxCRC }
                    )*
                }
                I2SCFGR {
                    $(
                        $i2scfgr Option;
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

map_spi! {
    "Extracts SPI1 register tokens.",
    periph_spi1,
    "SPI1 peripheral variant.",
    Spi1,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI1EN,
    SPI1RST,
    SPI1LPEN,
    SPI1,
    (CR1),
    (CRCPR),
    (RXCRCR),
    (TXCRCR),
    (),
    (),
}

map_spi! {
    "Extracts SPI2 register tokens.",
    periph_spi2,
    "SPI2 peripheral variant.",
    Spi2,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    SPI2EN,
    SPI2RST,
    SPI2LPEN,
    SPI2,
    (CR1),
    (CRCPR),
    (RXCRCR),
    (TXCRCR),
    (),
    (),
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
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    SPI3EN,
    SPI3RST,
    SPI3LPEN,
    SPI3,
    (CR1),
    (CRCPR),
    (RXCRCR),
    (TXCRCR),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f413",
))]
map_spi! {
    "Extracts SPI4 register tokens.",
    periph_spi4,
    "SPI4 peripheral variant.",
    Spi4,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI4EN,
    SPI4RST,
    SPI4LPEN,
    SPI4,
    (CR1),
    (CRCPR),
    (RXCRCR),
    (TXCRCR),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts SPI4 register tokens.",
    periph_spi4,
    "SPI4 peripheral variant.",
    Spi4,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI4ENR,//in svd it is called ENR 
    SPI4RST,
    SPI4LPEN,
    SPI4,
    (CR1),
    (CRCPR),
    (RXCRCR),
    (TXCRCR),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f410",
))]
map_spi! {
    "Extracts SPI5 register tokens.",
    periph_spi5,
    "SPI5 peripheral variant.",
    Spi5,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI5EN, 
    SPI5RST,
    SPI5LPEN,
    SPI5,
    (CR1),
    (CRCPR),
    (RXCRCR),
    (TXCRCR),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f413",
))]
map_spi! {
    "Extracts SPI5 register tokens.",
    periph_spi5,
    "SPI5 peripheral variant.",
    Spi5,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI5EN, 
    SPI5RST,
    SPI5LPEN,
    SPI5,
    (CR1),
    (CRCPR),
    (RXCRCR),
    (TXCRCR),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts SPI5 register tokens.",
    periph_spi5,
    "SPI5 peripheral variant.",
    Spi5,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI5ENR, //in svd it is called ENR 
    SPI5RST,
    SPI5LPEN,
    SPI5,
    (CR1),
    (CRCPR),
    (RXCRCR),
    (TXCRCR),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts SPI6 register tokens.",
    periph_spi6,
    "SPI6 peripheral variant.",
    Spi6,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI6ENR,//in svd it is called ENR 
    SPI6RST,
    SPI6LPEN,
    SPI6,
    (CR1),
    (CRCPR),
    (RXCRCR),
    (TXCRCR),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts I2S2 register tokens.",
    periph_i2s2ext,
    "I2S2 peripheral variant.",
    I2S2ext,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI1EN,
    SPI1RST,
    SPI1LPEN,
    I2S2ext,
    (),
    (),
    (),
    (),
    (I2SCFGR),
    (I2SPR),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
map_spi! {
    "Extracts I2S3 register tokens.",
    periph_i2s3ext,
    "I2S3 peripheral variant.",
    I2S3ext,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    SPI3EN,
    SPI3RST,
    SPI3LPEN,
    I2S3ext,
    (),
    (),
    (),
    (),
    (I2SCFGR),
    (I2SPR),
}

// The I2S1 peripheral of stm32f410 is derived from SPI1
// but has and has no extended node definition in the SVD!
#[cfg(any(
    stm32_mcu = "stm32f410",
))]
map_spi! {
    "Extracts I2S1 register tokens.",
    periph_i2s1,
    "I2S1 peripheral variant.",
    I2S1,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI1EN,
    SPI1RST,
    SPI1LPEN,
    SPI1,
    (),
    (),
    (),
    (),
    (I2SCFGR),
    (I2SPR),
}

// The I2S2 peripheral of stm32f410 is derived from SPI2
// but has and has no extended node definition in the SVD!
#[cfg(any(
    stm32_mcu = "stm32f410",
))]
map_spi! {
    "Extracts I2S2 register tokens.",
    periph_i2s2,
    "I2S2 peripheral variant.",
    I2S2,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    SPI2EN,
    SPI2RST,
    SPI2LPEN,
    SPI2,
    (),
    (),
    (),
    (),
    (I2SCFGR),
    (I2SPR),
}

// The I2S5 peripheral of stm32f410 is derived from SPI5
// but has and has no extended node definition in the SVD!
#[cfg(any(
    stm32_mcu = "stm32f410",
))]
map_spi! {
    "Extracts I2S5 register tokens.",
    periph_i2s5,
    "I2S2 peripheral variant.",
    I2S5,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI5EN, 
    SPI5RST,
    SPI5LPEN,
    SPI5,
    (),
    (),
    (),
    (),
    (I2SCFGR),
    (I2SPR),
}

// The I2S1 peripheral of stm32f446 is derived from SPI1
// but has and has no extended node definition in the SVD!
#[cfg(any(
    stm32_mcu = "stm32f446",
))]
map_spi! {
    "Extracts SPI1 register tokens.",
    periph_i2s1,
    "SPI1 peripheral variant.",
    I2S1,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI1EN,
    SPI1RST,
    SPI1LPEN,
    SPI1,
    (),
    (),
    (),
    (),
    (I2SCFGR),
    (I2SPR),
}

// The I2S2 peripheral of stm32f446 is derived from SPI2
// but has and has no extended node definition in the SVD!
#[cfg(any(
    stm32_mcu = "stm32f446",
))]
map_spi! {
    "Extracts I2S2 register tokens.",
    periph_i2s2,
    "I2S2 peripheral variant.",
    I2S2,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    SPI2EN,
    SPI2RST,
    SPI2LPEN,
    SPI2,
    (),
    (),
    (),
    (),
    (I2SCFGR),
    (I2SPR),
}

// The I2S3 peripheral of stm32f446 is derived from SPI3
// but has and has no extended node definition in the SVD!
#[cfg(any(
    stm32_mcu = "stm32f446",
))]
map_spi! {
    "Extracts I2S3 register tokens.",
    periph_i2s3,
    "I2S3 peripheral variant.",
    I2S3,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    SPI3EN,
    SPI3RST,
    SPI3LPEN,
    SPI3,
    (),
    (),
    (),
    (),
    (I2SCFGR),
    (I2SPR),
}


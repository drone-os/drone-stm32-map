//! Inter-Integrated Circuit.

#![feature(proc_macro_hygiene)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds)]
#![no_std]

use drone_core::periph;
use drone_cortex_m::reg::marker::*;

periph! {
    /// Generic I2C peripheral variant.
    pub trait I2CMap {}

    /// Generic I2C peripheral.
    pub struct I2CPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            I2CEN { RwRwRegFieldBitBand }
        }
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            I2CRST { RwRwRegFieldBitBand }
        }
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            I2CSMEN { RwRwRegFieldBitBand }
        }
        CCIPR {
            0x20 RwRegBitBand Shared;
            I2CSEL { RwRwRegFieldBits }
        }
    }
    I2C {
        CR1 {
            0x20 RwRegBitBand;
            ADDRIE { RwRwRegFieldBitBand }
            ALERTEN { RwRwRegFieldBitBand }
            ANFOFF { RwRwRegFieldBitBand }
            DNF { RwRwRegFieldBits }
            ERRIE { RwRwRegFieldBitBand }
            GCEN { RwRwRegFieldBitBand }
            NACKIE { RwRwRegFieldBitBand }
            NOSTRETCH { RwRwRegFieldBitBand }
            PECEN { RwRwRegFieldBitBand }
            PE { RwRwRegFieldBitBand }
            RXDMAEN { RwRwRegFieldBitBand }
            RXIE { RwRwRegFieldBitBand }
            SBC { RwRwRegFieldBitBand }
            SMBDEN { RwRwRegFieldBitBand }
            SMBHEN { RwRwRegFieldBitBand }
            STOPIE { RwRwRegFieldBitBand }
            TCIE { RwRwRegFieldBitBand }
            TXDMAEN { RwRwRegFieldBitBand }
            TXIE { RwRwRegFieldBitBand }
            WUPEN { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
            ADD10 { RwRwRegFieldBitBand }
            AUTOEND { RwRwRegFieldBitBand }
            HEAD10R { RwRwRegFieldBitBand }
            NACK { RwRwRegFieldBitBand }
            NBYTES { RwRwRegFieldBits }
            PECBYTE { RwRwRegFieldBitBand }
            RD_WRN { RwRwRegFieldBitBand }
            RELOAD { RwRwRegFieldBitBand }
            SADD { RwRwRegFieldBits }
            START { RwRwRegFieldBitBand }
            STOP { RwRwRegFieldBitBand }
        }
        OAR1 {
            0x20 RwRegBitBand;
            OA1EN { RwRwRegFieldBitBand }
            OA1MODE { RwRwRegFieldBitBand }
            OA1 { RwRwRegFieldBits }
        }
        OAR2 {
            0x20 RwRegBitBand;
            OA2EN { RwRwRegFieldBitBand }
            OA2MSK { RwRwRegFieldBits }
            OA2 { RwRwRegFieldBits }
        }
        TIMINGR {
            0x20 RwRegBitBand;
            PRESC { RwRwRegFieldBits }
            SCLDEL { RwRwRegFieldBits }
            SCLH { RwRwRegFieldBits }
            SCLL { RwRwRegFieldBits }
            SDADEL { RwRwRegFieldBits }
        }
        TIMEOUTR {
            0x20 RwRegBitBand;
            TEXTEN { RwRwRegFieldBitBand }
            TIDLE { RwRwRegFieldBitBand }
            TIMEOUTA { RwRwRegFieldBits }
            TIMEOUTB { RwRwRegFieldBits }
            TIMOUTEN { RwRwRegFieldBitBand }
        }
        ISR {
            0x20 RwRegBitBand;
            ADDCODE { RoRwRegFieldBits }
            ADDR { RoRwRegFieldBitBand }
            ALERT { RoRwRegFieldBitBand }
            ARLO { RoRwRegFieldBitBand }
            BERR { RoRwRegFieldBitBand }
            BUSY { RoRwRegFieldBitBand }
            DIR { RoRwRegFieldBitBand }
            NACKF { RoRwRegFieldBitBand }
            OVR { RoRwRegFieldBitBand }
            PECERR { RoRwRegFieldBitBand }
            RXNE { RoRwRegFieldBitBand }
            STOPF { RoRwRegFieldBitBand }
            TC { RoRwRegFieldBitBand }
            TCR { RoRwRegFieldBitBand }
            TIMEOUT { RoRwRegFieldBitBand }
            TXE { RwRwRegFieldBitBand }
            TXIS { RwRwRegFieldBitBand }
        }
        ICR {
            0x20 WoRegBitBand;
            ADDRCF { WoWoRegFieldBitBand }
            ALERTCF { WoWoRegFieldBitBand }
            ARLOCF { WoWoRegFieldBitBand }
            BERRCF { WoWoRegFieldBitBand }
            NACKCF { WoWoRegFieldBitBand }
            OVRCF { WoWoRegFieldBitBand }
            PECCF { WoWoRegFieldBitBand }
            STOPCF { WoWoRegFieldBitBand }
            TIMOUTCF { WoWoRegFieldBitBand }
        }
        PECR {
            0x20 RoRegBitBand;
            PEC { RoRoRegFieldBits }
        }
        RXDR {
            0x20 RoRegBitBand;
            RXDATA { RoRoRegFieldBits }
        }
        TXDR {
            0x20 RwRegBitBand;
            TXDATA { RwRwRegFieldBits }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_i2c {
    (
        $i2c_macro_doc:expr,
        $i2c_macro:ident,
        $i2c_ty_doc:expr,
        $i2c_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $ccipr:ident,
        $i2cen:ident,
        $i2crst:ident,
        $i2csmen:ident,
        $i2csel:ident,
        $i2c:ident,
    ) => {
        periph::map! {
            #[doc = $i2c_macro_doc]
            pub macro $i2c_macro;

            #[doc = $i2c_ty_doc]
            pub struct $i2c_ty;

            impl I2CMap for $i2c_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    I2CEN { $i2cen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    I2CRST { $i2crst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    I2CSMEN { $i2csmen }
                }
                CCIPR {
                    $ccipr Shared;
                    I2CSEL { $i2csel }
                }
            }
            I2C {
                $i2c;
                CR1 {
                    CR1;
                    ADDRIE { ADDRIE }
                    ALERTEN { ALERTEN }
                    ANFOFF { ANFOFF }
                    DNF { DNF }
                    ERRIE { ERRIE }
                    GCEN { GCEN }
                    NACKIE { NACKIE }
                    NOSTRETCH { NOSTRETCH }
                    PECEN { PECEN }
                    PE { PE }
                    RXDMAEN { RXDMAEN }
                    RXIE { RXIE }
                    SBC { SBC }
                    SMBDEN { SMBDEN }
                    SMBHEN { SMBHEN }
                    STOPIE { STOPIE }
                    TCIE { TCIE }
                    TXDMAEN { TXDMAEN }
                    TXIE { TXIE }
                    WUPEN { WUPEN }
                }
                CR2 {
                    CR2;
                    ADD10 { ADD10 }
                    AUTOEND { AUTOEND }
                    HEAD10R { HEAD10R }
                    NACK { NACK }
                    NBYTES { NBYTES }
                    PECBYTE { PECBYTE }
                    RD_WRN { RD_WRN }
                    RELOAD { RELOAD }
                    SADD { SADD }
                    START { START }
                    STOP { STOP }
                }
                OAR1 {
                    OAR1;
                    OA1EN { OA1EN }
                    OA1MODE { OA1MODE }
                    OA1 { OA1 }
                }
                OAR2 {
                    OAR2;
                    OA2EN { OA2EN }
                    OA2MSK { OA2MSK }
                    OA2 { OA2 }
                }
                TIMINGR {
                    TIMINGR;
                    PRESC { PRESC }
                    SCLDEL { SCLDEL }
                    SCLH { SCLH }
                    SCLL { SCLL }
                    SDADEL { SDADEL }
                }
                TIMEOUTR {
                    TIMEOUTR;
                    TEXTEN { TEXTEN }
                    TIDLE { TIDLE }
                    TIMEOUTA { TIMEOUTA }
                    TIMEOUTB { TIMEOUTB }
                    TIMOUTEN { TIMOUTEN }
                }
                ISR {
                    ISR;
                    ADDCODE { ADDCODE }
                    ADDR { ADDR }
                    ALERT { ALERT }
                    ARLO { ARLO }
                    BERR { BERR }
                    BUSY { BUSY }
                    DIR { DIR }
                    NACKF { NACKF }
                    OVR { OVR }
                    PECERR { PECERR }
                    RXNE { RXNE }
                    STOPF { STOPF }
                    TCR { TCR }
                    TC { TC }
                    TIMEOUT { TIMEOUT }
                    TXE { TXE }
                    TXIS { TXIS }
                }
                ICR {
                    ICR;
                    ADDRCF { ADDRCF }
                    ALERTCF { ALERTCF }
                    ARLOCF { ARLOCF }
                    BERRCF { BERRCF }
                    NACKCF { NACKCF }
                    OVRCF { OVRCF }
                    PECCF { PECCF }
                    STOPCF { STOPCF }
                    TIMOUTCF { TIMOUTCF }
                }
                PECR {
                    PECR;
                    PEC { PEC }
                }
                RXDR {
                    RXDR;
                    RXDATA { RXDATA }
                }
                TXDR {
                    TXDR;
                    TXDATA { TXDATA }
                }
            }
        }
    };
}

#[cfg(any(
    stm32_mcu = "stm32l4x1",
    stm32_mcu = "stm32l4x2",
    stm32_mcu = "stm32l4x3",
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_i2c! {
    "Extracts I2C1 register tokens.",
    periph_i2c1,
    "I2C1 peripheral variant.",
    I2C1,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    CCIPR,
    I2C1EN,
    I2C1RST,
    I2C1SMEN,
    I2C1SEL,
    I2C1,
}

#[cfg(any(
    stm32_mcu = "stm32l4x1",
    stm32_mcu = "stm32l4x2",
    stm32_mcu = "stm32l4x3",
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_i2c! {
    "Extracts I2C2 register tokens.",
    periph_i2c2,
    "I2C2 peripheral variant.",
    I2C2,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    CCIPR,
    I2C2EN,
    I2C2RST,
    I2C2SMEN,
    I2C2SEL,
    I2C2,
}

#[cfg(any(
    stm32_mcu = "stm32l4x1",
    stm32_mcu = "stm32l4x2",
    stm32_mcu = "stm32l4x3",
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_i2c! {
    "Extracts I2C3 register tokens.",
    periph_i2c3,
    "I2C3 peripheral variant.",
    I2C3,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    CCIPR,
    I2C3EN,
    I2C3RST,
    I2C3SMEN,
    I2C3SEL,
    I2C3,
}

#[cfg(any(
    stm32_mcu = "stm32l4x2",
    stm32_mcu = "stm32l4x6",
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_i2c! {
    "Extracts I2C4 register tokens.",
    periph_i2c4,
    "I2C4 peripheral variant.",
    I2C4,
    APB1ENR2,
    APB1RSTR2,
    APB1SMENR2,
    CCIPR2,
    I2C4EN,
    I2C4RST,
    I2C4SMEN,
    I2C4SEL,
    I2C4,
}

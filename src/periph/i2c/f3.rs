//! Inter-Integrated Circuit
//! for STM32F3 Series of mixed-signal MCUs with DSP and FPU instructions.

#[allow(unused_imports)]
use drone_core::periph;
#[allow(unused_imports)]
use drone_cortexm::reg::marker::*;

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
    }
    I2C {
        CR1 {
            0x20 RwRegBitBand;
            PE { RwRwRegFieldBitBand }
            TXIE { RwRwRegFieldBitBand }
            RXIE { RwRwRegFieldBitBand }
            ADDRIE { RwRwRegFieldBitBand }
            NACKIE { RwRwRegFieldBitBand }
            STOPIE { RwRwRegFieldBitBand }
            TCIE { RwRwRegFieldBitBand }
            ERRIE { RwRwRegFieldBitBand }
            DNF { RwRwRegFieldBits }
            ANFOFF { RwRwRegFieldBitBand }
            SWRST { RwRwRegFieldBitBand }
            TXDMAEN { RwRwRegFieldBitBand }
            RXDMAEN { RwRwRegFieldBitBand }
            SBC { RwRwRegFieldBitBand }
            NOSTRETCH { RwRwRegFieldBitBand }
            WUPEN { RwRwRegFieldBitBand }
            GCEN { RwRwRegFieldBitBand }
            SMBHEN { RwRwRegFieldBitBand }
            SMBDEN { RwRwRegFieldBitBand }
            ALERTEN { RwRwRegFieldBitBand }
            PECEN { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
            PECBYTE { RwRwRegFieldBitBand }
            AUTOEND { RwRwRegFieldBitBand }
            RELOAD { RwRwRegFieldBitBand }
            NBYTES { RwRwRegFieldBits }
            NACK { RwRwRegFieldBitBand }
            STOP { RwRwRegFieldBitBand }
            START { RwRwRegFieldBitBand }
            HEAD10R { RwRwRegFieldBitBand }
            ADD10 { RwRwRegFieldBitBand }
            RD_WRN { RwRwRegFieldBitBand }
            SADD8 { RwRwRegFieldBits }
            SADD1 { RwRwRegFieldBits }
            SADD0 { RwRwRegFieldBit }
        }
        OAR1 {
            0x20 RwRegBitBand;
            OA1_0 { RwRwRegFieldBit }
            OA1_1 { RwRwRegFieldBits }
            OA1_8 { RwRwRegFieldBits }
            OA1MODE { RwRwRegFieldBitBand }
            OA1EN { RwRwRegFieldBitBand }
        }
        OAR2 {
            0x20 RwRegBitBand;
            OA2 { RwRwRegFieldBits }
            OA2MSK { RwRwRegFieldBits }
            OA2EN { RwRwRegFieldBit }
        }
        TIMINGR {
            0x20 RwRegBitBand;
            SCLL { RwRwRegFieldBits }
            SCLH { RwRwRegFieldBits }
            SDADEL { RwRwRegFieldBits }
            SCLDEL { RwRwRegFieldBits }
            PRESC { RwRwRegFieldBits }
        }
        TIMEOUTR {
            0x20 RwRegBitBand;
            TIMEOUTA { RwRwRegFieldBits }
            TIDLE { RwRwRegFieldBitBand }
            TIMOUTEN { RwRwRegFieldBitBand }
            TIMEOUTB { RwRwRegFieldBits }
            TEXTEN { RwRwRegFieldBitBand }
        }
        ISR {
            0x20 RwRegBitBand;
            ADDCODE { RoRwRegFieldBits }
            DIR { RoRwRegFieldBitBand }
            BUSY { RoRwRegFieldBitBand }
            ALERT { RoRwRegFieldBitBand }
            TIMEOUT { RoRwRegFieldBitBand }
            PECERR { RoRwRegFieldBitBand }
            OVR { RoRwRegFieldBitBand }
            ARLO { RoRwRegFieldBitBand }
            BERR { RoRwRegFieldBitBand }
            TCR { RoRwRegFieldBitBand }
            TC { RoRwRegFieldBitBand }
            STOPF { RoRwRegFieldBitBand }
            NACKF { RoRwRegFieldBitBand }
            ADDR { RoRwRegFieldBitBand }
            RXNE { RoRwRegFieldBitBand }
            TXIS { RwRwRegFieldBitBand }
            TXE { RwRwRegFieldBitBand }
        }
        ICR {
            0x20 WoReg;
            ALERTCF { WoWoRegFieldBit }
            TIMOUTCF { WoWoRegFieldBit }
            PECCF { WoWoRegFieldBit }
            OVRCF { WoWoRegFieldBit }
            ARLOCF { WoWoRegFieldBit }
            BERRCF { WoWoRegFieldBit }
            STOPCF { WoWoRegFieldBit }
            NACKCF { WoWoRegFieldBit }
            ADDRCF { WoWoRegFieldBit }
        }
        PECR {
            0x20 RoReg;
            PEC { RoRoRegFieldBits }
        }
        RXDR {
            0x20 RoReg;
            RXDATA { RoRoRegFieldBits }
        }
        TXDR {
            0x20 RwReg;
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
        $i2c:ident,
        $i2cen:ident,
        $i2crst:ident,
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
            }
            I2C {
                $i2c;
                CR1 {
                    CR1;
                    PE { PE }
                    TXIE { TXIE }
                    RXIE { RXIE }
                    ADDRIE { ADDRIE }
                    NACKIE { NACKIE }
                    STOPIE { STOPIE }
                    TCIE { TCIE }
                    ERRIE { ERRIE }
                    DNF { DNF }
                    ANFOFF { ANFOFF }
                    SWRST { SWRST }
                    TXDMAEN { TXDMAEN }
                    RXDMAEN { RXDMAEN }
                    SBC { SBC }
                    NOSTRETCH { NOSTRETCH }
                    WUPEN { WUPEN }
                    GCEN { GCEN }
                    SMBHEN { SMBHEN }
                    SMBDEN { SMBDEN }
                    ALERTEN { ALERTEN }
                    PECEN { PECEN }
                }
                CR2 {
                    CR2;
                    PECBYTE { PECBYTE }
                    AUTOEND { AUTOEND }
                    RELOAD { RELOAD }
                    NBYTES { NBYTES }
                    NACK { NACK }
                    STOP { STOP }
                    START { START }
                    HEAD10R { HEAD10R }
                    ADD10 { ADD10 }
                    RD_WRN { RD_WRN }
                    SADD8 { SADD8 }
                    SADD1 { SADD1 }
                    SADD0 { SADD0 }
                }
                OAR1 {
                    OAR1;
                    OA1_0 { OA1_0 }
                    OA1_1 { OA1_1 }
                    OA1_8 { OA1_8 }
                    OA1MODE { OA1MODE }
                    OA1EN { OA1EN }
                }
                OAR2 {
                    OAR2;
                    OA2 { OA2 }
                    OA2MSK { OA2MSK }
                    OA2EN { OA2EN }
                }
                TIMINGR {
                    TIMINGR;
                    SCLL { SCLL }
                    SCLH { SCLH }
                    SDADEL { SDADEL }
                    SCLDEL { SCLDEL }
                    PRESC { PRESC }
                }
                TIMEOUTR {
                    TIMEOUTR;
                    TIMEOUTA { TIMEOUTA }
                    TIDLE { TIDLE }
                    TIMOUTEN { TIMOUTEN }
                    TIMEOUTB { TIMEOUTB }
                    TEXTEN { TEXTEN }
                }
                ISR {
                    ISR;
                    ADDCODE { ADDCODE }
                    DIR { DIR }
                    BUSY { BUSY }
                    ALERT { ALERT }
                    TIMEOUT { TIMEOUT }
                    PECERR { PECERR }
                    OVR { OVR }
                    ARLO { ARLO }
                    BERR { BERR }
                    TCR { TCR }
                    TC { TC }
                    STOPF { STOPF }
                    NACKF { NACKF }
                    ADDR { ADDR }
                    RXNE { RXNE }
                    TXIS { TXIS }
                    TXE { TXE }
                }
                ICR {
                    ICR;
                    ALERTCF { ALERTCF }
                    TIMOUTCF { TIMOUTCF }
                    PECCF { PECCF }
                    OVRCF { OVRCF }
                    ARLOCF { ARLOCF }
                    BERRCF { BERRCF }
                    STOPCF { STOPCF }
                    NACKCF { NACKCF }
                    ADDRCF { ADDRCF }
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

map_i2c! {
    "Extracts I2C1 register tokens.",
    periph_i2c1,
    "I2C1 peripheral variant.",
    I2C1,
    APB1ENR,
    APB1RSTR,
    I2C1,
    I2C1EN,
    I2C1RST,
}

map_i2c! {
    "Extracts I2C2 register tokens.",
    periph_i2c2,
    "I2C2 peripheral variant.",
    I2C2,
    APB1ENR,
    APB1RSTR,
    I2C2,
    I2C2EN,
    I2C2RST,
}

map_i2c! {
    "Extracts I2C3 register tokens.",
    periph_i2c3,
    "I2C3 peripheral variant.",
    I2C3,
    APB1ENR,
    APB1RSTR,
    I2C3,
    I2C3EN,
    I2C3RST,
}

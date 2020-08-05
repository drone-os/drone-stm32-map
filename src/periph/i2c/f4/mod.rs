//! Inter-Integrated Circuit.
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

#[cfg(any(stm32_mcu = "stm32f410", stm32_mcu = "stm32f412", stm32_mcu = "stm32f413"))]
pub mod fmp;

use drone_core::periph;
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
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            I2CSMEN { RwRwRegFieldBitBand }
        }
    }

    I2C {
        CR1 {
            0x20 RwRegBitBand;
            PE { RwRwRegFieldBitBand }
            SMBUS { RwRwRegFieldBitBand }
            SMBTYPE { RwRwRegFieldBitBand }
            ENARP { RwRwRegFieldBitBand }
            ENPEC { RwRwRegFieldBitBand }
            ENGC { RwRwRegFieldBitBand }
            NOSTRETCH { RwRwRegFieldBitBand }
            START { RwRwRegFieldBitBand }
            STOP { RwRwRegFieldBitBand }
            ACK { RwRwRegFieldBitBand }
            POS { RwRwRegFieldBitBand }
            PEC { RwRwRegFieldBitBand }
            ALERT { RwRwRegFieldBitBand }
            SWRST { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
            FREQ { RwRwRegFieldBits }
            ITERREN { RwRwRegFieldBitBand }
            ITEVTEN { RwRwRegFieldBitBand }
            ITBUFEN { RwRwRegFieldBitBand }
            DMAEN { RwRwRegFieldBitBand }
            LAST { RwRwRegFieldBitBand }
        }
        OAR1 {
            0x20 RwRegBitBand;
            ADD0 { RwRwRegFieldBitBand }
            ADD7 { RwRwRegFieldBits }
            ADD10 { RwRwRegFieldBits }
            ADDMODE { RwRwRegFieldBitBand }
        }
        OAR2 {
            0x20 RwRegBitBand;
            ENDUAL { RwRwRegFieldBitBand }
            ADD2 { RwRwRegFieldBits }
        }
        DR {
            0x20 RwRegBitBand;
            DR { RwRwRegFieldBits }
        }
        SR1 {
            0x20 RwRegBitBand;
            SB { RoRwRegFieldBitBand }
            ADDR { RoRwRegFieldBitBand }
            BTF { RoRwRegFieldBitBand }
            ADD10 { RoRwRegFieldBitBand }
            STOPF { RoRwRegFieldBitBand }
            RxNE { RoRwRegFieldBitBand }
            TxE { RoRwRegFieldBitBand }
            BERR { RwRwRegFieldBitBand }
            ARLO { RwRwRegFieldBitBand }
            AF { RwRwRegFieldBitBand }
            OVR { RwRwRegFieldBitBand }
            PECERR { RwRwRegFieldBitBand }
            TIMEOUT { RwRwRegFieldBitBand }
            SMBALERT { RwRwRegFieldBitBand }
        }
        SR2 {
            0x20 RoRegBitBand;
            MSL { RoRoRegFieldBitBand }
            BUSY { RoRoRegFieldBitBand }
            TRA { RoRoRegFieldBitBand }
            GENCALL { RoRoRegFieldBitBand }
            SMBDEFAULT { RoRoRegFieldBitBand }
            SMBHOST { RoRoRegFieldBitBand }
            DUALF { RoRoRegFieldBitBand }
            PEC { RoRoRegFieldBits }
        }
        CCR {
            0x20 RwRegBitBand;
            CCR { RwRwRegFieldBits }
            DUTY { RwRwRegFieldBitBand }
            F_S { RwRwRegFieldBitBand }
        }
        TRISE {
            0x20 RwRegBitBand;
            TRISE { RwRwRegFieldBits }
        }
        FLTR {
            0x20 RwRegBitBand;
            DNF { RwRwRegFieldBits }
            ANOFF { RwRwRegFieldBitBand }
        }
    }
}

macro_rules! map_i2c {
    (
        $i2c_macro_doc:expr,
        $i2c_macro:ident,
        $i2c_ty_doc:expr,
        $i2c_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $i2c:ident,
        $i2cen:ident,
        $i2crst:ident,
        $i2csmen:ident,
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
            }

            I2C {
                $i2c;
                CR1 {
                    CR1;
                    PE { PE }
                    SMBUS { SMBUS }
                    SMBTYPE { SMBTYPE }
                    ENARP { ENARP }
                    ENPEC { ENPEC }
                    ENGC { ENGC }
                    NOSTRETCH { NOSTRETCH }
                    START { START }
                    STOP { STOP }
                    ACK { ACK }
                    POS { POS }
                    PEC { PEC }
                    ALERT { ALERT }
                    SWRST { SWRST }
                }
                CR2 {
                    CR2;
                    FREQ { FREQ }
                    ITERREN { ITERREN }
                    ITEVTEN { ITEVTEN }
                    ITBUFEN { ITBUFEN }
                    DMAEN { DMAEN }
                    LAST { LAST }
                }
                OAR1 {
                    OAR1;
                    ADD0 { ADD0 }
                    ADD7 { ADD7 }
                    ADD10 { ADD10 }
                    ADDMODE { ADDMODE }
                }
                OAR2 {
                    OAR2;
                    ENDUAL { ENDUAL }
                    ADD2 { ADD2 }
                }
                DR {
                    DR;
                    DR { DR }
                }
                SR1 {
                    SR1;
                    SB { SB }
                    ADDR { ADDR }
                    BTF { BTF }
                    ADD10 { ADD10 }
                    STOPF { STOPF }
                    RxNE { RxNE }
                    TxE { TxE }
                    BERR { BERR }
                    ARLO { ARLO }
                    AF { AF }
                    OVR { OVR }
                    PECERR { PECERR }
                    TIMEOUT { TIMEOUT }
                    SMBALERT { SMBALERT }
                }
                SR2 {
                    SR2;
                    MSL { MSL }
                    BUSY { BUSY }
                    TRA { TRA }
                    GENCALL { GENCALL }
                    SMBDEFAULT { SMBDEFAULT }
                    SMBHOST { SMBHOST }
                    DUALF { DUALF }
                    PEC { PEC }
                }
                CCR {
                    CCR;
                    CCR { CCR }
                    DUTY { DUTY }
                    F_S { F_S }
                }
                TRISE {
                    TRISE;
                    TRISE { TRISE }
                }
                FLTR {
                    FLTR;
                    DNF { DNF }
                    ANOFF { ANOFF }
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
    APB1LPENR,
    I2C1,
    I2C1EN,
    I2C1RST,
    I2C1LPEN,
}

map_i2c! {
    "Extracts I2C2 register tokens.",
    periph_i2c2,
    "I2C2 peripheral variant.",
    I2C2,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    I2C2,
    I2C2EN,
    I2C2RST,
    I2C2LPEN,
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
    stm32_mcu = "stm32f469"
))]
map_i2c! {
    "Extracts I2C3 register tokens.",
    periph_i2c3,
    "I2C3 peripheral variant.",
    I2C3,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    I2C3,
    I2C3EN,
    I2C3RST,
    I2C3LPEN,
}

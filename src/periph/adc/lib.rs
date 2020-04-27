//! Analog-to-digital converters.

#![feature(proc_macro_hygiene)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds, clippy::wildcard_imports)]
#![no_std]

pub mod com;

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic ADC peripheral variant.
    pub trait AdcMap {}

    /// Generic ADC peripheral.
    pub struct AdcPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            ADCEN { RwRwRegFieldBitBand }
        }
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            ADCSMEN { RwRwRegFieldBitBand }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        CCIPR {
            0x20 RwRegBitBand Shared;
            ADCSEL { RwRwRegFieldBits }
        }
    }
    ADC {
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
            stm32_mcu = "stm32f469"
        ))]
        SR {
            0x20 RwReg;
            OVR { RwRwRegFieldBit }
            STRT { RwRwRegFieldBit }
            JSTRT { RwRwRegFieldBit }
            JEOC { RwRwRegFieldBit }
            EOC { RwRwRegFieldBit }
            AWD { RwRwRegFieldBit }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        ISR {
            0x20 RwReg;
            ADRDY { RwRwRegFieldBit }
            AWD1 { RwRwRegFieldBit }
            AWD2 { RwRwRegFieldBit }
            AWD3 { RwRwRegFieldBit }
            EOC { RwRwRegFieldBit }
            EOSMP { RwRwRegFieldBit }
            EOS { RwRwRegFieldBit }
            JEOC { RwRwRegFieldBit }
            JEOS { RwRwRegFieldBit }
            JQOVF { RwRwRegFieldBit }
            OVR { RwRwRegFieldBit }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        IER {
            0x20 RwReg;
            ADRDYIE { RwRwRegFieldBit }
            AWD1IE { RwRwRegFieldBit }
            AWD2IE { RwRwRegFieldBit }
            AWD3IE { RwRwRegFieldBit }
            EOCIE { RwRwRegFieldBit }
            EOSIE { RwRwRegFieldBit }
            EOSMPIE { RwRwRegFieldBit }
            JEOCIE { RwRwRegFieldBit }
            JEOSIE { RwRwRegFieldBit }
            JQOVFIE { RwRwRegFieldBit }
            OVRIE { RwRwRegFieldBit }
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
            stm32_mcu = "stm32f469"
        ))]
        CR1 {
            0x20 RwReg;
            OVRIE { RwRwRegFieldBit }
            RES { RwRwRegFieldBits }
            AWDEN { RwRwRegFieldBit }
            JAWDEN { RwRwRegFieldBit }
            DISCNUM { RwRwRegFieldBits }
            JDISCEN { RwRwRegFieldBit }
            DISCEN { RwRwRegFieldBit }
            JAUTO { RwRwRegFieldBit }
            AWDSGL { RwRwRegFieldBit }
            SCAN { RwRwRegFieldBit }
            JEOCIE { RwRwRegFieldBit }
            AWDIE { RwRwRegFieldBit }
            EOCIE { RwRwRegFieldBit }
            AWDCH { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        CR2 {
            0x20 RwReg;
            SWSTART { RwRwRegFieldBit }
            EXTEN { RwRwRegFieldBits }
            EXTSEL { RwRwRegFieldBits }
            JSWSTART { RwRwRegFieldBit }
            JEXTEN { RwRwRegFieldBits }
            JEXTSEL { RwRwRegFieldBits }
            ALIGN { RwRwRegFieldBit }
            EOCS { RwRwRegFieldBit }
            DDS { RwRwRegFieldBit }
            DMA { RwRwRegFieldBit }
            CONT { RwRwRegFieldBit }
            ADON { RwRwRegFieldBit }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        CR {
            0x20 RwReg;
            ADCALDIF { RwRwRegFieldBit }
            ADCAL { RwRwRegFieldBit }
            ADDIS { RwRwRegFieldBit }
            ADEN { RwRwRegFieldBit }
            ADSTART { RwRwRegFieldBit }
            ADSTP { RwRwRegFieldBit }
            ADVREGEN { RwRwRegFieldBit }
            DEEPPWD { RwRwRegFieldBit }
            JADSTART { RwRwRegFieldBit }
            JADSTP { RwRwRegFieldBit }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        CFGR {
            0x20 RwReg;
            ALIGN { RwRwRegFieldBit }
            AUTDLY { RwRwRegFieldBit }
            AWD1EN { RwRwRegFieldBit }
            AWD1SGL { RwRwRegFieldBit }
            AWDCH1CH { RwRwRegFieldBits }
            CONT { RwRwRegFieldBit }
            DISCEN { RwRwRegFieldBit }
            DISCNUM { RwRwRegFieldBits }
            DMACFG { RwRwRegFieldBit }
            DMAEN { RwRwRegFieldBit }
            EXTEN { RwRwRegFieldBits }
            EXTSEL { RwRwRegFieldBits }
            JAUTO { RwRwRegFieldBit }
            JAWD1EN { RwRwRegFieldBit }
            JDISCEN { RwRwRegFieldBit }
            JQDIS { RwRwRegFieldBit }
            JQM { RwRwRegFieldBit }
            OVRMOD { RwRwRegFieldBit }
            RES { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        CFGR2 {
            0x20 RwReg;
            JOVSE { RwRwRegFieldBit }
            OVSR { RwRwRegFieldBits }
            OVSS { RwRwRegFieldBits }
            ROVSE { RwRwRegFieldBit }
            ROVSM { RwRwRegFieldBit }
            TROVS { RwRwRegFieldBit }
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
            stm32_mcu = "stm32f469"
        ))]
        SMPR1 {
            0x20 RwReg;
            SMP18 { RwRwRegFieldBits }
            SMP17 { RwRwRegFieldBits }
            SMP16 { RwRwRegFieldBits }
            SMP15 { RwRwRegFieldBits }
            SMP14 { RwRwRegFieldBits }
            SMP13 { RwRwRegFieldBits }
            SMP12 { RwRwRegFieldBits }
            SMP11 { RwRwRegFieldBits }
            SMP10 { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        SMPR2 {
            0x20 RwReg;
            SMP9 { RwRwRegFieldBits }
            SMP8 { RwRwRegFieldBits }
            SMP7 { RwRwRegFieldBits }
            SMP6 { RwRwRegFieldBits }
            SMP5 { RwRwRegFieldBits }
            SMP4 { RwRwRegFieldBits }
            SMP3 { RwRwRegFieldBits }
            SMP2 { RwRwRegFieldBits }
            SMP1 { RwRwRegFieldBits }
            SMP0 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        SMPR1 {
            0x20 RwReg;
            SMP0 { RwRwRegFieldBits }
            SMP1 { RwRwRegFieldBits }
            SMP2 { RwRwRegFieldBits }
            SMP3 { RwRwRegFieldBits }
            SMP4 { RwRwRegFieldBits }
            SMP5 { RwRwRegFieldBits }
            SMP6 { RwRwRegFieldBits }
            SMP7 { RwRwRegFieldBits }
            SMP8 { RwRwRegFieldBits }
            SMP9 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        SMPR2 {
            0x20 RwReg;
            SMP10 { RwRwRegFieldBits }
            SMP11 { RwRwRegFieldBits }
            SMP12 { RwRwRegFieldBits }
            SMP13 { RwRwRegFieldBits }
            SMP14 { RwRwRegFieldBits }
            SMP15 { RwRwRegFieldBits }
            SMP16 { RwRwRegFieldBits }
            SMP17 { RwRwRegFieldBits }
            SMP18 { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        JOFR1 {
            0x20 RwReg;
            JOFFSET1 { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        JOFR2 {
            0x20 RwReg;
            JOFFSET2 { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        JOFR3 {
            0x20 RwReg;
            JOFFSET3 { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        JOFR4 {
            0x20 RwReg;
            JOFFSET4 { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        HTR {
            0x20 RwReg;
            HT { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        LTR {
            0x20 RwReg;
            LT { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        TR1 {
            0x20 RwReg;
            HT1 { RwRwRegFieldBits }
            LT1 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        TR2 {
            0x20 RwReg;
            HT2 { RwRwRegFieldBits }
            LT2 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        TR3 {
            0x20 RwReg;
            HT3 { RwRwRegFieldBits }
            LT3 { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        SQR1 {
            0x20 RwReg;
            L { RwRwRegFieldBits }
            SQ16 { RwRwRegFieldBits }
            SQ15 { RwRwRegFieldBits }
            SQ14 { RwRwRegFieldBits }
            SQ13 { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        SQR2 {
            0x20 RwReg;
            SQ12 { RwRwRegFieldBits }
            SQ11 { RwRwRegFieldBits }
            SQ10 { RwRwRegFieldBits }
            SQ9 { RwRwRegFieldBits }
            SQ8 { RwRwRegFieldBits }
            SQ7 { RwRwRegFieldBits }
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
            stm32_mcu = "stm32f469"
        ))]
        SQR3 {
            0x20 RwReg;
            SQ6 { RwRwRegFieldBits }
            SQ5 { RwRwRegFieldBits }
            SQ4 { RwRwRegFieldBits }
            SQ3 { RwRwRegFieldBits }
            SQ2 { RwRwRegFieldBits }
            SQ1 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        SQR1 {
            0x20 RwReg;
            L { RwRwRegFieldBits }
            SQ1 { RwRwRegFieldBits }
            SQ2 { RwRwRegFieldBits }
            SQ3 { RwRwRegFieldBits }
            SQ4 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        SQR2 {
            0x20 RwReg;
            SQ5 { RwRwRegFieldBits }
            SQ6 { RwRwRegFieldBits }
            SQ7 { RwRwRegFieldBits }
            SQ8 { RwRwRegFieldBits }
            SQ9 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        SQR3 {
            0x20 RwReg;
            SQ10 { RwRwRegFieldBits }
            SQ11 { RwRwRegFieldBits }
            SQ12 { RwRwRegFieldBits }
            SQ13 { RwRwRegFieldBits }
            SQ14 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        SQR4 {
            0x20 RwReg;
            SQ15 { RwRwRegFieldBits }
            SQ16 { RwRwRegFieldBits }
        }
        DR {
            0x20 RoReg;
            RDATA { RoRoRegFieldBits }
        }
        JSQR {
            0x20 RwReg;
            #[cfg(any(
                stm32_mcu = "stm32l4r5",
                stm32_mcu = "stm32l4r7",
                stm32_mcu = "stm32l4r9",
                stm32_mcu = "stm32l4s5",
                stm32_mcu = "stm32l4s7",
                stm32_mcu = "stm32l4s9"
            ))]
            JEXTEN { RwRwRegFieldBits }
            #[cfg(any(
                stm32_mcu = "stm32l4r5",
                stm32_mcu = "stm32l4r7",
                stm32_mcu = "stm32l4r9",
                stm32_mcu = "stm32l4s5",
                stm32_mcu = "stm32l4s7",
                stm32_mcu = "stm32l4s9"
            ))]
            JEXTSEL { RwRwRegFieldBits }
            JL { RwRwRegFieldBits }
            JSQ1 { RwRwRegFieldBits }
            JSQ2 { RwRwRegFieldBits }
            JSQ3 { RwRwRegFieldBits }
            JSQ4 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        OFR1 {
            0x20 RwReg;
            OFFSET1_CH { RwRwRegFieldBits }
            OFFSET1_EN { RwRwRegFieldBit }
            OFFSET1 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        OFR2 {
            0x20 RwReg;
            OFFSET2_CH { RwRwRegFieldBits }
            OFFSET2_EN { RwRwRegFieldBit }
            OFFSET2 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        OFR3 {
            0x20 RwReg;
            OFFSET3_CH { RwRwRegFieldBits }
            OFFSET3_EN { RwRwRegFieldBit }
            OFFSET3 { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        OFR4 {
            0x20 RwReg;
            OFFSET4_CH { RwRwRegFieldBits }
            OFFSET4_EN { RwRwRegFieldBit }
            OFFSET4 { RwRwRegFieldBits }
        }
        JDR1 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        JDR2 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        JDR3 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        JDR4 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        AWD2CR {
            0x20 RwReg;
            AWD2CH { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        AWD3CR {
            0x20 RwReg;
            AWD3CH { RwRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        DIFSEL {
            0x20 RwReg;
            DIFSEL_1_15 { RwRwRegFieldBits }
            DIFSEL_16_18 { RoRwRegFieldBits }
        }
        #[cfg(any(
            stm32_mcu = "stm32l4r5",
            stm32_mcu = "stm32l4r7",
            stm32_mcu = "stm32l4r9",
            stm32_mcu = "stm32l4s5",
            stm32_mcu = "stm32l4s7",
            stm32_mcu = "stm32l4s9"
        ))]
        CALFACT {
            0x20 RwReg;
            CALFACT_D { RwRwRegFieldBits }
            CALFACT_S { RwRwRegFieldBits }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_adc {
    (
        $adc_macro_doc:expr,
        $adc_macro:ident,
        $adc_ty_doc:expr,
        $adc_ty:ident,
        $adc:ident,
        $busenr:ident,
        $bussmenr:ident,
        $adcen:ident,
        $adcsmen:ident,
        $rdata:ident,
    ) => {
        periph::map! {
            #[doc = $adc_macro_doc]
            pub macro $adc_macro;

            #[doc = $adc_ty_doc]
            pub struct $adc_ty;

            impl AdcMap for $adc_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    ADCEN { $adcen }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    ADCSMEN { $adcsmen }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                CCIPR {
                    CCIPR Shared;
                    ADCSEL { ADCSEL }
                }
            }
            ADC {
                $adc;
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
                    stm32_mcu = "stm32f469"
                ))]
                SR {
                    SR;
                    OVR { OVR }
                    STRT { STRT }
                    JSTRT { JSTRT }
                    JEOC { JEOC }
                    EOC { EOC }
                    AWD { AWD }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                ISR {
                    ISR;
                    ADRDY { ADRDY }
                    AWD1 { AWD1 }
                    AWD2 { AWD2 }
                    AWD3 { AWD3 }
                    EOC { EOC }
                    EOSMP { EOSMP }
                    EOS { EOS }
                    JEOC { JEOC }
                    JEOS { JEOS }
                    JQOVF { JQOVF }
                    OVR { OVR }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                IER {
                    IER;
                    ADRDYIE { ADRDYIE }
                    AWD1IE { AWD1IE }
                    AWD2IE { AWD2IE }
                    AWD3IE { AWD3IE }
                    EOCIE { EOCIE }
                    EOSIE { EOSIE }
                    EOSMPIE { EOSMPIE }
                    JEOCIE { JEOCIE }
                    JEOSIE { JEOSIE }
                    JQOVFIE { JQOVFIE }
                    OVRIE { OVRIE }
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
                    stm32_mcu = "stm32f469"
                ))]
                CR1 {
                    CR1;
                    OVRIE { OVRIE }
                    RES { RES }
                    AWDEN { AWDEN }
                    JAWDEN { JAWDEN }
                    DISCNUM { DISCNUM }
                    JDISCEN { JDISCEN }
                    DISCEN { DISCEN }
                    JAUTO { JAUTO }
                    AWDSGL { AWDSGL }
                    SCAN { SCAN }
                    JEOCIE { JEOCIE }
                    AWDIE { AWDIE }
                    EOCIE { EOCIE }
                    AWDCH { AWDCH }
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
                    stm32_mcu = "stm32f469"
                ))]
                CR2 {
                    CR2;
                    SWSTART { SWSTART }
                    EXTEN { EXTEN }
                    EXTSEL { EXTSEL }
                    JSWSTART { JSWSTART }
                    JEXTEN { JEXTEN }
                    JEXTSEL { JEXTSEL }
                    ALIGN { ALIGN }
                    EOCS { EOCS }
                    DDS { DDS }
                    DMA { DMA }
                    CONT { CONT }
                    ADON { ADON }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                CR {
                    CR;
                    ADCALDIF { ADCALDIF }
                    ADCAL { ADCAL }
                    ADDIS { ADDIS }
                    ADEN { ADEN }
                    ADSTART { ADSTART }
                    ADSTP { ADSTP }
                    ADVREGEN { ADVREGEN }
                    DEEPPWD { DEEPPWD }
                    JADSTART { JADSTART }
                    JADSTP { JADSTP }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                CFGR {
                    CFGR;
                    ALIGN { ALIGN }
                    AUTDLY { AUTDLY }
                    AWD1EN { AWD1EN }
                    AWD1SGL { AWD1SGL }
                    AWDCH1CH { AWDCH1CH }
                    CONT { CONT }
                    DISCEN { DISCEN }
                    DISCNUM { DISCNUM }
                    DMACFG { DMACFG }
                    DMAEN { DMAEN }
                    EXTEN { EXTEN }
                    EXTSEL { EXTSEL }
                    JAUTO { JAUTO }
                    JAWD1EN { JAWD1EN }
                    JDISCEN { JDISCEN }
                    JQDIS { JQDIS }
                    JQM { JQM }
                    OVRMOD { OVRMOD }
                    RES { RES }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                CFGR2 {
                    CFGR2;
                    JOVSE { JOVSE }
                    OVSR { OVSR }
                    OVSS { OVSS }
                    ROVSE { ROVSE }
                    ROVSM { ROVSM }
                    TROVS { TROVS }
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
                    stm32_mcu = "stm32f469"
                ))]
                SMPR1 {
                    SMPR1;
                    SMP18 { SMP18 }
                    SMP17 { SMP17 }
                    SMP16 { SMP16 }
                    SMP15 { SMP15 }
                    SMP14 { SMP14 }
                    SMP13 { SMP13 }
                    SMP12 { SMP12 }
                    SMP11 { SMP11 }
                    SMP10 { SMP10 }
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
                    stm32_mcu = "stm32f469"
                ))]
                SMPR2 {
                    SMPR2;
                    SMP9 { SMP9 }
                    SMP8 { SMP8 }
                    SMP7 { SMP7 }
                    SMP6 { SMP6 }
                    SMP5 { SMP5 }
                    SMP4 { SMP4 }
                    SMP3 { SMP3 }
                    SMP2 { SMP2 }
                    SMP1 { SMP1 }
                    SMP0 { SMP0 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                SMPR1 {
                    SMPR1;
                    SMP0 { SMP0 }
                    SMP1 { SMP1 }
                    SMP2 { SMP2 }
                    SMP3 { SMP3 }
                    SMP4 { SMP4 }
                    SMP5 { SMP5 }
                    SMP6 { SMP6 }
                    SMP7 { SMP7 }
                    SMP8 { SMP8 }
                    SMP9 { SMP9 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                SMPR2 {
                    SMPR2;
                    SMP10 { SMP10 }
                    SMP11 { SMP11 }
                    SMP12 { SMP12 }
                    SMP13 { SMP13 }
                    SMP14 { SMP14 }
                    SMP15 { SMP15 }
                    SMP16 { SMP16 }
                    SMP17 { SMP17 }
                    SMP18 { SMP18 }
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
                    stm32_mcu = "stm32f469"
                ))]
                JOFR1 {
                    JOFR1;
                    JOFFSET1 { JOFFSET1 }
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
                    stm32_mcu = "stm32f469"
                ))]
                JOFR2 {
                    JOFR2;
                    JOFFSET2 { JOFFSET2 }
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
                    stm32_mcu = "stm32f469"
                ))]
                JOFR3 {
                    JOFR3;
                    JOFFSET3 { JOFFSET3 }
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
                    stm32_mcu = "stm32f469"
                ))]
                JOFR4 {
                    JOFR4;
                    JOFFSET4 { JOFFSET4 }
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
                    stm32_mcu = "stm32f469"
                ))]
                HTR {
                    HTR;
                    HT { HT }
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
                    stm32_mcu = "stm32f469"
                ))]
                LTR {
                    LTR;
                    LT { LT }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                TR1 {
                    TR1;
                    HT1 { HT1 }
                    LT1 { LT1 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                TR2 {
                    TR2;
                    HT2 { HT2 }
                    LT2 { LT2 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                TR3 {
                    TR3;
                    HT3 { HT3 }
                    LT3 { LT3 }
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
                    stm32_mcu = "stm32f469"
                ))]
                SQR1 {
                    SQR1;
                    L { L }
                    SQ16 { SQ16 }
                    SQ15 { SQ15 }
                    SQ14 { SQ14 }
                    SQ13 { SQ13 }
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
                    stm32_mcu = "stm32f469"
                ))]
                SQR2 {
                    SQR2;
                    SQ12 { SQ12 }
                    SQ11 { SQ11 }
                    SQ10 { SQ10 }
                    SQ9 { SQ9 }
                    SQ8 { SQ8 }
                    SQ7 { SQ7 }
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
                    stm32_mcu = "stm32f469"
                ))]
                SQR3 {
                    SQR3;
                    SQ6 { SQ6 }
                    SQ5 { SQ5 }
                    SQ4 { SQ4 }
                    SQ3 { SQ3 }
                    SQ2 { SQ2 }
                    SQ1 { SQ1 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                SQR1 {
                    SQR1;
                    L { L }
                    SQ1 { SQ1 }
                    SQ2 { SQ2 }
                    SQ3 { SQ3 }
                    SQ4 { SQ4 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                SQR2 {
                    SQR2;
                    SQ5 { SQ5 }
                    SQ6 { SQ6 }
                    SQ7 { SQ7 }
                    SQ8 { SQ8 }
                    SQ9 { SQ9 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                SQR3 {
                    SQR3;
                    SQ10 { SQ10 }
                    SQ11 { SQ11 }
                    SQ12 { SQ12 }
                    SQ13 { SQ13 }
                    SQ14 { SQ14 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                SQR4 {
                    SQR4;
                    SQ15 { SQ15 }
                    SQ16 { SQ16 }
                }
                DR {
                    DR;
                    RDATA { $rdata }
                }
                JSQR {
                    JSQR;
                    #[cfg(any(
                        stm32_mcu = "stm32l4r5",
                        stm32_mcu = "stm32l4r7",
                        stm32_mcu = "stm32l4r9",
                        stm32_mcu = "stm32l4s5",
                        stm32_mcu = "stm32l4s7",
                        stm32_mcu = "stm32l4s9"
                    ))]
                    JEXTEN { JEXTEN }
                    #[cfg(any(
                        stm32_mcu = "stm32l4r5",
                        stm32_mcu = "stm32l4r7",
                        stm32_mcu = "stm32l4r9",
                        stm32_mcu = "stm32l4s5",
                        stm32_mcu = "stm32l4s7",
                        stm32_mcu = "stm32l4s9"
                    ))]
                    JEXTSEL { JEXTSEL }
                    JL { JL }
                    JSQ1 { JSQ1 }
                    JSQ2 { JSQ2 }
                    JSQ3 { JSQ3 }
                    JSQ4 { JSQ4 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                OFR1 {
                    OFR1;
                    OFFSET1_CH { OFFSET1_CH }
                    OFFSET1_EN { OFFSET1_EN }
                    OFFSET1 { OFFSET1 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                OFR2 {
                    OFR2;
                    OFFSET2_CH { OFFSET2_CH }
                    OFFSET2_EN { OFFSET2_EN }
                    OFFSET2 { OFFSET2 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                OFR3 {
                    OFR3;
                    OFFSET3_CH { OFFSET3_CH }
                    OFFSET3_EN { OFFSET3_EN }
                    OFFSET3 { OFFSET3 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                OFR4 {
                    OFR4;
                    OFFSET4_CH { OFFSET4_CH }
                    OFFSET4_EN { OFFSET4_EN }
                    OFFSET4 { OFFSET4 }
                }
                JDR1 {
                    JDR1;
                    JDATA { JDATA }
                }
                JDR2 {
                    JDR2;
                    JDATA { JDATA }
                }
                JDR3 {
                    JDR3;
                    JDATA { JDATA }
                }
                JDR4 {
                    JDR4;
                    JDATA { JDATA }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                AWD2CR {
                    AWD2CR;
                    AWD2CH { AWD2CH }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                AWD3CR {
                    AWD3CR;
                    AWD3CH { AWD3CH }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                DIFSEL {
                    DIFSEL;
                    DIFSEL_1_15 { DIFSEL_1_15 }
                    DIFSEL_16_18 { DIFSEL_16_18 }
                }
                #[cfg(any(
                    stm32_mcu = "stm32l4r5",
                    stm32_mcu = "stm32l4r7",
                    stm32_mcu = "stm32l4r9",
                    stm32_mcu = "stm32l4s5",
                    stm32_mcu = "stm32l4s7",
                    stm32_mcu = "stm32l4s9"
                ))]
                CALFACT {
                    CALFACT;
                    CALFACT_D { CALFACT_D }
                    CALFACT_S { CALFACT_S }
                }
            }
        }
    };
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
    stm32_mcu = "stm32f469"
))]
map_adc! {
    "Extracts ADC1 register tokens.",
    periph_adc1,
    "ADC1 peripheral variant.",
    Adc1,
    ADC1,
    APB2ENR,
    APB2LPENR,
    ADC1EN,
    ADC1LPEN,
    DATA,
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469"
))]
map_adc! {
    "Extracts ADC2 register tokens.",
    periph_adc2,
    "ADC2 peripheral variant.",
    Adc2,
    ADC2,
    APB2ENR,
    APB2LPENR,
    ADC2EN,
    ADC2LPEN,
    DATA,
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469"
))]
map_adc! {
    "Extracts ADC3 register tokens.",
    periph_adc3,
    "ADC3 peripheral variant.",
    Adc3,
    ADC3,
    APB2ENR,
    APB2LPENR,
    ADC3EN,
    ADC3LPEN,
    DATA,
}

#[cfg(any(
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
map_adc! {
    "Extracts ADC register tokens.",
    periph_adc1,
    "ADC1 peripheral variant.",
    Adc1,
    ADC,
    AHB2ENR,
    AHB2SMENR,
    ADCEN,
    ADCSMEN,
    RDATA,
}

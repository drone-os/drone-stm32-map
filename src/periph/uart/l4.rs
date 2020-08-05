//! Universal Asynchronous Receiver/Transmitter.
//!
//! For STM32L4 series of ultra-low-power MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic UART peripheral variant.
    pub trait UartMap {}

    /// Generic UART peripheral.
    pub struct UartPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            UARTEN { RwRwRegFieldBitBand }
        }
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            UARTRST { RwRwRegFieldBitBand }
        }
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            UARTSMEN { RwRwRegFieldBitBand }
        }
        CCIPR {
            0x20 RwRegBitBand Shared;
            UARTSEL { RwRwRegFieldBits }
        }
    }

    UART {
        CR1 {
            0x20 RwRegBitBand;
            CMIE { RwRwRegFieldBitBand }
            DEAT0 { RwRwRegFieldBitBand }
            DEAT1 { RwRwRegFieldBitBand }
            DEAT2 { RwRwRegFieldBitBand }
            DEAT3 { RwRwRegFieldBitBand }
            DEAT4 { RwRwRegFieldBitBand }
            DEDT0 { RwRwRegFieldBitBand }
            DEDT1 { RwRwRegFieldBitBand }
            DEDT2 { RwRwRegFieldBitBand }
            DEDT3 { RwRwRegFieldBitBand }
            DEDT4 { RwRwRegFieldBitBand }
            EOBIE { RwRwRegFieldBitBand Option }
            IDLEIE { RwRwRegFieldBitBand }
            M0 { RwRwRegFieldBitBand }
            M1 { RwRwRegFieldBitBand }
            MME { RwRwRegFieldBitBand }
            OVER8 { RwRwRegFieldBitBand Option }
            PCE { RwRwRegFieldBitBand }
            PEIE { RwRwRegFieldBitBand }
            PS { RwRwRegFieldBitBand }
            RE { RwRwRegFieldBitBand }
            RTOIE { RwRwRegFieldBitBand Option }
            RXNEIE { RwRwRegFieldBitBand }
            TCIE { RwRwRegFieldBitBand }
            TE { RwRwRegFieldBitBand }
            TXEIE { RwRwRegFieldBitBand }
            UE { RwRwRegFieldBitBand }
            UESM { RwRwRegFieldBitBand }
            WAKE { RwRwRegFieldBitBand }
        }
        CR2 {
            0x20 RwRegBitBand;
            ABREN { RwRwRegFieldBitBand Option }
            ABRMOD0 { RwRwRegFieldBitBand Option }
            ABRMOD1 { RwRwRegFieldBitBand Option }
            ADD0_3 { RwRwRegFieldBits }
            ADD4_7 { RwRwRegFieldBits }
            ADDM7 { RwRwRegFieldBitBand }
            CLKEN { RwRwRegFieldBitBand }
            CPHA { RwRwRegFieldBitBand Option }
            CPOL { RwRwRegFieldBitBand Option }
            LBCL { RwRwRegFieldBitBand Option }
            LBDIE { RwRwRegFieldBitBand Option }
            LBDL { RwRwRegFieldBitBand Option }
            LINEN { RwRwRegFieldBitBand Option }
            MSBFIRST { RwRwRegFieldBitBand }
            RTOEN { RwRwRegFieldBitBand Option }
            RXINV { RwRwRegFieldBitBand }
            STOP { RwRwRegFieldBits }
            SWAP { RwRwRegFieldBitBand }
            TAINV { RwRwRegFieldBitBand }
            TXINV { RwRwRegFieldBitBand }
        }
        CR3 {
            0x20 RwRegBitBand;
            CTSE { RwRwRegFieldBitBand }
            CTSIE { RwRwRegFieldBitBand }
            DDRE { RwRwRegFieldBitBand }
            DEM { RwRwRegFieldBitBand }
            DEP { RwRwRegFieldBitBand }
            DMAR { RwRwRegFieldBitBand }
            DMAT { RwRwRegFieldBitBand }
            EIE { RwRwRegFieldBitBand }
            HDSEL { RwRwRegFieldBitBand }
            IREN { RwRwRegFieldBitBand Option }
            IRLP { RwRwRegFieldBitBand Option }
            NACK { RwRwRegFieldBitBand Option }
            ONEBIT { RwRwRegFieldBitBand Option }
            OVRDIS { RwRwRegFieldBitBand }
            RTSE { RwRwRegFieldBitBand }
            SCARCNT { RwRwRegFieldBits Option }
            SCEN { RwRwRegFieldBitBand Option }
            #[cfg(any(
                stm32_mcu = "stm32l4x1",
                stm32_mcu = "stm32l4x2",
            ))]
            TCBGTIE { RwRwRegFieldBitBand Option }
            #[cfg(any(
                stm32_mcu = "stm32l4x1",
                stm32_mcu = "stm32l4x2",
            ))]
            UCESM { RwRwRegFieldBitBand }
            WUFIE { RwRwRegFieldBitBand }
            WUS { RwRwRegFieldBits }
        }
        BRR {
            0x20 RwRegBitBand;
            BRR { RwRwRegFieldBits }
        }
        GTPR {
            0x20 RwRegBitBand Option;
            GT { RwRwRegFieldBits }
            PSC { RwRwRegFieldBits }
        }
        RTOR {
            0x20 RwRegBitBand Option;
            BLEN { RwRwRegFieldBits }
            RTO { RwRwRegFieldBits }
        }
        RQR {
            0x20 WoRegBitBand;
            ABRRQ { WoWoRegFieldBitBand Option }
            MMRQ { WoWoRegFieldBitBand }
            RXFRQ { WoWoRegFieldBitBand }
            SBKRQ { WoWoRegFieldBitBand }
            TXFRQ { WoWoRegFieldBitBand Option }
        }
        ISR {
            0x20 RoRegBitBand;
            REACK { RoRoRegFieldBitBand }
            TEACK { RoRoRegFieldBitBand }
            WUF { RoRoRegFieldBitBand }
            RWU { RoRoRegFieldBitBand }
            SBKF { RoRoRegFieldBitBand }
            CMF { RoRoRegFieldBitBand }
            BUSY { RoRoRegFieldBitBand }
            ABRF { RoRoRegFieldBitBand Option }
            ABRE { RoRoRegFieldBitBand Option }
            EOBF { RoRoRegFieldBitBand Option }
            RTOF { RoRoRegFieldBitBand Option }
            CTS { RoRoRegFieldBitBand }
            CTSIF { RoRoRegFieldBitBand }
            LBDF { RoRoRegFieldBitBand Option }
            TXE { RoRoRegFieldBitBand }
            TC { RoRoRegFieldBitBand }
            #[cfg(any(
                stm32_mcu = "stm32l4x1",
                stm32_mcu = "stm32l4x2",
            ))]
            TCBGT { RoRoRegFieldBitBand Option }
            RXNE { RoRoRegFieldBitBand }
            IDLE { RoRoRegFieldBitBand }
            ORE { RoRoRegFieldBitBand }
            NF { RoRoRegFieldBitBand }
            FE { RoRoRegFieldBitBand }
            PE { RoRoRegFieldBitBand }
        }
        ICR {
            0x20 WoRegBitBand;
            WUCF { WoWoRegFieldBitBand }
            CMCF { WoWoRegFieldBitBand }
            EOBCF { WoWoRegFieldBitBand Option }
            RTOCF { WoWoRegFieldBitBand Option }
            CTSCF { WoWoRegFieldBitBand }
            LBDCF { WoWoRegFieldBitBand Option }
            TCCF { WoWoRegFieldBitBand }
            IDLECF { WoWoRegFieldBitBand }
            ORECF { WoWoRegFieldBitBand }
            NCF { WoWoRegFieldBitBand }
            FECF { WoWoRegFieldBitBand }
            PECF { WoWoRegFieldBitBand }
        }
        RDR {
            0x20 RoRegBitBand;
            RDR { RoRoRegFieldBits }
        }
        TDR {
            0x20 RwRegBitBand;
            TDR { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_uart {
    (
        $uart_macro_doc:expr,
        $uart_macro:ident,
        $uart_ty_doc:expr,
        $uart_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $uarten:ident,
        $uartrst:ident,
        $uartsmen:ident,
        $uartsel:ident,
        $uart:ident,
        ($($eobie:ident)?),
        ($($over8:ident)?),
        ($($rtoie:ident)?),
        ($($abren:ident)?),
        ($($abrmod0:ident)?),
        ($($abrmod1:ident)?),
        ($($cpha:ident)?),
        ($($cpol:ident)?),
        ($($lbcl:ident)?),
        ($($lbdie:ident)?),
        ($($lbdl:ident)?),
        ($($linen:ident)?),
        ($($rtoen:ident)?),
        ($($iren:ident)?),
        ($($irlp:ident)?),
        ($($nack:ident)?),
        ($($onebit:ident)?),
        ($($scarcnt:ident)?),
        ($($scen:ident)?),
        ($($tcbgtie:ident)?),
        ($($gtpr:ident)?),
        ($($rtor:ident)?),
        ($($abrrq:ident)?),
        ($($txfrq:ident)?),
        ($($abrf:ident)?),
        ($($abre:ident)?),
        ($($eobf:ident)?),
        ($($rtof:ident)?),
        ($($lbdf:ident)?),
        ($($tcbgt:ident)?),
        ($($eobcf:ident)?),
        ($($rtocf:ident)?),
        ($($lbdcf:ident)?),
    ) => {
        periph::map! {
            #[doc = $uart_macro_doc]
            pub macro $uart_macro;

            #[doc = $uart_ty_doc]
            pub struct $uart_ty;

            impl UartMap for $uart_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    UARTEN { $uarten }
                }
                BUSRSTR {
                    $busrstr Shared;
                    UARTRST { $uartrst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    UARTSMEN { $uartsmen }
                }
                CCIPR {
                    CCIPR Shared;
                    UARTSEL { $uartsel }
                }
            }

            UART {
                $uart;
                CR1 {
                    CR1;
                    CMIE { CMIE }
                    DEAT0 { DEAT0 }
                    DEAT1 { DEAT1 }
                    DEAT2 { DEAT2 }
                    DEAT3 { DEAT3 }
                    DEAT4 { DEAT4 }
                    DEDT0 { DEDT0 }
                    DEDT1 { DEDT1 }
                    DEDT2 { DEDT2 }
                    DEDT3 { DEDT3 }
                    DEDT4 { DEDT4 }
                    EOBIE { $($eobie Option)* }
                    IDLEIE { IDLEIE }
                    M0 { M0 }
                    M1 { M1 }
                    MME { MME }
                    OVER8 { $($over8 Option)* }
                    PCE { PCE }
                    PEIE { PEIE }
                    PS { PS }
                    RE { RE }
                    RTOIE { $($rtoie Option)* }
                    RXNEIE { RXNEIE }
                    TCIE { TCIE }
                    TE { TE }
                    TXEIE { TXEIE }
                    UESM { UESM }
                    UE { UE }
                    WAKE { WAKE }
                }
                CR2 {
                    CR2;
                    ABREN { $($abren Option)* }
                    ABRMOD0 { $($abrmod0 Option)* }
                    ABRMOD1 { $($abrmod1 Option)* }
                    ADD0_3 { ADD0_3 }
                    ADD4_7 { ADD4_7 }
                    ADDM7 { ADDM7 }
                    CLKEN { CLKEN }
                    CPHA { $($cpha Option)* }
                    CPOL { $($cpol Option)* }
                    LBCL { $($lbcl Option)* }
                    LBDIE { $($lbdie Option)* }
                    LBDL { $($lbdl Option)* }
                    LINEN { $($linen Option)* }
                    MSBFIRST { MSBFIRST }
                    RTOEN { $($rtoen Option)* }
                    RXINV { RXINV }
                    STOP { STOP }
                    SWAP { SWAP }
                    TAINV { TAINV }
                    TXINV { TXINV }
                }
                CR3 {
                    CR3;
                    CTSE { CTSE }
                    CTSIE { CTSIE }
                    DDRE { DDRE }
                    DEM { DEM }
                    DEP { DEP }
                    DMAR { DMAR }
                    DMAT { DMAT }
                    EIE { EIE }
                    HDSEL { HDSEL }
                    IREN { $($iren Option)* }
                    IRLP { $($irlp Option)* }
                    NACK { $($nack Option)* }
                    ONEBIT { $($onebit Option)* }
                    OVRDIS { OVRDIS }
                    RTSE { RTSE }
                    SCARCNT { $($scarcnt Option)* }
                    SCEN { $($scen Option)* }
                    #[cfg(any(
                        stm32_mcu = "stm32l4x1",
                        stm32_mcu = "stm32l4x2",
                    ))]
                    TCBGTIE { $($tcbgtie Option)* }
                    #[cfg(any(
                        stm32_mcu = "stm32l4x1",
                        stm32_mcu = "stm32l4x2",
                    ))]
                    UCESM { UCESM }
                    WUFIE { WUFIE }
                    WUS { WUS }
                }
                BRR {
                    BRR;
                    BRR { BRR }
                }
                GTPR {
                    $(
                        $gtpr Option;
                        GT { GT }
                        PSC { PSC }
                    )*
                }
                RTOR {
                    $(
                        $rtor Option;
                        BLEN { BLEN }
                        RTO { RTO }
                    )*
                }
                RQR {
                    RQR;
                    ABRRQ { $($abrrq Option)* }
                    MMRQ { MMRQ }
                    RXFRQ { RXFRQ }
                    SBKRQ { SBKRQ }
                    TXFRQ { $($txfrq Option)* }
                }
                ISR {
                    ISR;
                    REACK { REACK }
                    TEACK { TEACK }
                    WUF { WUF }
                    RWU { RWU }
                    SBKF { SBKF }
                    CMF { CMF }
                    BUSY { BUSY }
                    ABRF { $($abrf Option)* }
                    ABRE { $($abre Option)* }
                    EOBF { $($eobf Option)* }
                    RTOF { $($rtof Option)* }
                    CTS { CTS }
                    CTSIF { CTSIF }
                    LBDF { $($lbdf Option)* }
                    TXE { TXE }
                    TC { TC }
                    #[cfg(any(
                        stm32_mcu = "stm32l4x1",
                        stm32_mcu = "stm32l4x2",
                    ))]
                    TCBGT { $($tcbgt Option)* }
                    RXNE { RXNE }
                    IDLE { IDLE }
                    ORE { ORE }
                    NF { NF }
                    FE { FE }
                    PE { PE }
                }
                ICR {
                    ICR;
                    WUCF { WUCF }
                    CMCF { CMCF }
                    EOBCF { $($eobcf Option)* }
                    RTOCF { $($rtocf Option)* }
                    CTSCF { CTSCF }
                    LBDCF { $($lbdcf Option)* }
                    TCCF { TCCF }
                    IDLECF { IDLECF }
                    ORECF { ORECF }
                    NCF { NCF }
                    FECF { FECF }
                    PECF { PECF }
                }
                RDR {
                    RDR;
                    RDR { RDR }
                }
                TDR {
                    TDR;
                    TDR { TDR }
                }
            }
        }
    };
}

map_uart! {
    "Extracts USART1 register tokens.",
    periph_usart1,
    "USART1 peripheral variant.",
    Usart1,
    APB2ENR,
    APB2RSTR,
    APB2SMENR,
    USART1EN,
    USART1RST,
    USART1SMEN,
    USART1SEL,
    USART1,
    (EOBIE),
    (OVER8),
    (RTOIE),
    (ABREN),
    (ABRMOD0),
    (ABRMOD1),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (LINEN),
    (RTOEN),
    (IREN),
    (IRLP),
    (NACK),
    (ONEBIT),
    (SCARCNT),
    (SCEN),
    (),
    (GTPR),
    (RTOR),
    (ABRRQ),
    (TXFRQ),
    (ABRF),
    (ABRE),
    (EOBF),
    (RTOF),
    (LBDF),
    (),
    (EOBCF),
    (RTOCF),
    (LBDCF),
}

map_uart! {
    "Extracts USART2 register tokens.",
    periph_usart2,
    "USART2 peripheral variant.",
    Usart2,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    USART2EN,
    USART2RST,
    USART2SMEN,
    USART2SEL,
    USART2,
    (EOBIE),
    (OVER8),
    (RTOIE),
    (ABREN),
    (ABRMOD0),
    (ABRMOD1),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (LINEN),
    (RTOEN),
    (IREN),
    (IRLP),
    (NACK),
    (ONEBIT),
    (SCARCNT),
    (SCEN),
    (),
    (GTPR),
    (RTOR),
    (ABRRQ),
    (TXFRQ),
    (ABRF),
    (ABRE),
    (EOBF),
    (RTOF),
    (LBDF),
    (),
    (EOBCF),
    (RTOCF),
    (LBDCF),
}

map_uart! {
    "Extracts USART3 register tokens.",
    periph_usart3,
    "USART3 peripheral variant.",
    Usart3,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    USART3EN,
    USART3RST,
    USART3SMEN,
    USART3SEL,
    USART3,
    (EOBIE),
    (OVER8),
    (RTOIE),
    (ABREN),
    (ABRMOD0),
    (ABRMOD1),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (LINEN),
    (RTOEN),
    (IREN),
    (IRLP),
    (NACK),
    (ONEBIT),
    (SCARCNT),
    (SCEN),
    (TCBGTIE),
    (GTPR),
    (RTOR),
    (ABRRQ),
    (TXFRQ),
    (ABRF),
    (ABRE),
    (EOBF),
    (RTOF),
    (LBDF),
    (TCBGT),
    (EOBCF),
    (RTOCF),
    (LBDCF),
}

#[cfg(any(
    stm32_mcu = "stm32l4x1",
    stm32_mcu = "stm32l4x2",
    stm32_mcu = "stm32l4x5",
    stm32_mcu = "stm32l4x6",
))]
map_uart! {
    "Extracts UART4 register tokens.",
    periph_uart4,
    "UART4 peripheral variant.",
    Uart4,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    UART4EN,
    UART4RST,
    UART4SMEN,
    UART4SEL,
    UART4,
    (EOBIE),
    (OVER8),
    (RTOIE),
    (ABREN),
    (ABRMOD0),
    (ABRMOD1),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (LINEN),
    (RTOEN),
    (IREN),
    (IRLP),
    (NACK),
    (ONEBIT),
    (SCARCNT),
    (SCEN),
    (),
    (GTPR),
    (RTOR),
    (ABRRQ),
    (TXFRQ),
    (ABRF),
    (ABRE),
    (EOBF),
    (RTOF),
    (LBDF),
    (),
    (EOBCF),
    (RTOCF),
    (LBDCF),
}

#[cfg(any(stm32_mcu = "stm32l4x5", stm32_mcu = "stm32l4x6"))]
map_uart! {
    "Extracts UART5 register tokens.",
    periph_uart5,
    "UART5 peripheral variant.",
    Uart5,
    APB1ENR1,
    APB1RSTR1,
    APB1SMENR1,
    UART5EN,
    UART5RST,
    UART5SMEN,
    UART5SEL,
    UART5,
    (EOBIE),
    (OVER8),
    (RTOIE),
    (ABREN),
    (ABRMOD0),
    (ABRMOD1),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (LINEN),
    (RTOEN),
    (IREN),
    (IRLP),
    (NACK),
    (ONEBIT),
    (SCARCNT),
    (SCEN),
    (),
    (GTPR),
    (RTOR),
    (ABRRQ),
    (TXFRQ),
    (ABRF),
    (ABRE),
    (EOBF),
    (RTOF),
    (LBDF),
    (),
    (EOBCF),
    (RTOCF),
    (LBDCF),
}

map_uart! {
    "Extracts LPUART1 register tokens.",
    periph_lpuart1,
    "LPUART1 peripheral variant.",
    Lpuart1,
    APB1ENR2,
    APB1RSTR2,
    APB1SMENR2,
    LPUART1EN,
    LPUART1RST,
    LPUART1SMEN,
    LPUART1SEL,
    LPUART1,
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
}

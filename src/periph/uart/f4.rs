//! Universal Asynchronous Receiver/Transmitter.
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

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
    }

    UART {
        SR {
            0x20 RwRegBitBand;
            CTS { RwRwRegFieldBit Option }
            LBD { RwRwRegFieldBit }
            TXE { RoRwRegFieldBit }
            TC { RwRwRegFieldBit }
            RXNE { RwRwRegFieldBit }
            IDLE { RoRwRegFieldBit }
            ORE { RoRwRegFieldBit }
            NF { RoRwRegFieldBit }
            FE { RoRwRegFieldBit }
            PE { RoRwRegFieldBit }
        }
        DR {
            0x20 RwRegBitBand;
            DR { RwRwRegFieldBits }
        }
        BRR {
            0x20 RwRegBitBand;
            DIV_Mantissa { RwRwRegFieldBits }
            DIV_Fraction { RwRwRegFieldBits }
        }
        CR1 {
            0x20 RwRegBitBand;
            OVER8 { RwRwRegFieldBit }
            UE { RwRwRegFieldBit }
            M { RwRwRegFieldBit }
            WAKE { RwRwRegFieldBit }
            PCE { RwRwRegFieldBit }
            PS { RwRwRegFieldBit }
            PEIE { RwRwRegFieldBit }
            TXEIE { RwRwRegFieldBit }
            TCIE { RwRwRegFieldBit }
            RXNEIE { RwRwRegFieldBit }
            IDLEIE { RwRwRegFieldBit }
            TE { RwRwRegFieldBit }
            RE { RwRwRegFieldBit }
            RWU { RwRwRegFieldBit }
            SBK { RwRwRegFieldBit }
        }
        CR2 {
            0x20 RwRegBitBand;
            LINEN { RwRwRegFieldBit }
            STOP { RwRwRegFieldBits }
            CLKEN { RwRwRegFieldBit Option }
            CPOL { RwRwRegFieldBit Option }
            CPHA { RwRwRegFieldBit Option }
            LBCL { RwRwRegFieldBit Option }
            LBDIE { RwRwRegFieldBit }
            LBDL { RwRwRegFieldBit }
            ADD { RwRwRegFieldBits }
        }
        CR3 {
            0x20 RwRegBitBand;
            ONEBIT { RwRwRegFieldBit }
            CTSIE { RwRwRegFieldBit Option }
            CTSE { RwRwRegFieldBit Option }
            RTSE { RwRwRegFieldBit Option }
            DMAT { RwRwRegFieldBit }
            DMAR { RwRwRegFieldBit }
            SCEN { RwRwRegFieldBit Option }
            NACK { RwRwRegFieldBit Option }
            HDSEL { RwRwRegFieldBit }
            IRLP { RwRwRegFieldBit }
            IREN { RwRwRegFieldBit }
            EIE { RwRwRegFieldBit }
        }
        GTPR {
            0x20 RwRegBitBand Option;
            GT { RwRwRegFieldBits }
            PSC { RwRwRegFieldBits }
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
        $uart:ident,
        ($($cts:ident)?),
        ($($clken:ident)?),
        ($($cpol:ident)?),
        ($($cpha:ident)?),
        ($($lbcl:ident)?),
        ($($ctsie:ident)?),
        ($($ctse:ident)?),
        ($($rtse:ident)?),
        ($($scen:ident)?),
        ($($nack:ident)?),
        ($($gtpr:ident)?),
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
            }

            UART {
                $uart;
                SR {
                    SR;
                    CTS { $($cts Option)* }
                    LBD { LBD }
                    TXE { TXE }
                    TC { TC }
                    RXNE { RXNE }
                    IDLE { IDLE }
                    ORE { ORE }
                    NF { NF }
                    FE { FE }
                    PE { PE }
                }
                DR {
                    DR;
                    DR { DR }
                }
                BRR {
                    BRR;
                    DIV_Mantissa { DIV_Mantissa }
                    DIV_Fraction { DIV_Fraction }
                }
                CR1 {
                    CR1;
                    OVER8 { OVER8 }
                    UE { UE }
                    M { M }
                    WAKE { WAKE }
                    PCE { PCE }
                    PS { PS }
                    PEIE { PEIE }
                    TXEIE { TXEIE }
                    TCIE { TCIE }
                    RXNEIE { RXNEIE }
                    IDLEIE { IDLEIE }
                    TE { TE }
                    RE { RE }
                    RWU { RWU }
                    SBK { SBK }
                }
                CR2 {
                    CR2;
                    LINEN { LINEN }
                    STOP { STOP }
                    CLKEN { $($clken Option)* }
                    CPOL { $($cpol Option)* }
                    CPHA { $($cpha Option)* }
                    LBCL { $($lbcl Option)* }
                    LBDIE { LBDIE }
                    LBDL { LBDL }
                    ADD { ADD }
                }
                CR3 {
                    CR3;
                    ONEBIT { ONEBIT }
                    CTSIE { $($ctsie Option)* }
                    CTSE { $($ctse Option)* }
                    RTSE { $($rtse Option)* }
                    DMAT { DMAT }
                    DMAR { DMAR }
                    SCEN { $($scen Option)* }
                    NACK { $($nack Option)* }
                    HDSEL { HDSEL }
                    IRLP { IRLP }
                    IREN { IREN }
                    EIE { EIE }
                }
                GTPR {
                    $(
                        $gtpr Option;
                        GT { GT }
                        PSC { PSC }
                    )*
                }
            }
        }
    }
}

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f410",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_uart! {
    "Extracts USART1 register tokens.",
    periph_usart1,
    "USART1 peripheral variant.",
    Usart1,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    USART1EN,
    USART1RST,
    USART1LPEN,
    USART1,
    (CTS),
    (CLKEN),
    (CPOL),
    (CPHA),
    (LBCL),
    (CTSIE),
    (CTSE),
    (RTSE),
    (SCEN),
    (NACK),
    (GTPR),
}

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f410",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_uart! {
    "Extracts USART2 register tokens.",
    periph_usart2,
    "USART2 peripheral variant.",
    Usart2,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    USART2EN,
    UART2RST,
    USART2LPEN,
    USART2,
    (CTS),
    (CLKEN),
    (CPOL),
    (CPHA),
    (LBCL),
    (CTSIE),
    (CTSE),
    (RTSE),
    (SCEN),
    (NACK),
    (GTPR),
}

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_uart! {
    "Extracts USART3 register tokens.",
    periph_usart3,
    "USART3 peripheral variant.",
    Usart3,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    USART3EN,
    USART3RST,
    USART3LPEN,
    USART3,
    (CTS),
    (CLKEN),
    (CPOL),
    (CPHA),
    (LBCL),
    (CTSIE),
    (CTSE),
    (RTSE),
    (SCEN),
    (NACK),
    (GTPR),
}

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_uart! {
    "Extracts UART4 register tokens.",
    periph_uart4,
    "UART4 peripheral variant.",
    Uart4,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    UART4EN,
    UART4RST,
    UART4LPEN,
    UART4,
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

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_uart! {
    "Extracts UART5 register tokens.",
    periph_uart5,
    "UART5 peripheral variant.",
    Uart5,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    UART5EN,
    UART5RST,
    UART5LPEN,
    UART5,
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

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f410",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
map_uart! {
    "Extracts USART6 register tokens.",
    periph_usart6,
    "USART6 peripheral variant.",
    Usart6,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    USART6EN,
    USART6RST,
    USART6LPEN,
    USART6,
    (CTS),
    (CLKEN),
    (CPOL),
    (CPHA),
    (LBCL),
    (CTSIE),
    (CTSE),
    (RTSE),
    (SCEN),
    (NACK),
    (GTPR),
}

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f469",
))]
map_uart! {
    "Extracts UART7 register tokens.",
    periph_uart7,
    "UART7 peripheral variant.",
    Uart7,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    UART7EN,
    UART7RST,
    UART7LPEN,
    UART7,
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

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f469",
))]
map_uart! {
    "Extracts UART8 register tokens.",
    periph_uart8,
    "UART8 peripheral variant.",
    Uart8,
    APB1ENR,
    APB1RSTR,
    APB1LPENR,
    UART8EN,
    UART8RST,
    UART8LPEN,
    UART8,
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

#[cfg(any(drone_stm32_map = "stm32f413",))]
map_uart! {
    "Extracts UART9 register tokens.",
    periph_uart9,
    "UART9 peripheral variant.",
    Uart9,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    UART9EN,
    UART9RST,
    UART9LPEN,
    UART9,
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

#[cfg(any(drone_stm32_map = "stm32f413",))]
map_uart! {
    "Extracts UART10 register tokens.",
    periph_uart10,
    "UART10 peripheral variant.",
    Uart10,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    UART10EN,
    UART10RST,
    UART10LPEN,
    UART10,
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

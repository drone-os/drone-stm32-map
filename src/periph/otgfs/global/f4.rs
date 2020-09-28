//! USB on the go full speed (OTG_FS) global peripherals. 
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic USB-OTGFS peripheral variant.
    pub trait GlobalOtgfsMap {}

    /// Generic USB-OTGFS peripheral.
    pub struct GlobalOtgfsPeriph;

    RCC {
        BUSENR {
            0x20 RwRegBitBand Shared;
            OTGFSEN { RwRwRegFieldBitBand }
        }
        BUSRSTR {
            0x20 RwRegBitBand Shared;
            OTGFSRST { RwRwRegFieldBitBand }
        }
        BUSSMENR {
            0x20 RwRegBitBand Shared;
            OTGFSSMEN { RwRwRegFieldBitBand }
        }
    }

    OTG_FS_GLOBAL {
        FS_GOTGCTL {
            0x20 RwReg;
            BSVLD { RoRwRegFieldBit }
            ASVLD { RoRwRegFieldBit }
            DBCT { RoRwRegFieldBit }
            CIDSTS { RoRwRegFieldBit }
            DHNPEN { RwRwRegFieldBit }
            HSHNPEN { RwRwRegFieldBit }
            HNPRQ { RwRwRegFieldBit }
            HNGSCS { RoRwRegFieldBit }
            SRQ { RwRwRegFieldBit }
            SRQSCS { RoRwRegFieldBit }
        }
        FS_GOTGINT {
            0x20 RwReg;
            DBCDNE { RwRwRegFieldBit }
            ADTOCHG { RwRwRegFieldBit }
            HNGDET { RwRwRegFieldBit }
            HNSSCHG { RwRwRegFieldBit }
            SRSSCHG { RwRwRegFieldBit }
            SEDET { RwRwRegFieldBit }
        }
        FS_GAHBCFG {
            0x20 RwReg;
            PTXFELVL { RwRwRegFieldBit }
            TXFELVL { RwRwRegFieldBit }
            GINTMSK { RwRwRegFieldBit }
        }
        FS_GUSBCFG {
            0x20 RwReg;
            CTXPKT { RwRwRegFieldBit }
            FDMOD { RwRwRegFieldBit }
            FHMOD { RwRwRegFieldBit }
            TRDT { RwRwRegFieldBits }
            HNPCAP { RwRwRegFieldBit }
            SRPCAP { RwRwRegFieldBit }
            PHYSEL { RoRwRegFieldBit }
            TOCAL { RwRwRegFieldBits }
        }
        FS_GRSTCTL {
            0x20 RwReg;
            AHBIDL { RoRwRegFieldBit }
            TXFNUM { RwRwRegFieldBits }
            TXFFLSH { RwRwRegFieldBit }
            RXFFLSH { RwRwRegFieldBit }
            FCRST { RwRwRegFieldBit }
            HSRST { RwRwRegFieldBit }
            CSRST { RwRwRegFieldBit }
        }
        FS_GINTSTS {
            0x20 RwReg;
            WKUPINT { RwRwRegFieldBit }
            SRQINT { RwRwRegFieldBit }
            DISCINT { RwRwRegFieldBit }
            CIDSCHG { RwRwRegFieldBit }
            PTXFE { RoRwRegFieldBit }
            HCINT { RoRwRegFieldBit }
            HPRTINT { RoRwRegFieldBit }
            IPXFR_INCOMPISOOUT { RwRwRegFieldBit }
            IISOIXFR { RwRwRegFieldBit }
            OEPINT { RoRwRegFieldBit }
            IEPINT { RoRwRegFieldBit }
            EOPF { RwRwRegFieldBit }
            ISOODRP { RwRwRegFieldBit }
            ENUMDNE { RwRwRegFieldBit }
            USBRST { RwRwRegFieldBit }
            USBSUSP { RwRwRegFieldBit }
            ESUSP { RwRwRegFieldBit }
            GONAKEFF { RoRwRegFieldBit }
            GINAKEFF { RoRwRegFieldBit }
            NPTXFE { RoRwRegFieldBit }
            RXFLVL { RoRwRegFieldBit }
            SOF { RwRwRegFieldBit }
            OTGINT { RoRwRegFieldBit }
            MMIS { RwRwRegFieldBit }
            CMOD { RoRwRegFieldBit }
        }
        FS_GINTMSK {
            0x20 RwReg;
            WUIM { RwRwRegFieldBit }
            SRQIM { RwRwRegFieldBit }
            DISCINT { RwRwRegFieldBit }
            CIDSCHGM { RwRwRegFieldBit }
            PTXFEM { RwRwRegFieldBit }
            HCIM { RwRwRegFieldBit }
            PRTIM { RwRwRegFieldBit }
            IPXFRM_IISOOXFRM { RwRwRegFieldBit }
            IISOIXFRM { RwRwRegFieldBit }
            OEPINT { RwRwRegFieldBit }
            IEPINT { RwRwRegFieldBit }
            EPMISM  { RwRwRegFieldBit }
            EOPFM { RwRwRegFieldBit }
            ISOODRPM { RwRwRegFieldBit }
            ENUMDNEM { RwRwRegFieldBit }
            USBRST { RwRwRegFieldBit }
            USBSUSPM { RwRwRegFieldBit }
            ESUSPM { RwRwRegFieldBit }
            GONAKEFFM { RwRwRegFieldBit }
            GINAKEFFM { RwRwRegFieldBit }
            NPTXFEM { RwRwRegFieldBit }
            RXFLVLM { RwRwRegFieldBit }
            SOFM { RwRwRegFieldBit }
            OTGINT { RwRwRegFieldBit }
            MMISM { RwRwRegFieldBit }
        }
        FS_GRXSTSR_Device {
            0x20 RoReg;
            FRMNUM { RoRoRegFieldBits }
            PKTSTS { RoRoRegFieldBits }
            DPID { RoRoRegFieldBits }
            BCNT { RoRoRegFieldBits }
            EPNUM { RoRoRegFieldBits }
        }
        FS_GRXSTSR_Host {
            0x20 RoReg;
            PKTSTS { RoRoRegFieldBits }
            DPID { RoRoRegFieldBits }
            BCNT { RoRoRegFieldBits }
            CHNUM { RoRoRegFieldBits }
        }
        FS_GRXSTSP_Device {
            0x20 RoReg;
            FRMNUM { RoRoRegFieldBits }
            PKTSTS { RoRoRegFieldBits }
            DPID { RoRoRegFieldBits }
            BCNT { RoRoRegFieldBits }
            EPNUM { RoRoRegFieldBits }
        }
        FS_GRXSTSP_Host {
            0x20 RoReg;
            PKTSTS { RoRoRegFieldBits }
            DPID { RoRoRegFieldBits }
            BCNT { RoRoRegFieldBits }
            CHNUM { RoRoRegFieldBits }
        }
        FS_GRXFSIZ {
            0x20 RwReg;
            RXFD { RwRwRegFieldBits }
        }
        FS_GNPTXFSIZ_Device {
            0x20 RwReg;
            TX0FD { RwRwRegFieldBits }
            TX0FSA { RwRwRegFieldBits }
        }
        FS_GNPTXFSIZ_Host {
            0x20 RwReg;
            NPTXFD { RwRwRegFieldBits }
            NPTXFSA { RwRwRegFieldBits }
        }
        FS_GNPTXSTS {
            0x20 RoReg;
            NPTXQTOP { RoRoRegFieldBits }
            NPTQXSAV { RoRoRegFieldBits }
            NPTXFSAV { RoRoRegFieldBits }
        }
        FS_GCCFG {
            0x20 RwReg;
            NOVBUSSENS { RwRwRegFieldBit }
            SOFOUTEN { RwRwRegFieldBit }
            VBUSBSEN { RwRwRegFieldBit }
            VBUSASEN { RwRwRegFieldBit }
            PWRDWN { RwRwRegFieldBit }
        }
        FS_CID {
            0x20 RwReg;
            PRODUCT_ID { RwRwRegFieldBits }
        }
        FS_HPTXFSIZ {
            0x20 RwReg;
            PTXFSIZ { RwRwRegFieldBits }
            PTXSA { RwRwRegFieldBits }
        }
        FS_DIEPTXF1 {
            0x20 RwReg;
            INEPTXFD { RwRwRegFieldBits }
            INEPTXSA { RwRwRegFieldBits }
        }
        FS_DIEPTXF2 {
            0x20 RwReg;
            INEPTXFD { RwRwRegFieldBits }
            INEPTXSA { RwRwRegFieldBits }
        }
        FS_DIEPTXF3 {
            0x20 RwReg;
            INEPTXFD { RwRwRegFieldBits }
            INEPTXSA { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_otgfs_global {
    (
        $otgfs_macro_doc:expr,
        $otgfs_macro:ident,
        $otgfs_ty_doc:expr,
        $otgfs_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $otgfsen:ident,
        $otgfsrst:ident,
        $otgfssmen:ident,
        $otgfs:ident,
    ) => {
        periph::map! {
            #[doc = $otgfs_macro_doc]
            pub macro $otgfs_macro;

            #[doc = $otgfs_ty_doc]
            pub struct $otgfs_ty;

            impl GlobalOtgfsMap for $otgfs_ty {}

            drone_stm32_map_pieces::reg;
            crate::global;

            RCC {
                BUSENR {
                    $busenr Shared;
                    OTGFSEN { $otgfsen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    OTGFSRST { $otgfsrst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    OTGFSSMEN { $otgfssmen }
                }
            }

            OTG_FS_GLOBAL {
                FS_GOTGCTL {
                    FS_GOTGCTL;
                    BSVLD { BSVLD }
                    ASVLD { ASVLD }
                    DBCT { DBCT }
                    CIDSTS { CIDSTS }
                    DHNPEN { DHNPEN }
                    HSHNPEN { HSHNPEN }
                    HNPRQ { HNPRQ }
                    HNGSCS { HNGSCS }
                    SRQ { SRQ }
                    SRQSCS { SRQSCS }
                }
                FS_GOTGINT {
                    FS_GOTGINT;
                    DBCDNE { DBCDNE }
                    ADTOCHG { ADTOCHG }
                    HNGDET { HNGDET }
                    HNSSCHG { HNSSCHG }
                    SRSSCHG { SRSSCHG }
                    SEDET { SEDET }
                }
                FS_GAHBCFG {  
                    FS_GAHBCFG;
                    PTXFELVL { PTXFELVL } 
                    TXFELVL { TXFELVL } 
                    GINTMSK { GINTMSK } 
                }
                FS_GUSBCFG {   
                    FS_GUSBCFG;                
                    CTXPKT { CTXPKT }
                    FDMOD { FDMOD } 
                    FHMOD { FHMOD }
                    TRDT { TRDT }       
                    HNPCAP { HNPCAP } 
                    SRPCAP { SRPCAP } 
                    PHYSEL { PHYSEL }
                    TOCAL { TOCAL }
                }
                FS_GRSTCTL {
                    FS_GRSTCTL; 
                    AHBIDL { AHBIDL }
                    TXFNUM { TXFNUM }
                    TXFFLSH { TXFFLSH }
                    RXFFLSH { RXFFLSH }
                    FCRST { FCRST }
                    HSRST { HSRST }
                    CSRST { CSRST }
                } 
                FS_GINTSTS {  
                    FS_GINTSTS;
                    WKUPINT { WKUPINT } 
                    SRQINT { SRQINT }
                    DISCINT { DISCINT }
                    CIDSCHG { CIDSCHG } 
                    PTXFE { PTXFE }
                    HCINT { HCINT }
                    HPRTINT { HPRTINT } 
                    IPXFR_INCOMPISOOUT { IPXFR_INCOMPISOOUT }
                    IISOIXFR { IISOIXFR }
                    OEPINT { OEPINT }
                    IEPINT { IEPINT }
                    EOPF { EOPF }
                    ISOODRP { ISOODRP } 
                    ENUMDNE { ENUMDNE } 
                    USBRST { USBRST } 
                    USBSUSP { USBSUSP } 
                    ESUSP { ESUSP }  
                    GONAKEFF { GONAKEFF } 
                    GINAKEFF { GINAKEFF } 
                    NPTXFE { NPTXFE } 
                    RXFLVL { RXFLVL } 
                    SOF { SOF }
                    OTGINT { OTGINT }
                    MMIS { MMIS }
                    CMOD { CMOD }  
                }
                FS_GINTMSK {
                    FS_GINTMSK;
                    WUIM { WUIM }
                    SRQIM { SRQIM }
                    DISCINT { DISCINT }
                    CIDSCHGM { CIDSCHGM }
                    PTXFEM { PTXFEM }
                    HCIM { HCIM }
                    PRTIM { PRTIM }
                    IPXFRM_IISOOXFRM { IPXFRM_IISOOXFRM }
                    IISOIXFRM { IISOIXFRM }
                    OEPINT { OEPINT }
                    IEPINT { IEPINT }
                    EPMISM { EPMISM }
                    EOPFM { EOPFM }
                    ISOODRPM { ISOODRPM }
                    ENUMDNEM { ENUMDNEM }
                    USBRST { USBRST }
                    USBSUSPM { USBSUSPM }
                    ESUSPM { ESUSPM }
                    GONAKEFFM { GONAKEFFM }
                    GINAKEFFM { GINAKEFFM }
                    NPTXFEM { NPTXFEM }
                    RXFLVLM { RXFLVLM }
                    SOFM { SOFM }
                    OTGINT { OTGINT }
                    MMISM { MMISM }
                }
                FS_GRXSTSR_Device {
                    FS_GRXSTSR_Device;
                    FRMNUM { FRMNUM }
                    PKTSTS { PKTSTS }
                    DPID { DPID }
                    BCNT { BCNT }
                    EPNUM { EPNUM }
                }
                FS_GRXSTSR_Host {
                    FS_GRXSTSR_Host;
                    PKTSTS { PKTSTS }
                    DPID { DPID }
                    BCNT { BCNT }
                    CHNUM { CHNUM }
                }
                FS_GRXSTSP_Device {
                    FS_GRXSTSP_Device;
                    FRMNUM { FRMNUM }
                    PKTSTS { PKTSTS }
                    DPID { DPID }
                    BCNT { BCNT }
                    EPNUM { EPNUM }
                }
                FS_GRXSTSP_Host {
                    FS_GRXSTSP_Host;
                    PKTSTS { PKTSTS }
                    DPID { DPID }
                    BCNT { BCNT }
                    CHNUM { CHNUM }
                }
                FS_GRXFSIZ {
                    FS_GRXFSIZ;
                    RXFD { RXFD }
                }
                FS_GNPTXFSIZ_Device {
                    FS_GNPTXFSIZ_Device;
                    TX0FD { TX0FD }
                    TX0FSA { TX0FSA }
                }
                FS_GNPTXFSIZ_Host {
                    FS_GNPTXFSIZ_Host;
                    NPTXFD { NPTXFD }
                    NPTXFSA { NPTXFSA }
                }
                FS_GNPTXSTS {
                    FS_GNPTXSTS;
                    NPTXQTOP { NPTXQTOP }
                    NPTQXSAV { NPTQXSAV }
                    NPTXFSAV { NPTXFSAV }
                }
                FS_GCCFG {
                    FS_GCCFG;
                    NOVBUSSENS { NOVBUSSENS }
                    SOFOUTEN { SOFOUTEN }
                    VBUSBSEN { VBUSBSEN }
                    VBUSASEN { VBUSASEN }
                    PWRDWN { PWRDWN }
                }
                FS_CID {
                    FS_CID;
                    PRODUCT_ID { PRODUCT_ID }
                }
                FS_HPTXFSIZ {
                    FS_HPTXFSIZ;
                    PTXFSIZ { PTXFSIZ }
                    PTXSA { PTXSA }
                }
                FS_DIEPTXF1 {
                    FS_DIEPTXF1;
                    INEPTXFD { INEPTXFD }
                    INEPTXSA { INEPTXSA }
                }
                FS_DIEPTXF2 {
                    FS_DIEPTXF2;
                    INEPTXFD { INEPTXFD }
                    INEPTXSA { INEPTXSA }
                }
                FS_DIEPTXF3 {
                    FS_DIEPTXF3;
                    INEPTXFD { INEPTXFD }
                    INEPTXSA { INEPTXSA }
                }
            }
        }
    }
}

#[cfg(any(
    stm32_mcu = "stm32f411",
))]
map_otgfs_global! {
    "Extracts USB-OTGFS register tokens.",
    periph_otgfs_global,
    "USB-OTGFS peripheral variant.",
    Otgfs,
    AHB2ENR,
    AHB2RSTR,
    AHB2LPENR,
    OTGFSEN,
    OTGFSRST,
    OTGFSLPEN,
    OTGFS,
}


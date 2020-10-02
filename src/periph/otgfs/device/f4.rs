//! USB on the go full speed (OTG_FS) device peripherals. 
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic USB-OTGFS peripheral variant.
    pub trait DeviceOtgfsMap {}

    /// Generic USB-OTGFS peripheral.
    pub struct DeviceOtgfsPeriph;

    OTG_FS_DEVICE {
	    FS_DCFG {
	        0x20 RwReg;
            PFIVL { RwRwRegFieldBits }
            DAD { RwRwRegFieldBits }
            NZLSOHSK { RwRwRegFieldBit }
            DSPD { RwRwRegFieldBits }
	    }
	    FS_DCTL {
	        0x20 RwReg;
            POPRGDNE { RwRwRegFieldBit }
            CGONAK { WoRwRegFieldBit }
            SGONAK { WoRwRegFieldBit }
            CGINAK { WoRwRegFieldBit }
            SGINAK { WoRwRegFieldBit }
            TCTL { RwRwRegFieldBits }
            GONSTS { RoRwRegFieldBit }
            GINSTS { RoRwRegFieldBit }
            SDIS { RwRwRegFieldBit }
            RWUSIG { RwRwRegFieldBit }
	    }
	    FS_DSTS {
            0x20 RoReg;
            FNSOF { RoRoRegFieldBits }
            EERR { RoRoRegFieldBit }
            ENUMSPD { RoRoRegFieldBits }
            SUSPSTS { RoRoRegFieldBit }
	    }
	    FS_DIEPMSK {
	        0x20 RwReg;
            NAKM { RwRwRegFieldBit }
            INEPNEM { RwRwRegFieldBit }
            INEPNMM { RwRwRegFieldBit }
            ITTXFEMSK { RwRwRegFieldBit }
            TOM { RwRwRegFieldBit }
            EPDM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
	    }
	    FS_DOEPMSK {	
	        0x20 RwReg;
            OTEPDM { RwRwRegFieldBit }
            STUPM { RwRwRegFieldBit }
            EPDM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
	    }
	    FS_DAINT {
	        0x20 RoReg;
            OEPINT { RoRoRegFieldBits }
            IEPINT { RoRoRegFieldBits }
	    }
	    FS_DAINTMSK {
	        0x20 RwReg;
            OEPINT { RwRwRegFieldBits }
            IEPM { RwRwRegFieldBits }
	    }
	    DVBUSDIS {
	        0x20 RwReg;
            VBUSDT { RwRwRegFieldBits }
	    }
	    DVBUSPULSE {
	        0x20 RwReg;
            DVBUSP { RwRwRegFieldBits }
	    }
	    DIEPEMPMSK {
	        0x20 RwReg;
            INEPTXFEM { RwRwRegFieldBits }
	    }
	    DIEPCTL0 {
	        0x20 RwReg;
            EPENA { RoRwRegFieldBit }
            EPDIS { RoRwRegFieldBit }
            SNAK { WoRwRegFieldBit }
            CNAK { WoRwRegFieldBit }
            TXFNUM { RwRwRegFieldBits }
            STALL { RwRwRegFieldBit }
            EPTYP { RoRwRegFieldBits }
            NAKSTS { RoRwRegFieldBit }
            USBAEP { RoRwRegFieldBit }
            MPSIZ { RwRwRegFieldBits }
	    }
	    DIEPCTL1 {
	        0x20 RwReg;
            EPENA { RwRwRegFieldBit }
            EPDIS { RwRwRegFieldBit }
            SODDFRM_SD1PID { WoRwRegFieldBit }
            SD0PID_SEVNFRM { WoRwRegFieldBit }
            SNAK { WoRwRegFieldBit }
            CNAK { WoRwRegFieldBit }
            TXFNUM { RwRwRegFieldBits }
            STALL { RwRwRegFieldBit }
            EPTYP { RwRwRegFieldBits }
            NAKSTS { RoRwRegFieldBit }
            EONUM_DPID { RoRwRegFieldBit }
            USBAEP { RwRwRegFieldBit }
            MPSIZ { RwRwRegFieldBits }
	    }
	    DIEPCTL2 {
	        0x20 RwReg;
            EPENA { RwRwRegFieldBit }
            EPDIS { RwRwRegFieldBit }
            SODDFRM_SD1PID { WoRwRegFieldBit }
            SD0PID_SEVNFRM { WoRwRegFieldBit }
            SNAK { WoRwRegFieldBit }
            CNAK { WoRwRegFieldBit }
            TXFNUM { RwRwRegFieldBits }
            STALL { RwRwRegFieldBit }
            EPTYP { RwRwRegFieldBits }
            NAKSTS { RoRwRegFieldBit }
            EONUM_DPID { RoRwRegFieldBit }
            USBAEP { RwRwRegFieldBit }
            MPSIZ { RwRwRegFieldBits }
	    }
	    DIEPCTL3 {
	        0x20 RwReg;
            EPENA { RwRwRegFieldBit }
            EPDIS { RwRwRegFieldBit }
            SODDFRM_SD1PID { WoRwRegFieldBit }
            SD0PID_SEVNFRM { WoRwRegFieldBit }
            SNAK { WoRwRegFieldBit }
            CNAK { WoRwRegFieldBit }
            TXFNUM { RwRwRegFieldBits }
            STALL { RwRwRegFieldBit }
            EPTYP { RwRwRegFieldBits }
            NAKSTS { RoRwRegFieldBit }
            EONUM_DPID { RoRwRegFieldBit }
            USBAEP { RwRwRegFieldBit }
            MPSIZ { RwRwRegFieldBits }
	    }
	    DOEPCTL0 {
	        0x20 RwReg;
            EPENA { WoRwRegFieldBit }
            EPDIS { RoRwRegFieldBit }
            SNAK { WoRwRegFieldBit }
            CNAK { WoRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            SNPM { RwRwRegFieldBit }
            EPTYP { RoRwRegFieldBits }
            NAKSTS { RoRwRegFieldBit }
            USBAEP { RoRwRegFieldBit }
            MPSIZ { RoRwRegFieldBits }
	    }
	    DOEPCTL1 {
	        0x20 RwReg;
            EPENA { RwRwRegFieldBit }
            EPDIS { RwRwRegFieldBit }
            SODDFRM_SD1PID { WoRwRegFieldBit }
            SD0PID_SEVNFRM { WoRwRegFieldBit }
            SNAK { WoRwRegFieldBit }
            CNAK { WoRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            SNPM { RwRwRegFieldBit }
            EPTYP { RwRwRegFieldBits }
            NAKSTS { RoRwRegFieldBit }
            EONUM_DPID { RoRwRegFieldBit }
            USBAEP { RwRwRegFieldBit }
            MPSIZ { RwRwRegFieldBits }
	    }
	    DOEPCTL2 {
	        0x20 RwReg;
            EPENA { RwRwRegFieldBit }
            EPDIS { RwRwRegFieldBit }
            SODDFRM_SD1PID { WoRwRegFieldBit }
            SD0PID_SEVNFRM { WoRwRegFieldBit }
            SNAK { WoRwRegFieldBit }
            CNAK { WoRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            SNPM { RwRwRegFieldBit }
            EPTYP { RwRwRegFieldBits }
            NAKSTS { RoRwRegFieldBit }
            EONUM_DPID { RoRwRegFieldBit }
            USBAEP { RwRwRegFieldBit }
            MPSIZ { RwRwRegFieldBits }
	    }
	    DOEPCTL3 {
	        0x20 RwReg;
            EPENA { RwRwRegFieldBit }
            EPDIS { RwRwRegFieldBit }
            SODDFRM_SD1PID { WoRwRegFieldBit }
            SD0PID_SEVNFRM { WoRwRegFieldBit }
            SNAK { WoRwRegFieldBit }
            CNAK { WoRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            SNPM { RwRwRegFieldBit }
            EPTYP { RwRwRegFieldBits }
            NAKSTS { RoRwRegFieldBit }
            EONUM_DPID { RoRwRegFieldBit }
            USBAEP { RwRwRegFieldBit }
            MPSIZ { RwRwRegFieldBits }
	    }
	    DIEPINT0 {
	        0x20 RwReg;
            TXFE { RoRwRegFieldBit }
            INEPNE { RwRwRegFieldBit }
            ITTXFE { RwRwRegFieldBit }
            TOC { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DIEPINT1 {
	        0x20 RwReg;
            TXFE { RoRwRegFieldBit }
            INEPNE { RwRwRegFieldBit }
            ITTXFE { RwRwRegFieldBit }
            TOC { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DIEPINT2 {
	        0x20 RwReg;
            TXFE { RoRwRegFieldBit }
            INEPNE { RwRwRegFieldBit }
            ITTXFE { RwRwRegFieldBit }
            TOC { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DIEPINT3 {
	        0x20 RwReg;
            TXFE { RoRwRegFieldBit }
            INEPNE { RwRwRegFieldBit }
            ITTXFE { RwRwRegFieldBit }
            TOC { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DOEPINT0 {
	        0x20 RwReg;
            B2BSTUP { RwRwRegFieldBit }
            OTEPDIS { RwRwRegFieldBit }
            STUP { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DOEPINT1 {
	        0x20 RwReg;
            B2BSTUP { RwRwRegFieldBit }
            OTEPDIS { RwRwRegFieldBit }
            STUP { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DOEPINT2 {
	        0x20 RwReg;
            B2BSTUP { RwRwRegFieldBit }
            OTEPDIS { RwRwRegFieldBit }
            STUP { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DOEPINT3 {
	        0x20 RwReg;
            B2BSTUP { RwRwRegFieldBit }
            OTEPDIS { RwRwRegFieldBit }
            STUP { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DIEPTSIZ0 {
	        0x20 RwReg;
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
	    }
	    DOEPTSIZ0 {
	        0x20 RwReg;
            STUPCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBit }
            XFRSIZ { RwRwRegFieldBits }
	    }
        DIEPTSIZ1 {
	        0x20 RwReg;
            MCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        DIEPTSIZ2 {
	        0x20 RwReg;
            MCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        DIEPTSIZ3 {
	        0x20 RwReg;
            MCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
	    DTXFSTS0 {
	        0x20 RoReg;
            INEPTFSAV { RoRoRegFieldBits }
	    }
	    DTXFSTS1 {
	        0x20 RoReg;
            INEPTFSAV { RoRoRegFieldBits }
	    }
	    DTXFSTS2 {
	        0x20 RoReg;
            INEPTFSAV { RoRoRegFieldBits }
	    }
	    DTXFSTS3 {
	        0x20 RoReg;
            INEPTFSAV { RoRoRegFieldBits }
	    }
	    DOEPTSIZ1 {
	        0x20 RwReg;
            RXDPID_STUPCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
	    DOEPTSIZ2 {
	        0x20 RwReg;
            RXDPID_STUPCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
	    DOEPTSIZ3 {
	        0x20 RwReg;
            RXDPID_STUPCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_otgfs_device {
    (
        $otgfs_macro_doc:expr,
        $otgfs_macro:ident,
        $otgfs_ty_doc:expr,
        $otgfs_ty:ident,
        $otgfs:ident,
    ) => {
        periph::map! {
            #[doc = $otgfs_macro_doc]
            pub macro $otgfs_macro;

            #[doc = $otgfs_ty_doc]
            pub struct $otgfs_ty;

            impl DeviceOtgfsMap for $otgfs_ty {}

            drone_stm32_map_pieces::reg;
            crate::device;

            OTG_FS_DEVICE {
                FS_DCFG {
        	        FS_DCFG;
                    PFIVL { PFIVL }
                    DAD { DAD }
                    NZLSOHSK { NZLSOHSK }
                    DSPD { DSPD }
        	    }
        	    FS_DCTL {
        	        FS_DCTL;
                    POPRGDNE { POPRGDNE }
                    CGONAK { CGONAK }
                    SGONAK { SGONAK }
                    CGINAK { CGINAK }
                    SGINAK { SGINAK }
                    TCTL { TCTL }
                    GONSTS { GONSTS }
                    GINSTS { GINSTS }
                    SDIS { SDIS }
                    RWUSIG { RWUSIG }
        	    }
        	    FS_DSTS {
                    FS_DSTS;
                    FNSOF { FNSOF }
                    EERR { EERR }
                    ENUMSPD { ENUMSPD }
                    SUSPSTS { SUSPSTS }
        	    }
        	    FS_DIEPMSK {
        	        FS_DIEPMSK;
                    NAKM { NAKM }
                    INEPNEM { INEPNEM }
                    INEPNMM { INEPNMM }
                    ITTXFEMSK { ITTXFEMSK }
                    TOM { TOM }
                    EPDM { EPDM }
                    XFRCM { XFRCM }
        	    }
        	    FS_DOEPMSK {	
        	        FS_DOEPMSK;
                    OTEPDM { OTEPDM }
                    STUPM { STUPM }
                    EPDM { EPDM }
                    XFRCM { XFRCM }
        	    }
        	    FS_DAINT {
        	        FS_DAINT;
                    OEPINT { OEPINT }
                    IEPINT { IEPINT }
        	    }
        	    FS_DAINTMSK {
        	        FS_DAINTMSK;
                    OEPINT { OEPINT }
                    IEPM { IEPM }
        	    }
        	    DVBUSDIS {
        	        DVBUSDIS;
                    VBUSDT { VBUSDT }
        	    }
        	    DVBUSPULSE {
        	        DVBUSPULSE;
                    DVBUSP { DVBUSP }
        	    }
        	    DIEPEMPMSK {
        	        DIEPEMPMSK;
                    INEPTXFEM { INEPTXFEM }
        	    }
        	    DIEPCTL0 {
        	        DIEPCTL0;
                    EPENA { EPENA }
                    EPDIS { EPDIS }
                    SNAK { SNAK }
                    CNAK { CNAK }
                    TXFNUM { TXFNUM }
                    STALL { STALL }
                    EPTYP { EPTYP }
                    NAKSTS { NAKSTS }
                    USBAEP { USBAEP }
                    MPSIZ { MPSIZ }
        	    }
        	    DIEPCTL1 {
        	        DIEPCTL1;
                    EPENA { EPENA }
                    EPDIS { EPDIS }
                    SODDFRM_SD1PID { SODDFRM_SD1PID }
                    SD0PID_SEVNFRM { SD0PID_SEVNFRM }
                    SNAK { SNAK }
                    CNAK { CNAK }
                    TXFNUM { TXFNUM }
                    STALL { STALL }
                    EPTYP { EPTYP }
                    NAKSTS { NAKSTS }
                    EONUM_DPID { EONUM_DPID }
                    USBAEP { USBAEP }
                    MPSIZ { MPSIZ }
        	    }
        	    DIEPCTL2 {
        	        DIEPCTL2;
                    EPENA { EPENA }
                    EPDIS { EPDIS }
                    SODDFRM_SD1PID { SODDFRM_SD1PID }
                    SD0PID_SEVNFRM { SD0PID_SEVNFRM }
                    SNAK { SNAK }
                    CNAK { CNAK }
                    TXFNUM { TXFNUM }
                    STALL { STALL }
                    EPTYP { EPTYP }
                    NAKSTS { NAKSTS }
                    EONUM_DPID { EONUM_DPID }
                    USBAEP { USBAEP }
                    MPSIZ { MPSIZ }
        	    }
        	    DIEPCTL3 {
        	        DIEPCTL3;
                    EPENA { EPENA }
                    EPDIS { EPDIS }
                    SODDFRM_SD1PID { SODDFRM_SD1PID }
                    SD0PID_SEVNFRM { SD0PID_SEVNFRM }
                    SNAK { SNAK }
                    CNAK { CNAK }
                    TXFNUM { TXFNUM }
                    STALL { STALL }
                    EPTYP { EPTYP }
                    NAKSTS { NAKSTS }
                    EONUM_DPID { EONUM_DPID }
                    USBAEP { USBAEP }
                    MPSIZ { MPSIZ }
        	    }
        	    DOEPCTL0 {
        	        DOEPCTL0;
                    EPENA { EPENA }
                    EPDIS { EPDIS }
                    SNAK { SNAK }
                    CNAK { CNAK }
                    STALL { STALL }
                    SNPM { SNPM }
                    EPTYP { EPTYP }
                    NAKSTS { NAKSTS }
                    USBAEP { USBAEP }
                    MPSIZ { MPSIZ }
        	    }
        	    DOEPCTL1 {
        	        DOEPCTL1;
                    EPENA { EPENA }
                    EPDIS { EPDIS }
                    SODDFRM_SD1PID { SODDFRM_SD1PID }
                    SD0PID_SEVNFRM { SD0PID_SEVNFRM }
                    SNAK { SNAK }
                    CNAK { CNAK }
                    STALL { STALL }
                    SNPM { SNPM }
                    EPTYP { EPTYP }
                    NAKSTS { NAKSTS }
                    EONUM_DPID { EONUM_DPID }
                    USBAEP { USBAEP }
                    MPSIZ { MPSIZ }
        	    }
        	    DOEPCTL2 {
        	        DOEPCTL2;
                    EPENA { EPENA }
                    EPDIS { EPDIS }
                    SODDFRM_SD1PID { SODDFRM_SD1PID }
                    SD0PID_SEVNFRM { SD0PID_SEVNFRM }
                    SNAK { SNAK }
                    CNAK { CNAK }
                    STALL { STALL }
                    SNPM { SNPM }
                    EPTYP { EPTYP }
                    NAKSTS { NAKSTS }
                    EONUM_DPID { EONUM_DPID }
                    USBAEP { USBAEP }
                    MPSIZ { MPSIZ }
        	    }
        	    DOEPCTL3 {
        	        DOEPCTL3;
                    EPENA { EPENA }
                    EPDIS { EPDIS }
                    SODDFRM_SD1PID { SODDFRM_SD1PID }
                    SD0PID_SEVNFRM { SD0PID_SEVNFRM }
                    SNAK { SNAK }
                    CNAK { CNAK }
                    STALL { STALL }
                    SNPM { SNPM }
                    EPTYP { EPTYP }
                    NAKSTS { NAKSTS }
                    EONUM_DPID { EONUM_DPID }
                    USBAEP { USBAEP }
                    MPSIZ { MPSIZ }
        	    }
        	    DIEPINT0 {
        	        DIEPINT0;
                    TXFE { TXFE }
                    INEPNE { INEPNE }
                    ITTXFE { ITTXFE }
                    TOC { TOC }
                    EPDISD { EPDISD }
                    XFRC { XFRC }
        	    }
        	    DIEPINT1 {
        	        DIEPINT1;
                    TXFE { TXFE }
                    INEPNE { INEPNE }
                    ITTXFE { ITTXFE }
                    TOC { TOC }
                    EPDISD { EPDISD }
                    XFRC { XFRC }
        	    }
        	    DIEPINT2 {
        	        DIEPINT2;
                    TXFE { TXFE }
                    INEPNE { INEPNE }
                    ITTXFE { ITTXFE }
                    TOC { TOC }
                    EPDISD { EPDISD }
                    XFRC { XFRC }
        	    }
        	    DIEPINT3 {
        	        DIEPINT3;
                    TXFE { TXFE }
                    INEPNE { INEPNE }
                    ITTXFE { ITTXFE }
                    TOC { TOC }
                    EPDISD { EPDISD }
                    XFRC { XFRC }
        	    }
        	    DOEPINT0 {
        	        DOEPINT0;
                    B2BSTUP { B2BSTUP }
                    OTEPDIS { OTEPDIS }
                    STUP { STUP }
                    EPDISD { EPDISD }
                    XFRC { XFRC }
        	    }
        	    DOEPINT1 {
        	        DOEPINT1;
                    B2BSTUP { B2BSTUP }
                    OTEPDIS { OTEPDIS }
                    STUP { STUP }
                    EPDISD { EPDISD }
                    XFRC { XFRC }
        	    }
        	    DOEPINT2 {
        	        DOEPINT2;
                    B2BSTUP { B2BSTUP }
                    OTEPDIS { OTEPDIS }
                    STUP { STUP }
                    EPDISD { EPDISD }
                    XFRC { XFRC }
        	    }
        	    DOEPINT3 {
        	        DOEPINT3;
                    B2BSTUP { B2BSTUP }
                    OTEPDIS { OTEPDIS }
                    STUP { STUP }
                    EPDISD { EPDISD }
                    XFRC { XFRC }
        	    }
        	    DIEPTSIZ0 {
        	        DIEPTSIZ0;
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
        	    }
        	    DOEPTSIZ0 {
        	        DOEPTSIZ0;
                    STUPCNT { STUPCNT }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
        	    }
                DIEPTSIZ1 {
                    DIEPTSIZ1;
                    MCNT { MCNT }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
                DIEPTSIZ2 {
                    DIEPTSIZ2;
                    MCNT { MCNT }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
                DIEPTSIZ3 {
                    DIEPTSIZ3;
                    MCNT { MCNT }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
        	    DTXFSTS0 {
        	        DTXFSTS0;
                    INEPTFSAV { INEPTFSAV }
        	    }
        	    DTXFSTS1 {
        	        DTXFSTS1;
                    INEPTFSAV { INEPTFSAV }
        	    }
        	    DTXFSTS2 {
        	        DTXFSTS2;
                    INEPTFSAV { INEPTFSAV }
        	    }
        	    DTXFSTS3 {
        	        DTXFSTS3;
                    INEPTFSAV { INEPTFSAV }
        	    }
        	    DOEPTSIZ1 {
        	        DOEPTSIZ1;
                    RXDPID_STUPCNT { RXDPID_STUPCNT }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
        	    DOEPTSIZ2 {
        	        DOEPTSIZ2;
                    RXDPID_STUPCNT { RXDPID_STUPCNT }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
        	    DOEPTSIZ3 {
        	        DOEPTSIZ3;
                    RXDPID_STUPCNT { RXDPID_STUPCNT }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
            }
        }
    }
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_device! {
    "Extracts USB-OTGFS register tokens.",
    periph_otgfs_device,
    "USB-OTGFS peripheral variant.",
    Otgfs,
    OTGFS,
}


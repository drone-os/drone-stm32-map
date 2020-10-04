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
	        0x20 RwReg Option;
            PFIVL { RwRwRegFieldBits }
            DAD { RwRwRegFieldBits }
            NZLSOHSK { RwRwRegFieldBit }
            DSPD { RwRwRegFieldBits }
	    }
	    FS_DCTL {
	        0x20 RwReg Option;
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
            0x20 RoReg Option;
            FNSOF { RoRoRegFieldBits }
            EERR { RoRoRegFieldBit }
            ENUMSPD { RoRoRegFieldBits }
            SUSPSTS { RoRoRegFieldBit }
	    }
	    FS_DIEPMSK {
	        0x20 RwReg Option;
            NAKM { RwRwRegFieldBit }
            INEPNEM { RwRwRegFieldBit }
            INEPNMM { RwRwRegFieldBit }
            ITTXFEMSK { RwRwRegFieldBit }
            TOM { RwRwRegFieldBit }
            EPDM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
	    }
	    FS_DOEPMSK {	
	        0x20 RwReg Option;
            OTEPDM { RwRwRegFieldBit }
            STUPM { RwRwRegFieldBit }
            EPDM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
	    }
	    FS_DAINT {
	        0x20 RoReg Option;
            OEPINT { RoRoRegFieldBits }
            IEPINT { RoRoRegFieldBits }
	    }
	    FS_DAINTMSK {
	        0x20 RwReg Option;
            OEPINT { RwRwRegFieldBits }
            IEPM { RwRwRegFieldBits }
	    }
	    DVBUSDIS {
	        0x20 RwReg Option;
            VBUSDT { RwRwRegFieldBits }
	    }
	    DVBUSPULSE {
	        0x20 RwReg Option;
            DVBUSP { RwRwRegFieldBits }
	    }
	    DIEPEMPMSK {
	        0x20 RwReg Option;
            INEPTXFEM { RwRwRegFieldBits }
	    }
	    DIEPCTL0 {
	        0x20 RwReg Option;
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
	        0x20 RwReg Option;
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
	        0x20 RwReg Option;
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
	        0x20 RwReg Option;
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
	        0x20 RwReg Option;
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
	        0x20 RwReg Option;
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
	        0x20 RwReg Option;
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
	        0x20 RwReg Option;
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
	        0x20 RwReg Option;
            TXFE { RoRwRegFieldBit }
            INEPNE { RwRwRegFieldBit }
            ITTXFE { RwRwRegFieldBit }
            TOC { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DIEPINT1 {
	        0x20 RwReg Option;
            TXFE { RoRwRegFieldBit }
            INEPNE { RwRwRegFieldBit }
            ITTXFE { RwRwRegFieldBit }
            TOC { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DIEPINT2 {
	        0x20 RwReg Option;
            TXFE { RoRwRegFieldBit }
            INEPNE { RwRwRegFieldBit }
            ITTXFE { RwRwRegFieldBit }
            TOC { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DIEPINT3 {
	        0x20 RwReg Option;
            TXFE { RoRwRegFieldBit }
            INEPNE { RwRwRegFieldBit }
            ITTXFE { RwRwRegFieldBit }
            TOC { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DOEPINT0 {
	        0x20 RwReg Option;
            B2BSTUP { RwRwRegFieldBit }
            OTEPDIS { RwRwRegFieldBit }
            STUP { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DOEPINT1 {
	        0x20 RwReg Option;
            B2BSTUP { RwRwRegFieldBit }
            OTEPDIS { RwRwRegFieldBit }
            STUP { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DOEPINT2 {
	        0x20 RwReg Option;
            B2BSTUP { RwRwRegFieldBit }
            OTEPDIS { RwRwRegFieldBit }
            STUP { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DOEPINT3 {
	        0x20 RwReg Option;
            B2BSTUP { RwRwRegFieldBit }
            OTEPDIS { RwRwRegFieldBit }
            STUP { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
	    }
	    DIEPTSIZ0 {
	        0x20 RwReg Option;
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
	    }
        DIEPTSIZ1 {
	        0x20 RwReg Option;
            MCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        DIEPTSIZ2 {
	        0x20 RwReg Option;
            MCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        DIEPTSIZ3 {
	        0x20 RwReg Option;
            MCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
	    DTXFSTS0 {
	        0x20 RoReg Option;
            INEPTFSAV { RoRoRegFieldBits }
	    }
	    DTXFSTS1 {
	        0x20 RoReg Option;
            INEPTFSAV { RoRoRegFieldBits }
	    }
	    DTXFSTS2 {
	        0x20 RoReg Option;
            INEPTFSAV { RoRoRegFieldBits }
	    }
	    DTXFSTS3 {
	        0x20 RoReg Option;
            INEPTFSAV { RoRoRegFieldBits }
	    }
	    DOEPTSIZ0 {
	        0x20 RwReg Option;
            STUPCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBit }
            XFRSIZ { RwRwRegFieldBits }
	    }
	    DOEPTSIZ1 {
	        0x20 RwReg Option;
            RXDPID_STUPCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
	    DOEPTSIZ2 {
	        0x20 RwReg Option;
            RXDPID_STUPCNT { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
	    DOEPTSIZ3 {
	        0x20 RwReg Option;
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
        ($($fs_dcfg:ident)?),
        ($($fs_dctl:ident)?),
        ($($fs_dsts:ident)?),
        ($($fs_diepmsk:ident)?),
        ($($fs_doepmsk:ident)?),
        ($($fs_daint:ident)?),
        ($($fs_daintmsk:ident)?),
        ($($dvbusdis:ident)?),
        ($($dvbuspulse:ident)?),
        ($($diepempmsk:ident)?),
        ($($diepctl0:ident)?),
        ($($diepctl1:ident)?),
        ($($diepctl2:ident)?),
        ($($diepctl3:ident)?),
        ($($doepctl0:ident)?),
        ($($doepctl1:ident)?),
        ($($doepctl2:ident)?),
        ($($doepctl3:ident)?),
        ($($diepint0:ident)?),
        ($($diepint1:ident)?),
        ($($diepint2:ident)?),
        ($($diepint3:ident)?),
        ($($doepint0:ident)?),
        ($($doepint1:ident)?),
        ($($doepint2:ident)?),
        ($($doepint3:ident)?),
        ($($dieptsiz0:ident)?),
        ($($dieptsiz1:ident)?),
        ($($dieptsiz2:ident)?),
        ($($dieptsiz3:ident)?),
        ($($dtxfsts0:ident)?),
        ($($dtxfsts1:ident)?),
        ($($dtxfsts2:ident)?),
        ($($dtxfsts3:ident)?),
        ($($doeptsiz0:ident)?),
        ($($doeptsiz1:ident)?),
        ($($doeptsiz2:ident)?),
        ($($doeptsiz3:ident)?),
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
                    $(
                        $fs_dcfg Option;
                        PFIVL { PFIVL }
                        DAD { DAD }
                        NZLSOHSK { NZLSOHSK }
                        DSPD { DSPD }
                    )*
        	    }
        	    FS_DCTL {
                    $(
                        $fs_dctl Option;
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
                    )*
        	    }
        	    FS_DSTS {
                    $(
                        $fs_dsts Option;
                        FNSOF { FNSOF }
                        EERR { EERR }
                        ENUMSPD { ENUMSPD }
                        SUSPSTS { SUSPSTS }
                    )*
        	    }
        	    FS_DIEPMSK {
                    $(
                        $fs_diepmsk Option;
                        NAKM { NAKM }
                        INEPNEM { INEPNEM }
                        INEPNMM { INEPNMM }
                        ITTXFEMSK { ITTXFEMSK }
                        TOM { TOM }
                        EPDM { EPDM }
                        XFRCM { XFRCM }
                    )*
        	    }
        	    FS_DOEPMSK {	
                    $(
                        $fs_doepmsk Option;
                        OTEPDM { OTEPDM }
                        STUPM { STUPM }
                        EPDM { EPDM }
                        XFRCM { XFRCM }
                    )*
        	    }
        	    FS_DAINT {
                    $(
                        $fs_daint Option;
                        OEPINT { OEPINT }
                        IEPINT { IEPINT }
                    )*
        	    }
        	    FS_DAINTMSK {
                    $(
                        $fs_daintmsk Option;
                        OEPINT { OEPINT }
                        IEPM { IEPM }
                    )*
        	    }
        	    DVBUSDIS {
                    $(
                        $dvbusdis Option;
                        VBUSDT { VBUSDT }
                    )*
        	    }
        	    DVBUSPULSE {
                    $(
                        $dvbuspulse Option;
                        DVBUSP { DVBUSP }
                    )*
        	    }
        	    DIEPEMPMSK {
                    $(
                        $diepempmsk Option;
                        INEPTXFEM { INEPTXFEM }
                    )*
        	    }
        	    DIEPCTL0 {
                    $(
                        $diepctl0 Option;
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
                    )*
        	    }
        	    DIEPCTL1 {
                    $(
                        $diepctl1 Option;
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
                    )*
        	    }
        	    DIEPCTL2 {
                    $(
                        $diepctl2 Option;
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
                    )*
        	    }
        	    DIEPCTL3 {
                    $(
                        $diepctl3 Option;
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
                    )*
        	    }
        	    DOEPCTL0 {
                    $(
                        $doepctl0 Option;
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
                    )*
        	    }
        	    DOEPCTL1 {
                    $(
                        $doepctl1 Option;
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
                    )*
        	    }
        	    DOEPCTL2 {
                    $(
                        $doepctl2 Option;
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
                    )*
        	    }
        	    DOEPCTL3 {
                    $(
                        $doepctl3 Option;
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
                    )*
        	    }
        	    DIEPINT0 {
                    $(
                        $diepint0 Option;
                        TXFE { TXFE }
                        INEPNE { INEPNE }
                        ITTXFE { ITTXFE }
                        TOC { TOC }
                        EPDISD { EPDISD }
                        XFRC { XFRC }
                    )*
        	    }
        	    DIEPINT1 {
                    $(
                        $diepint1 Option;
                        TXFE { TXFE }
                        INEPNE { INEPNE }
                        ITTXFE { ITTXFE }
                        TOC { TOC }
                        EPDISD { EPDISD }
                        XFRC { XFRC }
                    )*
        	    }
        	    DIEPINT2 {
                    $(
                        $diepint2 Option;
                        TXFE { TXFE }
                        INEPNE { INEPNE }
                        ITTXFE { ITTXFE }
                        TOC { TOC }
                        EPDISD { EPDISD }
                        XFRC { XFRC }
                    )*
        	    }
        	    DIEPINT3 {
                    $(
                        $diepint3 Option;
                        TXFE { TXFE }
                        INEPNE { INEPNE }
                        ITTXFE { ITTXFE }
                        TOC { TOC }
                        EPDISD { EPDISD }
                        XFRC { XFRC }
                    )*
        	    }
        	    DOEPINT0 {
                    $(
                        $doepint0 Option;
                        B2BSTUP { B2BSTUP }
                        OTEPDIS { OTEPDIS }
                        STUP { STUP }
                        EPDISD { EPDISD }
                        XFRC { XFRC }
                    )*
        	    }
        	    DOEPINT1 {
                    $(
                        $doepint1 Option;
                        B2BSTUP { B2BSTUP }
                        OTEPDIS { OTEPDIS }
                        STUP { STUP }
                        EPDISD { EPDISD }
                        XFRC { XFRC }
                    )*
        	    }
        	    DOEPINT2 {
                    $(
                        $doepint2 Option;
                        B2BSTUP { B2BSTUP }
                        OTEPDIS { OTEPDIS }
                        STUP { STUP }
                        EPDISD { EPDISD }
                        XFRC { XFRC }
                    )*
        	    }
        	    DOEPINT3 {
                    $(
                        $doepint3 Option;
                        B2BSTUP { B2BSTUP }
                        OTEPDIS { OTEPDIS }
                        STUP { STUP }
                        EPDISD { EPDISD }
                        XFRC { XFRC }
                    )*
        	    }
        	    DIEPTSIZ0 {
                    $(
                        $dieptsiz0 Option;
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
        	    }
                DIEPTSIZ1 {
                    $(
                        $dieptsiz1 Option;
                        MCNT { MCNT }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
                DIEPTSIZ2 {
                    $(
                        $dieptsiz2 Option;
                        MCNT { MCNT }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
                DIEPTSIZ3 {
                    $(
                        $dieptsiz3 Option;
                        MCNT { MCNT }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
        	    DTXFSTS0 {
                    $(
                        $dtxfsts0 Option;
                        INEPTFSAV { INEPTFSAV }
                    )*
        	    }
        	    DTXFSTS1 {
                    $(
                        $dtxfsts1 Option;
                        INEPTFSAV { INEPTFSAV }
                    )*
        	    }
        	    DTXFSTS2 {
                    $(
                        $dtxfsts2 Option;
                        INEPTFSAV { INEPTFSAV }
                    )*
        	    }
        	    DTXFSTS3 {
                    $(
                        $dtxfsts3 Option;
                        INEPTFSAV { INEPTFSAV }
                    )*
        	    }
        	    DOEPTSIZ0 {
                    $(
                        $doeptsiz0 Option;
                        STUPCNT { STUPCNT }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
        	    }
        	    DOEPTSIZ1 {
                    $(
                        $doeptsiz1 Option;
                        RXDPID_STUPCNT { RXDPID_STUPCNT }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
        	    DOEPTSIZ2 {
                    $(
                        $doeptsiz2 Option;
                        RXDPID_STUPCNT { RXDPID_STUPCNT }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
        	    DOEPTSIZ3 {
                    $(
                        $doeptsiz3 Option;
                        RXDPID_STUPCNT { RXDPID_STUPCNT }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
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
    (FS_DCFG),
    (FS_DCTL),
    (FS_DSTS),
    (FS_DIEPMSK),
    (FS_DOEPMSK),
    (FS_DAINT),
    (FS_DAINTMSK),
    (DVBUSDIS),
    (DVBUSPULSE),
    (DIEPEMPMSK),
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

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_device! {
    "Extracts USB-OTGFS register tokens for Endpoint 0.",
    periph_otgfs_device_ep0,
    "Endpoint 0 USB-OTGFS peripheral variant.",
    OtgfsEp0,
    OTGFSEP0,
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
    (DIEPCTL0),
    (),
    (),
    (),
    (DOEPCTL0),
    (),
    (),
    (),
    (DIEPINT0),
    (),
    (),
    (),
    (DOEPINT0),
    (),
    (),
    (),
    (DIEPTSIZ0),
    (),
    (),
    (),
    (DTXFSTS0),
    (),
    (),
    (),
    (DOEPTSIZ0),
    (),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_device! {
    "Extracts USB-OTGFS register tokens for Endpoint 1.",
    periph_otgfs_device_ep1,
    "Endpoint 1 USB-OTGFS peripheral variant.",
    OtgfsEp1,
    OTGFSEP1,
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
    (DIEPCTL1),
    (),
    (),
    (),
    (DOEPCTL1),
    (),
    (),
    (),
    (DIEPINT1),
    (),
    (),
    (),
    (DOEPINT1),
    (),
    (),
    (),
    (DIEPTSIZ1),
    (),
    (),
    (),
    (DTXFSTS1),
    (),
    (),
    (),
    (DOEPTSIZ1),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_device! {
    "Extracts USB-OTGFS register tokens for Endpoint 2.",
    periph_otgfs_device_ep2,
    "Endpoint 2 USB-OTGFS peripheral variant.",
    OtgfsEp2,
    OTGFSEP2,
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
    (DIEPCTL2),
    (),
    (),
    (),
    (DOEPCTL2),
    (),
    (),
    (),
    (DIEPINT2),
    (),
    (),
    (),
    (DOEPINT2),
    (),
    (),
    (),
    (DIEPTSIZ2),
    (),
    (),
    (),
    (DTXFSTS2),
    (),
    (),
    (),
    (DOEPTSIZ2),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_device! {
    "Extracts USB-OTGFS register tokens for Endpoint 3.",
    periph_otgfs_device_ep3,
    "Endpoint 3 USB-OTGFS peripheral variant.",
    OtgfsEp3,
    OTGFSEP3,
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
    (DIEPCTL3),
    (),
    (),
    (),
    (DOEPCTL3),
    (),
    (),
    (),
    (DIEPINT3),
    (),
    (),
    (),
    (DOEPINT3),
    (),
    (),
    (),
    (DIEPTSIZ3),
    (),
    (),
    (),
    (DTXFSTS3),
    (),
    (),
    (),
    (DOEPTSIZ3),
}


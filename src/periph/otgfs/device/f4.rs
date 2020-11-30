//! USB on the go full speed (`OTG_FS`) device peripherals. 
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
        DIEPCTL {
            0x20 RwReg Option;
            EPENA_RO { RoRwRegFieldBit Option }
            EPENA { RwRwRegFieldBit Option }
            EPDIS_RO { RoRwRegFieldBit Option }
            EPDIS { RwRwRegFieldBit Option }
            SODDFRM_SD1PID { WoRwRegFieldBit Option }
            SD0PID_SEVNFRM { WoRwRegFieldBit Option }
            SNAK { WoRwRegFieldBit }
            CNAK { WoRwRegFieldBit }
            TXFNUM { RwRwRegFieldBits }
            STALL { RwRwRegFieldBit }
            EPTYP_RO { RoRwRegFieldBits Option }
            EPTYP { RwRwRegFieldBits Option }
            NAKSTS { RoRwRegFieldBit }
            EONUM_DPID { RoRwRegFieldBit Option }
            USBAEP_RO { RoRwRegFieldBit Option }
            USBAEP { RwRwRegFieldBit Option }
            MPSIZ { RwRwRegFieldBits }
        }
        DOEPCTL {
            0x20 RwReg Option;
            EPENA_WO { WoRwRegFieldBit Option }
            EPENA { RwRwRegFieldBit Option }
            EPDIS_RO { RoRwRegFieldBit Option }
            EPDIS { RwRwRegFieldBit Option }
            SODDFRM_SD1PID { WoRwRegFieldBit Option }
            SD0PID_SEVNFRM { WoRwRegFieldBit Option }
            SNAK { WoRwRegFieldBit }
            CNAK { WoRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            SNPM { RwRwRegFieldBit }
            EPTYP_RO { RoRwRegFieldBits Option }
            EPTYP { RwRwRegFieldBits Option }
            NAKSTS { RoRwRegFieldBit }
            EONUM_DPID { RoRwRegFieldBit Option }
            USBAEP_RO { RoRwRegFieldBit Option }
            USBAEP { RwRwRegFieldBit Option }
            MPSIZ_RO { RoRwRegFieldBits Option }
            MPSIZ { RwRwRegFieldBits Option }
        }
        DIEPINT {
            0x20 RwReg Option;
            TXFE { RoRwRegFieldBit }
            INEPNE { RwRwRegFieldBit }
            ITTXFE { RwRwRegFieldBit }
            TOC { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
        }
        DOEPINT {
            0x20 RwReg Option;
            B2BSTUP { RwRwRegFieldBit }
            OTEPDIS { RwRwRegFieldBit }
            STUP { RwRwRegFieldBit }
            EPDISD { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
        }
        DIEPTSIZ {
            0x20 RwReg Option;
            MCNT { RwRwRegFieldBits Option }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        DTXFSTS {
            0x20 RoReg Option;
            INEPTFSAV { RoRoRegFieldBits }
        }
        DOEPTSIZ {
            0x20 RwReg Option;
            STUPCNT { RwRwRegFieldBits Option }
            RXDPID_STUPCNT { RwRwRegFieldBits Option }
            PKTCNT_BIT { RwRwRegFieldBit Option }
            PKTCNT_BITS { RwRwRegFieldBits Option }
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
        ($(
            $diepctl:ident,
            $($epena_ro:ident)?,
            $($epena:ident)?,
            $($epdis_ro:ident)?,
            $($epdis:ident)?,
            $($sodfrm_sd1pid:ident)?,
            $($sd0pid_sevnfrm:ident)?,
            $($eptyp_ro:ident)?,
            $($eptyp:ident)?,
            $($eonum_dpid:ident)?,
            $($usbaep_ro:ident)?,
            $($usbaep:ident)?,
        )?),
        ($(
            $doepctl:ident,
            $($do_epena_wo:ident)?,
            $($do_epena:ident)?,
            $($do_epdis_ro:ident)?,
            $($do_epdis:ident)?,
            $($do_sodfrm_sd1pid:ident)?,
            $($do_sd0pid_sevnfrm:ident)?,
            $($do_eptyp_ro:ident)?,
            $($do_eptyp:ident)?,
            $($do_eonum_dpid:ident)?,
            $($do_usbaep_ro:ident)?,
            $($do_usbaep:ident)?,
            $($do_mpsiz_ro:ident)?,
            $($do_mpsiz:ident)?,
        )?),
        ($($diepint:ident)?),
        ($($doepint:ident)?),
        ($(
            $dieptsiz:ident,
            $($mcnt:ident)?,
        )?),
        ($($dtxfsts:ident)?),
        ($(
            $doeptsiz:ident,
            $($stupcnt:ident)?,
            $($rxdpid_stupcnt:ident)?,
            $($pktcnt_bit:ident)?,
            $($pktcnt_bits:ident)?,
        )?),
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
                DIEPCTL {
                    $(
                        $diepctl Option;
                        EPENA_RO { $($epena_ro Option)* } 
                        EPENA { $($epena Option)* } 
                        EPDIS_RO { $($epdis_ro Option)* }
                        EPDIS { $($epdis Option)* }
                        SODDFRM_SD1PID { $($sodfrm_sd1pid Option)* } 
                        SD0PID_SEVNFRM { $($sd0pid_sevnfrm Option)* }
                        SNAK { SNAK }
                        CNAK { CNAK }
                        TXFNUM { TXFNUM }
                        STALL { STALL }
                        EPTYP_RO { $($eptyp_ro Option)* }
                        EPTYP { $($eptyp Option)* }
                        NAKSTS { NAKSTS }
                        EONUM_DPID { $($eonum_dpid Option)* }
                        USBAEP_RO { $($usbaep_ro Option)* }
                        USBAEP { $($usbaep Option)* }
                        MPSIZ { MPSIZ }
                    )*
                }
                DOEPCTL {
                    $(
                        $doepctl Option;
                        EPENA_WO { $($do_epena_wo Option)* } 
                        EPENA { $($do_epena Option)* } 
                        EPDIS_RO { $($do_epdis_ro Option)* }
                        EPDIS { $($do_epdis Option)* }
                        SODDFRM_SD1PID { $($do_sodfrm_sd1pid Option)* } 
                        SD0PID_SEVNFRM { $($do_sd0pid_sevnfrm Option)* }
                        SNAK { SNAK }
                        CNAK { CNAK }
                        STALL { STALL }
                        SNPM { SNPM }
                        EPTYP_RO { $($do_eptyp_ro Option)* }
                        EPTYP { $($do_eptyp Option)* }
                        NAKSTS { NAKSTS }
                        EONUM_DPID { $($do_eonum_dpid Option)* }
                        USBAEP_RO { $($do_usbaep_ro Option)* }
                        USBAEP { $($do_usbaep Option)* }
                        MPSIZ_RO { $($do_mpsiz_ro Option)* }
                        MPSIZ { $($do_mpsiz Option)* }
                    )*
                }
                DIEPINT {
                    $(
                        $diepint Option;
                        TXFE { TXFE }
                        INEPNE { INEPNE }
                        ITTXFE { ITTXFE }
                        TOC { TOC }
                        EPDISD { EPDISD }
                        XFRC { XFRC }
                    )*
                }
                DOEPINT {
                    $(
                        $doepint Option;
                        B2BSTUP { B2BSTUP }
                        OTEPDIS { OTEPDIS }
                        STUP { STUP }
                        EPDISD { EPDISD }
                        XFRC { XFRC }
                    )*
                }
                DIEPTSIZ {
                    $(
                        $dieptsiz Option;
                        MCNT { $($mcnt Option)* }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
                DTXFSTS {
                    $(
                        $dtxfsts Option;
                        INEPTFSAV { INEPTFSAV }
                    )*
                }
                DOEPTSIZ {
                    $(
                        $doeptsiz Option;
                        STUPCNT { $($stupcnt Option)* }
                        RXDPID_STUPCNT { $($rxdpid_stupcnt Option)* }
                        PKTCNT_BIT { $($pktcnt_bit Option)* }
                        PKTCNT_BITS { $($pktcnt_bits Option)* }
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
    "Extracts USB-OTGFS register tokens.", periph_otgfs_device,
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
    (DIEPCTL0,EPENA,,EPDIS,,,,EPTYP,,,USBAEP,,),
    (DOEPCTL0,EPENA,,EPDIS,,,,EPTYP,,,USBAEP,,MPSIZ,,),
    (DIEPINT0),
    (DOEPINT0),
    (DIEPTSIZ0,,),
    (DTXFSTS0),
    (DOEPTSIZ0,STUPCNT,,PKTCNT,,),
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
    (DIEPCTL1,,EPENA,,EPDIS,SODDFRM_SD1PID,SD0PID_SEVNFRM,,EPTYP,EONUM_DPID,,USBAEP,),
    (DOEPCTL1,,EPENA,,EPDIS,SODDFRM_SD1PID,SD0PID_SEVNFRM,,EPTYP,EONUM_DPID,,USBAEP,,MPSIZ,),
    (DIEPINT1),
    (DOEPINT1),
    (DIEPTSIZ1,MCNT,),
    (DTXFSTS1),
    (DOEPTSIZ1,,RXDPID_STUPCNT,,PKTCNT,),
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
    (DIEPCTL2,,EPENA,,EPDIS,SODDFRM_SD1PID,SD0PID_SEVNFRM,,EPTYP,EONUM_DPID,,USBAEP,),
    (DOEPCTL2,,EPENA,,EPDIS,SODDFRM_SD1PID,SD0PID_SEVNFRM,,EPTYP,EONUM_DPID,,USBAEP,,MPSIZ,),
    (DIEPINT2),
    (DOEPINT2),
    (DIEPTSIZ2,MCNT,),
    (DTXFSTS2),
    (DOEPTSIZ2,,RXDPID_STUPCNT,,PKTCNT,),
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
    (DIEPCTL3,,EPENA,,EPDIS,SODDFRM_SD1PID,SD0PID_SEVNFRM,,EPTYP,EONUM_DPID,,USBAEP,),
    (DOEPCTL3,,EPENA,,EPDIS,SODDFRM_SD1PID,SD0PID_SEVNFRM,,EPTYP,EONUM_DPID,,USBAEP,,MPSIZ,),
    (DIEPINT3),
    (DOEPINT3),
    (DIEPTSIZ3,MCNT,),
    (DTXFSTS3),
    (DOEPTSIZ3,,RXDPID_STUPCNT,,PKTCNT,),
}

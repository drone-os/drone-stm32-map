//! USB on the go full speed (OTG_FS) host peripherals. 
//!
//! For STM32F4 series of high-performance MCUs with DSP and FPU instructions.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic USB-OTGFS peripheral variant.
    pub trait HostOtgfsMap {}

    /// Generic USB-OTGFS peripheral.
    pub struct HostOtgfsPeriph;

    OTG_FS_HOST {
        FS_HCFG {
            0x20 RwReg Option;
            FSLSS { RoRwRegFieldBit }
            FSLSPCS { RwRwRegFieldBits }
        }
        HFIR {
            0x20 RwReg Option;
            FRIVL { RwRwRegFieldBits }
        }
        FS_HFNUM {
            0x20 RoReg Option;
            FTREM { RoRoRegFieldBits }
            FRNUM { RoRoRegFieldBits }
        }
        FS_HPTXSTS {
            0x20 RwReg Option;
            PTXQTOP { RoRwRegFieldBits }
            PTXQSAV { RoRwRegFieldBits }
            PTXFSAVL { RwRwRegFieldBits }
        }
        HAINT {
            0x20 RoReg Option;
            HAINT { RoRoRegFieldBits }
        }
        HAINTMSK {
            0x20 RwReg Option;
            HAINTM { RwRwRegFieldBits }
        }
        FS_HPRT {
            0x20 RwReg Option;
            PSPD { RoRwRegFieldBits }
            PTCTL { RwRwRegFieldBits }
            PPWR { RwRwRegFieldBit }
            PLSTS { RoRwRegFieldBits }
            PRST { RwRwRegFieldBit }
            PSUSP { RwRwRegFieldBit }
            PRES { RwRwRegFieldBit }
            POCCHNG { RwRwRegFieldBit }
            POCA { RoRwRegFieldBit }
            PENCHNG { RwRwRegFieldBit }
            PENA { RwRwRegFieldBit }
            PCDET { RwRwRegFieldBit }
            PCSTS { RoRwRegFieldBit }
        }
        FS_HCCHAR0 {
            0x20 RwReg Option;
            CHENA { RwRwRegFieldBit }
            CHDIS { RwRwRegFieldBit }
            ODDFRM { RwRwRegFieldBit }
            DAD { RwRwRegFieldBits }
            MCNT { RwRwRegFieldBits }
            EPTYP { RwRwRegFieldBits }
            LSDEV { RwRwRegFieldBit }
            EPDIR { RwRwRegFieldBit }
            EPNUM { RwRwRegFieldBits }
            MPSIZ { RwRwRegFieldBits }
        }
        FS_HCCHAR1 {
            0x20 RwReg Option;
            CHENA { RwRwRegFieldBit }
            CHDIS { RwRwRegFieldBit }
            ODDFRM { RwRwRegFieldBit }
            DAD { RwRwRegFieldBits }
            MCNT { RwRwRegFieldBits }
            EPTYP { RwRwRegFieldBits }
            LSDEV { RwRwRegFieldBit }
            EPDIR { RwRwRegFieldBit }
            EPNUM { RwRwRegFieldBits }
            MPSIZ { RwRwRegFieldBits }
        }
        FS_HCCHAR2 {
            0x20 RwReg Option;
            CHENA { RwRwRegFieldBit }
            CHDIS { RwRwRegFieldBit }
            ODDFRM { RwRwRegFieldBit }
            DAD { RwRwRegFieldBits }
            MCNT { RwRwRegFieldBits }
            EPTYP { RwRwRegFieldBits }
            LSDEV { RwRwRegFieldBit }
            EPDIR { RwRwRegFieldBit }
            EPNUM { RwRwRegFieldBits }
            MPSIZ { RwRwRegFieldBits }
        }
        FS_HCCHAR3 {
            0x20 RwReg Option;
            CHENA { RwRwRegFieldBit }
            CHDIS { RwRwRegFieldBit }
            ODDFRM { RwRwRegFieldBit }
            DAD { RwRwRegFieldBits }
            MCNT { RwRwRegFieldBits }
            EPTYP { RwRwRegFieldBits }
            LSDEV { RwRwRegFieldBit }
            EPDIR { RwRwRegFieldBit }
            EPNUM { RwRwRegFieldBits }
            MPSIZ { RwRwRegFieldBits }
        }
        FS_HCCHAR4 {
            0x20 RwReg Option;
            CHENA { RwRwRegFieldBit }
            CHDIS { RwRwRegFieldBit }
            ODDFRM { RwRwRegFieldBit }
            DAD { RwRwRegFieldBits }
            MCNT { RwRwRegFieldBits }
            EPTYP { RwRwRegFieldBits }
            LSDEV { RwRwRegFieldBit }
            EPDIR { RwRwRegFieldBit }
            EPNUM { RwRwRegFieldBits }
            MPSIZ { RwRwRegFieldBits }
        }
        FS_HCCHAR5 {
            0x20 RwReg Option;
            CHENA { RwRwRegFieldBit }
            CHDIS { RwRwRegFieldBit }
            ODDFRM { RwRwRegFieldBit }
            DAD { RwRwRegFieldBits }
            MCNT { RwRwRegFieldBits }
            EPTYP { RwRwRegFieldBits }
            LSDEV { RwRwRegFieldBit }
            EPDIR { RwRwRegFieldBit }
            EPNUM { RwRwRegFieldBits }
            MPSIZ { RwRwRegFieldBits }
        }
        FS_HCCHAR6 {
            0x20 RwReg Option;
            CHENA { RwRwRegFieldBit }
            CHDIS { RwRwRegFieldBit }
            ODDFRM { RwRwRegFieldBit }
            DAD { RwRwRegFieldBits }
            MCNT { RwRwRegFieldBits }
            EPTYP { RwRwRegFieldBits }
            LSDEV { RwRwRegFieldBit }
            EPDIR { RwRwRegFieldBit }
            EPNUM { RwRwRegFieldBits }
            MPSIZ { RwRwRegFieldBits }
        }
        FS_HCCHAR7 {
            0x20 RwReg Option;
            CHENA { RwRwRegFieldBit }
            CHDIS { RwRwRegFieldBit }
            ODDFRM { RwRwRegFieldBit }
            DAD { RwRwRegFieldBits }
            MCNT { RwRwRegFieldBits }
            EPTYP { RwRwRegFieldBits }
            LSDEV { RwRwRegFieldBit }
            EPDIR { RwRwRegFieldBit }
            EPNUM { RwRwRegFieldBits }
            MPSIZ { RwRwRegFieldBits }
        }
        FS_HCINT0 {
            0x20 RwReg Option;
            DTERR { RwRwRegFieldBit }
            FRMOR { RwRwRegFieldBit }
            BBERR { RwRwRegFieldBit }
            TXERR { RwRwRegFieldBit }
            ACK { RwRwRegFieldBit }
            NAK { RwRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            CHH { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
        }
        FS_HCINT1 {
            0x20 RwReg Option;
            DTERR { RwRwRegFieldBit }
            FRMOR { RwRwRegFieldBit }
            BBERR { RwRwRegFieldBit }
            TXERR { RwRwRegFieldBit }
            ACK { RwRwRegFieldBit }
            NAK { RwRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            CHH { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
        }
        FS_HCINT2 {
            0x20 RwReg Option;
            DTERR { RwRwRegFieldBit }
            FRMOR { RwRwRegFieldBit }
            BBERR { RwRwRegFieldBit }
            TXERR { RwRwRegFieldBit }
            ACK { RwRwRegFieldBit }
            NAK { RwRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            CHH { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
        }
        FS_HCINT3 {
            0x20 RwReg Option;
            DTERR { RwRwRegFieldBit }
            FRMOR { RwRwRegFieldBit }
            BBERR { RwRwRegFieldBit }
            TXERR { RwRwRegFieldBit }
            ACK { RwRwRegFieldBit }
            NAK { RwRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            CHH { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
        }
        FS_HCINT4 {
            0x20 RwReg Option;
            DTERR { RwRwRegFieldBit }
            FRMOR { RwRwRegFieldBit }
            BBERR { RwRwRegFieldBit }
            TXERR { RwRwRegFieldBit }
            ACK { RwRwRegFieldBit }
            NAK { RwRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            CHH { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
        }
        FS_HCINT5 {
            0x20 RwReg Option;
            DTERR { RwRwRegFieldBit }
            FRMOR { RwRwRegFieldBit }
            BBERR { RwRwRegFieldBit }
            TXERR { RwRwRegFieldBit }
            ACK { RwRwRegFieldBit }
            NAK { RwRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            CHH { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
        }
        FS_HCINT6 {
            0x20 RwReg Option;
            DTERR { RwRwRegFieldBit }
            FRMOR { RwRwRegFieldBit }
            BBERR { RwRwRegFieldBit }
            TXERR { RwRwRegFieldBit }
            ACK { RwRwRegFieldBit }
            NAK { RwRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            CHH { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
        }
        FS_HCINT7 {
            0x20 RwReg Option;
            DTERR { RwRwRegFieldBit }
            FRMOR { RwRwRegFieldBit }
            BBERR { RwRwRegFieldBit }
            TXERR { RwRwRegFieldBit }
            ACK { RwRwRegFieldBit }
            NAK { RwRwRegFieldBit }
            STALL { RwRwRegFieldBit }
            CHH { RwRwRegFieldBit }
            XFRC { RwRwRegFieldBit }
        }
        FS_HCINTMSK0 {
            0x20 RwReg Option;
            DTERRM { RwRwRegFieldBit }
            FRMORM { RwRwRegFieldBit }
            BBERRM { RwRwRegFieldBit }
            TXERRM { RwRwRegFieldBit }
            NYET { RwRwRegFieldBit }
            ACKM { RwRwRegFieldBit }
            NAKM { RwRwRegFieldBit }
            STALLM { RwRwRegFieldBit }
            CHHM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
        }
        FS_HCINTMSK1 {
            0x20 RwReg Option;
            DTERRM { RwRwRegFieldBit }
            FRMORM { RwRwRegFieldBit }
            BBERRM { RwRwRegFieldBit }
            TXERRM { RwRwRegFieldBit }
            NYET { RwRwRegFieldBit }
            ACKM { RwRwRegFieldBit }
            NAKM { RwRwRegFieldBit }
            STALLM { RwRwRegFieldBit }
            CHHM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
        }
        FS_HCINTMSK2 {
            0x20 RwReg Option;
            DTERRM { RwRwRegFieldBit }
            FRMORM { RwRwRegFieldBit }
            BBERRM { RwRwRegFieldBit }
            TXERRM { RwRwRegFieldBit }
            NYET { RwRwRegFieldBit }
            ACKM { RwRwRegFieldBit }
            NAKM { RwRwRegFieldBit }
            STALLM { RwRwRegFieldBit }
            CHHM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
        }
        FS_HCINTMSK3 {
            0x20 RwReg Option;
            DTERRM { RwRwRegFieldBit }
            FRMORM { RwRwRegFieldBit }
            BBERRM { RwRwRegFieldBit }
            TXERRM { RwRwRegFieldBit }
            NYET { RwRwRegFieldBit }
            ACKM { RwRwRegFieldBit }
            NAKM { RwRwRegFieldBit }
            STALLM { RwRwRegFieldBit }
            CHHM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
        }
        FS_HCINTMSK4 {
            0x20 RwReg Option;
            DTERRM { RwRwRegFieldBit }
            FRMORM { RwRwRegFieldBit }
            BBERRM { RwRwRegFieldBit }
            TXERRM { RwRwRegFieldBit }
            NYET { RwRwRegFieldBit }
            ACKM { RwRwRegFieldBit }
            NAKM { RwRwRegFieldBit }
            STALLM { RwRwRegFieldBit }
            CHHM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
        }
        FS_HCINTMSK5 {
            0x20 RwReg Option;
            DTERRM { RwRwRegFieldBit }
            FRMORM { RwRwRegFieldBit }
            BBERRM { RwRwRegFieldBit }
            TXERRM { RwRwRegFieldBit }
            NYET { RwRwRegFieldBit }
            ACKM { RwRwRegFieldBit }
            NAKM { RwRwRegFieldBit }
            STALLM { RwRwRegFieldBit }
            CHHM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
        }
        FS_HCINTMSK6 {
            0x20 RwReg Option;
            DTERRM { RwRwRegFieldBit }
            FRMORM { RwRwRegFieldBit }
            BBERRM { RwRwRegFieldBit }
            TXERRM { RwRwRegFieldBit }
            NYET { RwRwRegFieldBit }
            ACKM { RwRwRegFieldBit }
            NAKM { RwRwRegFieldBit }
            STALLM { RwRwRegFieldBit }
            CHHM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
        }
        FS_HCINTMSK7 {
            0x20 RwReg Option;
            DTERRM { RwRwRegFieldBit }
            FRMORM { RwRwRegFieldBit }
            BBERRM { RwRwRegFieldBit }
            TXERRM { RwRwRegFieldBit }
            NYET { RwRwRegFieldBit }
            ACKM { RwRwRegFieldBit }
            NAKM { RwRwRegFieldBit }
            STALLM { RwRwRegFieldBit }
            CHHM { RwRwRegFieldBit }
            XFRCM { RwRwRegFieldBit }
        }
        FS_HCTSIZ0 {
            0x20 RwReg Option;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ1 {
            0x20 RwReg Option;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ2 {
            0x20 RwReg Option;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ3 {
            0x20 RwReg Option;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ4 {
            0x20 RwReg Option;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ5 {
            0x20 RwReg Option;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ6 {
            0x20 RwReg Option;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ7 {
            0x20 RwReg Option;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_otgfs_host {
    (
        $otgfs_macro_doc:expr,
        $otgfs_macro:ident,
        $otgfs_ty_doc:expr,
        $otgfs_ty:ident,
        $otgfs:ident,
        ($($fs_hcfg:ident)?),
        ($($hfir:ident)?),
        ($($fs_hfnum:ident)?),
        ($($fs_hptxsts:ident)?),
        ($($haint:ident)?),
        ($($haintm:ident)?),
        ($($fs_hprt:ident)?),
        ($($fs_hcchar0:ident)?),
        ($($fs_hcchar1:ident)?),
        ($($fs_hcchar2:ident)?),
        ($($fs_hcchar3:ident)?),
        ($($fs_hcchar4:ident)?),
        ($($fs_hcchar5:ident)?),
        ($($fs_hcchar6:ident)?),
        ($($fs_hcchar7:ident)?),
        ($($fs_hcint0:ident)?),
        ($($fs_hcint1:ident)?),
        ($($fs_hcint2:ident)?),
        ($($fs_hcint3:ident)?),
        ($($fs_hcint4:ident)?),
        ($($fs_hcint5:ident)?),
        ($($fs_hcint6:ident)?),
        ($($fs_hcint7:ident)?),
        ($($fs_hcintmsk0:ident)?),
        ($($fs_hcintmsk1:ident)?),
        ($($fs_hcintmsk2:ident)?),
        ($($fs_hcintmsk3:ident)?),
        ($($fs_hcintmsk4:ident)?),
        ($($fs_hcintmsk5:ident)?),
        ($($fs_hcintmsk6:ident)?),
        ($($fs_hcintmsk7:ident)?),
        ($($fs_hctsiz0:ident)?),
        ($($fs_hctsiz1:ident)?),
        ($($fs_hctsiz2:ident)?),
        ($($fs_hctsiz3:ident)?),
        ($($fs_hctsiz4:ident)?),
        ($($fs_hctsiz5:ident)?),
        ($($fs_hctsiz6:ident)?),
        ($($fs_hctsiz7:ident)?),
    ) => {
        periph::map! {
            #[doc = $otgfs_macro_doc]
            pub macro $otgfs_macro;

            #[doc = $otgfs_ty_doc]
            pub struct $otgfs_ty;

            impl HostOtgfsMap for $otgfs_ty {}

            drone_stm32_map_pieces::reg;
            crate::host;

            OTG_FS_HOST {
                FS_HCFG {
                    $(
                        $fs_hcfg Option;
                        FSLSS { FSLSS }
                        FSLSPCS { FSLSPCS }
                    )*
                }
                HFIR {
                    $(
                        $hfir Option;
                        FRIVL { FRIVL}
                    )*
                }
                FS_HFNUM {
                    $(
                        $fs_hfnum Option;
                        FTREM { FTREM }
                        FRNUM { FRNUM }
                    )*
                }
                FS_HPTXSTS {
                    $(
                        $fs_hptxsts Option;
                        PTXQTOP { PTXQTOP }
                        PTXQSAV { PTXQSAV }
                        PTXFSAVL { PTXFSAVL }
                    )*
                }
                HAINT {
                    $(
                        $haint Option;
                        HAINT { HAINT }
                    )*
                }
                HAINTMSK {
                    $(
                        $haintm Option;
                        HAINTM { HAINTM }
                    )*
                }
                FS_HPRT {
                    $(
                        $fs_hprt Option;
                        PSPD { PSPD }
                        PTCTL { PTCTL }
                        PPWR { PPWR }
                        PLSTS { PLSTS }
                        PRST { PRST }
                        PSUSP { PSUSP }
                        PRES { PRES }
                        POCCHNG { POCCHNG }
                        POCA { POCA }
                        PENCHNG { PENCHNG }
                        PENA { PENA }
                        PCDET { PCDET }
                        PCSTS { PCSTS }
                    )*
                }
                FS_HCCHAR0 {
                    $(
                        $fs_hcchar0 Option;
                        CHENA { CHENA }
                        CHDIS { CHDIS }
                        ODDFRM { ODDFRM }
                        DAD { DAD }
                        MCNT { MCNT }
                        EPTYP { EPTYP }
                        LSDEV { LSDEV }
                        EPDIR { EPDIR }
                        EPNUM { EPNUM }
                        MPSIZ { MPSIZ }
                    )*
                }
                FS_HCCHAR1 {
                    $(
                        $fs_hcchar1 Option;
                        CHENA { CHENA }
                        CHDIS { CHDIS }
                        ODDFRM { ODDFRM }
                        DAD { DAD }
                        MCNT { MCNT }
                        EPTYP { EPTYP }
                        LSDEV { LSDEV }
                        EPDIR { EPDIR }
                        EPNUM { EPNUM }
                        MPSIZ { MPSIZ }
                    )*
                }
                FS_HCCHAR2 {
                    $(
                        $fs_hcchar2 Option;
                        CHENA { CHENA }
                        CHDIS { CHDIS }
                        ODDFRM { ODDFRM }
                        DAD { DAD }
                        MCNT { MCNT }
                        EPTYP { EPTYP }
                        LSDEV { LSDEV }
                        EPDIR { EPDIR }
                        EPNUM { EPNUM }
                        MPSIZ { MPSIZ }
                    )*
                }
                FS_HCCHAR3 {
                    $(
                        $fs_hcchar3 Option;
                        CHENA { CHENA }
                        CHDIS { CHDIS }
                        ODDFRM { ODDFRM }
                        DAD { DAD }
                        MCNT { MCNT }
                        EPTYP { EPTYP }
                        LSDEV { LSDEV }
                        EPDIR { EPDIR }
                        EPNUM { EPNUM }
                        MPSIZ { MPSIZ }
                    )*
                }
                FS_HCCHAR4 {
                    $(
                        $fs_hcchar4 Option;
                        CHENA { CHENA }
                        CHDIS { CHDIS }
                        ODDFRM { ODDFRM }
                        DAD { DAD }
                        MCNT { MCNT }
                        EPTYP { EPTYP }
                        LSDEV { LSDEV }
                        EPDIR { EPDIR }
                        EPNUM { EPNUM }
                        MPSIZ { MPSIZ }
                    )*
                }
                FS_HCCHAR5 {
                    $(
                        $fs_hcchar5 Option;
                        CHENA { CHENA }
                        CHDIS { CHDIS }
                        ODDFRM { ODDFRM }
                        DAD { DAD }
                        MCNT { MCNT }
                        EPTYP { EPTYP }
                        LSDEV { LSDEV }
                        EPDIR { EPDIR }
                        EPNUM { EPNUM }
                        MPSIZ { MPSIZ }
                    )*
                }
                FS_HCCHAR6 {
                    $(
                        $fs_hcchar6 Option;
                        CHENA { CHENA }
                        CHDIS { CHDIS }
                        ODDFRM { ODDFRM }
                        DAD { DAD }
                        MCNT { MCNT }
                        EPTYP { EPTYP }
                        LSDEV { LSDEV }
                        EPDIR { EPDIR }
                        EPNUM { EPNUM }
                        MPSIZ { MPSIZ }
                    )*
                }
                FS_HCCHAR7 {
                    $(
                        $fs_hcchar7 Option;
                        CHENA { CHENA }
                        CHDIS { CHDIS }
                        ODDFRM { ODDFRM }
                        DAD { DAD }
                        MCNT { MCNT }
                        EPTYP { EPTYP }
                        LSDEV { LSDEV }
                        EPDIR { EPDIR }
                        EPNUM { EPNUM }
                        MPSIZ { MPSIZ }
                    )*
                }
                FS_HCINT0 {
                    $(
                        $fs_hcint0 Option;
                        DTERR { DTERR }
                        FRMOR { FRMOR }
                        BBERR { BBERR }
                        TXERR { TXERR }
                        ACK { ACK }
                        NAK { NAK }
                        STALL { STALL }
                        CHH { CHH }
                        XFRC { XFRC }
                    )*
                }
                FS_HCINT1 {
                    $(
                        $fs_hcint1 Option;
                        DTERR { DTERR }
                        FRMOR { FRMOR }
                        BBERR { BBERR }
                        TXERR { TXERR }
                        ACK { ACK }
                        NAK { NAK }
                        STALL { STALL }
                        CHH { CHH }
                        XFRC { XFRC }
                    )*
                }
                FS_HCINT2 {
                    $(
                        $fs_hcint2 Option;
                        DTERR { DTERR }
                        FRMOR { FRMOR }
                        BBERR { BBERR }
                        TXERR { TXERR }
                        ACK { ACK }
                        NAK { NAK }
                        STALL { STALL }
                        CHH { CHH }
                        XFRC { XFRC }
                    )*
                }
                FS_HCINT3 {
                    $(
                        $fs_hcint3 Option;
                        DTERR { DTERR }
                        FRMOR { FRMOR }
                        BBERR { BBERR }
                        TXERR { TXERR }
                        ACK { ACK }
                        NAK { NAK }
                        STALL { STALL }
                        CHH { CHH }
                        XFRC { XFRC }
                    )*
                }
                FS_HCINT4 {
                    $(
                        $fs_hcint4 Option;
                        DTERR { DTERR }
                        FRMOR { FRMOR }
                        BBERR { BBERR }
                        TXERR { TXERR }
                        ACK { ACK }
                        NAK { NAK }
                        STALL { STALL }
                        CHH { CHH }
                        XFRC { XFRC }
                    )*
                }
                FS_HCINT5 {
                    $(
                        $fs_hcint5 Option;
                        DTERR { DTERR }
                        FRMOR { FRMOR }
                        BBERR { BBERR }
                        TXERR { TXERR }
                        ACK { ACK }
                        NAK { NAK }
                        STALL { STALL }
                        CHH { CHH }
                        XFRC { XFRC }
                    )*
                }
                FS_HCINT6 {
                    $(
                        $fs_hcint6 Option;
                        DTERR { DTERR }
                        FRMOR { FRMOR }
                        BBERR { BBERR }
                        TXERR { TXERR }
                        ACK { ACK }
                        NAK { NAK }
                        STALL { STALL }
                        CHH { CHH }
                        XFRC { XFRC }
                    )*
                }
                FS_HCINT7 {
                    $(
                        $fs_hcint7 Option;
                        DTERR { DTERR }
                        FRMOR { FRMOR }
                        BBERR { BBERR }
                        TXERR { TXERR }
                        ACK { ACK }
                        NAK { NAK }
                        STALL { STALL }
                        CHH { CHH }
                        XFRC { XFRC }
                    )*
                }
                FS_HCINTMSK0 {
                    $(
                        $fs_hcintmsk0 Option;
                        DTERRM { DTERRM }
                        FRMORM { FRMORM }
                        BBERRM { BBERRM }
                        TXERRM { TXERRM }
                        NYET { NYET }
                        ACKM { ACKM }
                        NAKM { NAKM }
                        STALLM { STALLM }
                        CHHM { CHHM }
                        XFRCM { XFRCM }
                    )*
                }
                FS_HCINTMSK1 {
                    $(
                        $fs_hcintmsk1 Option;
                        DTERRM { DTERRM }
                        FRMORM { FRMORM }
                        BBERRM { BBERRM }
                        TXERRM { TXERRM }
                        NYET { NYET }
                        ACKM { ACKM }
                        NAKM { NAKM }
                        STALLM { STALLM }
                        CHHM { CHHM }
                        XFRCM { XFRCM }
                    )*
                }
                FS_HCINTMSK2 {
                    $(
                        $fs_hcintmsk2 Option;
                        DTERRM { DTERRM }
                        FRMORM { FRMORM }
                        BBERRM { BBERRM }
                        TXERRM { TXERRM }
                        NYET { NYET }
                        ACKM { ACKM }
                        NAKM { NAKM }
                        STALLM { STALLM }
                        CHHM { CHHM }
                        XFRCM { XFRCM }
                    )*
                }
                FS_HCINTMSK3 {
                    $(
                        $fs_hcintmsk3 Option;
                        DTERRM { DTERRM }
                        FRMORM { FRMORM }
                        BBERRM { BBERRM }
                        TXERRM { TXERRM }
                        NYET { NYET }
                        ACKM { ACKM }
                        NAKM { NAKM }
                        STALLM { STALLM }
                        CHHM { CHHM }
                        XFRCM { XFRCM }
                    )*
                }
                FS_HCINTMSK4 {
                    $(
                        $fs_hcintmsk4 Option;
                        DTERRM { DTERRM }
                        FRMORM { FRMORM }
                        BBERRM { BBERRM }
                        TXERRM { TXERRM }
                        NYET { NYET }
                        ACKM { ACKM }
                        NAKM { NAKM }
                        STALLM { STALLM }
                        CHHM { CHHM }
                        XFRCM { XFRCM }
                    )*
                }
                FS_HCINTMSK5 {
                    $(
                        $fs_hcintmsk5 Option;
                        DTERRM { DTERRM }
                        FRMORM { FRMORM }
                        BBERRM { BBERRM }
                        TXERRM { TXERRM }
                        NYET { NYET }
                        ACKM { ACKM }
                        NAKM { NAKM }
                        STALLM { STALLM }
                        CHHM { CHHM }
                        XFRCM { XFRCM }
                    )*
                }
                FS_HCINTMSK6 {
                    $(
                        $fs_hcintmsk6 Option;
                        DTERRM { DTERRM }
                        FRMORM { FRMORM }
                        BBERRM { BBERRM }
                        TXERRM { TXERRM }
                        NYET { NYET }
                        ACKM { ACKM }
                        NAKM { NAKM }
                        STALLM { STALLM }
                        CHHM { CHHM }
                        XFRCM { XFRCM }
                    )*
                }
                FS_HCINTMSK7 {
                    $(
                        $fs_hcintmsk7 Option;
                        DTERRM { DTERRM }
                        FRMORM { FRMORM }
                        BBERRM { BBERRM }
                        TXERRM { TXERRM }
                        NYET { NYET }
                        ACKM { ACKM }
                        NAKM { NAKM }
                        STALLM { STALLM }
                        CHHM { CHHM }
                        XFRCM { XFRCM }
                    )*
                }
                FS_HCTSIZ0 {
                    $(
                        $fs_hctsiz0 Option;
                        DPID { DPID }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
                FS_HCTSIZ1 {
                    $(
                        $fs_hctsiz1 Option;
                        DPID { DPID }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
                FS_HCTSIZ2 {
                    $(
                        $fs_hctsiz2 Option;
                        DPID { DPID }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
                FS_HCTSIZ3 {
                    $(
                        $fs_hctsiz3 Option;
                        DPID { DPID }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
                FS_HCTSIZ4 {
                    $(
                        $fs_hctsiz4 Option;
                        DPID { DPID }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
                FS_HCTSIZ5 {
                    $(
                        $fs_hctsiz5 Option;
                        DPID { DPID }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
                FS_HCTSIZ6 {
                    $(
                        $fs_hctsiz6 Option;
                        DPID { DPID }
                        PKTCNT { PKTCNT }
                        XFRSIZ { XFRSIZ }
                    )*
                }
                FS_HCTSIZ7 {
                    $(
                        $fs_hctsiz7 Option;
                        DPID { DPID }
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
map_otgfs_host! {
    "Extracts USB-OTGFS register tokens for general host.",
    periph_otgfs_host,
    "General host USB-OTGFS peripheral variant.",
    Otgfs,
    OTGFS,
    (FS_HCFG),
    (HFIR),
    (FS_HFNUM),
    (FS_HPTXSTS),
    (HAINT),
    (HAINTMSK),
    (FS_HPRT),
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

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_host! {
    "Extracts USB-OTGFS register tokens for channel 0.",
    periph_otgfs_host_ch0,
    "Channel 0 USB-OTGFS peripheral variant.",
    OtgfsCh0,
    OTGFSCH0,
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCCHAR0),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINT0),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINTMSK0),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCTSIZ0),
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
map_otgfs_host! {
    "Extracts USB-OTGFS register tokens for channel 1.",
    periph_otgfs_host_ch1,
    "Channel 1 USB-OTGFS peripheral variant.",
    OtgfsCh1,
    OTGFSCH1,
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCCHAR1),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINT1),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINTMSK1),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCTSIZ1),
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
map_otgfs_host! {
    "Extracts USB-OTGFS register tokens for channel 2.",
    periph_otgfs_host_ch2,
    "Channel 2 USB-OTGFS peripheral variant.",
    OtgfsCh2,
    OTGFSCH2,
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCCHAR2),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINT2),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINTMSK2),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCTSIZ2),
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
map_otgfs_host! {
    "Extracts USB-OTGFS register tokens for channel 3.",
    periph_otgfs_host_ch3,
    "Channel 3 USB-OTGFS peripheral variant.",
    OtgfsCh3,
    OTGFSCH3,
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
    (FS_HCCHAR3),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINT3),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINTMSK3),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCTSIZ3),
    (),
    (),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_host! {
    "Extracts USB-OTGFS register tokens for channel 4.",
    periph_otgfs_host_ch4,
    "Channel 4 USB-OTGFS peripheral variant.",
    OtgfsCh4,
    OTGFSCH4,
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
    (FS_HCCHAR4),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINT4),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINTMSK4),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCTSIZ4),
    (),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_host! {
    "Extracts USB-OTGFS register tokens for channel 5.",
    periph_otgfs_host_ch5,
    "Channel 5 USB-OTGFS peripheral variant.",
    OtgfsCh5,
    OTGFSCH5,
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
    (FS_HCCHAR5),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINT5),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINTMSK5),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCTSIZ5),
    (),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_host! {
    "Extracts USB-OTGFS register tokens for channel 6.",
    periph_otgfs_host_ch6,
    "Channel 6 USB-OTGFS peripheral variant.",
    OtgfsCh6,
    OTGFSCH6,
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
    (FS_HCCHAR6),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINT6),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINTMSK6),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCTSIZ6),
    (),
}

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f411",
))]
map_otgfs_host! {
    "Extracts USB-OTGFS register tokens for channel 7.",
    periph_otgfs_host_ch7,
    "Channel 7 USB-OTGFS peripheral variant.",
    OtgfsCh7,
    OTGFSCH7,
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
    (FS_HCCHAR7),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINT7),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCINTMSK7),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (FS_HCTSIZ7),
}


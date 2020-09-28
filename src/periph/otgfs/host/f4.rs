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
            0x20 RwReg;
            FSLSS { RoRwRegFieldBit }
            FSLSPCS { RwRwRegFieldBits }
        }
        HFIR {
            0x20 RwReg;
            FRIVL { RwRwRegFieldBits }
        }
        FS_HFNUM {
            0x20 RoReg;
            FTREM { RoRoRegFieldBits }
            FRNUM { RoRoRegFieldBits }
        }
        FS_HPTXSTS {
            0x20 RwReg;
            PTXQTOP { RoRwRegFieldBits }
            PTXQSAV { RoRwRegFieldBits }
            PTXFSAVL { RwRwRegFieldBits }
        }
        HAINT {
            0x20 RoReg;
            HAINT { RoRoRegFieldBits }
        }
        HAINTMSK {
            0x20 RwReg;
            HAINTM { RwRwRegFieldBits }
        }
        FS_HPRT {
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
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
            0x20 RwReg;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ1 {
            0x20 RwReg;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ2 {
            0x20 RwReg;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ3 {
            0x20 RwReg;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ4 {
            0x20 RwReg;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ5 {
            0x20 RwReg;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ6 {
            0x20 RwReg;
            DPID { RwRwRegFieldBits }
            PKTCNT { RwRwRegFieldBits }
            XFRSIZ { RwRwRegFieldBits }
        }
        FS_HCTSIZ7 {
            0x20 RwReg;
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
                    FS_HCFG;
                    FSLSS { FSLSS }
                    FSLSPCS { FSLSPCS }
                }
                HFIR {
                    HFIR;
                    FRIVL { FRIVL}
                }
                FS_HFNUM {
                    FS_HFNUM;
                    FTREM { FTREM }
                    FRNUM { FRNUM }
                }
                FS_HPTXSTS {
                    FS_HPTXSTS;
                    PTXQTOP { PTXQTOP }
                    PTXQSAV { PTXQSAV }
                    PTXFSAVL { PTXFSAVL }
                }
                HAINT {
                    HAINT;
                    HAINT { HAINT }
                }
                HAINTMSK {
                    HAINTMSK;
                    HAINTM { HAINTM }
                }
                FS_HPRT {
                    FS_HPRT;
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
                }
                FS_HCCHAR0 {
                    FS_HCCHAR0;
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
                }
                FS_HCCHAR1 {
                    FS_HCCHAR1;
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
                }
                FS_HCCHAR2 {
                    FS_HCCHAR2;
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
                }
                FS_HCCHAR3 {
                    FS_HCCHAR3;
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
                }
                FS_HCCHAR4 {
                    FS_HCCHAR4;
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
                }
                FS_HCCHAR5 {
                    FS_HCCHAR5;
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
                }
                FS_HCCHAR6 {
                    FS_HCCHAR6;
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
                }
                FS_HCCHAR7 {
                    FS_HCCHAR7;
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
                }
                FS_HCINT0 {
                    FS_HCINT0;
                    DTERR { DTERR }
                    FRMOR { FRMOR }
                    BBERR { BBERR }
                    TXERR { TXERR }
                    ACK { ACK }
                    NAK { NAK }
                    STALL { STALL }
                    CHH { CHH }
                    XFRC { XFRC }
                }
                FS_HCINT1 {
                    FS_HCINT1;
                    DTERR { DTERR }
                    FRMOR { FRMOR }
                    BBERR { BBERR }
                    TXERR { TXERR }
                    ACK { ACK }
                    NAK { NAK }
                    STALL { STALL }
                    CHH { CHH }
                    XFRC { XFRC }
                }
                FS_HCINT2 {
                    FS_HCINT2;
                    DTERR { DTERR }
                    FRMOR { FRMOR }
                    BBERR { BBERR }
                    TXERR { TXERR }
                    ACK { ACK }
                    NAK { NAK }
                    STALL { STALL }
                    CHH { CHH }
                    XFRC { XFRC }
                }
                FS_HCINT3 {
                    FS_HCINT3;
                    DTERR { DTERR }
                    FRMOR { FRMOR }
                    BBERR { BBERR }
                    TXERR { TXERR }
                    ACK { ACK }
                    NAK { NAK }
                    STALL { STALL }
                    CHH { CHH }
                    XFRC { XFRC }
                }
                FS_HCINT4 {
                    FS_HCINT4;
                    DTERR { DTERR }
                    FRMOR { FRMOR }
                    BBERR { BBERR }
                    TXERR { TXERR }
                    ACK { ACK }
                    NAK { NAK }
                    STALL { STALL }
                    CHH { CHH }
                    XFRC { XFRC }
                }
                FS_HCINT5 {
                    FS_HCINT5;
                    DTERR { DTERR }
                    FRMOR { FRMOR }
                    BBERR { BBERR }
                    TXERR { TXERR }
                    ACK { ACK }
                    NAK { NAK }
                    STALL { STALL }
                    CHH { CHH }
                    XFRC { XFRC }
                }
                FS_HCINT6 {
                    FS_HCINT6;
                    DTERR { DTERR }
                    FRMOR { FRMOR }
                    BBERR { BBERR }
                    TXERR { TXERR }
                    ACK { ACK }
                    NAK { NAK }
                    STALL { STALL }
                    CHH { CHH }
                    XFRC { XFRC }
                }
                FS_HCINT7 {
                    FS_HCINT7;
                    DTERR { DTERR }
                    FRMOR { FRMOR }
                    BBERR { BBERR }
                    TXERR { TXERR }
                    ACK { ACK }
                    NAK { NAK }
                    STALL { STALL }
                    CHH { CHH }
                    XFRC { XFRC }
                }
                FS_HCINTMSK0 {
                    FS_HCINTMSK0;
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
                }
                FS_HCINTMSK1 {
                    FS_HCINTMSK1;
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
                }
                FS_HCINTMSK2 {
                    FS_HCINTMSK2;
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
                }
                FS_HCINTMSK3 {
                    FS_HCINTMSK3;
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
                }
                FS_HCINTMSK4 {
                    FS_HCINTMSK4;
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
                }
                FS_HCINTMSK5 {
                    FS_HCINTMSK5;
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
                }
                FS_HCINTMSK6 {
                    FS_HCINTMSK6;
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
                }
                FS_HCINTMSK7 {
                    FS_HCINTMSK7;
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
                }
                FS_HCTSIZ0 {
                    FS_HCTSIZ0;
                    DPID { DPID }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
                FS_HCTSIZ1 {
                    FS_HCTSIZ1;
                    DPID { DPID }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
                FS_HCTSIZ2 {
                    FS_HCTSIZ2;
                    DPID { DPID }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
                FS_HCTSIZ3 {
                    FS_HCTSIZ3;
                    DPID { DPID }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
                FS_HCTSIZ4 {
                    FS_HCTSIZ4;
                    DPID { DPID }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
                FS_HCTSIZ5 {
                    FS_HCTSIZ5;
                    DPID { DPID }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
                FS_HCTSIZ6 {
                    FS_HCTSIZ6;
                    DPID { DPID }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
                FS_HCTSIZ7 {
                    FS_HCTSIZ7;
                    DPID { DPID }
                    PKTCNT { PKTCNT }
                    XFRSIZ { XFRSIZ }
                }
            }
        }
    }
}

#[cfg(any(
    stm32_mcu = "stm32f411",
))]
map_otgfs_host! {
    "Extracts USB-OTGFS register tokens.",
    periph_otgfs_host,
    "USB-OTGFS peripheral variant.",
    Otgfs,
    OTGFS,
}


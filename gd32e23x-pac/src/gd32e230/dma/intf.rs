#[doc = "Reader of register INTF"]
pub type R = crate::R<u32, super::INTF>;
#[doc = "Reader of field `GIF0`"]
pub type GIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF0`"]
pub type FTFIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF0`"]
pub type HTFIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF0`"]
pub type ERRIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF1`"]
pub type GIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF1`"]
pub type FTFIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF1`"]
pub type HTFIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF1`"]
pub type ERRIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF2`"]
pub type GIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF2`"]
pub type FTFIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF2`"]
pub type HTFIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF2`"]
pub type ERRIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF3`"]
pub type GIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF3`"]
pub type FTFIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF3`"]
pub type HTFIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF3`"]
pub type ERRIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF4`"]
pub type GIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF4`"]
pub type FTFIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF4`"]
pub type HTFIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF4`"]
pub type ERRIF4_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Global interrupt flag"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 0 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif0(&self) -> FTFIF0_R {
        FTFIF0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif0(&self) -> HTFIF0_R {
        HTFIF0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 0 Error flag"]
    #[inline(always)]
    pub fn errif0(&self) -> ERRIF0_R {
        ERRIF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 1 Global interrupt flag"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif1(&self) -> FTFIF1_R {
        FTFIF1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 1 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif1(&self) -> HTFIF1_R {
        HTFIF1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 1 Error flag"]
    #[inline(always)]
    pub fn errif1(&self) -> ERRIF1_R {
        ERRIF1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 2 Global interrupt flag"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 2 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif2(&self) -> FTFIF2_R {
        FTFIF2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif2(&self) -> HTFIF2_R {
        HTFIF2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 2 Error flag"]
    #[inline(always)]
    pub fn errif2(&self) -> ERRIF2_R {
        ERRIF2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 3 Global interrupt flag"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 3 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif3(&self) -> FTFIF3_R {
        FTFIF3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 3 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif3(&self) -> HTFIF3_R {
        HTFIF3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 3 Error flag"]
    #[inline(always)]
    pub fn errif3(&self) -> ERRIF3_R {
        ERRIF3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 4 Global interrupt flag"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 4 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif4(&self) -> FTFIF4_R {
        FTFIF4_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 4 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif4(&self) -> HTFIF4_R {
        HTFIF4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 4 Error flag"]
    #[inline(always)]
    pub fn errif4(&self) -> ERRIF4_R {
        ERRIF4_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}

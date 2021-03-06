#[doc = "Reader of register CTL0"]
pub type R = crate::R<u32, super::CTL0>;
#[doc = "Writer for register CTL0"]
pub type W = crate::W<u32, super::CTL0>;
#[doc = "Register CTL0 `reset()`'s with value 0x83"]
impl crate::ResetValue for super::CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x83
    }
}
#[doc = "Reader of field `PLLSTB`"]
pub type PLLSTB_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLEN`"]
pub type PLLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLEN`"]
pub struct PLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CKMEN`"]
pub type CKMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKMEN`"]
pub struct CKMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `HXTALBPS`"]
pub type HXTALBPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HXTALBPS`"]
pub struct HXTALBPS_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTALBPS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `HXTALSTB`"]
pub type HXTALSTB_R = crate::R<bool, bool>;
#[doc = "Reader of field `HXTALEN`"]
pub type HXTALEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HXTALEN`"]
pub struct HXTALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTALEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `IRC8MCALIB`"]
pub type IRC8MCALIB_R = crate::R<u8, u8>;
#[doc = "Reader of field `IRC8MADJ`"]
pub type IRC8MADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRC8MADJ`"]
pub struct IRC8MADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC8MADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `IRC8MSTB`"]
pub type IRC8MSTB_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRC8MEN`"]
pub type IRC8MEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRC8MEN`"]
pub struct IRC8MEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC8MEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - PLL Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pllstb(&self) -> PLLSTB_R {
        PLLSTB_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&self) -> CKMEN_R {
        CKMEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&self) -> HXTALBPS_R {
        HXTALBPS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - External crystal oscillator (HXTAL) clock stabilization flag"]
    #[inline(always)]
    pub fn hxtalstb(&self) -> HXTALSTB_R {
        HXTALSTB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&self) -> HXTALEN_R {
        HXTALEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - High Speed Internal Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc8mcalib(&self) -> IRC8MCALIB_R {
        IRC8MCALIB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 3:7 - High Speed Internal Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc8madj(&self) -> IRC8MADJ_R {
        IRC8MADJ_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - IRC8M High Speed Internal Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc8mstb(&self) -> IRC8MSTB_R {
        IRC8MSTB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Internal High Speed oscillator Enable"]
    #[inline(always)]
    pub fn irc8men(&self) -> IRC8MEN_R {
        IRC8MEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W { w: self }
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&mut self) -> CKMEN_W {
        CKMEN_W { w: self }
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&mut self) -> HXTALBPS_W {
        HXTALBPS_W { w: self }
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&mut self) -> HXTALEN_W {
        HXTALEN_W { w: self }
    }
    #[doc = "Bits 3:7 - High Speed Internal Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc8madj(&mut self) -> IRC8MADJ_W {
        IRC8MADJ_W { w: self }
    }
    #[doc = "Bit 0 - Internal High Speed oscillator Enable"]
    #[inline(always)]
    pub fn irc8men(&mut self) -> IRC8MEN_W {
        IRC8MEN_W { w: self }
    }
}

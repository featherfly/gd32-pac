#[doc = "Reader of register CHC"]
pub type R = crate::R<u32, super::CHC>;
#[doc = "Writer for register CHC"]
pub type W = crate::W<u32, super::CHC>;
#[doc = "Register CHC `reset()`'s with value 0"]
impl crate::ResetValue for super::CHC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPERR`"]
pub type EPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPERR`"]
pub struct EPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `HCM`"]
pub type HCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCM`"]
pub struct HCM_W<'a> {
    w: &'a mut W,
}
impl<'a> HCM_W<'a> {
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
    #[doc = "Bit 8 - Early parity error flag"]
    #[inline(always)]
    pub fn eperr(&self) -> EPERR_R {
        EPERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Hardware flow control coherence mode"]
    #[inline(always)]
    pub fn hcm(&self) -> HCM_R {
        HCM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Early parity error flag"]
    #[inline(always)]
    pub fn eperr(&mut self) -> EPERR_W {
        EPERR_W { w: self }
    }
    #[doc = "Bit 0 - Hardware flow control coherence mode"]
    #[inline(always)]
    pub fn hcm(&mut self) -> HCM_W {
        HCM_W { w: self }
    }
}

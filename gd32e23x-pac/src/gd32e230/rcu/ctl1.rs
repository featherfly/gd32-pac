#[doc = "Reader of register CTL1"]
pub type R = crate::R<u32, super::CTL1>;
#[doc = "Writer for register CTL1"]
pub type W = crate::W<u32, super::CTL1>;
#[doc = "Register CTL1 `reset()`'s with value 0x80"]
impl crate::ResetValue for super::CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `IRC28MCALIB`"]
pub type IRC28MCALIB_R = crate::R<u8, u8>;
#[doc = "Reader of field `IRC28MADJ`"]
pub type IRC28MADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRC28MADJ`"]
pub struct IRC28MADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC28MADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `IRC28MSTB`"]
pub type IRC28MSTB_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRC28MEN`"]
pub type IRC28MEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRC28MEN`"]
pub struct IRC28MEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC28MEN_W<'a> {
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
    #[doc = "Bits 8:15 - Internal 28M RC Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc28mcalib(&self) -> IRC28MCALIB_R {
        IRC28MCALIB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 3:7 - Internal 28M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc28madj(&self) -> IRC28MADJ_R {
        IRC28MADJ_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - IRC28M Internal 28M RC Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc28mstb(&self) -> IRC28MSTB_R {
        IRC28MSTB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IRC28M Internal 28M RC oscillator Enable"]
    #[inline(always)]
    pub fn irc28men(&self) -> IRC28MEN_R {
        IRC28MEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:7 - Internal 28M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc28madj(&mut self) -> IRC28MADJ_W {
        IRC28MADJ_W { w: self }
    }
    #[doc = "Bit 0 - IRC28M Internal 28M RC oscillator Enable"]
    #[inline(always)]
    pub fn irc28men(&mut self) -> IRC28MEN_W {
        IRC28MEN_W { w: self }
    }
}

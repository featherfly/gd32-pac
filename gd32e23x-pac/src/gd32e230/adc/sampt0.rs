#[doc = "Reader of register SAMPT0"]
pub type R = crate::R<u32, super::SAMPT0>;
#[doc = "Writer for register SAMPT0"]
pub type W = crate::W<u32, super::SAMPT0>;
#[doc = "Register SAMPT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAMPT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPT16`"]
pub type SPT16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT16`"]
pub struct SPT16_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `SPT17`"]
pub type SPT17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT17`"]
pub struct SPT17_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn spt16(&self) -> SPT16_R {
        SPT16_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn spt17(&self) -> SPT17_R {
        SPT17_R::new(((self.bits >> 21) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn spt16(&mut self) -> SPT16_W {
        SPT16_W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn spt17(&mut self) -> SPT17_W {
        SPT17_W { w: self }
    }
}

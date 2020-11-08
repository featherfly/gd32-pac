#[doc = "Reader of register ALRM0SS"]
pub type R = crate::R<u32, super::ALRM0SS>;
#[doc = "Writer for register ALRM0SS"]
pub type W = crate::W<u32, super::ALRM0SS>;
#[doc = "Register ALRM0SS `reset()`'s with value 0"]
impl crate::ResetValue for super::ALRM0SS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSKSSC`"]
pub type MSKSSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSKSSC`"]
pub struct MSKSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKSSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `SSC`"]
pub type SSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SSC`"]
pub struct SSC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - Mask control bit of SSC"]
    #[inline(always)]
    pub fn mskssc(&self) -> MSKSSC_R {
        MSKSSC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:14 - Alarm sub second value"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:27 - Mask control bit of SSC"]
    #[inline(always)]
    pub fn mskssc(&mut self) -> MSKSSC_W {
        MSKSSC_W { w: self }
    }
    #[doc = "Bits 0:14 - Alarm sub second value"]
    #[inline(always)]
    pub fn ssc(&mut self) -> SSC_W {
        SSC_W { w: self }
    }
}

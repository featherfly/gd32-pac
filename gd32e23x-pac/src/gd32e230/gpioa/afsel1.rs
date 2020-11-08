#[doc = "Reader of register AFSEL1"]
pub type R = crate::R<u32, super::AFSEL1>;
#[doc = "Writer for register AFSEL1"]
pub type W = crate::W<u32, super::AFSEL1>;
#[doc = "Register AFSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AFSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL15`"]
pub type SEL15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL15`"]
pub struct SEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `SEL14`"]
pub type SEL14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL14`"]
pub struct SEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `SEL13`"]
pub type SEL13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL13`"]
pub struct SEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `SEL12`"]
pub type SEL12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL12`"]
pub struct SEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SEL11`"]
pub type SEL11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL11`"]
pub struct SEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `SEL10`"]
pub type SEL10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL10`"]
pub struct SEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SEL9`"]
pub type SEL9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL9`"]
pub struct SEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SEL8`"]
pub type SEL8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL8`"]
pub struct SEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel15(&self) -> SEL15_R {
        SEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel14(&self) -> SEL14_R {
        SEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel13(&self) -> SEL13_R {
        SEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel11(&self) -> SEL11_R {
        SEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel10(&self) -> SEL10_R {
        SEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel9(&self) -> SEL9_R {
        SEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel15(&mut self) -> SEL15_W {
        SEL15_W { w: self }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel14(&mut self) -> SEL14_W {
        SEL14_W { w: self }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel13(&mut self) -> SEL13_W {
        SEL13_W { w: self }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel12(&mut self) -> SEL12_W {
        SEL12_W { w: self }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel11(&mut self) -> SEL11_W {
        SEL11_W { w: self }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel10(&mut self) -> SEL10_W {
        SEL10_W { w: self }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel9(&mut self) -> SEL9_W {
        SEL9_W { w: self }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel8(&mut self) -> SEL8_W {
        SEL8_W { w: self }
    }
}

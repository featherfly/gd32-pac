#[doc = "Reader of register OSPD"]
pub type R = crate::R<u32, super::OSPD>;
#[doc = "Writer for register OSPD"]
pub type W = crate::W<u32, super::OSPD>;
#[doc = "Register OSPD `reset()`'s with value 0"]
impl crate::ResetValue for super::OSPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSPD15`"]
pub type OSPD15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD15`"]
pub struct OSPD15_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `OSPD14`"]
pub type OSPD14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD14`"]
pub struct OSPD14_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `OSPD13`"]
pub type OSPD13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD13`"]
pub struct OSPD13_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `OSPD12`"]
pub type OSPD12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD12`"]
pub struct OSPD12_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `OSPD11`"]
pub type OSPD11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD11`"]
pub struct OSPD11_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `OSPD10`"]
pub type OSPD10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD10`"]
pub struct OSPD10_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `OSPD9`"]
pub type OSPD9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD9`"]
pub struct OSPD9_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `OSPD8`"]
pub type OSPD8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD8`"]
pub struct OSPD8_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `OSPD7`"]
pub type OSPD7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD7`"]
pub struct OSPD7_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `OSPD6`"]
pub type OSPD6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD6`"]
pub struct OSPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `OSPD5`"]
pub type OSPD5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD5`"]
pub struct OSPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `OSPD4`"]
pub type OSPD4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD4`"]
pub struct OSPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `OSPD3`"]
pub type OSPD3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD3`"]
pub struct OSPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `OSPD2`"]
pub type OSPD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD2`"]
pub struct OSPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `OSPD1`"]
pub type OSPD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD1`"]
pub struct OSPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `OSPD0`"]
pub type OSPD0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPD0`"]
pub struct OSPD0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd15(&self) -> OSPD15_R {
        OSPD15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd14(&self) -> OSPD14_R {
        OSPD14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd13(&self) -> OSPD13_R {
        OSPD13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd12(&self) -> OSPD12_R {
        OSPD12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd11(&self) -> OSPD11_R {
        OSPD11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd10(&self) -> OSPD10_R {
        OSPD10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd9(&self) -> OSPD9_R {
        OSPD9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd8(&self) -> OSPD8_R {
        OSPD8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd7(&self) -> OSPD7_R {
        OSPD7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd6(&self) -> OSPD6_R {
        OSPD6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd5(&self) -> OSPD5_R {
        OSPD5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd4(&self) -> OSPD4_R {
        OSPD4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd3(&self) -> OSPD3_R {
        OSPD3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd2(&self) -> OSPD2_R {
        OSPD2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd1(&self) -> OSPD1_R {
        OSPD1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd0(&self) -> OSPD0_R {
        OSPD0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd15(&mut self) -> OSPD15_W {
        OSPD15_W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd14(&mut self) -> OSPD14_W {
        OSPD14_W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd13(&mut self) -> OSPD13_W {
        OSPD13_W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd12(&mut self) -> OSPD12_W {
        OSPD12_W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd11(&mut self) -> OSPD11_W {
        OSPD11_W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd10(&mut self) -> OSPD10_W {
        OSPD10_W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd9(&mut self) -> OSPD9_W {
        OSPD9_W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd8(&mut self) -> OSPD8_W {
        OSPD8_W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd7(&mut self) -> OSPD7_W {
        OSPD7_W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd6(&mut self) -> OSPD6_W {
        OSPD6_W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd5(&mut self) -> OSPD5_W {
        OSPD5_W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd4(&mut self) -> OSPD4_W {
        OSPD4_W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd3(&mut self) -> OSPD3_W {
        OSPD3_W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd2(&mut self) -> OSPD2_W {
        OSPD2_W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd1(&mut self) -> OSPD1_W {
        OSPD1_W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd0(&mut self) -> OSPD0_W {
        OSPD0_W { w: self }
    }
}

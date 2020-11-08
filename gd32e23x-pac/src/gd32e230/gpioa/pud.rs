#[doc = "Reader of register PUD"]
pub type R = crate::R<u32, super::PUD>;
#[doc = "Writer for register PUD"]
pub type W = crate::W<u32, super::PUD>;
#[doc = "Register PUD `reset()`'s with value 0x2400_0000"]
impl crate::ResetValue for super::PUD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2400_0000
    }
}
#[doc = "Reader of field `PUD15`"]
pub type PUD15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD15`"]
pub struct PUD15_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `PUD14`"]
pub type PUD14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD14`"]
pub struct PUD14_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `PUD13`"]
pub type PUD13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD13`"]
pub struct PUD13_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `PUD12`"]
pub type PUD12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD12`"]
pub struct PUD12_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `PUD11`"]
pub type PUD11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD11`"]
pub struct PUD11_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PUD10`"]
pub type PUD10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD10`"]
pub struct PUD10_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `PUD9`"]
pub type PUD9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD9`"]
pub struct PUD9_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `PUD8`"]
pub type PUD8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD8`"]
pub struct PUD8_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PUD7`"]
pub type PUD7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD7`"]
pub struct PUD7_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PUD6`"]
pub type PUD6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD6`"]
pub struct PUD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `PUD5`"]
pub type PUD5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD5`"]
pub struct PUD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `PUD4`"]
pub type PUD4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD4`"]
pub struct PUD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `PUD3`"]
pub type PUD3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD3`"]
pub struct PUD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PUD2`"]
pub type PUD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD2`"]
pub struct PUD2_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PUD1`"]
pub type PUD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD1`"]
pub struct PUD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PUD0`"]
pub type PUD0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUD0`"]
pub struct PUD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD0_W<'a> {
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
    pub fn pud15(&self) -> PUD15_R {
        PUD15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud14(&self) -> PUD14_R {
        PUD14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud13(&self) -> PUD13_R {
        PUD13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud12(&self) -> PUD12_R {
        PUD12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud11(&self) -> PUD11_R {
        PUD11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud10(&self) -> PUD10_R {
        PUD10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud9(&self) -> PUD9_R {
        PUD9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud8(&self) -> PUD8_R {
        PUD8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud7(&self) -> PUD7_R {
        PUD7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud6(&self) -> PUD6_R {
        PUD6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud5(&self) -> PUD5_R {
        PUD5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud4(&self) -> PUD4_R {
        PUD4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud3(&self) -> PUD3_R {
        PUD3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud2(&self) -> PUD2_R {
        PUD2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud1(&self) -> PUD1_R {
        PUD1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud0(&self) -> PUD0_R {
        PUD0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud15(&mut self) -> PUD15_W {
        PUD15_W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud14(&mut self) -> PUD14_W {
        PUD14_W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud13(&mut self) -> PUD13_W {
        PUD13_W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud12(&mut self) -> PUD12_W {
        PUD12_W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud11(&mut self) -> PUD11_W {
        PUD11_W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud10(&mut self) -> PUD10_W {
        PUD10_W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud9(&mut self) -> PUD9_W {
        PUD9_W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud8(&mut self) -> PUD8_W {
        PUD8_W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud7(&mut self) -> PUD7_W {
        PUD7_W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud6(&mut self) -> PUD6_W {
        PUD6_W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud5(&mut self) -> PUD5_W {
        PUD5_W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud4(&mut self) -> PUD4_W {
        PUD4_W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud3(&mut self) -> PUD3_W {
        PUD3_W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud2(&mut self) -> PUD2_W {
        PUD2_W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud1(&mut self) -> PUD1_W {
        PUD1_W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud0(&mut self) -> PUD0_W {
        PUD0_W { w: self }
    }
}

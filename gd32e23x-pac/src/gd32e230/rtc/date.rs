#[doc = "Reader of register DATE"]
pub type R = crate::R<u32, super::DATE>;
#[doc = "Writer for register DATE"]
pub type W = crate::W<u32, super::DATE>;
#[doc = "Register DATE `reset()`'s with value 0x2101"]
impl crate::ResetValue for super::DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2101
    }
}
#[doc = "Reader of field `YRT`"]
pub type YRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YRT`"]
pub struct YRT_W<'a> {
    w: &'a mut W,
}
impl<'a> YRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `YRU`"]
pub type YRU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YRU`"]
pub struct YRU_W<'a> {
    w: &'a mut W,
}
impl<'a> YRU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DOW`"]
pub type DOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DOW`"]
pub struct DOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `MONT`"]
pub type MONT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONT`"]
pub struct MONT_W<'a> {
    w: &'a mut W,
}
impl<'a> MONT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `MONU`"]
pub type MONU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MONU`"]
pub struct MONU_W<'a> {
    w: &'a mut W,
}
impl<'a> MONU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DAYT`"]
pub type DAYT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYT`"]
pub struct DAYT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DAYU`"]
pub type DAYU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYU`"]
pub struct DAYU_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - Year tens in BCD code"]
    #[inline(always)]
    pub fn yrt(&self) -> YRT_R {
        YRT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Year units in BCD code"]
    #[inline(always)]
    pub fn yru(&self) -> YRU_R {
        YRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Days of the week"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD code"]
    #[inline(always)]
    pub fn mont(&self) -> MONT_R {
        MONT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Month units in BCD code"]
    #[inline(always)]
    pub fn monu(&self) -> MONU_R {
        MONU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD code"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - Date units in BCD code"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - Year tens in BCD code"]
    #[inline(always)]
    pub fn yrt(&mut self) -> YRT_W {
        YRT_W { w: self }
    }
    #[doc = "Bits 16:19 - Year units in BCD code"]
    #[inline(always)]
    pub fn yru(&mut self) -> YRU_W {
        YRU_W { w: self }
    }
    #[doc = "Bits 13:15 - Days of the week"]
    #[inline(always)]
    pub fn dow(&mut self) -> DOW_W {
        DOW_W { w: self }
    }
    #[doc = "Bit 12 - Month tens in BCD code"]
    #[inline(always)]
    pub fn mont(&mut self) -> MONT_W {
        MONT_W { w: self }
    }
    #[doc = "Bits 8:11 - Month units in BCD code"]
    #[inline(always)]
    pub fn monu(&mut self) -> MONU_W {
        MONU_W { w: self }
    }
    #[doc = "Bits 4:5 - Date tens in BCD code"]
    #[inline(always)]
    pub fn dayt(&mut self) -> DAYT_W {
        DAYT_W { w: self }
    }
    #[doc = "Bits 0:3 - Date units in BCD code"]
    #[inline(always)]
    pub fn dayu(&mut self) -> DAYU_W {
        DAYU_W { w: self }
    }
}

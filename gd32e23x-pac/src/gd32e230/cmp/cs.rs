#[doc = "Reader of register CS"]
pub type R = crate::R<u32, super::CS>;
#[doc = "Writer for register CS"]
pub type W = crate::W<u32, super::CS>;
#[doc = "Register CS `reset()`'s with value 0"]
impl crate::ResetValue for super::CS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPEN`"]
pub type CMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPEN`"]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
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
#[doc = "Reader of field `CMPSW`"]
pub type CMPSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPSW`"]
pub struct CMPSW_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPSW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CMPM`"]
pub type CMPM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPM`"]
pub struct CMPM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CMPMSEL`"]
pub type CMPMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPMSEL`"]
pub struct CMPMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `CMPOSEL`"]
pub type CMPOSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPOSEL`"]
pub struct CMPOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPOSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMPPL`"]
pub type CMPPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPPL`"]
pub struct CMPPL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPPL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CMPHST`"]
pub type CMPHST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPHST`"]
pub struct CMPHST_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPHST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CMPO`"]
pub type CMPO_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPLK`"]
pub type CMPLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPLK`"]
pub struct CMPLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator switch"]
    #[inline(always)]
    pub fn cmpsw(&self) -> CMPSW_R {
        CMPSW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator mode"]
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator input selection"]
    #[inline(always)]
    pub fn cmpmsel(&self) -> CMPMSEL_R {
        CMPMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Comparator output selection"]
    #[inline(always)]
    pub fn cmposel(&self) -> CMPOSEL_R {
        CMPOSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Polarity of comparator output"]
    #[inline(always)]
    pub fn cmppl(&self) -> CMPPL_R {
        CMPPL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Comparator hysteresis"]
    #[inline(always)]
    pub fn cmphst(&self) -> CMPHST_R {
        CMPHST_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Comparator 0 output"]
    #[inline(always)]
    pub fn cmpo(&self) -> CMPO_R {
        CMPO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 0 lock"]
    #[inline(always)]
    pub fn cmplk(&self) -> CMPLK_R {
        CMPLK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator enable"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
    #[doc = "Bit 1 - Comparator switch"]
    #[inline(always)]
    pub fn cmpsw(&mut self) -> CMPSW_W {
        CMPSW_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator mode"]
    #[inline(always)]
    pub fn cmpm(&mut self) -> CMPM_W {
        CMPM_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator input selection"]
    #[inline(always)]
    pub fn cmpmsel(&mut self) -> CMPMSEL_W {
        CMPMSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - Comparator output selection"]
    #[inline(always)]
    pub fn cmposel(&mut self) -> CMPOSEL_W {
        CMPOSEL_W { w: self }
    }
    #[doc = "Bit 11 - Polarity of comparator output"]
    #[inline(always)]
    pub fn cmppl(&mut self) -> CMPPL_W {
        CMPPL_W { w: self }
    }
    #[doc = "Bits 12:13 - Comparator hysteresis"]
    #[inline(always)]
    pub fn cmphst(&mut self) -> CMPHST_W {
        CMPHST_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 0 lock"]
    #[inline(always)]
    pub fn cmplk(&mut self) -> CMPLK_W {
        CMPLK_W { w: self }
    }
}

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
#[doc = "Reader of field `WUPEN6`"]
pub type WUPEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPEN6`"]
pub struct WUPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `WUPEN5`"]
pub type WUPEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPEN5`"]
pub struct WUPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `WUPEN1`"]
pub type WUPEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPEN1`"]
pub struct WUPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `WUPEN0`"]
pub type WUPEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPEN0`"]
pub struct WUPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPEN0_W<'a> {
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
#[doc = "Reader of field `LVDF`"]
pub type LVDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `STBF`"]
pub type STBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUF`"]
pub type WUF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 14 - WKUP pin6 Enable"]
    #[inline(always)]
    pub fn wupen6(&self) -> WUPEN6_R {
        WUPEN6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WKUP pin5 Enable"]
    #[inline(always)]
    pub fn wupen5(&self) -> WUPEN5_R {
        WUPEN5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WKUP pin1 Enable"]
    #[inline(always)]
    pub fn wupen1(&self) -> WUPEN1_R {
        WUPEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WKUP pin0 Enable"]
    #[inline(always)]
    pub fn wupen0(&self) -> WUPEN0_R {
        WUPEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Voltage Detector Status Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LVDF_R {
        LVDF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn stbf(&self) -> STBF_R {
        STBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - WKUP pin6 Enable"]
    #[inline(always)]
    pub fn wupen6(&mut self) -> WUPEN6_W {
        WUPEN6_W { w: self }
    }
    #[doc = "Bit 13 - WKUP pin5 Enable"]
    #[inline(always)]
    pub fn wupen5(&mut self) -> WUPEN5_W {
        WUPEN5_W { w: self }
    }
    #[doc = "Bit 9 - WKUP pin1 Enable"]
    #[inline(always)]
    pub fn wupen1(&mut self) -> WUPEN1_W {
        WUPEN1_W { w: self }
    }
    #[doc = "Bit 8 - WKUP pin0 Enable"]
    #[inline(always)]
    pub fn wupen0(&mut self) -> WUPEN0_W {
        WUPEN0_W { w: self }
    }
}

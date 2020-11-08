#[doc = "Reader of register SMCFG"]
pub type R = crate::R<u32, super::SMCFG>;
#[doc = "Writer for register SMCFG"]
pub type W = crate::W<u32, super::SMCFG>;
#[doc = "Register SMCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ETP`"]
pub type ETP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETP`"]
pub struct ETP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETP_W<'a> {
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
#[doc = "Reader of field `SCM1`"]
pub type SCM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCM1`"]
pub struct SCM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCM1_W<'a> {
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
#[doc = "Reader of field `ETPSC`"]
pub type ETPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETPSC`"]
pub struct ETPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `ETFC`"]
pub type ETFC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETFC`"]
pub struct ETFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MSM`"]
pub type MSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSM`"]
pub struct MSM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TRGS`"]
pub type TRGS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRGS`"]
pub struct TRGS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `OCRC`"]
pub type OCRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCRC`"]
pub struct OCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SMC`"]
pub type SMC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMC`"]
pub struct SMC_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Part of SMC for enable External clock mode1"]
    #[inline(always)]
    pub fn scm1(&self) -> SCM1_R {
        SCM1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpsc(&self) -> ETPSC_R {
        ETPSC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etfc(&self) -> ETFC_R {
        ETFC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&self) -> TRGS_R {
        TRGS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Trigger selection"]
    #[inline(always)]
    pub fn ocrc(&self) -> OCRC_R {
        OCRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W {
        ETP_W { w: self }
    }
    #[doc = "Bit 14 - Part of SMC for enable External clock mode1"]
    #[inline(always)]
    pub fn scm1(&mut self) -> SCM1_W {
        SCM1_W { w: self }
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpsc(&mut self) -> ETPSC_W {
        ETPSC_W { w: self }
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etfc(&mut self) -> ETFC_W {
        ETFC_W { w: self }
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W { w: self }
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&mut self) -> TRGS_W {
        TRGS_W { w: self }
    }
    #[doc = "Bit 3 - Trigger selection"]
    #[inline(always)]
    pub fn ocrc(&mut self) -> OCRC_W {
        OCRC_W { w: self }
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smc(&mut self) -> SMC_W {
        SMC_W { w: self }
    }
}

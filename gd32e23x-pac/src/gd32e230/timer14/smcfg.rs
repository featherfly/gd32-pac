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
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
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
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smc(&mut self) -> SMC_W {
        SMC_W { w: self }
    }
}

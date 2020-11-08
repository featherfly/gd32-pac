#[doc = "Reader of register CFG2"]
pub type R = crate::R<u32, super::CFG2>;
#[doc = "Writer for register CFG2"]
pub type W = crate::W<u32, super::CFG2>;
#[doc = "Register CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRAM_PCEF`"]
pub type SRAM_PCEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM_PCEF`"]
pub struct SRAM_PCEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PCEF_W<'a> {
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
#[doc = "Reader of field `LVD_LOCK`"]
pub type LVD_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVD_LOCK`"]
pub struct LVD_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LVD_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SRAM_PARITY_ERROR_LOCK`"]
pub type SRAM_PARITY_ERROR_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM_PARITY_ERROR_LOCK`"]
pub struct SRAM_PARITY_ERROR_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PARITY_ERROR_LOCK_W<'a> {
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
#[doc = "Reader of field `LOCKUP_LOCK`"]
pub type LOCKUP_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKUP_LOCK`"]
pub struct LOCKUP_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_LOCK_W<'a> {
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
impl R {
    #[doc = "Bit 8 - SRAM parity check error flag"]
    #[inline(always)]
    pub fn sram_pcef(&self) -> SRAM_PCEF_R {
        SRAM_PCEF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LVD lock"]
    #[inline(always)]
    pub fn lvd_lock(&self) -> LVD_LOCK_R {
        LVD_LOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM parity check error lock"]
    #[inline(always)]
    pub fn sram_parity_error_lock(&self) -> SRAM_PARITY_ERROR_LOCK_R {
        SRAM_PARITY_ERROR_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Cortex-M4 LOCKUP output lock"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SRAM parity check error flag"]
    #[inline(always)]
    pub fn sram_pcef(&mut self) -> SRAM_PCEF_W {
        SRAM_PCEF_W { w: self }
    }
    #[doc = "Bit 2 - LVD lock"]
    #[inline(always)]
    pub fn lvd_lock(&mut self) -> LVD_LOCK_W {
        LVD_LOCK_W { w: self }
    }
    #[doc = "Bit 1 - SRAM parity check error lock"]
    #[inline(always)]
    pub fn sram_parity_error_lock(&mut self) -> SRAM_PARITY_ERROR_LOCK_W {
        SRAM_PARITY_ERROR_LOCK_W { w: self }
    }
    #[doc = "Bit 0 - Cortex-M4 LOCKUP output lock"]
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W {
        LOCKUP_LOCK_W { w: self }
    }
}

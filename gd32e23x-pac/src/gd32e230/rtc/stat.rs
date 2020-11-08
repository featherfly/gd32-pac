#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0x07"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Reader of field `SCPF`"]
pub type SCPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TP1F`"]
pub type TP1F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TP1F`"]
pub struct TP1F_W<'a> {
    w: &'a mut W,
}
impl<'a> TP1F_W<'a> {
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
#[doc = "Reader of field `TP0F`"]
pub type TP0F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TP0F`"]
pub struct TP0F_W<'a> {
    w: &'a mut W,
}
impl<'a> TP0F_W<'a> {
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
#[doc = "Reader of field `TSOVRF`"]
pub type TSOVRF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSOVRF`"]
pub struct TSOVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSOVRF_W<'a> {
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
#[doc = "Reader of field `TSF`"]
pub type TSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSF`"]
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
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
#[doc = "Reader of field `ALRM0F`"]
pub type ALRM0F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALRM0F`"]
pub struct ALRM0F_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRM0F_W<'a> {
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
#[doc = "Reader of field `INITM`"]
pub type INITM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INITM`"]
pub struct INITM_W<'a> {
    w: &'a mut W,
}
impl<'a> INITM_W<'a> {
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
#[doc = "Reader of field `INITF`"]
pub type INITF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSYNF`"]
pub type RSYNF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSYNF`"]
pub struct RSYNF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSYNF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `YCM`"]
pub type YCM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOPF`"]
pub type SOPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALRM0WF`"]
pub type ALRM0WF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn scpf(&self) -> SCPF_R {
        SCPF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RTC_TAMP1 detection flag"]
    #[inline(always)]
    pub fn tp1f(&self) -> TP1F_R {
        TP1F_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RTC_TAMP0 detection flag"]
    #[inline(always)]
    pub fn tp0f(&self) -> TP0F_R {
        TP0F_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovrf(&self) -> TSOVRF_R {
        TSOVRF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alrm0f(&self) -> ALRM0F_R {
        ALRM0F_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn initm(&self) -> INITM_R {
        INITM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsynf(&self) -> RSYNF_R {
        RSYNF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn ycm(&self) -> YCM_R {
        YCM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn sopf(&self) -> SOPF_R {
        SOPF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrm0wf(&self) -> ALRM0WF_R {
        ALRM0WF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - RTC_TAMP1 detection flag"]
    #[inline(always)]
    pub fn tp1f(&mut self) -> TP1F_W {
        TP1F_W { w: self }
    }
    #[doc = "Bit 13 - RTC_TAMP0 detection flag"]
    #[inline(always)]
    pub fn tp0f(&mut self) -> TP0F_W {
        TP0F_W { w: self }
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovrf(&mut self) -> TSOVRF_W {
        TSOVRF_W { w: self }
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alrm0f(&mut self) -> ALRM0F_W {
        ALRM0F_W { w: self }
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn initm(&mut self) -> INITM_W {
        INITM_W { w: self }
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsynf(&mut self) -> RSYNF_W {
        RSYNF_W { w: self }
    }
}

#[doc = "Reader of register HRFC"]
pub type R = crate::R<u32, super::HRFC>;
#[doc = "Writer for register HRFC"]
pub type W = crate::W<u32, super::HRFC>;
#[doc = "Register HRFC `reset()`'s with value 0"]
impl crate::ResetValue for super::HRFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FREQI`"]
pub type FREQI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FREQI`"]
pub struct FREQI_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQI_W<'a> {
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
#[doc = "Reader of field `CWND8`"]
pub type CWND8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CWND8`"]
pub struct CWND8_W<'a> {
    w: &'a mut W,
}
impl<'a> CWND8_W<'a> {
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
#[doc = "Reader of field `CWND16`"]
pub type CWND16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CWND16`"]
pub struct CWND16_W<'a> {
    w: &'a mut W,
}
impl<'a> CWND16_W<'a> {
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
#[doc = "Reader of field `CMSK`"]
pub type CMSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMSK`"]
pub struct CMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Increase RTC frequency by 488.5PPM"]
    #[inline(always)]
    pub fn freqi(&self) -> FREQI_R {
        FREQI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Frequency compensation window 8 second selected"]
    #[inline(always)]
    pub fn cwnd8(&self) -> CWND8_R {
        CWND8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Frequency compensation window 16 second selected"]
    #[inline(always)]
    pub fn cwnd16(&self) -> CWND16_R {
        CWND16_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 0:8 - Calibration mask number"]
    #[inline(always)]
    pub fn cmsk(&self) -> CMSK_R {
        CMSK_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Increase RTC frequency by 488.5PPM"]
    #[inline(always)]
    pub fn freqi(&mut self) -> FREQI_W {
        FREQI_W { w: self }
    }
    #[doc = "Bit 14 - Frequency compensation window 8 second selected"]
    #[inline(always)]
    pub fn cwnd8(&mut self) -> CWND8_W {
        CWND8_W { w: self }
    }
    #[doc = "Bit 13 - Frequency compensation window 16 second selected"]
    #[inline(always)]
    pub fn cwnd16(&mut self) -> CWND16_W {
        CWND16_W { w: self }
    }
    #[doc = "Bits 0:8 - Calibration mask number"]
    #[inline(always)]
    pub fn cmsk(&mut self) -> CMSK_W {
        CMSK_W { w: self }
    }
}

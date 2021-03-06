#[doc = "Reader of register CTL1"]
pub type R = crate::R<u32, super::CTL1>;
#[doc = "Writer for register CTL1"]
pub type W = crate::W<u32, super::CTL1>;
#[doc = "Register CTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_HOLD`"]
pub type RTC_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_HOLD`"]
pub struct RTC_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TIMER14_HOLD`"]
pub type TIMER14_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER14_HOLD`"]
pub struct TIMER14_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER14_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TIMER15_HOLD`"]
pub type TIMER15_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER15_HOLD`"]
pub struct TIMER15_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER15_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TIMER16_HOLD`"]
pub type TIMER16_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER16_HOLD`"]
pub struct TIMER16_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER16_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - RTC hold register"]
    #[inline(always)]
    pub fn rtc_hold(&self) -> RTC_HOLD_R {
        RTC_HOLD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timer 14 hold register"]
    #[inline(always)]
    pub fn timer14_hold(&self) -> TIMER14_HOLD_R {
        TIMER14_HOLD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer 15 hold register"]
    #[inline(always)]
    pub fn timer15_hold(&self) -> TIMER15_HOLD_R {
        TIMER15_HOLD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer 16 hold register"]
    #[inline(always)]
    pub fn timer16_hold(&self) -> TIMER16_HOLD_R {
        TIMER16_HOLD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - RTC hold register"]
    #[inline(always)]
    pub fn rtc_hold(&mut self) -> RTC_HOLD_W {
        RTC_HOLD_W { w: self }
    }
    #[doc = "Bit 16 - Timer 14 hold register"]
    #[inline(always)]
    pub fn timer14_hold(&mut self) -> TIMER14_HOLD_W {
        TIMER14_HOLD_W { w: self }
    }
    #[doc = "Bit 17 - Timer 15 hold register"]
    #[inline(always)]
    pub fn timer15_hold(&mut self) -> TIMER15_HOLD_W {
        TIMER15_HOLD_W { w: self }
    }
    #[doc = "Bit 18 - Timer 16 hold register"]
    #[inline(always)]
    pub fn timer16_hold(&mut self) -> TIMER16_HOLD_W {
        TIMER16_HOLD_W { w: self }
    }
}

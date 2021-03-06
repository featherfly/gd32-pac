#[doc = "Reader of register CHCTL2"]
pub type R = crate::R<u32, super::CHCTL2>;
#[doc = "Writer for register CHCTL2"]
pub type W = crate::W<u32, super::CHCTL2>;
#[doc = "Register CHCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH1NP`"]
pub type CH1NP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1NP`"]
pub struct CH1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1NP_W<'a> {
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
#[doc = "Reader of field `CH1P`"]
pub type CH1P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1P`"]
pub struct CH1P_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1P_W<'a> {
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
#[doc = "Reader of field `CH1EN`"]
pub type CH1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1EN`"]
pub struct CH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH0NP`"]
pub type CH0NP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0NP`"]
pub struct CH0NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0NP_W<'a> {
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
#[doc = "Reader of field `CH0NEN`"]
pub type CH0NEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0NEN`"]
pub struct CH0NEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0NEN_W<'a> {
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
#[doc = "Reader of field `CH0P`"]
pub type CH0P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0P`"]
pub struct CH0P_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0P_W<'a> {
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
#[doc = "Reader of field `CH0EN`"]
pub type CH0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0EN`"]
pub struct CH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0EN_W<'a> {
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
    #[doc = "Bit 7 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn ch1np(&self) -> CH1NP_R {
        CH1NP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn ch1p(&self) -> CH1P_R {
        CH1P_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn ch1en(&self) -> CH1EN_R {
        CH1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> CH0NP_R {
        CH0NP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&self) -> CH0NEN_R {
        CH0NEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> CH0P_R {
        CH0P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Capture/Compare 0 output enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn ch1np(&mut self) -> CH1NP_W {
        CH1NP_W { w: self }
    }
    #[doc = "Bit 5 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn ch1p(&mut self) -> CH1P_W {
        CH1P_W { w: self }
    }
    #[doc = "Bit 4 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> CH1EN_W {
        CH1EN_W { w: self }
    }
    #[doc = "Bit 3 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0np(&mut self) -> CH0NP_W {
        CH0NP_W { w: self }
    }
    #[doc = "Bit 2 - Capture/Compare 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&mut self) -> CH0NEN_W {
        CH0NEN_W { w: self }
    }
    #[doc = "Bit 1 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0p(&mut self) -> CH0P_W {
        CH0P_W { w: self }
    }
    #[doc = "Bit 0 - Capture/Compare 0 output enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> CH0EN_W {
        CH0EN_W { w: self }
    }
}

#[doc = "Reader of register INTF"]
pub type R = crate::R<u32, super::INTF>;
#[doc = "Writer for register INTF"]
pub type W = crate::W<u32, super::INTF>;
#[doc = "Register INTF `reset()`'s with value 0"]
impl crate::ResetValue for super::INTF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0OF`"]
pub type CH0OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0OF`"]
pub struct CH0OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OF_W<'a> {
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
#[doc = "Reader of field `CH0IF`"]
pub type CH0IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0IF`"]
pub struct CH0IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0IF_W<'a> {
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
#[doc = "Reader of field `UPIF`"]
pub type UPIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPIF`"]
pub struct UPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UPIF_W<'a> {
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
    #[doc = "Bit 9 - Capture/Compare 0 overcapture flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 0 interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> CH0IF_R {
        CH0IF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UPIF_R {
        UPIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Capture/Compare 0 overcapture flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> CH0OF_W {
        CH0OF_W { w: self }
    }
    #[doc = "Bit 1 - Capture/compare 0 interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&mut self) -> CH0IF_W {
        CH0IF_W { w: self }
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&mut self) -> UPIF_W {
        UPIF_W { w: self }
    }
}

#[doc = "Reader of register CHCTL0_Output"]
pub type R = crate::R<u32, super::CHCTL0_OUTPUT>;
#[doc = "Writer for register CHCTL0_Output"]
pub type W = crate::W<u32, super::CHCTL0_OUTPUT>;
#[doc = "Register CHCTL0_Output `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTL0_OUTPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0MS`"]
pub type CH0MS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0MS`"]
pub struct CH0MS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CH0COMFEN`"]
pub type CH0COMFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0COMFEN`"]
pub struct CH0COMFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0COMFEN_W<'a> {
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
#[doc = "Reader of field `CH0COMSEN`"]
pub type CH0COMSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0COMSEN`"]
pub struct CH0COMSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0COMSEN_W<'a> {
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
#[doc = "Reader of field `CH0COMCTL`"]
pub type CH0COMCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0COMCTL`"]
pub struct CH0COMCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0COMCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 0 selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> CH0MS_R {
        CH0MS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Output compare 0 fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&self) -> CH0COMFEN_R {
        CH0COMFEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Compare 0 preload enable"]
    #[inline(always)]
    pub fn ch0comsen(&self) -> CH0COMSEN_R {
        CH0COMSEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Output Compare 0 mode"]
    #[inline(always)]
    pub fn ch0comctl(&self) -> CH0COMCTL_R {
        CH0COMCTL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 0 selection"]
    #[inline(always)]
    pub fn ch0ms(&mut self) -> CH0MS_W {
        CH0MS_W { w: self }
    }
    #[doc = "Bit 2 - Output compare 0 fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&mut self) -> CH0COMFEN_W {
        CH0COMFEN_W { w: self }
    }
    #[doc = "Bit 3 - Output Compare 0 preload enable"]
    #[inline(always)]
    pub fn ch0comsen(&mut self) -> CH0COMSEN_W {
        CH0COMSEN_W { w: self }
    }
    #[doc = "Bits 4:6 - Output Compare 0 mode"]
    #[inline(always)]
    pub fn ch0comctl(&mut self) -> CH0COMCTL_W {
        CH0COMCTL_W { w: self }
    }
}

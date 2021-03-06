#[doc = "Writer for register SWEVG"]
pub type W = crate::W<u32, super::SWEVG>;
#[doc = "Register SWEVG `reset()`'s with value 0"]
impl crate::ResetValue for super::SWEVG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TRGG`"]
pub struct TRGG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `CH3G`"]
pub struct CH3G_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3G_W<'a> {
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
#[doc = "Write proxy for field `CH2G`"]
pub struct CH2G_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2G_W<'a> {
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
#[doc = "Write proxy for field `CH1G`"]
pub struct CH1G_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1G_W<'a> {
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
#[doc = "Write proxy for field `CH0G`"]
pub struct CH0G_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0G_W<'a> {
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
#[doc = "Write proxy for field `UPG`"]
pub struct UPG_W<'a> {
    w: &'a mut W,
}
impl<'a> UPG_W<'a> {
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
impl W {
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn trgg(&mut self) -> TRGG_W {
        TRGG_W { w: self }
    }
    #[doc = "Bit 4 - Capture/compare 3 generation"]
    #[inline(always)]
    pub fn ch3g(&mut self) -> CH3G_W {
        CH3G_W { w: self }
    }
    #[doc = "Bit 3 - Capture/compare 2 generation"]
    #[inline(always)]
    pub fn ch2g(&mut self) -> CH2G_W {
        CH2G_W { w: self }
    }
    #[doc = "Bit 2 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn ch1g(&mut self) -> CH1G_W {
        CH1G_W { w: self }
    }
    #[doc = "Bit 1 - Capture/compare 0 generation"]
    #[inline(always)]
    pub fn ch0g(&mut self) -> CH0G_W {
        CH0G_W { w: self }
    }
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn upg(&mut self) -> UPG_W {
        UPG_W { w: self }
    }
}

#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXFCMD`"]
pub struct TXFCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFCMD_W<'a> {
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
#[doc = "Write proxy for field `RXFCMD`"]
pub struct RXFCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFCMD_W<'a> {
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
#[doc = "Write proxy for field `MMCMD`"]
pub struct MMCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> MMCMD_W<'a> {
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
#[doc = "Write proxy for field `SBKCMD`"]
pub struct SBKCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SBKCMD_W<'a> {
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
#[doc = "Write proxy for field `ABDCMD`"]
pub struct ABDCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ABDCMD_W<'a> {
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
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfcmd(&mut self) -> TXFCMD_W {
        TXFCMD_W { w: self }
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfcmd(&mut self) -> RXFCMD_W {
        RXFCMD_W { w: self }
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmcmd(&mut self) -> MMCMD_W {
        MMCMD_W { w: self }
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkcmd(&mut self) -> SBKCMD_W {
        SBKCMD_W { w: self }
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abdcmd(&mut self) -> ABDCMD_W {
        ABDCMD_W { w: self }
    }
}

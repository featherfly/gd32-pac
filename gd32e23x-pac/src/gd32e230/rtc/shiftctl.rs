#[doc = "Writer for register SHIFTCTL"]
pub type W = crate::W<u32, super::SHIFTCTL>;
#[doc = "Register SHIFTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `A1S`"]
pub struct A1S_W<'a> {
    w: &'a mut W,
}
impl<'a> A1S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Write proxy for field `SFS`"]
pub struct SFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl W {
    #[doc = "Bit 31 - One second add"]
    #[inline(always)]
    pub fn a1s(&mut self) -> A1S_W {
        A1S_W { w: self }
    }
    #[doc = "Bits 0:14 - Subtract a fraction of a second"]
    #[inline(always)]
    pub fn sfs(&mut self) -> SFS_W {
        SFS_W { w: self }
    }
}

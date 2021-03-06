#[doc = "Reader of register CFG1"]
pub type R = crate::R<u32, super::CFG1>;
#[doc = "Writer for register CFG1"]
pub type W = crate::W<u32, super::CFG1>;
#[doc = "Register CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PREDV`"]
pub type PREDV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PREDV`"]
pub struct PREDV_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CK_HXTAL or CK_IRC48M divider previous PLL"]
    #[inline(always)]
    pub fn predv(&self) -> PREDV_R {
        PREDV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CK_HXTAL or CK_IRC48M divider previous PLL"]
    #[inline(always)]
    pub fn predv(&mut self) -> PREDV_W {
        PREDV_W { w: self }
    }
}

#[doc = "Reader of register CH0CV"]
pub type R = crate::R<u32, super::CH0CV>;
#[doc = "Writer for register CH0CV"]
pub type W = crate::W<u32, super::CH0CV>;
#[doc = "Register CH0CV `reset()`'s with value 0"]
impl crate::ResetValue for super::CH0CV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0VAL`"]
pub type CH0VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH0VAL`"]
pub struct CH0VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 0 value"]
    #[inline(always)]
    pub fn ch0val(&self) -> CH0VAL_R {
        CH0VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 0 value"]
    #[inline(always)]
    pub fn ch0val(&mut self) -> CH0VAL_W {
        CH0VAL_W { w: self }
    }
}

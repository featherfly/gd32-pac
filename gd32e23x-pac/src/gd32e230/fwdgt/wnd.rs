#[doc = "Reader of register WND"]
pub type R = crate::R<u32, super::WND>;
#[doc = "Writer for register WND"]
pub type W = crate::W<u32, super::WND>;
#[doc = "Register WND `reset()`'s with value 0x0fff"]
impl crate::ResetValue for super::WND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff
    }
}
#[doc = "Reader of field `WND`"]
pub type WND_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WND`"]
pub struct WND_W<'a> {
    w: &'a mut W,
}
impl<'a> WND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&self) -> WND_R {
        WND_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&mut self) -> WND_W {
        WND_W { w: self }
    }
}

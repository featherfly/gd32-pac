#[doc = "Reader of register WP"]
pub type R = crate::R<u32, super::WP>;
#[doc = "Reader of field `WP`"]
pub type WP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Store WP\\[15:0\\]
of option bytes block after system reset"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new((self.bits & 0xffff) as u16)
    }
}

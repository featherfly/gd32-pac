#[doc = "Reader of register CFG0"]
pub type R = crate::R<u32, super::CFG0>;
#[doc = "Writer for register CFG0"]
pub type W = crate::W<u32, super::CFG0>;
#[doc = "Register CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PB9_HCCE`"]
pub type PB9_HCCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB9_HCCE`"]
pub struct PB9_HCCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PB9_HCCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TIMER16_DMA_RMP`"]
pub type TIMER16_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER16_DMA_RMP`"]
pub struct TIMER16_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER16_DMA_RMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TIMER15_DMA_RMP`"]
pub type TIMER15_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER15_DMA_RMP`"]
pub struct TIMER15_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER15_DMA_RMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `USART0_RX_DMA_RMP`"]
pub type USART0_RX_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART0_RX_DMA_RMP`"]
pub struct USART0_RX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART0_RX_DMA_RMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `USART0_TX_DMA_RMP`"]
pub type USART0_TX_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART0_TX_DMA_RMP`"]
pub struct USART0_TX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART0_TX_DMA_RMP_W<'a> {
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
#[doc = "Reader of field `ADC_DMA_RMP`"]
pub type ADC_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_DMA_RMP`"]
pub struct ADC_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DMA_RMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PA11_PA12_RMP`"]
pub type PA11_PA12_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA11_PA12_RMP`"]
pub struct PA11_PA12_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA11_PA12_RMP_W<'a> {
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
#[doc = "Reader of field `BOOT_MODE`"]
pub type BOOT_MODE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 19 - PB9 pin high current capability enable"]
    #[inline(always)]
    pub fn pb9_hcce(&self) -> PB9_HCCE_R {
        PB9_HCCE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timer 16 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer16_dma_rmp(&self) -> TIMER16_DMA_RMP_R {
        TIMER16_DMA_RMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Timer 15 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer15_dma_rmp(&self) -> TIMER15_DMA_RMP_R {
        TIMER15_DMA_RMP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART0_RX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_rx_dma_rmp(&self) -> USART0_RX_DMA_RMP_R {
        USART0_RX_DMA_RMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USART0_TX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_tx_dma_rmp(&self) -> USART0_TX_DMA_RMP_R {
        USART0_TX_DMA_RMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC DMA request remapping enable"]
    #[inline(always)]
    pub fn adc_dma_rmp(&self) -> ADC_DMA_RMP_R {
        ADC_DMA_RMP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> PA11_PA12_RMP_R {
        PA11_PA12_RMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Boot mode"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 19 - PB9 pin high current capability enable"]
    #[inline(always)]
    pub fn pb9_hcce(&mut self) -> PB9_HCCE_W {
        PB9_HCCE_W { w: self }
    }
    #[doc = "Bit 12 - Timer 16 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer16_dma_rmp(&mut self) -> TIMER16_DMA_RMP_W {
        TIMER16_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 11 - Timer 15 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer15_dma_rmp(&mut self) -> TIMER15_DMA_RMP_W {
        TIMER15_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 10 - USART0_RX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_rx_dma_rmp(&mut self) -> USART0_RX_DMA_RMP_W {
        USART0_RX_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 9 - USART0_TX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_tx_dma_rmp(&mut self) -> USART0_TX_DMA_RMP_W {
        USART0_TX_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 8 - ADC DMA request remapping enable"]
    #[inline(always)]
    pub fn adc_dma_rmp(&mut self) -> ADC_DMA_RMP_W {
        ADC_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&mut self) -> PA11_PA12_RMP_W {
        PA11_PA12_RMP_W { w: self }
    }
}

#[doc = "Reader of register CTL1"]
pub type R = crate::R<u32, super::CTL1>;
#[doc = "Writer for register CTL1"]
pub type W = crate::W<u32, super::CTL1>;
#[doc = "Register CTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDMA_ODD`"]
pub type TXDMA_ODD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDMA_ODD`"]
pub struct TXDMA_ODD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_ODD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RXDMA_ODD`"]
pub type RXDMA_ODD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDMA_ODD`"]
pub struct RXDMA_ODD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_ODD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `BYTEN`"]
pub type BYTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTEN`"]
pub struct BYTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTEN_W<'a> {
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
#[doc = "Reader of field `DZ`"]
pub type DZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DZ`"]
pub struct DZ_W<'a> {
    w: &'a mut W,
}
impl<'a> DZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TBEIE`"]
pub type TBEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBEIE`"]
pub struct TBEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RBNEIE`"]
pub type RBNEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RBNEIE`"]
pub struct RBNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBNEIE_W<'a> {
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
#[doc = "Reader of field `ERRIE`"]
pub type ERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRIE`"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TMOD`"]
pub type TMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMOD`"]
pub struct TMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMOD_W<'a> {
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
#[doc = "Reader of field `NSSP`"]
pub type NSSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSSP`"]
pub struct NSSP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSSP_W<'a> {
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
#[doc = "Reader of field `NSSDRV`"]
pub type NSSDRV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSSDRV`"]
pub struct NSSDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> NSSDRV_W<'a> {
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
#[doc = "Reader of field `DMATEN`"]
pub type DMATEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMATEN`"]
pub struct DMATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATEN_W<'a> {
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
#[doc = "Reader of field `DMAREN`"]
pub type DMAREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAREN`"]
pub struct DMAREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREN_W<'a> {
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
    #[doc = "Bit 14 - Odd bytes in TX DMA channel"]
    #[inline(always)]
    pub fn txdma_odd(&self) -> TXDMA_ODD_R {
        TXDMA_ODD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Odd bytes in RX DMA channel"]
    #[inline(always)]
    pub fn rxdma_odd(&self) -> RXDMA_ODD_R {
        RXDMA_ODD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Byte access enable"]
    #[inline(always)]
    pub fn byten(&self) -> BYTEN_R {
        BYTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Date size"]
    #[inline(always)]
    pub fn dz(&self) -> DZ_R {
        DZ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Buffer Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SPI NSS Pulse Mode Enable"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NSS output enable"]
    #[inline(always)]
    pub fn nssdrv(&self) -> NSSDRV_R {
        NSSDRV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Odd bytes in TX DMA channel"]
    #[inline(always)]
    pub fn txdma_odd(&mut self) -> TXDMA_ODD_W {
        TXDMA_ODD_W { w: self }
    }
    #[doc = "Bit 13 - Odd bytes in RX DMA channel"]
    #[inline(always)]
    pub fn rxdma_odd(&mut self) -> RXDMA_ODD_W {
        RXDMA_ODD_W { w: self }
    }
    #[doc = "Bit 12 - Byte access enable"]
    #[inline(always)]
    pub fn byten(&mut self) -> BYTEN_W {
        BYTEN_W { w: self }
    }
    #[doc = "Bits 8:11 - Date size"]
    #[inline(always)]
    pub fn dz(&mut self) -> DZ_W {
        DZ_W { w: self }
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tbeie(&mut self) -> TBEIE_W {
        TBEIE_W { w: self }
    }
    #[doc = "Bit 6 - Receive Buffer Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rbneie(&mut self) -> RBNEIE_W {
        RBNEIE_W { w: self }
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W {
        TMOD_W { w: self }
    }
    #[doc = "Bit 3 - SPI NSS Pulse Mode Enable"]
    #[inline(always)]
    pub fn nssp(&mut self) -> NSSP_W {
        NSSP_W { w: self }
    }
    #[doc = "Bit 2 - NSS output enable"]
    #[inline(always)]
    pub fn nssdrv(&mut self) -> NSSDRV_W {
        NSSDRV_W { w: self }
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaten(&mut self) -> DMATEN_W {
        DMATEN_W { w: self }
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W {
        DMAREN_W { w: self }
    }
}

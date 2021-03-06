#[doc = "Reader of register APB2EN"]
pub type R = crate::R<u32, super::APB2EN>;
#[doc = "Writer for register APB2EN"]
pub type W = crate::W<u32, super::APB2EN>;
#[doc = "Register APB2EN `reset()`'s with value 0"]
impl crate::ResetValue for super::APB2EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBGMCUEN`"]
pub type DBGMCUEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGMCUEN`"]
pub struct DBGMCUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGMCUEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TIMER16EN`"]
pub type TIMER16EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER16EN`"]
pub struct TIMER16EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER16EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TIMER15EN`"]
pub type TIMER15EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER15EN`"]
pub struct TIMER15EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER15EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TIMER14EN`"]
pub type TIMER14EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER14EN`"]
pub struct TIMER14EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER14EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `USART0EN`"]
pub type USART0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART0EN`"]
pub struct USART0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART0EN_W<'a> {
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
#[doc = "Reader of field `SPI0EN`"]
pub type SPI0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0EN`"]
pub struct SPI0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0EN_W<'a> {
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
#[doc = "Reader of field `TIMER0EN`"]
pub type TIMER0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0EN`"]
pub struct TIMER0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0EN_W<'a> {
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
#[doc = "Reader of field `ADCEN`"]
pub type ADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCEN`"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
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
#[doc = "Reader of field `CFGCMPEN`"]
pub type CFGCMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFGCMPEN`"]
pub struct CFGCMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGCMPEN_W<'a> {
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
    #[doc = "Bit 22 - DBGMCU clock enable"]
    #[inline(always)]
    pub fn dbgmcuen(&self) -> DBGMCUEN_R {
        DBGMCUEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIMER16 timer clock enable"]
    #[inline(always)]
    pub fn timer16en(&self) -> TIMER16EN_R {
        TIMER16EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIMER15 timer clock enable"]
    #[inline(always)]
    pub fn timer15en(&self) -> TIMER15EN_R {
        TIMER15EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIMER14 timer clock enable"]
    #[inline(always)]
    pub fn timer14en(&self) -> TIMER14EN_R {
        TIMER14EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&self) -> USART0EN_R {
        USART0EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&self) -> SPI0EN_R {
        SPI0EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIMER0 timer clock enable"]
    #[inline(always)]
    pub fn timer0en(&self) -> TIMER0EN_R {
        TIMER0EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 0 - System configuration and comparator clock enable"]
    #[inline(always)]
    pub fn cfgcmpen(&self) -> CFGCMPEN_R {
        CFGCMPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - DBGMCU clock enable"]
    #[inline(always)]
    pub fn dbgmcuen(&mut self) -> DBGMCUEN_W {
        DBGMCUEN_W { w: self }
    }
    #[doc = "Bit 18 - TIMER16 timer clock enable"]
    #[inline(always)]
    pub fn timer16en(&mut self) -> TIMER16EN_W {
        TIMER16EN_W { w: self }
    }
    #[doc = "Bit 17 - TIMER15 timer clock enable"]
    #[inline(always)]
    pub fn timer15en(&mut self) -> TIMER15EN_W {
        TIMER15EN_W { w: self }
    }
    #[doc = "Bit 16 - TIMER14 timer clock enable"]
    #[inline(always)]
    pub fn timer14en(&mut self) -> TIMER14EN_W {
        TIMER14EN_W { w: self }
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&mut self) -> USART0EN_W {
        USART0EN_W { w: self }
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&mut self) -> SPI0EN_W {
        SPI0EN_W { w: self }
    }
    #[doc = "Bit 11 - TIMER0 timer clock enable"]
    #[inline(always)]
    pub fn timer0en(&mut self) -> TIMER0EN_W {
        TIMER0EN_W { w: self }
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    #[doc = "Bit 0 - System configuration and comparator clock enable"]
    #[inline(always)]
    pub fn cfgcmpen(&mut self) -> CFGCMPEN_W {
        CFGCMPEN_W { w: self }
    }
}

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
#[doc = "Reader of field `PLLDV`"]
pub type PLLDV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLDV`"]
pub struct PLLDV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLDV_W<'a> {
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
#[doc = "Reader of field `CKOUTDIV`"]
pub type CKOUTDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKOUTDIV`"]
pub struct CKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `PLLMF_MSB`"]
pub type PLLMF_MSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLMF_MSB`"]
pub struct PLLMF_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMF_MSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `CKOUTSEL`"]
pub type CKOUTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKOUTSEL`"]
pub struct CKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `PLLMF`"]
pub type PLLMF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLMF`"]
pub struct PLLMF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `PLLPREDV`"]
pub type PLLPREDV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLPREDV`"]
pub struct PLLPREDV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPREDV_W<'a> {
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
#[doc = "Reader of field `PLLSEL`"]
pub type PLLSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSEL`"]
pub struct PLLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSEL_W<'a> {
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
#[doc = "Reader of field `ADCPSC`"]
pub type ADCPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCPSC`"]
pub struct ADCPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `APB2PSC`"]
pub type APB2PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB2PSC`"]
pub struct APB2PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `APB1PSC`"]
pub type APB1PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB1PSC`"]
pub struct APB1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> APB1PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `AHBPSC`"]
pub type AHBPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHBPSC`"]
pub struct AHBPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SCSS`"]
pub type SCSS_R = crate::R<u8, u8>;
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCS`"]
pub struct SCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - The CK_PLL divide by 1 or 2 for CK_OUT"]
    #[inline(always)]
    pub fn plldv(&self) -> PLLDV_R {
        PLLDV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - The CK_OUT divider which the CK_OUT frequency can be reduced"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Bit 4 of PLLMF register"]
    #[inline(always)]
    pub fn pllmf_msb(&self) -> PLLMF_MSB_R {
        PLLMF_MSB_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - CK_OUT Clock Source Selection"]
    #[inline(always)]
    pub fn ckoutsel(&self) -> CKOUTSEL_R {
        CKOUTSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 18:21 - PLL multiply factor"]
    #[inline(always)]
    pub fn pllmf(&self) -> PLLMF_R {
        PLLMF_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - HXTAL divider for PLL source clock selection."]
    #[inline(always)]
    pub fn pllpredv(&self) -> PLLPREDV_R {
        PLLPREDV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&self) -> PLLSEL_R {
        PLLSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc(&self) -> ADCPSC_R {
        ADCPSC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&self) -> APB2PSC_R {
        APB2PSC_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&self) -> APB1PSC_R {
        APB1PSC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&self) -> AHBPSC_R {
        AHBPSC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn scss(&self) -> SCSS_R {
        SCSS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - The CK_PLL divide by 1 or 2 for CK_OUT"]
    #[inline(always)]
    pub fn plldv(&mut self) -> PLLDV_W {
        PLLDV_W { w: self }
    }
    #[doc = "Bits 28:30 - The CK_OUT divider which the CK_OUT frequency can be reduced"]
    #[inline(always)]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W {
        CKOUTDIV_W { w: self }
    }
    #[doc = "Bit 27 - Bit 4 of PLLMF register"]
    #[inline(always)]
    pub fn pllmf_msb(&mut self) -> PLLMF_MSB_W {
        PLLMF_MSB_W { w: self }
    }
    #[doc = "Bits 24:26 - CK_OUT Clock Source Selection"]
    #[inline(always)]
    pub fn ckoutsel(&mut self) -> CKOUTSEL_W {
        CKOUTSEL_W { w: self }
    }
    #[doc = "Bits 18:21 - PLL multiply factor"]
    #[inline(always)]
    pub fn pllmf(&mut self) -> PLLMF_W {
        PLLMF_W { w: self }
    }
    #[doc = "Bit 17 - HXTAL divider for PLL source clock selection."]
    #[inline(always)]
    pub fn pllpredv(&mut self) -> PLLPREDV_W {
        PLLPREDV_W { w: self }
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&mut self) -> PLLSEL_W {
        PLLSEL_W { w: self }
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc(&mut self) -> ADCPSC_W {
        ADCPSC_W { w: self }
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&mut self) -> APB2PSC_W {
        APB2PSC_W { w: self }
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&mut self) -> APB1PSC_W {
        APB1PSC_W { w: self }
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&mut self) -> AHBPSC_W {
        AHBPSC_W { w: self }
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W { w: self }
    }
}

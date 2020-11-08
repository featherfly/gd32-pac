#[doc = "Reader of register SAMCS"]
pub type R = crate::R<u32, super::SAMCS>;
#[doc = "Writer for register SAMCS"]
pub type W = crate::W<u32, super::SAMCS>;
#[doc = "Register SAMCS `reset()`'s with value 0"]
impl crate::ResetValue for super::SAMCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFR`"]
pub type RFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFR`"]
pub struct RFR_W<'a> {
    w: &'a mut W,
}
impl<'a> RFR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RFF`"]
pub type RFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFF`"]
pub struct RFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFF_W<'a> {
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
#[doc = "Reader of field `TFR`"]
pub type TFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFR`"]
pub struct TFR_W<'a> {
    w: &'a mut W,
}
impl<'a> TFR_W<'a> {
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
#[doc = "Reader of field `TFF`"]
pub type TFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFF`"]
pub struct TFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFF_W<'a> {
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
#[doc = "Reader of field `RXF`"]
pub type RXF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXF`"]
pub struct RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXF_W<'a> {
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
#[doc = "Reader of field `TXF`"]
pub type TXF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXF`"]
pub struct TXF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXF_W<'a> {
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
#[doc = "Reader of field `RFRIE`"]
pub type RFRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFRIE`"]
pub struct RFRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFRIE_W<'a> {
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
#[doc = "Reader of field `RFFIE`"]
pub type RFFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFFIE`"]
pub struct RFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFFIE_W<'a> {
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
#[doc = "Reader of field `TFRIE`"]
pub type TFRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFRIE`"]
pub struct TFRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFRIE_W<'a> {
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
#[doc = "Reader of field `TFFIE`"]
pub type TFFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFFIE`"]
pub struct TFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFFIE_W<'a> {
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
#[doc = "Reader of field `STOEN`"]
pub type STOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOEN`"]
pub struct STOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STOEN_W<'a> {
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
#[doc = "Reader of field `SAMEN`"]
pub type SAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAMEN`"]
pub struct SAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMEN_W<'a> {
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
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    pub fn rfr(&self) -> RFR_R {
        RFR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - level of rx frame signal"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - level of tx frame signal"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx frame rise interrupt enable"]
    #[inline(always)]
    pub fn rfrie(&self) -> RFRIE_R {
        RFRIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx frame fall interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RFFIE_R {
        RFFIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Tx frame rise interrupt enable"]
    #[inline(always)]
    pub fn tfrie(&self) -> TFRIE_R {
        TFRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tx frame fall interrupt enable"]
    #[inline(always)]
    pub fn tffie(&self) -> TFFIE_R {
        TFFIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    pub fn stoen(&self) -> STOEN_R {
        STOEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    pub fn samen(&self) -> SAMEN_R {
        SAMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    pub fn rfr(&mut self) -> RFR_W {
        RFR_W { w: self }
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    pub fn rff(&mut self) -> RFF_W {
        RFF_W { w: self }
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    pub fn tfr(&mut self) -> TFR_W {
        TFR_W { w: self }
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    pub fn tff(&mut self) -> TFF_W {
        TFF_W { w: self }
    }
    #[doc = "Bit 9 - level of rx frame signal"]
    #[inline(always)]
    pub fn rxf(&mut self) -> RXF_W {
        RXF_W { w: self }
    }
    #[doc = "Bit 8 - level of tx frame signal"]
    #[inline(always)]
    pub fn txf(&mut self) -> TXF_W {
        TXF_W { w: self }
    }
    #[doc = "Bit 7 - Rx frame rise interrupt enable"]
    #[inline(always)]
    pub fn rfrie(&mut self) -> RFRIE_W {
        RFRIE_W { w: self }
    }
    #[doc = "Bit 6 - Rx frame fall interrupt enable"]
    #[inline(always)]
    pub fn rffie(&mut self) -> RFFIE_W {
        RFFIE_W { w: self }
    }
    #[doc = "Bit 5 - Tx frame rise interrupt enable"]
    #[inline(always)]
    pub fn tfrie(&mut self) -> TFRIE_W {
        TFRIE_W { w: self }
    }
    #[doc = "Bit 4 - Tx frame fall interrupt enable"]
    #[inline(always)]
    pub fn tffie(&mut self) -> TFFIE_W {
        TFFIE_W { w: self }
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    pub fn stoen(&mut self) -> STOEN_W {
        STOEN_W { w: self }
    }
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    pub fn samen(&mut self) -> SAMEN_W {
        SAMEN_W { w: self }
    }
}

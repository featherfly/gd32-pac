#[doc = "Reader of register OMODE"]
pub type R = crate::R<u32, super::OMODE>;
#[doc = "Writer for register OMODE"]
pub type W = crate::W<u32, super::OMODE>;
#[doc = "Register OMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::OMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OM15`"]
pub type OM15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM15`"]
pub struct OM15_W<'a> {
    w: &'a mut W,
}
impl<'a> OM15_W<'a> {
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
#[doc = "Reader of field `OM14`"]
pub type OM14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM14`"]
pub struct OM14_W<'a> {
    w: &'a mut W,
}
impl<'a> OM14_W<'a> {
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
#[doc = "Reader of field `OM13`"]
pub type OM13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM13`"]
pub struct OM13_W<'a> {
    w: &'a mut W,
}
impl<'a> OM13_W<'a> {
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
#[doc = "Reader of field `OM12`"]
pub type OM12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM12`"]
pub struct OM12_W<'a> {
    w: &'a mut W,
}
impl<'a> OM12_W<'a> {
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
#[doc = "Reader of field `OM11`"]
pub type OM11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM11`"]
pub struct OM11_W<'a> {
    w: &'a mut W,
}
impl<'a> OM11_W<'a> {
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
#[doc = "Reader of field `OM10`"]
pub type OM10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM10`"]
pub struct OM10_W<'a> {
    w: &'a mut W,
}
impl<'a> OM10_W<'a> {
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
#[doc = "Reader of field `OM9`"]
pub type OM9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM9`"]
pub struct OM9_W<'a> {
    w: &'a mut W,
}
impl<'a> OM9_W<'a> {
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
#[doc = "Reader of field `OM8`"]
pub type OM8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM8`"]
pub struct OM8_W<'a> {
    w: &'a mut W,
}
impl<'a> OM8_W<'a> {
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
#[doc = "Reader of field `OM7`"]
pub type OM7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM7`"]
pub struct OM7_W<'a> {
    w: &'a mut W,
}
impl<'a> OM7_W<'a> {
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
#[doc = "Reader of field `OM6`"]
pub type OM6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM6`"]
pub struct OM6_W<'a> {
    w: &'a mut W,
}
impl<'a> OM6_W<'a> {
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
#[doc = "Reader of field `OM5`"]
pub type OM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM5`"]
pub struct OM5_W<'a> {
    w: &'a mut W,
}
impl<'a> OM5_W<'a> {
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
#[doc = "Reader of field `OM4`"]
pub type OM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM4`"]
pub struct OM4_W<'a> {
    w: &'a mut W,
}
impl<'a> OM4_W<'a> {
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
#[doc = "Reader of field `OM3`"]
pub type OM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM3`"]
pub struct OM3_W<'a> {
    w: &'a mut W,
}
impl<'a> OM3_W<'a> {
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
#[doc = "Reader of field `OM2`"]
pub type OM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM2`"]
pub struct OM2_W<'a> {
    w: &'a mut W,
}
impl<'a> OM2_W<'a> {
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
#[doc = "Reader of field `OM1`"]
pub type OM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM1`"]
pub struct OM1_W<'a> {
    w: &'a mut W,
}
impl<'a> OM1_W<'a> {
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
#[doc = "Reader of field `OM0`"]
pub type OM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OM0`"]
pub struct OM0_W<'a> {
    w: &'a mut W,
}
impl<'a> OM0_W<'a> {
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
    #[doc = "Bit 15 - Port x configuration bit 15"]
    #[inline(always)]
    pub fn om15(&self) -> OM15_R {
        OM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port x configuration bit 14"]
    #[inline(always)]
    pub fn om14(&self) -> OM14_R {
        OM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port x configuration bit 13"]
    #[inline(always)]
    pub fn om13(&self) -> OM13_R {
        OM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port x configuration bit 12"]
    #[inline(always)]
    pub fn om12(&self) -> OM12_R {
        OM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port x configuration bit 11"]
    #[inline(always)]
    pub fn om11(&self) -> OM11_R {
        OM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port x configuration bit 10"]
    #[inline(always)]
    pub fn om10(&self) -> OM10_R {
        OM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port x configuration bit 9"]
    #[inline(always)]
    pub fn om9(&self) -> OM9_R {
        OM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port x configuration bit 8"]
    #[inline(always)]
    pub fn om8(&self) -> OM8_R {
        OM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port x configuration bit 7"]
    #[inline(always)]
    pub fn om7(&self) -> OM7_R {
        OM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port x configuration bit 6"]
    #[inline(always)]
    pub fn om6(&self) -> OM6_R {
        OM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port x configuration bit 5"]
    #[inline(always)]
    pub fn om5(&self) -> OM5_R {
        OM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port x configuration bit 4"]
    #[inline(always)]
    pub fn om4(&self) -> OM4_R {
        OM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port x configuration bit 3"]
    #[inline(always)]
    pub fn om3(&self) -> OM3_R {
        OM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port x configuration bit 2"]
    #[inline(always)]
    pub fn om2(&self) -> OM2_R {
        OM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port x configuration bit 1"]
    #[inline(always)]
    pub fn om1(&self) -> OM1_R {
        OM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port x configuration bit 0"]
    #[inline(always)]
    pub fn om0(&self) -> OM0_R {
        OM0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port x configuration bit 15"]
    #[inline(always)]
    pub fn om15(&mut self) -> OM15_W {
        OM15_W { w: self }
    }
    #[doc = "Bit 14 - Port x configuration bit 14"]
    #[inline(always)]
    pub fn om14(&mut self) -> OM14_W {
        OM14_W { w: self }
    }
    #[doc = "Bit 13 - Port x configuration bit 13"]
    #[inline(always)]
    pub fn om13(&mut self) -> OM13_W {
        OM13_W { w: self }
    }
    #[doc = "Bit 12 - Port x configuration bit 12"]
    #[inline(always)]
    pub fn om12(&mut self) -> OM12_W {
        OM12_W { w: self }
    }
    #[doc = "Bit 11 - Port x configuration bit 11"]
    #[inline(always)]
    pub fn om11(&mut self) -> OM11_W {
        OM11_W { w: self }
    }
    #[doc = "Bit 10 - Port x configuration bit 10"]
    #[inline(always)]
    pub fn om10(&mut self) -> OM10_W {
        OM10_W { w: self }
    }
    #[doc = "Bit 9 - Port x configuration bit 9"]
    #[inline(always)]
    pub fn om9(&mut self) -> OM9_W {
        OM9_W { w: self }
    }
    #[doc = "Bit 8 - Port x configuration bit 8"]
    #[inline(always)]
    pub fn om8(&mut self) -> OM8_W {
        OM8_W { w: self }
    }
    #[doc = "Bit 7 - Port x configuration bit 7"]
    #[inline(always)]
    pub fn om7(&mut self) -> OM7_W {
        OM7_W { w: self }
    }
    #[doc = "Bit 6 - Port x configuration bit 6"]
    #[inline(always)]
    pub fn om6(&mut self) -> OM6_W {
        OM6_W { w: self }
    }
    #[doc = "Bit 5 - Port x configuration bit 5"]
    #[inline(always)]
    pub fn om5(&mut self) -> OM5_W {
        OM5_W { w: self }
    }
    #[doc = "Bit 4 - Port x configuration bit 4"]
    #[inline(always)]
    pub fn om4(&mut self) -> OM4_W {
        OM4_W { w: self }
    }
    #[doc = "Bit 3 - Port x configuration bit 3"]
    #[inline(always)]
    pub fn om3(&mut self) -> OM3_W {
        OM3_W { w: self }
    }
    #[doc = "Bit 2 - Port x configuration bit 2"]
    #[inline(always)]
    pub fn om2(&mut self) -> OM2_W {
        OM2_W { w: self }
    }
    #[doc = "Bit 1 - Port x configuration bit 1"]
    #[inline(always)]
    pub fn om1(&mut self) -> OM1_W {
        OM1_W { w: self }
    }
    #[doc = "Bit 0 - Port x configuration bit 0"]
    #[inline(always)]
    pub fn om0(&mut self) -> OM0_W {
        OM0_W { w: self }
    }
}

#[doc = "Reader of register ALRM0TD"]
pub type R = crate::R<u32, super::ALRM0TD>;
#[doc = "Writer for register ALRM0TD"]
pub type W = crate::W<u32, super::ALRM0TD>;
#[doc = "Register ALRM0TD `reset()`'s with value 0"]
impl crate::ResetValue for super::ALRM0TD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSKD`"]
pub type MSKD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSKD`"]
pub struct MSKD_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKD_W<'a> {
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
#[doc = "Reader of field `DOWS`"]
pub type DOWS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOWS`"]
pub struct DOWS_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DAYT`"]
pub type DAYT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYT`"]
pub struct DAYT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `DAYU`"]
pub type DAYU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYU`"]
pub struct DAYU_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `MSKH`"]
pub type MSKH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSKH`"]
pub struct MSKH_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `PM`"]
pub type PM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PM`"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
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
#[doc = "Reader of field `HRT`"]
pub type HRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HRT`"]
pub struct HRT_W<'a> {
    w: &'a mut W,
}
impl<'a> HRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `HRU`"]
pub type HRU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HRU`"]
pub struct HRU_W<'a> {
    w: &'a mut W,
}
impl<'a> HRU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MSKM`"]
pub type MSKM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSKM`"]
pub struct MSKM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKM_W<'a> {
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
#[doc = "Reader of field `MNT`"]
pub type MNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MNT`"]
pub struct MNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `MNU`"]
pub type MNU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MNU`"]
pub struct MNU_W<'a> {
    w: &'a mut W,
}
impl<'a> MNU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MSKS`"]
pub type MSKS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSKS`"]
pub struct MSKS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKS_W<'a> {
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
#[doc = "Reader of field `SCT`"]
pub type SCT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT`"]
pub struct SCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `SCU`"]
pub type SCU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCU`"]
pub struct SCU_W<'a> {
    w: &'a mut W,
}
impl<'a> SCU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Alarm date mask"]
    #[inline(always)]
    pub fn mskd(&self) -> MSKD_R {
        MSKD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn dows(&self) -> DOWS_R {
        DOWS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format."]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format."]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Alarm hours mask"]
    #[inline(always)]
    pub fn mskh(&self) -> MSKH_R {
        MSKH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    pub fn hrt(&self) -> HRT_R {
        HRT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    pub fn hru(&self) -> HRU_R {
        HRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes mask"]
    #[inline(always)]
    pub fn mskm(&self) -> MSKM_R {
        MSKM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Alarm seconds mask"]
    #[inline(always)]
    pub fn msks(&self) -> MSKS_R {
        MSKS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn scu(&self) -> SCU_R {
        SCU_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Alarm date mask"]
    #[inline(always)]
    pub fn mskd(&mut self) -> MSKD_W {
        MSKD_W { w: self }
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn dows(&mut self) -> DOWS_W {
        DOWS_W { w: self }
    }
    #[doc = "Bits 28:29 - Date tens in BCD format."]
    #[inline(always)]
    pub fn dayt(&mut self) -> DAYT_W {
        DAYT_W { w: self }
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format."]
    #[inline(always)]
    pub fn dayu(&mut self) -> DAYU_W {
        DAYU_W { w: self }
    }
    #[doc = "Bit 23 - Alarm hours mask"]
    #[inline(always)]
    pub fn mskh(&mut self) -> MSKH_W {
        MSKH_W { w: self }
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    pub fn hrt(&mut self) -> HRT_W {
        HRT_W { w: self }
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    pub fn hru(&mut self) -> HRU_W {
        HRU_W { w: self }
    }
    #[doc = "Bit 15 - Alarm minutes mask"]
    #[inline(always)]
    pub fn mskm(&mut self) -> MSKM_W {
        MSKM_W { w: self }
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W {
        MNT_W { w: self }
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W {
        MNU_W { w: self }
    }
    #[doc = "Bit 7 - Alarm seconds mask"]
    #[inline(always)]
    pub fn msks(&mut self) -> MSKS_W {
        MSKS_W { w: self }
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn sct(&mut self) -> SCT_W {
        SCT_W { w: self }
    }
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn scu(&mut self) -> SCU_W {
        SCU_W { w: self }
    }
}

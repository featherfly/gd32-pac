#[doc = "Reader of register TTS"]
pub type R = crate::R<u32, super::TTS>;
#[doc = "Reader of field `PM`"]
pub type PM_R = crate::R<bool, bool>;
#[doc = "Reader of field `HRT`"]
pub type HRT_R = crate::R<u8, u8>;
#[doc = "Reader of field `HRU`"]
pub type HRU_R = crate::R<u8, u8>;
#[doc = "Reader of field `MNT`"]
pub type MNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `MNU`"]
pub type MNU_R = crate::R<u8, u8>;
#[doc = "Reader of field `SCT`"]
pub type SCT_R = crate::R<u8, u8>;
#[doc = "Reader of field `SCU`"]
pub type SCU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 22 - AM/PM mark"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD code"]
    #[inline(always)]
    pub fn hrt(&self) -> HRT_R {
        HRT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD code"]
    #[inline(always)]
    pub fn hru(&self) -> HRU_R {
        HRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD code"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD code"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD code"]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Second units in BCD code"]
    #[inline(always)]
    pub fn scu(&self) -> SCU_R {
        SCU_R::new((self.bits & 0x0f) as u8)
    }
}

#[doc = "Reader of register DTS"]
pub type R = crate::R<u32, super::DTS>;
#[doc = "Reader of field `DOW`"]
pub type DOW_R = crate::R<u8, u8>;
#[doc = "Reader of field `MONT`"]
pub type MONT_R = crate::R<bool, bool>;
#[doc = "Reader of field `MONU`"]
pub type MONU_R = crate::R<u8, u8>;
#[doc = "Reader of field `DAYT`"]
pub type DAYT_R = crate::R<u8, u8>;
#[doc = "Reader of field `DAYU`"]
pub type DAYU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD code"]
    #[inline(always)]
    pub fn mont(&self) -> MONT_R {
        MONT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Month units in BCD code"]
    #[inline(always)]
    pub fn monu(&self) -> MONU_R {
        MONU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - Date tens in BCD code"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - Date units in BCD code"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new((self.bits & 0x1f) as u8)
    }
}

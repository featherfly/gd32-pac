# [ doc = "Reader of register CFG" ] pub type R = crate :: R < u32 , super :: CFG > ; # [ doc = "Writer for register CFG" ] pub type W = crate :: W < u32 , super :: CFG > ; # [ doc = "Register CFG `reset()`'s with value 0" ] impl crate :: ResetValue for super :: CFG { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `SFT`" ] pub type SFT_R = crate :: R < u8 , u8 > ; # [ doc = "Write proxy for field `SFT`" ] pub struct SFT_W < 'a > { w : & 'a mut W , } impl < 'a > SFT_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u8 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x07 ) | ( ( value as u32 ) & 0x07 ) ; self . w } } # [ doc = "Reader of field `RTOL`" ] pub type RTOL_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `RTOL`" ] pub struct RTOL_W < 'a > { w : & 'a mut W , } impl < 'a > RTOL_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 3 ) ) | ( ( ( value as u32 ) & 0x01 ) << 3 ) ; self . w } } # [ doc = "Reader of field `RBRESTP`" ] pub type RBRESTP_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `RBRESTP`" ] pub struct RBRESTP_W < 'a > { w : & 'a mut W , } impl < 'a > RBRESTP_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 4 ) ) | ( ( ( value as u32 ) & 0x01 ) << 4 ) ; self . w } } # [ doc = "Reader of field `RBREGEN`" ] pub type RBREGEN_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `RBREGEN`" ] pub struct RBREGEN_W < 'a > { w : & 'a mut W , } impl < 'a > RBREGEN_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 5 ) ) | ( ( ( value as u32 ) & 0x01 ) << 5 ) ; self . w } } # [ doc = "Reader of field `RLBPEGEN`" ] pub type RLBPEGEN_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `RLBPEGEN`" ] pub struct RLBPEGEN_W < 'a > { w : & 'a mut W , } impl < 'a > RLBPEGEN_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 6 ) ) | ( ( ( value as u32 ) & 0x01 ) << 6 ) ; self . w } } # [ doc = "Reader of field `BCNG`" ] pub type BCNG_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `BCNG`" ] pub struct BCNG_W < 'a > { w : & 'a mut W , } impl < 'a > BCNG_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 7 ) ) | ( ( ( value as u32 ) & 0x01 ) << 7 ) ; self . w } } # [ doc = "Reader of field `SFTOPT`" ] pub type SFTOPT_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SFTOPT`" ] pub struct SFTOPT_W < 'a > { w : & 'a mut W , } impl < 'a > SFTOPT_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 8 ) ) | ( ( ( value as u32 ) & 0x01 ) << 8 ) ; self . w } } # [ doc = "Reader of field `OADR`" ] pub type OADR_R = crate :: R < u16 , u16 > ; # [ doc = "Write proxy for field `OADR`" ] pub struct OADR_W < 'a > { w : & 'a mut W , } impl < 'a > OADR_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u16 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x7fff << 16 ) ) | ( ( ( value as u32 ) & 0x7fff ) << 16 ) ; self . w } } # [ doc = "Reader of field `LMEN`" ] pub type LMEN_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `LMEN`" ] pub struct LMEN_W < 'a > { w : & 'a mut W , } impl < 'a > LMEN_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 31 ) ) | ( ( ( value as u32 ) & 0x01 ) << 31 ) ; self . w } } impl R { # [ doc = "Bits 0:2 - Signal Free Time" ] # [ inline ( always ) ] pub fn sft ( & self ) -> SFT_R { SFT_R :: new ( ( self . bits & 0x07 ) as u8 ) } # [ doc = "Bit 3 - Reception bit timing tolerance" ] # [ inline ( always ) ] pub fn rtol ( & self ) -> RTOL_R { RTOL_R :: new ( ( ( self . bits >> 3 ) & 0x01 ) != 0 ) } # [ doc = "Bit 4 - Whether stop receive message when detected RBRE" ] # [ inline ( always ) ] pub fn rbrestp ( & self ) -> RBRESTP_R { RBRESTP_R :: new ( ( ( self . bits >> 4 ) & 0x01 ) != 0 ) } # [ doc = "Bit 5 - Generate Error-bit when detected RBRE in singlecast" ] # [ inline ( always ) ] pub fn rbregen ( & self ) -> RBREGEN_R { RBREGEN_R :: new ( ( ( self . bits >> 5 ) & 0x01 ) != 0 ) } # [ doc = "Bit 6 - Generate Error-bit when detected RLBPE in singlecast" ] # [ inline ( always ) ] pub fn rlbpegen ( & self ) -> RLBPEGEN_R { RLBPEGEN_R :: new ( ( ( self . bits >> 6 ) & 0x01 ) != 0 ) } # [ doc = "Bit 7 - Do not generate Error-bit in broadcast message" ] # [ inline ( always ) ] pub fn bcng ( & self ) -> BCNG_R { BCNG_R :: new ( ( ( self . bits >> 7 ) & 0x01 ) != 0 ) } # [ doc = "Bit 8 - The SFT start option" ] # [ inline ( always ) ] pub fn sftopt ( & self ) -> SFTOPT_R { SFTOPT_R :: new ( ( ( self . bits >> 8 ) & 0x01 ) != 0 ) } # [ doc = "Bits 16:30 - Own Address" ] # [ inline ( always ) ] pub fn oadr ( & self ) -> OADR_R { OADR_R :: new ( ( ( self . bits >> 16 ) & 0x7fff ) as u16 ) } # [ doc = "Bit 31 - Listen mode enable" ] # [ inline ( always ) ] pub fn lmen ( & self ) -> LMEN_R { LMEN_R :: new ( ( ( self . bits >> 31 ) & 0x01 ) != 0 ) } } impl W { # [ doc = "Bits 0:2 - Signal Free Time" ] # [ inline ( always ) ] pub fn sft ( & mut self ) -> SFT_W { SFT_W { w : self } } # [ doc = "Bit 3 - Reception bit timing tolerance" ] # [ inline ( always ) ] pub fn rtol ( & mut self ) -> RTOL_W { RTOL_W { w : self } } # [ doc = "Bit 4 - Whether stop receive message when detected RBRE" ] # [ inline ( always ) ] pub fn rbrestp ( & mut self ) -> RBRESTP_W { RBRESTP_W { w : self } } # [ doc = "Bit 5 - Generate Error-bit when detected RBRE in singlecast" ] # [ inline ( always ) ] pub fn rbregen ( & mut self ) -> RBREGEN_W { RBREGEN_W { w : self } } # [ doc = "Bit 6 - Generate Error-bit when detected RLBPE in singlecast" ] # [ inline ( always ) ] pub fn rlbpegen ( & mut self ) -> RLBPEGEN_W { RLBPEGEN_W { w : self } } # [ doc = "Bit 7 - Do not generate Error-bit in broadcast message" ] # [ inline ( always ) ] pub fn bcng ( & mut self ) -> BCNG_W { BCNG_W { w : self } } # [ doc = "Bit 8 - The SFT start option" ] # [ inline ( always ) ] pub fn sftopt ( & mut self ) -> SFTOPT_W { SFTOPT_W { w : self } } # [ doc = "Bits 16:30 - Own Address" ] # [ inline ( always ) ] pub fn oadr ( & mut self ) -> OADR_W { OADR_W { w : self } } # [ doc = "Bit 31 - Listen mode enable" ] # [ inline ( always ) ] pub fn lmen ( & mut self ) -> LMEN_W { LMEN_W { w : self } } }
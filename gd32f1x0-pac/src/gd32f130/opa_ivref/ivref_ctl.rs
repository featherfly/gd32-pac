# [ doc = "Reader of register IVREF_CTL" ] pub type R = crate :: R < u32 , super :: IVREF_CTL > ; # [ doc = "Writer for register IVREF_CTL" ] pub type W = crate :: W < u32 , super :: IVREF_CTL > ; # [ doc = "Register IVREF_CTL `reset()`'s with value 0x1000_0f00" ] impl crate :: ResetValue for super :: IVREF_CTL { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0x1000_0f00 } } # [ doc = "Reader of field `CSDT`" ] pub type CSDT_R = crate :: R < u8 , u8 > ; # [ doc = "Write proxy for field `CSDT`" ] pub struct CSDT_W < 'a > { w : & 'a mut W , } impl < 'a > CSDT_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u8 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x3f ) | ( ( value as u32 ) & 0x3f ) ; self . w } } # [ doc = "Reader of field `SCMOD`" ] pub type SCMOD_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SCMOD`" ] pub struct SCMOD_W < 'a > { w : & 'a mut W , } impl < 'a > SCMOD_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 7 ) ) | ( ( ( value as u32 ) & 0x01 ) << 7 ) ; self . w } } # [ doc = "Reader of field `CPT`" ] pub type CPT_R = crate :: R < u8 , u8 > ; # [ doc = "Write proxy for field `CPT`" ] pub struct CPT_W < 'a > { w : & 'a mut W , } impl < 'a > CPT_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u8 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x1f << 8 ) ) | ( ( ( value as u32 ) & 0x1f ) << 8 ) ; self . w } } # [ doc = "Reader of field `SSEL`" ] pub type SSEL_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SSEL`" ] pub struct SSEL_W < 'a > { w : & 'a mut W , } impl < 'a > SSEL_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 14 ) ) | ( ( ( value as u32 ) & 0x01 ) << 14 ) ; self . w } } # [ doc = "Reader of field `CREN`" ] pub type CREN_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `CREN`" ] pub struct CREN_W < 'a > { w : & 'a mut W , } impl < 'a > CREN_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 15 ) ) | ( ( ( value as u32 ) & 0x01 ) << 15 ) ; self . w } } # [ doc = "Reader of field `VPT`" ] pub type VPT_R = crate :: R < u8 , u8 > ; # [ doc = "Write proxy for field `VPT`" ] pub struct VPT_W < 'a > { w : & 'a mut W , } impl < 'a > VPT_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u8 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x1f << 24 ) ) | ( ( ( value as u32 ) & 0x1f ) << 24 ) ; self . w } } # [ doc = "Reader of field `DECAP`" ] pub type DECAP_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `DECAP`" ] pub struct DECAP_W < 'a > { w : & 'a mut W , } impl < 'a > DECAP_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 30 ) ) | ( ( ( value as u32 ) & 0x01 ) << 30 ) ; self . w } } # [ doc = "Reader of field `VREN`" ] pub type VREN_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `VREN`" ] pub struct VREN_W < 'a > { w : & 'a mut W , } impl < 'a > VREN_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 31 ) ) | ( ( ( value as u32 ) & 0x01 ) << 31 ) ; self . w } } impl R { # [ doc = "Bits 0:5 - Current step data" ] # [ inline ( always ) ] pub fn csdt ( & self ) -> CSDT_R { CSDT_R :: new ( ( self . bits & 0x3f ) as u8 ) } # [ doc = "Bit 7 - Sink current mode" ] # [ inline ( always ) ] pub fn scmod ( & self ) -> SCMOD_R { SCMOD_R :: new ( ( ( self . bits >> 7 ) & 0x01 ) != 0 ) } # [ doc = "Bits 8:12 - Current precision trim" ] # [ inline ( always ) ] pub fn cpt ( & self ) -> CPT_R { CPT_R :: new ( ( ( self . bits >> 8 ) & 0x1f ) as u8 ) } # [ doc = "Bit 14 - Step selection" ] # [ inline ( always ) ] pub fn ssel ( & self ) -> SSEL_R { SSEL_R :: new ( ( ( self . bits >> 14 ) & 0x01 ) != 0 ) } # [ doc = "Bit 15 - Current reference enable" ] # [ inline ( always ) ] pub fn cren ( & self ) -> CREN_R { CREN_R :: new ( ( ( self . bits >> 15 ) & 0x01 ) != 0 ) } # [ doc = "Bits 24:28 - Voltage precision tirm" ] # [ inline ( always ) ] pub fn vpt ( & self ) -> VPT_R { VPT_R :: new ( ( ( self . bits >> 24 ) & 0x1f ) as u8 ) } # [ doc = "Bit 30 - Disconnect external capacitor" ] # [ inline ( always ) ] pub fn decap ( & self ) -> DECAP_R { DECAP_R :: new ( ( ( self . bits >> 30 ) & 0x01 ) != 0 ) } # [ doc = "Bit 31 - Voltage reference enable" ] # [ inline ( always ) ] pub fn vren ( & self ) -> VREN_R { VREN_R :: new ( ( ( self . bits >> 31 ) & 0x01 ) != 0 ) } } impl W { # [ doc = "Bits 0:5 - Current step data" ] # [ inline ( always ) ] pub fn csdt ( & mut self ) -> CSDT_W { CSDT_W { w : self } } # [ doc = "Bit 7 - Sink current mode" ] # [ inline ( always ) ] pub fn scmod ( & mut self ) -> SCMOD_W { SCMOD_W { w : self } } # [ doc = "Bits 8:12 - Current precision trim" ] # [ inline ( always ) ] pub fn cpt ( & mut self ) -> CPT_W { CPT_W { w : self } } # [ doc = "Bit 14 - Step selection" ] # [ inline ( always ) ] pub fn ssel ( & mut self ) -> SSEL_W { SSEL_W { w : self } } # [ doc = "Bit 15 - Current reference enable" ] # [ inline ( always ) ] pub fn cren ( & mut self ) -> CREN_W { CREN_W { w : self } } # [ doc = "Bits 24:28 - Voltage precision tirm" ] # [ inline ( always ) ] pub fn vpt ( & mut self ) -> VPT_W { VPT_W { w : self } } # [ doc = "Bit 30 - Disconnect external capacitor" ] # [ inline ( always ) ] pub fn decap ( & mut self ) -> DECAP_W { DECAP_W { w : self } } # [ doc = "Bit 31 - Voltage reference enable" ] # [ inline ( always ) ] pub fn vren ( & mut self ) -> VREN_W { VREN_W { w : self } } }
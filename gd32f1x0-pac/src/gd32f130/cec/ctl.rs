# [ doc = "Reader of register CTL" ] pub type R = crate :: R < u32 , super :: CTL > ; # [ doc = "Writer for register CTL" ] pub type W = crate :: W < u32 , super :: CTL > ; # [ doc = "Register CTL `reset()`'s with value 0" ] impl crate :: ResetValue for super :: CTL { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `ENDOM`" ] pub type ENDOM_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `ENDOM`" ] pub struct ENDOM_W < 'a > { w : & 'a mut W , } impl < 'a > ENDOM_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 2 ) ) | ( ( ( value as u32 ) & 0x01 ) << 2 ) ; self . w } } # [ doc = "Reader of field `SOM`" ] pub type SOM_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SOM`" ] pub struct SOM_W < 'a > { w : & 'a mut W , } impl < 'a > SOM_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( ( value as u32 ) & 0x01 ) << 1 ) ; self . w } } # [ doc = "Reader of field `CECEN`" ] pub type CECEN_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `CECEN`" ] pub struct CECEN_W < 'a > { w : & 'a mut W , } impl < 'a > CECEN_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u32 ) & 0x01 ) ; self . w } } impl R { # [ doc = "Bit 2 - ENDOM bit value in the next frame in TX mode" ] # [ inline ( always ) ] pub fn endom ( & self ) -> ENDOM_R { ENDOM_R :: new ( ( ( self . bits >> 2 ) & 0x01 ) != 0 ) } # [ doc = "Bit 1 - Start of sending a message" ] # [ inline ( always ) ] pub fn som ( & self ) -> SOM_R { SOM_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bit 0 - Enable/disable HDMI-CEC controller" ] # [ inline ( always ) ] pub fn cecen ( & self ) -> CECEN_R { CECEN_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 2 - ENDOM bit value in the next frame in TX mode" ] # [ inline ( always ) ] pub fn endom ( & mut self ) -> ENDOM_W { ENDOM_W { w : self } } # [ doc = "Bit 1 - Start of sending a message" ] # [ inline ( always ) ] pub fn som ( & mut self ) -> SOM_W { SOM_W { w : self } } # [ doc = "Bit 0 - Enable/disable HDMI-CEC controller" ] # [ inline ( always ) ] pub fn cecen ( & mut self ) -> CECEN_W { CECEN_W { w : self } } }
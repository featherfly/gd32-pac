# [ doc = "Reader of register WSEN" ] pub type R = crate :: R < u32 , super :: WSEN > ; # [ doc = "Reader of field `WSEN`" ] pub type WSEN_R = crate :: R < bool , bool > ; # [ doc = "Reader of field `BPEN`" ] pub type BPEN_R = crate :: R < bool , bool > ; impl R { # [ doc = "Bit 0 - FMC wait state enable register" ] # [ inline ( always ) ] pub fn wsen ( & self ) -> WSEN_R { WSEN_R :: new ( ( self . bits & 0x01 ) != 0 ) } # [ doc = "Bit 1 - FMC bit program enable register" ] # [ inline ( always ) ] pub fn bpen ( & self ) -> BPEN_R { BPEN_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } }
# [ doc = r" Value read from the register" ] pub struct R { bits : u32 } # [ doc = r" Value to write to the register" ] pub struct W { bits : u32 } impl super :: USBPLLCON { # [ doc = r" Modifies the contents of the register" ] # [ inline ] pub fn modify < F > ( & self , f : F ) where for < 'w > F : FnOnce ( & R , & 'w mut W ) -> & 'w mut W { let bits = self . register . get ( ) ; let r = R { bits : bits } ; let mut w = W { bits : bits } ; f ( & r , & mut w ) ; self . register . set ( w . bits ) ; } # [ doc = r" Reads the contents of the register" ] # [ inline ] pub fn read ( & self ) -> R { R { bits : self . register . get ( ) } } # [ doc = r" Writes to the register" ] # [ inline ] pub fn write < F > ( & self , f : F ) where F : FnOnce ( & mut W ) -> & mut W { let mut w = W :: reset_value ( ) ; f ( & mut w ) ; self . register . set ( w . bits ) ; } # [ doc = r" Writes the reset value to the register" ] # [ inline ] pub fn reset ( & self ) { self . write ( | w | w ) } } # [ doc = "Possible values of the field `VCOBYP`" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] pub enum VCOBYPR { # [ doc = "Normal operation, VCO is not bypassed" ] VALUE1 , # [ doc = "Prescaler Mode, VCO is bypassed" ] VALUE2 , } impl VCOBYPR { # [ doc = r" Returns `true` if the bit is clear (0)" ] # [ inline ] pub fn bit_is_clear ( & self ) -> bool { ! self . bit ( ) } # [ doc = r" Returns `true` if the bit is set (1)" ] # [ inline ] pub fn bit_is_set ( & self ) -> bool { self . bit ( ) } # [ doc = r" Value of the field as raw bits" ] # [ inline ] pub fn bit ( & self ) -> bool { match * self { VCOBYPR :: VALUE1 => false , VCOBYPR :: VALUE2 => true , } } # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _from ( value : bool ) -> VCOBYPR { match value { false => VCOBYPR :: VALUE1 , true => VCOBYPR :: VALUE2 , } } # [ doc = "Checks if the value of the field is `VALUE1`" ] # [ inline ] pub fn is_value1 ( & self ) -> bool { * self == VCOBYPR :: VALUE1 } # [ doc = "Checks if the value of the field is `VALUE2`" ] # [ inline ] pub fn is_value2 ( & self ) -> bool { * self == VCOBYPR :: VALUE2 } } # [ doc = "Possible values of the field `VCOPWD`" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] pub enum VCOPWDR { # [ doc = "Normal behavior" ] VALUE1 , # [ doc = "The VCO is put into a Power Saving Mode" ] VALUE2 , } impl VCOPWDR { # [ doc = r" Returns `true` if the bit is clear (0)" ] # [ inline ] pub fn bit_is_clear ( & self ) -> bool { ! self . bit ( ) } # [ doc = r" Returns `true` if the bit is set (1)" ] # [ inline ] pub fn bit_is_set ( & self ) -> bool { self . bit ( ) } # [ doc = r" Value of the field as raw bits" ] # [ inline ] pub fn bit ( & self ) -> bool { match * self { VCOPWDR :: VALUE1 => false , VCOPWDR :: VALUE2 => true , } } # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _from ( value : bool ) -> VCOPWDR { match value { false => VCOPWDR :: VALUE1 , true => VCOPWDR :: VALUE2 , } } # [ doc = "Checks if the value of the field is `VALUE1`" ] # [ inline ] pub fn is_value1 ( & self ) -> bool { * self == VCOPWDR :: VALUE1 } # [ doc = "Checks if the value of the field is `VALUE2`" ] # [ inline ] pub fn is_value2 ( & self ) -> bool { * self == VCOPWDR :: VALUE2 } } # [ doc = "Possible values of the field `VCOTR`" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] pub enum VCOTRR { # [ doc = "VCO bandwidth is operating in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz." ] VALUE1 , # [ doc = "VCO bandwidth is operating in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz." ] VALUE2 , } impl VCOTRR { # [ doc = r" Returns `true` if the bit is clear (0)" ] # [ inline ] pub fn bit_is_clear ( & self ) -> bool { ! self . bit ( ) } # [ doc = r" Returns `true` if the bit is set (1)" ] # [ inline ] pub fn bit_is_set ( & self ) -> bool { self . bit ( ) } # [ doc = r" Value of the field as raw bits" ] # [ inline ] pub fn bit ( & self ) -> bool { match * self { VCOTRR :: VALUE1 => false , VCOTRR :: VALUE2 => true , } } # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _from ( value : bool ) -> VCOTRR { match value { false => VCOTRR :: VALUE1 , true => VCOTRR :: VALUE2 , } } # [ doc = "Checks if the value of the field is `VALUE1`" ] # [ inline ] pub fn is_value1 ( & self ) -> bool { * self == VCOTRR :: VALUE1 } # [ doc = "Checks if the value of the field is `VALUE2`" ] # [ inline ] pub fn is_value2 ( & self ) -> bool { * self == VCOTRR :: VALUE2 } } # [ doc = "Possible values of the field `FINDIS`" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] pub enum FINDISR { # [ doc = "Connect oscillator to the VCO part" ] VALUE1 , # [ doc = "Disconnect oscillator from the VCO part." ] VALUE2 , } impl FINDISR { # [ doc = r" Returns `true` if the bit is clear (0)" ] # [ inline ] pub fn bit_is_clear ( & self ) -> bool { ! self . bit ( ) } # [ doc = r" Returns `true` if the bit is set (1)" ] # [ inline ] pub fn bit_is_set ( & self ) -> bool { self . bit ( ) } # [ doc = r" Value of the field as raw bits" ] # [ inline ] pub fn bit ( & self ) -> bool { match * self { FINDISR :: VALUE1 => false , FINDISR :: VALUE2 => true , } } # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _from ( value : bool ) -> FINDISR { match value { false => FINDISR :: VALUE1 , true => FINDISR :: VALUE2 , } } # [ doc = "Checks if the value of the field is `VALUE1`" ] # [ inline ] pub fn is_value1 ( & self ) -> bool { * self == FINDISR :: VALUE1 } # [ doc = "Checks if the value of the field is `VALUE2`" ] # [ inline ] pub fn is_value2 ( & self ) -> bool { * self == FINDISR :: VALUE2 } } # [ doc = "Possible values of the field `OSCDISCDIS`" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] pub enum OSCDISCDISR { # [ doc = "In case of a PLL loss-of-lock bit FINDIS is set" ] VALUE1 , # [ doc = "In case of a PLL loss-of-lock bit FINDIS is cleared" ] VALUE2 , } impl OSCDISCDISR { # [ doc = r" Returns `true` if the bit is clear (0)" ] # [ inline ] pub fn bit_is_clear ( & self ) -> bool { ! self . bit ( ) } # [ doc = r" Returns `true` if the bit is set (1)" ] # [ inline ] pub fn bit_is_set ( & self ) -> bool { self . bit ( ) } # [ doc = r" Value of the field as raw bits" ] # [ inline ] pub fn bit ( & self ) -> bool { match * self { OSCDISCDISR :: VALUE1 => false , OSCDISCDISR :: VALUE2 => true , } } # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _from ( value : bool ) -> OSCDISCDISR { match value { false => OSCDISCDISR :: VALUE1 , true => OSCDISCDISR :: VALUE2 , } } # [ doc = "Checks if the value of the field is `VALUE1`" ] # [ inline ] pub fn is_value1 ( & self ) -> bool { * self == OSCDISCDISR :: VALUE1 } # [ doc = "Checks if the value of the field is `VALUE2`" ] # [ inline ] pub fn is_value2 ( & self ) -> bool { * self == OSCDISCDISR :: VALUE2 } } # [ doc = r" Value of the field" ] pub struct NDIVR { bits : u8 } impl NDIVR { # [ doc = r" Value of the field as raw bits" ] # [ inline ] pub fn bits ( & self ) -> u8 { self . bits } } # [ doc = "Possible values of the field `PLLPWD`" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] pub enum PLLPWDR { # [ doc = "Normal behavior" ] VALUE1 , # [ doc = "The complete PLL block is put into a Power Saving Mode. Only the Bypass Mode is active if previously selected." ] VALUE2 , } impl PLLPWDR { # [ doc = r" Returns `true` if the bit is clear (0)" ] # [ inline ] pub fn bit_is_clear ( & self ) -> bool { ! self . bit ( ) } # [ doc = r" Returns `true` if the bit is set (1)" ] # [ inline ] pub fn bit_is_set ( & self ) -> bool { self . bit ( ) } # [ doc = r" Value of the field as raw bits" ] # [ inline ] pub fn bit ( & self ) -> bool { match * self { PLLPWDR :: VALUE1 => false , PLLPWDR :: VALUE2 => true , } } # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _from ( value : bool ) -> PLLPWDR { match value { false => PLLPWDR :: VALUE1 , true => PLLPWDR :: VALUE2 , } } # [ doc = "Checks if the value of the field is `VALUE1`" ] # [ inline ] pub fn is_value1 ( & self ) -> bool { * self == PLLPWDR :: VALUE1 } # [ doc = "Checks if the value of the field is `VALUE2`" ] # [ inline ] pub fn is_value2 ( & self ) -> bool { * self == PLLPWDR :: VALUE2 } } # [ doc = r" Value of the field" ] pub struct PDIVR { bits : u8 } impl PDIVR { # [ doc = r" Value of the field as raw bits" ] # [ inline ] pub fn bits ( & self ) -> u8 { self . bits } } # [ doc = "Values that can be written to the field `VCOBYP`" ] pub enum VCOBYPW { # [ doc = "Normal operation, VCO is not bypassed" ] VALUE1 , # [ doc = "Prescaler Mode, VCO is bypassed" ] VALUE2 , } impl VCOBYPW { # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _bits ( & self ) -> bool { match * self { VCOBYPW :: VALUE1 => false , VCOBYPW :: VALUE2 => true , } } } # [ doc = r" Proxy" ] pub struct _VCOBYPW < 'a > { w : & 'a mut W } impl < 'a > _VCOBYPW < 'a > { # [ doc = r" Writes `variant` to the field" ] # [ inline ] pub fn variant ( self , variant : VCOBYPW ) -> & 'a mut W { { self . bit ( variant . _bits ( ) ) } } # [ doc = "Normal operation, VCO is not bypassed" ] # [ inline ] pub fn value1 ( self ) -> & 'a mut W { self . variant ( VCOBYPW :: VALUE1 ) } # [ doc = "Prescaler Mode, VCO is bypassed" ] # [ inline ] pub fn value2 ( self ) -> & 'a mut W { self . variant ( VCOBYPW :: VALUE2 ) } # [ doc = r" Sets the field bit" ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r" Clears the field bit" ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r" Writes raw bits to the field" ] # [ inline ] pub fn bit ( self , value : bool ) -> & 'a mut W { const MASK : bool = true ; const OFFSET : u8 = 0 ; self . w . bits &= ! ( ( MASK as u32 ) << OFFSET ) ; self . w . bits |= ( ( value & MASK ) as u32 ) << OFFSET ; self . w } } # [ doc = "Values that can be written to the field `VCOPWD`" ] pub enum VCOPWDW { # [ doc = "Normal behavior" ] VALUE1 , # [ doc = "The VCO is put into a Power Saving Mode" ] VALUE2 , } impl VCOPWDW { # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _bits ( & self ) -> bool { match * self { VCOPWDW :: VALUE1 => false , VCOPWDW :: VALUE2 => true , } } } # [ doc = r" Proxy" ] pub struct _VCOPWDW < 'a > { w : & 'a mut W } impl < 'a > _VCOPWDW < 'a > { # [ doc = r" Writes `variant` to the field" ] # [ inline ] pub fn variant ( self , variant : VCOPWDW ) -> & 'a mut W { { self . bit ( variant . _bits ( ) ) } } # [ doc = "Normal behavior" ] # [ inline ] pub fn value1 ( self ) -> & 'a mut W { self . variant ( VCOPWDW :: VALUE1 ) } # [ doc = "The VCO is put into a Power Saving Mode" ] # [ inline ] pub fn value2 ( self ) -> & 'a mut W { self . variant ( VCOPWDW :: VALUE2 ) } # [ doc = r" Sets the field bit" ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r" Clears the field bit" ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r" Writes raw bits to the field" ] # [ inline ] pub fn bit ( self , value : bool ) -> & 'a mut W { const MASK : bool = true ; const OFFSET : u8 = 1 ; self . w . bits &= ! ( ( MASK as u32 ) << OFFSET ) ; self . w . bits |= ( ( value & MASK ) as u32 ) << OFFSET ; self . w } } # [ doc = "Values that can be written to the field `VCOTR`" ] pub enum VCOTRW { # [ doc = "VCO bandwidth is operating in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz." ] VALUE1 , # [ doc = "VCO bandwidth is operating in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz." ] VALUE2 , } impl VCOTRW { # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _bits ( & self ) -> bool { match * self { VCOTRW :: VALUE1 => false , VCOTRW :: VALUE2 => true , } } } # [ doc = r" Proxy" ] pub struct _VCOTRW < 'a > { w : & 'a mut W } impl < 'a > _VCOTRW < 'a > { # [ doc = r" Writes `variant` to the field" ] # [ inline ] pub fn variant ( self , variant : VCOTRW ) -> & 'a mut W { { self . bit ( variant . _bits ( ) ) } } # [ doc = "VCO bandwidth is operating in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz." ] # [ inline ] pub fn value1 ( self ) -> & 'a mut W { self . variant ( VCOTRW :: VALUE1 ) } # [ doc = "VCO bandwidth is operating in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz." ] # [ inline ] pub fn value2 ( self ) -> & 'a mut W { self . variant ( VCOTRW :: VALUE2 ) } # [ doc = r" Sets the field bit" ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r" Clears the field bit" ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r" Writes raw bits to the field" ] # [ inline ] pub fn bit ( self , value : bool ) -> & 'a mut W { const MASK : bool = true ; const OFFSET : u8 = 2 ; self . w . bits &= ! ( ( MASK as u32 ) << OFFSET ) ; self . w . bits |= ( ( value & MASK ) as u32 ) << OFFSET ; self . w } } # [ doc = "Values that can be written to the field `FINDIS`" ] pub enum FINDISW { # [ doc = "Connect oscillator to the VCO part" ] VALUE1 , # [ doc = "Disconnect oscillator from the VCO part." ] VALUE2 , } impl FINDISW { # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _bits ( & self ) -> bool { match * self { FINDISW :: VALUE1 => false , FINDISW :: VALUE2 => true , } } } # [ doc = r" Proxy" ] pub struct _FINDISW < 'a > { w : & 'a mut W } impl < 'a > _FINDISW < 'a > { # [ doc = r" Writes `variant` to the field" ] # [ inline ] pub fn variant ( self , variant : FINDISW ) -> & 'a mut W { { self . bit ( variant . _bits ( ) ) } } # [ doc = "Connect oscillator to the VCO part" ] # [ inline ] pub fn value1 ( self ) -> & 'a mut W { self . variant ( FINDISW :: VALUE1 ) } # [ doc = "Disconnect oscillator from the VCO part." ] # [ inline ] pub fn value2 ( self ) -> & 'a mut W { self . variant ( FINDISW :: VALUE2 ) } # [ doc = r" Sets the field bit" ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r" Clears the field bit" ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r" Writes raw bits to the field" ] # [ inline ] pub fn bit ( self , value : bool ) -> & 'a mut W { const MASK : bool = true ; const OFFSET : u8 = 4 ; self . w . bits &= ! ( ( MASK as u32 ) << OFFSET ) ; self . w . bits |= ( ( value & MASK ) as u32 ) << OFFSET ; self . w } } # [ doc = "Values that can be written to the field `OSCDISCDIS`" ] pub enum OSCDISCDISW { # [ doc = "In case of a PLL loss-of-lock bit FINDIS is set" ] VALUE1 , # [ doc = "In case of a PLL loss-of-lock bit FINDIS is cleared" ] VALUE2 , } impl OSCDISCDISW { # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _bits ( & self ) -> bool { match * self { OSCDISCDISW :: VALUE1 => false , OSCDISCDISW :: VALUE2 => true , } } } # [ doc = r" Proxy" ] pub struct _OSCDISCDISW < 'a > { w : & 'a mut W } impl < 'a > _OSCDISCDISW < 'a > { # [ doc = r" Writes `variant` to the field" ] # [ inline ] pub fn variant ( self , variant : OSCDISCDISW ) -> & 'a mut W { { self . bit ( variant . _bits ( ) ) } } # [ doc = "In case of a PLL loss-of-lock bit FINDIS is set" ] # [ inline ] pub fn value1 ( self ) -> & 'a mut W { self . variant ( OSCDISCDISW :: VALUE1 ) } # [ doc = "In case of a PLL loss-of-lock bit FINDIS is cleared" ] # [ inline ] pub fn value2 ( self ) -> & 'a mut W { self . variant ( OSCDISCDISW :: VALUE2 ) } # [ doc = r" Sets the field bit" ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r" Clears the field bit" ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r" Writes raw bits to the field" ] # [ inline ] pub fn bit ( self , value : bool ) -> & 'a mut W { const MASK : bool = true ; const OFFSET : u8 = 6 ; self . w . bits &= ! ( ( MASK as u32 ) << OFFSET ) ; self . w . bits |= ( ( value & MASK ) as u32 ) << OFFSET ; self . w } } # [ doc = r" Proxy" ] pub struct _NDIVW < 'a > { w : & 'a mut W } impl < 'a > _NDIVW < 'a > { # [ doc = r" Writes raw bits to the field" ] # [ inline ] pub unsafe fn bits ( self , value : u8 ) -> & 'a mut W { const MASK : u8 = 127 ; const OFFSET : u8 = 8 ; self . w . bits &= ! ( ( MASK as u32 ) << OFFSET ) ; self . w . bits |= ( ( value & MASK ) as u32 ) << OFFSET ; self . w } } # [ doc = "Values that can be written to the field `PLLPWD`" ] pub enum PLLPWDW { # [ doc = "Normal behavior" ] VALUE1 , # [ doc = "The complete PLL block is put into a Power Saving Mode. Only the Bypass Mode is active if previously selected." ] VALUE2 , } impl PLLPWDW { # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] # [ inline ] pub fn _bits ( & self ) -> bool { match * self { PLLPWDW :: VALUE1 => false , PLLPWDW :: VALUE2 => true , } } } # [ doc = r" Proxy" ] pub struct _PLLPWDW < 'a > { w : & 'a mut W } impl < 'a > _PLLPWDW < 'a > { # [ doc = r" Writes `variant` to the field" ] # [ inline ] pub fn variant ( self , variant : PLLPWDW ) -> & 'a mut W { { self . bit ( variant . _bits ( ) ) } } # [ doc = "Normal behavior" ] # [ inline ] pub fn value1 ( self ) -> & 'a mut W { self . variant ( PLLPWDW :: VALUE1 ) } # [ doc = "The complete PLL block is put into a Power Saving Mode. Only the Bypass Mode is active if previously selected." ] # [ inline ] pub fn value2 ( self ) -> & 'a mut W { self . variant ( PLLPWDW :: VALUE2 ) } # [ doc = r" Sets the field bit" ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r" Clears the field bit" ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r" Writes raw bits to the field" ] # [ inline ] pub fn bit ( self , value : bool ) -> & 'a mut W { const MASK : bool = true ; const OFFSET : u8 = 16 ; self . w . bits &= ! ( ( MASK as u32 ) << OFFSET ) ; self . w . bits |= ( ( value & MASK ) as u32 ) << OFFSET ; self . w } } # [ doc = r" Proxy" ] pub struct _RESLDW < 'a > { w : & 'a mut W } impl < 'a > _RESLDW < 'a > { # [ doc = r" Sets the field bit" ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r" Clears the field bit" ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r" Writes raw bits to the field" ] # [ inline ] pub fn bit ( self , value : bool ) -> & 'a mut W { const MASK : bool = true ; const OFFSET : u8 = 18 ; self . w . bits &= ! ( ( MASK as u32 ) << OFFSET ) ; self . w . bits |= ( ( value & MASK ) as u32 ) << OFFSET ; self . w } } # [ doc = r" Proxy" ] pub struct _PDIVW < 'a > { w : & 'a mut W } impl < 'a > _PDIVW < 'a > { # [ doc = r" Writes raw bits to the field" ] # [ inline ] pub unsafe fn bits ( self , value : u8 ) -> & 'a mut W { const MASK : u8 = 15 ; const OFFSET : u8 = 24 ; self . w . bits &= ! ( ( MASK as u32 ) << OFFSET ) ; self . w . bits |= ( ( value & MASK ) as u32 ) << OFFSET ; self . w } } impl R { # [ doc = r" Value of the register as raw bits" ] # [ inline ] pub fn bits ( & self ) -> u32 { self . bits } # [ doc = "Bit 0 - VCO Bypass" ] # [ inline ] pub fn vcobyp ( & self ) -> VCOBYPR { VCOBYPR :: _from ( { const MASK : bool = true ; const OFFSET : u8 = 0 ; ( ( self . bits >> OFFSET ) & MASK as u32 ) != 0 } ) } # [ doc = "Bit 1 - VCO Power Saving Mode" ] # [ inline ] pub fn vcopwd ( & self ) -> VCOPWDR { VCOPWDR :: _from ( { const MASK : bool = true ; const OFFSET : u8 = 1 ; ( ( self . bits >> OFFSET ) & MASK as u32 ) != 0 } ) } # [ doc = "Bit 2 - VCO Trim Control" ] # [ inline ] pub fn vcotr ( & self ) -> VCOTRR { VCOTRR :: _from ( { const MASK : bool = true ; const OFFSET : u8 = 2 ; ( ( self . bits >> OFFSET ) & MASK as u32 ) != 0 } ) } # [ doc = "Bit 4 - Disconnect Oscillator from VCO" ] # [ inline ] pub fn findis ( & self ) -> FINDISR { FINDISR :: _from ( { const MASK : bool = true ; const OFFSET : u8 = 4 ; ( ( self . bits >> OFFSET ) & MASK as u32 ) != 0 } ) } # [ doc = "Bit 6 - Oscillator Disconnect Disable" ] # [ inline ] pub fn oscdiscdis ( & self ) -> OSCDISCDISR { OSCDISCDISR :: _from ( { const MASK : bool = true ; const OFFSET : u8 = 6 ; ( ( self . bits >> OFFSET ) & MASK as u32 ) != 0 } ) } # [ doc = "Bits 8:14 - N-Divider Value" ] # [ inline ] pub fn ndiv ( & self ) -> NDIVR { let bits = { const MASK : u8 = 127 ; const OFFSET : u8 = 8 ; ( ( self . bits >> OFFSET ) & MASK as u32 ) as u8 } ; NDIVR { bits } } # [ doc = "Bit 16 - PLL Power Saving Mode" ] # [ inline ] pub fn pllpwd ( & self ) -> PLLPWDR { PLLPWDR :: _from ( { const MASK : bool = true ; const OFFSET : u8 = 16 ; ( ( self . bits >> OFFSET ) & MASK as u32 ) != 0 } ) } # [ doc = "Bits 24:27 - P-Divider Value" ] # [ inline ] pub fn pdiv ( & self ) -> PDIVR { let bits = { const MASK : u8 = 15 ; const OFFSET : u8 = 24 ; ( ( self . bits >> OFFSET ) & MASK as u32 ) as u8 } ; PDIVR { bits } } } impl W { # [ doc = r" Reset value of the register" ] # [ inline ] pub fn reset_value ( ) -> W { W { bits : 65539 } } # [ doc = r" Writes raw bits to the register" ] # [ inline ] pub unsafe fn bits ( & mut self , bits : u32 ) -> & mut Self { self . bits = bits ; self } # [ doc = "Bit 0 - VCO Bypass" ] # [ inline ] pub fn vcobyp ( & mut self ) -> _VCOBYPW { _VCOBYPW { w : self } } # [ doc = "Bit 1 - VCO Power Saving Mode" ] # [ inline ] pub fn vcopwd ( & mut self ) -> _VCOPWDW { _VCOPWDW { w : self } } # [ doc = "Bit 2 - VCO Trim Control" ] # [ inline ] pub fn vcotr ( & mut self ) -> _VCOTRW { _VCOTRW { w : self } } # [ doc = "Bit 4 - Disconnect Oscillator from VCO" ] # [ inline ] pub fn findis ( & mut self ) -> _FINDISW { _FINDISW { w : self } } # [ doc = "Bit 6 - Oscillator Disconnect Disable" ] # [ inline ] pub fn oscdiscdis ( & mut self ) -> _OSCDISCDISW { _OSCDISCDISW { w : self } } # [ doc = "Bits 8:14 - N-Divider Value" ] # [ inline ] pub fn ndiv ( & mut self ) -> _NDIVW { _NDIVW { w : self } } # [ doc = "Bit 16 - PLL Power Saving Mode" ] # [ inline ] pub fn pllpwd ( & mut self ) -> _PLLPWDW { _PLLPWDW { w : self } } # [ doc = "Bit 18 - Restart VCO Lock Detection" ] # [ inline ] pub fn resld ( & mut self ) -> _RESLDW { _RESLDW { w : self } } # [ doc = "Bits 24:27 - P-Divider Value" ] # [ inline ] pub fn pdiv ( & mut self ) -> _PDIVW { _PDIVW { w : self } } }
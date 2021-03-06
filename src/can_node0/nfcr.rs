#[doc = "Reader of register NFCR"]
pub type R = crate::R<u32, super::NFCR>;
#[doc = "Writer for register NFCR"]
pub type W = crate::W<u32, super::NFCR>;
#[doc = "Register NFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::NFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFC`"]
pub type CFC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CFC`"]
pub struct CFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "CAN Frame Count Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFSEL_A {
    #[doc = "0: The frame counter is incremented (internally) at the beginning of a new bit time. The value is sampled during the SOF bit of a new frame. The sampled value is visible in the CFC field."]
    VALUE1 = 0,
}
impl From<CFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFSEL`"]
pub type CFSEL_R = crate::R<u8, CFSEL_A>;
impl CFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CFSEL_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFSEL_A::VALUE1
    }
}
#[doc = "Write proxy for field `CFSEL`"]
pub struct CFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The frame counter is incremented (internally) at the beginning of a new bit time. The value is sampled during the SOF bit of a new frame. The sampled value is visible in the CFC field."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFSEL_A::VALUE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "CAN Frame Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFMOD_A {
    #[doc = "0: Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    VALUE1 = 0,
    #[doc = "1: Time Stamp Mode: The frame counter is used to count bit times."]
    VALUE2 = 1,
    #[doc = "2: Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    VALUE3 = 2,
}
impl From<CFMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CFMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFMOD`"]
pub type CFMOD_R = crate::R<u8, CFMOD_A>;
impl CFMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CFMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CFMOD_A::VALUE1),
            1 => Val(CFMOD_A::VALUE2),
            2 => Val(CFMOD_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFMOD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFMOD_A::VALUE3
    }
}
#[doc = "Write proxy for field `CFMOD`"]
pub struct CFMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CFMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE1)
    }
    #[doc = "Time Stamp Mode: The frame counter is used to count bit times."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE2)
    }
    #[doc = "Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "CAN Frame Count Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFCIE_A {
    #[doc = "0: CAN frame counter overflow interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: CAN frame counter overflow interrupt is enabled."]
    VALUE2 = 1,
}
impl From<CFCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CFCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFCIE`"]
pub type CFCIE_R = crate::R<bool, CFCIE_A>;
impl CFCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFCIE_A {
        match self.bits {
            false => CFCIE_A::VALUE1,
            true => CFCIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFCIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFCIE_A::VALUE2
    }
}
#[doc = "Write proxy for field `CFCIE`"]
pub struct CFCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAN frame counter overflow interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCIE_A::VALUE1)
    }
    #[doc = "CAN frame counter overflow interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCIE_A::VALUE2)
    }
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
#[doc = "CAN Frame Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFCOV_A {
    #[doc = "0: No overflow has occurred since last flag reset."]
    VALUE1 = 0,
    #[doc = "1: An overflow has occurred since last flag reset."]
    VALUE2 = 1,
}
impl From<CFCOV_A> for bool {
    #[inline(always)]
    fn from(variant: CFCOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFCOV`"]
pub type CFCOV_R = crate::R<bool, CFCOV_A>;
impl CFCOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFCOV_A {
        match self.bits {
            false => CFCOV_A::VALUE1,
            true => CFCOV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFCOV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFCOV_A::VALUE2
    }
}
#[doc = "Write proxy for field `CFCOV`"]
pub struct CFCOV_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFCOV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCOV_A::VALUE1)
    }
    #[doc = "An overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCOV_A::VALUE2)
    }
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
impl R {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    pub fn cfc(&self) -> CFC_R {
        CFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    pub fn cfsel(&self) -> CFSEL_R {
        CFSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    pub fn cfmod(&self) -> CFMOD_R {
        CFMOD_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline(always)]
    pub fn cfcie(&self) -> CFCIE_R {
        CFCIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn cfcov(&self) -> CFCOV_R {
        CFCOV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    pub fn cfc(&mut self) -> CFC_W {
        CFC_W { w: self }
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    pub fn cfsel(&mut self) -> CFSEL_W {
        CFSEL_W { w: self }
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    pub fn cfmod(&mut self) -> CFMOD_W {
        CFMOD_W { w: self }
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline(always)]
    pub fn cfcie(&mut self) -> CFCIE_W {
        CFCIE_W { w: self }
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn cfcov(&mut self) -> CFCOV_W {
        CFCOV_W { w: self }
    }
}

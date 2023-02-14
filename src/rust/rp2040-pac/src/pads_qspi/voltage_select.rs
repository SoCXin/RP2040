#[doc = "Reader of register VOLTAGE_SELECT"]
pub type R = crate::R<u32, super::VOLTAGE_SELECT>;
#[doc = "Writer for register VOLTAGE_SELECT"]
pub type W = crate::W<u32, super::VOLTAGE_SELECT>;
#[doc = "Register VOLTAGE_SELECT `reset()`'s with value 0"]
impl crate::ResetValue for super::VOLTAGE_SELECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOLTAGE_SELECT_A {
    #[doc = "0: Set voltage to 3.3V (DVDD >= 2V5)"]
    _3V3 = 0,
    #[doc = "1: Set voltage to 1.8V (DVDD <= 1V8)"]
    _1V8 = 1,
}
impl From<VOLTAGE_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: VOLTAGE_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VOLTAGE_SELECT`"]
pub type VOLTAGE_SELECT_R = crate::R<bool, VOLTAGE_SELECT_A>;
impl VOLTAGE_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOLTAGE_SELECT_A {
        match self.bits {
            false => VOLTAGE_SELECT_A::_3V3,
            true => VOLTAGE_SELECT_A::_1V8,
        }
    }
    #[doc = "Checks if the value of the field is `_3V3`"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        *self == VOLTAGE_SELECT_A::_3V3
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == VOLTAGE_SELECT_A::_1V8
    }
}
#[doc = "Write proxy for field `VOLTAGE_SELECT`"]
pub struct VOLTAGE_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> VOLTAGE_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VOLTAGE_SELECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set voltage to 3.3V (DVDD >= 2V5)"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut W {
        self.variant(VOLTAGE_SELECT_A::_3V3)
    }
    #[doc = "Set voltage to 1.8V (DVDD <= 1V8)"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut W {
        self.variant(VOLTAGE_SELECT_A::_1V8)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn voltage_select(&self) -> VOLTAGE_SELECT_R {
        VOLTAGE_SELECT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn voltage_select(&mut self) -> VOLTAGE_SELECT_W {
        VOLTAGE_SELECT_W { w: self }
    }
}

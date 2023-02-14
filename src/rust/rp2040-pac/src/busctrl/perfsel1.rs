#[doc = "Reader of register PERFSEL1"]
pub type R = crate::R<u32, super::PERFSEL1>;
#[doc = "Writer for register PERFSEL1"]
pub type W = crate::W<u32, super::PERFSEL1>;
#[doc = "Register PERFSEL1 `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::PERFSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Select an event for PERFCTR1. Count either contested accesses, or all accesses, on a downstream port of the main crossbar.\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERFSEL1_A {
    #[doc = "0: `0`"]
    APB_CONTESTED = 0,
    #[doc = "1: `1`"]
    APB = 1,
    #[doc = "2: `10`"]
    FASTPERI_CONTESTED = 2,
    #[doc = "3: `11`"]
    FASTPERI = 3,
    #[doc = "4: `100`"]
    SRAM5_CONTESTED = 4,
    #[doc = "5: `101`"]
    SRAM5 = 5,
    #[doc = "6: `110`"]
    SRAM4_CONTESTED = 6,
    #[doc = "7: `111`"]
    SRAM4 = 7,
    #[doc = "8: `1000`"]
    SRAM3_CONTESTED = 8,
    #[doc = "9: `1001`"]
    SRAM3 = 9,
    #[doc = "10: `1010`"]
    SRAM2_CONTESTED = 10,
    #[doc = "11: `1011`"]
    SRAM2 = 11,
    #[doc = "12: `1100`"]
    SRAM1_CONTESTED = 12,
    #[doc = "13: `1101`"]
    SRAM1 = 13,
    #[doc = "14: `1110`"]
    SRAM0_CONTESTED = 14,
    #[doc = "15: `1111`"]
    SRAM0 = 15,
    #[doc = "16: `10000`"]
    XIP_MAIN_CONTESTED = 16,
    #[doc = "17: `10001`"]
    XIP_MAIN = 17,
    #[doc = "18: `10010`"]
    ROM_CONTESTED = 18,
    #[doc = "19: `10011`"]
    ROM = 19,
}
impl From<PERFSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: PERFSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PERFSEL1`"]
pub type PERFSEL1_R = crate::R<u8, PERFSEL1_A>;
impl PERFSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PERFSEL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PERFSEL1_A::APB_CONTESTED),
            1 => Val(PERFSEL1_A::APB),
            2 => Val(PERFSEL1_A::FASTPERI_CONTESTED),
            3 => Val(PERFSEL1_A::FASTPERI),
            4 => Val(PERFSEL1_A::SRAM5_CONTESTED),
            5 => Val(PERFSEL1_A::SRAM5),
            6 => Val(PERFSEL1_A::SRAM4_CONTESTED),
            7 => Val(PERFSEL1_A::SRAM4),
            8 => Val(PERFSEL1_A::SRAM3_CONTESTED),
            9 => Val(PERFSEL1_A::SRAM3),
            10 => Val(PERFSEL1_A::SRAM2_CONTESTED),
            11 => Val(PERFSEL1_A::SRAM2),
            12 => Val(PERFSEL1_A::SRAM1_CONTESTED),
            13 => Val(PERFSEL1_A::SRAM1),
            14 => Val(PERFSEL1_A::SRAM0_CONTESTED),
            15 => Val(PERFSEL1_A::SRAM0),
            16 => Val(PERFSEL1_A::XIP_MAIN_CONTESTED),
            17 => Val(PERFSEL1_A::XIP_MAIN),
            18 => Val(PERFSEL1_A::ROM_CONTESTED),
            19 => Val(PERFSEL1_A::ROM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APB_CONTESTED`"]
    #[inline(always)]
    pub fn is_apb_contested(&self) -> bool {
        *self == PERFSEL1_A::APB_CONTESTED
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == PERFSEL1_A::APB
    }
    #[doc = "Checks if the value of the field is `FASTPERI_CONTESTED`"]
    #[inline(always)]
    pub fn is_fastperi_contested(&self) -> bool {
        *self == PERFSEL1_A::FASTPERI_CONTESTED
    }
    #[doc = "Checks if the value of the field is `FASTPERI`"]
    #[inline(always)]
    pub fn is_fastperi(&self) -> bool {
        *self == PERFSEL1_A::FASTPERI
    }
    #[doc = "Checks if the value of the field is `SRAM5_CONTESTED`"]
    #[inline(always)]
    pub fn is_sram5_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM5_CONTESTED
    }
    #[doc = "Checks if the value of the field is `SRAM5`"]
    #[inline(always)]
    pub fn is_sram5(&self) -> bool {
        *self == PERFSEL1_A::SRAM5
    }
    #[doc = "Checks if the value of the field is `SRAM4_CONTESTED`"]
    #[inline(always)]
    pub fn is_sram4_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM4_CONTESTED
    }
    #[doc = "Checks if the value of the field is `SRAM4`"]
    #[inline(always)]
    pub fn is_sram4(&self) -> bool {
        *self == PERFSEL1_A::SRAM4
    }
    #[doc = "Checks if the value of the field is `SRAM3_CONTESTED`"]
    #[inline(always)]
    pub fn is_sram3_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM3_CONTESTED
    }
    #[doc = "Checks if the value of the field is `SRAM3`"]
    #[inline(always)]
    pub fn is_sram3(&self) -> bool {
        *self == PERFSEL1_A::SRAM3
    }
    #[doc = "Checks if the value of the field is `SRAM2_CONTESTED`"]
    #[inline(always)]
    pub fn is_sram2_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM2_CONTESTED
    }
    #[doc = "Checks if the value of the field is `SRAM2`"]
    #[inline(always)]
    pub fn is_sram2(&self) -> bool {
        *self == PERFSEL1_A::SRAM2
    }
    #[doc = "Checks if the value of the field is `SRAM1_CONTESTED`"]
    #[inline(always)]
    pub fn is_sram1_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM1_CONTESTED
    }
    #[doc = "Checks if the value of the field is `SRAM1`"]
    #[inline(always)]
    pub fn is_sram1(&self) -> bool {
        *self == PERFSEL1_A::SRAM1
    }
    #[doc = "Checks if the value of the field is `SRAM0_CONTESTED`"]
    #[inline(always)]
    pub fn is_sram0_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM0_CONTESTED
    }
    #[doc = "Checks if the value of the field is `SRAM0`"]
    #[inline(always)]
    pub fn is_sram0(&self) -> bool {
        *self == PERFSEL1_A::SRAM0
    }
    #[doc = "Checks if the value of the field is `XIP_MAIN_CONTESTED`"]
    #[inline(always)]
    pub fn is_xip_main_contested(&self) -> bool {
        *self == PERFSEL1_A::XIP_MAIN_CONTESTED
    }
    #[doc = "Checks if the value of the field is `XIP_MAIN`"]
    #[inline(always)]
    pub fn is_xip_main(&self) -> bool {
        *self == PERFSEL1_A::XIP_MAIN
    }
    #[doc = "Checks if the value of the field is `ROM_CONTESTED`"]
    #[inline(always)]
    pub fn is_rom_contested(&self) -> bool {
        *self == PERFSEL1_A::ROM_CONTESTED
    }
    #[doc = "Checks if the value of the field is `ROM`"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        *self == PERFSEL1_A::ROM
    }
}
#[doc = "Write proxy for field `PERFSEL1`"]
pub struct PERFSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERFSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn apb_contested(self) -> &'a mut W {
        self.variant(PERFSEL1_A::APB_CONTESTED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(PERFSEL1_A::APB)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fastperi_contested(self) -> &'a mut W {
        self.variant(PERFSEL1_A::FASTPERI_CONTESTED)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn fastperi(self) -> &'a mut W {
        self.variant(PERFSEL1_A::FASTPERI)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sram5_contested(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM5_CONTESTED)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sram5(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sram4_contested(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM4_CONTESTED)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sram4(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM4)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn sram3_contested(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM3_CONTESTED)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn sram3(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM3)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn sram2_contested(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM2_CONTESTED)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn sram2(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM2)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn sram1_contested(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM1_CONTESTED)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn sram1(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn sram0_contested(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM0_CONTESTED)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn sram0(self) -> &'a mut W {
        self.variant(PERFSEL1_A::SRAM0)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn xip_main_contested(self) -> &'a mut W {
        self.variant(PERFSEL1_A::XIP_MAIN_CONTESTED)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn xip_main(self) -> &'a mut W {
        self.variant(PERFSEL1_A::XIP_MAIN)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn rom_contested(self) -> &'a mut W {
        self.variant(PERFSEL1_A::ROM_CONTESTED)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn rom(self) -> &'a mut W {
        self.variant(PERFSEL1_A::ROM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select an event for PERFCTR1. Count either contested accesses, or all accesses, on a downstream port of the main crossbar."]
    #[inline(always)]
    pub fn perfsel1(&self) -> PERFSEL1_R {
        PERFSEL1_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select an event for PERFCTR1. Count either contested accesses, or all accesses, on a downstream port of the main crossbar."]
    #[inline(always)]
    pub fn perfsel1(&mut self) -> PERFSEL1_W {
        PERFSEL1_W { w: self }
    }
}

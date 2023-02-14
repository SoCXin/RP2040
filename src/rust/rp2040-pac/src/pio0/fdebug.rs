#[doc = "Reader of register FDEBUG"]
pub type R = crate::R<u32, super::FDEBUG>;
#[doc = "Writer for register FDEBUG"]
pub type W = crate::W<u32, super::FDEBUG>;
#[doc = "Register FDEBUG `reset()`'s with value 0"]
impl crate::ResetValue for super::FDEBUG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXSTALL`"]
pub type TXSTALL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXSTALL`"]
pub struct TXSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `TXOVER`"]
pub type TXOVER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXOVER`"]
pub struct TXOVER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOVER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RXUNDER`"]
pub type RXUNDER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXUNDER`"]
pub struct RXUNDER_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUNDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RXSTALL`"]
pub type RXSTALL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXSTALL`"]
pub struct RXSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
    #[inline(always)]
    pub fn txstall(&self) -> TXSTALL_R {
        TXSTALL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
    #[inline(always)]
    pub fn txover(&self) -> TXOVER_R {
        TXOVER_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
    #[inline(always)]
    pub fn rxunder(&self) -> RXUNDER_R {
        RXUNDER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear."]
    #[inline(always)]
    pub fn rxstall(&self) -> RXSTALL_R {
        RXSTALL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
    #[inline(always)]
    pub fn txstall(&mut self) -> TXSTALL_W {
        TXSTALL_W { w: self }
    }
    #[doc = "Bits 16:19 - TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
    #[inline(always)]
    pub fn txover(&mut self) -> TXOVER_W {
        TXOVER_W { w: self }
    }
    #[doc = "Bits 8:11 - RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
    #[inline(always)]
    pub fn rxunder(&mut self) -> RXUNDER_W {
        RXUNDER_W { w: self }
    }
    #[doc = "Bits 0:3 - State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear."]
    #[inline(always)]
    pub fn rxstall(&mut self) -> RXSTALL_W {
        RXSTALL_W { w: self }
    }
}

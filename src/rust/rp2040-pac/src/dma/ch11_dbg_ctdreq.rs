#[doc = "Reader of register CH11_DBG_CTDREQ"]
pub type R = crate::R<u32, super::CH11_DBG_CTDREQ>;
#[doc = "Reader of field `CH11_DBG_CTDREQ`"]
pub type CH11_DBG_CTDREQ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch11_dbg_ctdreq(&self) -> CH11_DBG_CTDREQ_R {
        CH11_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}

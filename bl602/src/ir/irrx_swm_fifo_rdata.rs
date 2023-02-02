#[doc = "Register `irrx_swm_fifo_rdata` reader"]
pub struct R(crate::R<IRRX_SWM_FIFO_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_SWM_FIFO_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRRX_SWM_FIFO_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRRX_SWM_FIFO_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rx_fifo_rdata` reader - "]
pub type RX_FIFO_RDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_fifo_rdata(&self) -> RX_FIFO_RDATA_R {
        RX_FIFO_RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "irrx_swm_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_swm_fifo_rdata](index.html) module"]
pub struct IRRX_SWM_FIFO_RDATA_SPEC;
impl crate::RegisterSpec for IRRX_SWM_FIFO_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_swm_fifo_rdata::R](R) reader structure"]
impl crate::Readable for IRRX_SWM_FIFO_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets irrx_swm_fifo_rdata to value 0"]
impl crate::Resettable for IRRX_SWM_FIFO_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

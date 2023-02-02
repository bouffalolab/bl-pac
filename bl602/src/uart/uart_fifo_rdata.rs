#[doc = "Register `uart_fifo_rdata` reader"]
pub struct R(crate::R<UART_FIFO_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_FIFO_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_FIFO_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_FIFO_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `uart_fifo_rdata` reader - "]
pub type UART_FIFO_RDATA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_fifo_rdata(&self) -> UART_FIFO_RDATA_R {
        UART_FIFO_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "uart_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo_rdata](index.html) module"]
pub struct UART_FIFO_RDATA_SPEC;
impl crate::RegisterSpec for UART_FIFO_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_fifo_rdata::R](R) reader structure"]
impl crate::Readable for UART_FIFO_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets uart_fifo_rdata to value 0"]
impl crate::Resettable for UART_FIFO_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

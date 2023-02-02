#[doc = "Register `spi_fifo_rdata` reader"]
pub struct R(crate::R<SPI_FIFO_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FIFO_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FIFO_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FIFO_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `spi_fifo_rdata` reader - "]
pub type SPI_FIFO_RDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_fifo_rdata(&self) -> SPI_FIFO_RDATA_R {
        SPI_FIFO_RDATA_R::new(self.bits)
    }
}
#[doc = "spi_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_rdata](index.html) module"]
pub struct SPI_FIFO_RDATA_SPEC;
impl crate::RegisterSpec for SPI_FIFO_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_fifo_rdata::R](R) reader structure"]
impl crate::Readable for SPI_FIFO_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets spi_fifo_rdata to value 0"]
impl crate::Resettable for SPI_FIFO_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

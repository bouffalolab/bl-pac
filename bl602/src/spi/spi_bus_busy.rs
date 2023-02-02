#[doc = "Register `spi_bus_busy` reader"]
pub struct R(crate::R<SPI_BUS_BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_BUS_BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_BUS_BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_BUS_BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sts_spi_bus_busy` reader - "]
pub type STS_SPI_BUS_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_spi_bus_busy(&self) -> STS_SPI_BUS_BUSY_R {
        STS_SPI_BUS_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "spi_bus_busy.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_bus_busy](index.html) module"]
pub struct SPI_BUS_BUSY_SPEC;
impl crate::RegisterSpec for SPI_BUS_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_bus_busy::R](R) reader structure"]
impl crate::Readable for SPI_BUS_BUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets spi_bus_busy to value 0"]
impl crate::Resettable for SPI_BUS_BUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

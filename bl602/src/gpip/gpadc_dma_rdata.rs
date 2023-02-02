#[doc = "Register `gpadc_dma_rdata` reader"]
pub struct R(crate::R<GPADC_DMA_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_DMA_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_DMA_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_DMA_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `gpadc_dma_rdata` reader - "]
pub type GPADC_DMA_RDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `rsvd_31_26` reader - "]
pub type RSVD_31_26_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&self) -> GPADC_DMA_RDATA_R {
        GPADC_DMA_RDATA_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rsvd_31_26(&self) -> RSVD_31_26_R {
        RSVD_31_26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[doc = "gpadc_dma_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_dma_rdata](index.html) module"]
pub struct GPADC_DMA_RDATA_SPEC;
impl crate::RegisterSpec for GPADC_DMA_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_dma_rdata::R](R) reader structure"]
impl crate::Readable for GPADC_DMA_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets gpadc_dma_rdata to value 0"]
impl crate::Resettable for GPADC_DMA_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

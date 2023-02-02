#[doc = "Register `DMA_IntErrClr` reader"]
pub struct R(crate::R<DMA_INT_ERR_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_ERR_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_ERR_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_ERR_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_IntErrClr` writer"]
pub struct W(crate::W<DMA_INT_ERR_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_ERR_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_INT_ERR_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_ERR_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IntErrClr` writer - "]
pub type INT_ERR_CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_INT_ERR_CLR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn int_err_clr(&mut self) -> INT_ERR_CLR_W<0> {
        INT_ERR_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IntErrClr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_err_clr](index.html) module"]
pub struct DMA_INT_ERR_CLR_SPEC;
impl crate::RegisterSpec for DMA_INT_ERR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_err_clr::R](R) reader structure"]
impl crate::Readable for DMA_INT_ERR_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int_err_clr::W](W) writer structure"]
impl crate::Writable for DMA_INT_ERR_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_IntErrClr to value 0"]
impl crate::Resettable for DMA_INT_ERR_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

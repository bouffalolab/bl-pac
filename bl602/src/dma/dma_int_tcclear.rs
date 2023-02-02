#[doc = "Register `DMA_IntTCClear` reader"]
pub struct R(crate::R<DMA_INT_TCCLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_TCCLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_TCCLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_TCCLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_IntTCClear` writer"]
pub struct W(crate::W<DMA_INT_TCCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_TCCLEAR_SPEC>;
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
impl From<crate::W<DMA_INT_TCCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_TCCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IntTCClear` writer - "]
pub type INT_TCCLEAR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_INT_TCCLEAR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn int_tcclear(&mut self) -> INT_TCCLEAR_W<0> {
        INT_TCCLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IntTCClear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_tcclear](index.html) module"]
pub struct DMA_INT_TCCLEAR_SPEC;
impl crate::RegisterSpec for DMA_INT_TCCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_tcclear::R](R) reader structure"]
impl crate::Readable for DMA_INT_TCCLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int_tcclear::W](W) writer structure"]
impl crate::Writable for DMA_INT_TCCLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_IntTCClear to value 0"]
impl crate::Resettable for DMA_INT_TCCLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

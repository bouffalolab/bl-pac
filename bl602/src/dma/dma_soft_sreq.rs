#[doc = "Register `DMA_SoftSReq` reader"]
pub struct R(crate::R<DMA_SOFT_SREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SOFT_SREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SOFT_SREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SOFT_SREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_SoftSReq` writer"]
pub struct W(crate::W<DMA_SOFT_SREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SOFT_SREQ_SPEC>;
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
impl From<crate::W<DMA_SOFT_SREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SOFT_SREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SoftSReq` reader - "]
pub type SOFT_SREQ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SoftSReq` writer - "]
pub type SOFT_SREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_SOFT_SREQ_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_sreq(&self) -> SOFT_SREQ_R {
        SOFT_SREQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn soft_sreq(&mut self) -> SOFT_SREQ_W<0> {
        SOFT_SREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_SoftSReq.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_soft_sreq](index.html) module"]
pub struct DMA_SOFT_SREQ_SPEC;
impl crate::RegisterSpec for DMA_SOFT_SREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_soft_sreq::R](R) reader structure"]
impl crate::Readable for DMA_SOFT_SREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_soft_sreq::W](W) writer structure"]
impl crate::Writable for DMA_SOFT_SREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_SoftSReq to value 0"]
impl crate::Resettable for DMA_SOFT_SREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

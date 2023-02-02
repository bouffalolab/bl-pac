#[doc = "Register `DMA_C3LLI` reader"]
pub struct R(crate::R<DMA_C3LLI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_C3LLI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_C3LLI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_C3LLI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_C3LLI` writer"]
pub struct W(crate::W<DMA_C3LLI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_C3LLI_SPEC>;
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
impl From<crate::W<DMA_C3LLI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_C3LLI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLI` reader - "]
pub type LLI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LLI` writer - "]
pub type LLI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_C3LLI_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn lli(&self) -> LLI_R {
        LLI_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31"]
    #[inline(always)]
    #[must_use]
    pub fn lli(&mut self) -> LLI_W<2> {
        LLI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_C3LLI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c3lli](index.html) module"]
pub struct DMA_C3LLI_SPEC;
impl crate::RegisterSpec for DMA_C3LLI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_c3lli::R](R) reader structure"]
impl crate::Readable for DMA_C3LLI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_c3lli::W](W) writer structure"]
impl crate::Writable for DMA_C3LLI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_C3LLI to value 0"]
impl crate::Resettable for DMA_C3LLI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

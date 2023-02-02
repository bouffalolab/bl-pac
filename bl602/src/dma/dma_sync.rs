#[doc = "Register `DMA_Sync` reader"]
pub struct R(crate::R<DMA_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_Sync` writer"]
pub struct W(crate::W<DMA_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SYNC_SPEC>;
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
impl From<crate::W<DMA_SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_Sync` reader - "]
pub type DMA_SYNC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMA_Sync` writer - "]
pub type DMA_SYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_SYNC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_sync(&self) -> DMA_SYNC_R {
        DMA_SYNC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sync(&mut self) -> DMA_SYNC_W<0> {
        DMA_SYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_Sync.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_sync](index.html) module"]
pub struct DMA_SYNC_SPEC;
impl crate::RegisterSpec for DMA_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_sync::R](R) reader structure"]
impl crate::Readable for DMA_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_sync::W](W) writer structure"]
impl crate::Writable for DMA_SYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_Sync to value 0"]
impl crate::Resettable for DMA_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `DMA_Top_Config` reader"]
pub struct R(crate::R<DMA_TOP_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TOP_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TOP_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TOP_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_Top_Config` writer"]
pub struct W(crate::W<DMA_TOP_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_TOP_CONFIG_SPEC>;
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
impl From<crate::W<DMA_TOP_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_TOP_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `E` reader - "]
pub type E_R = crate::BitReader<bool>;
#[doc = "Field `E` writer - "]
pub type E_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_TOP_CONFIG_SPEC, bool, O>;
#[doc = "Field `M` reader - "]
pub type M_R = crate::BitReader<bool>;
#[doc = "Field `M` writer - "]
pub type M_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_TOP_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn e(&mut self) -> E_W<0> {
        E_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<1> {
        M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_Top_Config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_top_config](index.html) module"]
pub struct DMA_TOP_CONFIG_SPEC;
impl crate::RegisterSpec for DMA_TOP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_top_config::R](R) reader structure"]
impl crate::Readable for DMA_TOP_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_top_config::W](W) writer structure"]
impl crate::Writable for DMA_TOP_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_Top_Config to value 0"]
impl crate::Resettable for DMA_TOP_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

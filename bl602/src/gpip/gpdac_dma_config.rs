#[doc = "Register `gpdac_dma_config` reader"]
pub struct R(crate::R<GPDAC_DMA_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_DMA_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_DMA_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_DMA_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_dma_config` writer"]
pub struct W(crate::W<GPDAC_DMA_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_DMA_CONFIG_SPEC>;
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
impl From<crate::W<GPDAC_DMA_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDAC_DMA_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpdac_dma_tx_en` reader - "]
pub type GPDAC_DMA_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpdac_dma_tx_en` writer - "]
pub type GPDAC_DMA_TX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPDAC_DMA_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpdac_dma_format` reader - "]
pub type GPDAC_DMA_FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpdac_dma_format` writer - "]
pub type GPDAC_DMA_FORMAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPDAC_DMA_CONFIG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_dma_tx_en(&self) -> GPDAC_DMA_TX_EN_R {
        GPDAC_DMA_TX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gpdac_dma_format(&self) -> GPDAC_DMA_FORMAT_R {
        GPDAC_DMA_FORMAT_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_dma_tx_en(&mut self) -> GPDAC_DMA_TX_EN_W<0> {
        GPDAC_DMA_TX_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_dma_format(&mut self) -> GPDAC_DMA_FORMAT_W<4> {
        GPDAC_DMA_FORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpdac_dma_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_dma_config](index.html) module"]
pub struct GPDAC_DMA_CONFIG_SPEC;
impl crate::RegisterSpec for GPDAC_DMA_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_dma_config::R](R) reader structure"]
impl crate::Readable for GPDAC_DMA_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_dma_config::W](W) writer structure"]
impl crate::Writable for GPDAC_DMA_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpdac_dma_config to value 0"]
impl crate::Resettable for GPDAC_DMA_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

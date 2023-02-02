#[doc = "Register `gpadc_config` reader"]
pub struct R(crate::R<GPADC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_config` writer"]
pub struct W(crate::W<GPADC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_CONFIG_SPEC>;
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
impl From<crate::W<GPADC_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_dma_en` reader - "]
pub type GPADC_DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_dma_en` writer - "]
pub type GPADC_DMA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_fifo_clr` reader - "]
pub type GPADC_FIFO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_clr` writer - "]
pub type GPADC_FIFO_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_fifo_ne` reader - "]
pub type GPADC_FIFO_NE_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_full` reader - "]
pub type GPADC_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_rdy` reader - "]
pub type GPADC_RDY_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_overrun` reader - "]
pub type GPADC_FIFO_OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_underrun` reader - "]
pub type GPADC_FIFO_UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_rdy_clr` reader - "]
pub type GPADC_RDY_CLR_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_rdy_clr` writer - "]
pub type GPADC_RDY_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_fifo_overrun_clr` reader - "]
pub type GPADC_FIFO_OVERRUN_CLR_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_overrun_clr` writer - "]
pub type GPADC_FIFO_OVERRUN_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_fifo_underrun_clr` reader - "]
pub type GPADC_FIFO_UNDERRUN_CLR_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_underrun_clr` writer - "]
pub type GPADC_FIFO_UNDERRUN_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_rdy_mask` reader - "]
pub type GPADC_RDY_MASK_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_rdy_mask` writer - "]
pub type GPADC_RDY_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_fifo_overrun_mask` reader - "]
pub type GPADC_FIFO_OVERRUN_MASK_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_overrun_mask` writer - "]
pub type GPADC_FIFO_OVERRUN_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_fifo_underrun_mask` reader - "]
pub type GPADC_FIFO_UNDERRUN_MASK_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_fifo_underrun_mask` writer - "]
pub type GPADC_FIFO_UNDERRUN_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpadc_fifo_data_count` reader - "]
pub type GPADC_FIFO_DATA_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_fifo_thl` reader - "]
pub type GPADC_FIFO_THL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_fifo_thl` writer - "]
pub type GPADC_FIFO_THL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `rsvd_31_24` reader - "]
pub type RSVD_31_24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rsvd_31_24` writer - "]
pub type RSVD_31_24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_CONFIG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_dma_en(&self) -> GPADC_DMA_EN_R {
        GPADC_DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_fifo_clr(&self) -> GPADC_FIFO_CLR_R {
        GPADC_FIFO_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_fifo_ne(&self) -> GPADC_FIFO_NE_R {
        GPADC_FIFO_NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_fifo_full(&self) -> GPADC_FIFO_FULL_R {
        GPADC_FIFO_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_rdy(&self) -> GPADC_RDY_R {
        GPADC_RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun(&self) -> GPADC_FIFO_OVERRUN_R {
        GPADC_FIFO_OVERRUN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun(&self) -> GPADC_FIFO_UNDERRUN_R {
        GPADC_FIFO_UNDERRUN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_rdy_clr(&self) -> GPADC_RDY_CLR_R {
        GPADC_RDY_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_clr(&self) -> GPADC_FIFO_OVERRUN_CLR_R {
        GPADC_FIFO_OVERRUN_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_clr(&self) -> GPADC_FIFO_UNDERRUN_CLR_R {
        GPADC_FIFO_UNDERRUN_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpadc_rdy_mask(&self) -> GPADC_RDY_MASK_R {
        GPADC_RDY_MASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_mask(&self) -> GPADC_FIFO_OVERRUN_MASK_R {
        GPADC_FIFO_OVERRUN_MASK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_mask(&self) -> GPADC_FIFO_UNDERRUN_MASK_R {
        GPADC_FIFO_UNDERRUN_MASK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn gpadc_fifo_data_count(&self) -> GPADC_FIFO_DATA_COUNT_R {
        GPADC_FIFO_DATA_COUNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn gpadc_fifo_thl(&self) -> GPADC_FIFO_THL_R {
        GPADC_FIFO_THL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&self) -> RSVD_31_24_R {
        RSVD_31_24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_dma_en(&mut self) -> GPADC_DMA_EN_W<0> {
        GPADC_DMA_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_fifo_clr(&mut self) -> GPADC_FIFO_CLR_W<1> {
        GPADC_FIFO_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_rdy_clr(&mut self) -> GPADC_RDY_CLR_W<8> {
        GPADC_RDY_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_fifo_overrun_clr(&mut self) -> GPADC_FIFO_OVERRUN_CLR_W<9> {
        GPADC_FIFO_OVERRUN_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_fifo_underrun_clr(&mut self) -> GPADC_FIFO_UNDERRUN_CLR_W<10> {
        GPADC_FIFO_UNDERRUN_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_rdy_mask(&mut self) -> GPADC_RDY_MASK_W<12> {
        GPADC_RDY_MASK_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_fifo_overrun_mask(&mut self) -> GPADC_FIFO_OVERRUN_MASK_W<13> {
        GPADC_FIFO_OVERRUN_MASK_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_fifo_underrun_mask(&mut self) -> GPADC_FIFO_UNDERRUN_MASK_W<14> {
        GPADC_FIFO_UNDERRUN_MASK_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_fifo_thl(&mut self) -> GPADC_FIFO_THL_W<22> {
        GPADC_FIFO_THL_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_31_24(&mut self) -> RSVD_31_24_W<24> {
        RSVD_31_24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_config](index.html) module"]
pub struct GPADC_CONFIG_SPEC;
impl crate::RegisterSpec for GPADC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_config::R](R) reader structure"]
impl crate::Readable for GPADC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_config::W](W) writer structure"]
impl crate::Writable for GPADC_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_config to value 0"]
impl crate::Resettable for GPADC_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

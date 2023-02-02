#[doc = "Register `clkpll_sdm` reader"]
pub struct R(crate::R<CLKPLL_SDM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_SDM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPLL_SDM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPLL_SDM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_sdm` writer"]
pub struct W(crate::W<CLKPLL_SDM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_SDM_SPEC>;
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
impl From<crate::W<CLKPLL_SDM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPLL_SDM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_sdmin` reader - "]
pub type CLKPLL_SDMIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `clkpll_sdmin` writer - "]
pub type CLKPLL_SDMIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_SDM_SPEC, u32, u32, 24, O>;
#[doc = "Field `clkpll_dither_sel` reader - "]
pub type CLKPLL_DITHER_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_dither_sel` writer - "]
pub type CLKPLL_DITHER_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_SDM_SPEC, u8, u8, 2, O>;
#[doc = "Field `clkpll_sdm_flag` reader - "]
pub type CLKPLL_SDM_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_sdm_flag` writer - "]
pub type CLKPLL_SDM_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKPLL_SDM_SPEC, bool, O>;
#[doc = "Field `clkpll_sdm_bypass` reader - "]
pub type CLKPLL_SDM_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_sdm_bypass` writer - "]
pub type CLKPLL_SDM_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKPLL_SDM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn clkpll_sdmin(&self) -> CLKPLL_SDMIN_R {
        CLKPLL_SDMIN_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_dither_sel(&self) -> CLKPLL_DITHER_SEL_R {
        CLKPLL_DITHER_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clkpll_sdm_flag(&self) -> CLKPLL_SDM_FLAG_R {
        CLKPLL_SDM_FLAG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clkpll_sdm_bypass(&self) -> CLKPLL_SDM_BYPASS_R {
        CLKPLL_SDM_BYPASS_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_sdmin(&mut self) -> CLKPLL_SDMIN_W<0> {
        CLKPLL_SDMIN_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_dither_sel(&mut self) -> CLKPLL_DITHER_SEL_W<24> {
        CLKPLL_DITHER_SEL_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_sdm_flag(&mut self) -> CLKPLL_SDM_FLAG_W<28> {
        CLKPLL_SDM_FLAG_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_sdm_bypass(&mut self) -> CLKPLL_SDM_BYPASS_W<29> {
        CLKPLL_SDM_BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_sdm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_sdm](index.html) module"]
pub struct CLKPLL_SDM_SPEC;
impl crate::RegisterSpec for CLKPLL_SDM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_sdm::R](R) reader structure"]
impl crate::Readable for CLKPLL_SDM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_sdm::W](W) writer structure"]
impl crate::Writable for CLKPLL_SDM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clkpll_sdm to value 0x1060_0000"]
impl crate::Resettable for CLKPLL_SDM_SPEC {
    const RESET_VALUE: Self::Ux = 0x1060_0000;
}

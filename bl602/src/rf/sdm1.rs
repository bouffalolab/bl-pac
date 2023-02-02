#[doc = "Register `sdm1` reader"]
pub struct R(crate::R<SDM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sdm1` writer"]
pub struct W(crate::W<SDM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDM1_SPEC>;
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
impl From<crate::W<SDM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdm_dither_sel_hw` reader - "]
pub type LO_SDM_DITHER_SEL_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_hw` writer - "]
pub type LO_SDM_DITHER_SEL_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDM1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_bypass_hw` reader - "]
pub type LO_SDM_BYPASS_HW_R = crate::BitReader<bool>;
#[doc = "Field `lo_sdm_bypass_hw` writer - "]
pub type LO_SDM_BYPASS_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDM1_SPEC, bool, O>;
#[doc = "Field `lo_sdm_dither_sel` reader - "]
pub type LO_SDM_DITHER_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel` writer - "]
pub type LO_SDM_DITHER_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDM1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_bypass` reader - "]
pub type LO_SDM_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `lo_sdm_bypass` writer - "]
pub type LO_SDM_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDM1_SPEC, bool, O>;
#[doc = "Field `lo_sdm_rstb` reader - "]
pub type LO_SDM_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `lo_sdm_rstb` writer - "]
pub type LO_SDM_RSTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDM1_SPEC, bool, O>;
#[doc = "Field `lo_sdm_rstb_hw` reader - "]
pub type LO_SDM_RSTB_HW_R = crate::BitReader<bool>;
#[doc = "Field `lo_sdm_rstb_hw` writer - "]
pub type LO_SDM_RSTB_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDM1_SPEC, bool, O>;
#[doc = "Field `lo_sdm_flag` reader - "]
pub type LO_SDM_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `lo_sdm_flag` writer - "]
pub type LO_SDM_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDM1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_hw(&self) -> LO_SDM_DITHER_SEL_HW_R {
        LO_SDM_DITHER_SEL_HW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_sdm_bypass_hw(&self) -> LO_SDM_BYPASS_HW_R {
        LO_SDM_BYPASS_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel(&self) -> LO_SDM_DITHER_SEL_R {
        LO_SDM_DITHER_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_sdm_bypass(&self) -> LO_SDM_BYPASS_R {
        LO_SDM_BYPASS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_sdm_rstb(&self) -> LO_SDM_RSTB_R {
        LO_SDM_RSTB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lo_sdm_rstb_hw(&self) -> LO_SDM_RSTB_HW_R {
        LO_SDM_RSTB_HW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_sdm_flag(&self) -> LO_SDM_FLAG_R {
        LO_SDM_FLAG_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_hw(&mut self) -> LO_SDM_DITHER_SEL_HW_W<0> {
        LO_SDM_DITHER_SEL_HW_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_bypass_hw(&mut self) -> LO_SDM_BYPASS_HW_W<4> {
        LO_SDM_BYPASS_HW_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel(&mut self) -> LO_SDM_DITHER_SEL_W<8> {
        LO_SDM_DITHER_SEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_bypass(&mut self) -> LO_SDM_BYPASS_W<12> {
        LO_SDM_BYPASS_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_rstb(&mut self) -> LO_SDM_RSTB_W<16> {
        LO_SDM_RSTB_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_rstb_hw(&mut self) -> LO_SDM_RSTB_HW_W<17> {
        LO_SDM_RSTB_HW_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_flag(&mut self) -> LO_SDM_FLAG_W<20> {
        LO_SDM_FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdm1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdm1](index.html) module"]
pub struct SDM1_SPEC;
impl crate::RegisterSpec for SDM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdm1::R](R) reader structure"]
impl crate::Readable for SDM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdm1::W](W) writer structure"]
impl crate::Writable for SDM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sdm1 to value 0"]
impl crate::Resettable for SDM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

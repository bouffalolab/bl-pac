#[doc = "Register `tbb` reader"]
pub struct R(crate::R<TBB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tbb` writer"]
pub struct W(crate::W<TBB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBB_SPEC>;
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
impl From<crate::W<TBB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tbb_bm_sf` reader - "]
pub type TBB_BM_SF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tbb_bm_sf` writer - "]
pub type TBB_BM_SF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBB_SPEC, u8, u8, 2, O>;
#[doc = "Field `tbb_bm_cg` reader - "]
pub type TBB_BM_CG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tbb_bm_cg` writer - "]
pub type TBB_BM_CG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBB_SPEC, u8, u8, 2, O>;
#[doc = "Field `tbb_vcm` reader - "]
pub type TBB_VCM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tbb_vcm` writer - "]
pub type TBB_VCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBB_SPEC, u8, u8, 2, O>;
#[doc = "Field `tbb_cflt` reader - "]
pub type TBB_CFLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tbb_cflt` writer - "]
pub type TBB_CFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBB_SPEC, u8, u8, 2, O>;
#[doc = "Field `tbb_iq_bias_short` reader - "]
pub type TBB_IQ_BIAS_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `tbb_iq_bias_short` writer - "]
pub type TBB_IQ_BIAS_SHORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBB_SPEC, bool, O>;
#[doc = "Field `tbb_atest_out_en` reader - "]
pub type TBB_ATEST_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `tbb_atest_out_en` writer - "]
pub type TBB_ATEST_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBB_SPEC, bool, O>;
#[doc = "Field `tbb_tosdac_q` reader - "]
pub type TBB_TOSDAC_Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tbb_tosdac_q` writer - "]
pub type TBB_TOSDAC_Q_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBB_SPEC, u8, u8, 6, O>;
#[doc = "Field `tbb_tosdac_i` reader - "]
pub type TBB_TOSDAC_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tbb_tosdac_i` writer - "]
pub type TBB_TOSDAC_I_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBB_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tbb_bm_sf(&self) -> TBB_BM_SF_R {
        TBB_BM_SF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tbb_bm_cg(&self) -> TBB_BM_CG_R {
        TBB_BM_CG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tbb_vcm(&self) -> TBB_VCM_R {
        TBB_VCM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tbb_cflt(&self) -> TBB_CFLT_R {
        TBB_CFLT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tbb_iq_bias_short(&self) -> TBB_IQ_BIAS_SHORT_R {
        TBB_IQ_BIAS_SHORT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tbb_atest_out_en(&self) -> TBB_ATEST_OUT_EN_R {
        TBB_ATEST_OUT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_q(&self) -> TBB_TOSDAC_Q_R {
        TBB_TOSDAC_Q_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_i(&self) -> TBB_TOSDAC_I_R {
        TBB_TOSDAC_I_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_bm_sf(&mut self) -> TBB_BM_SF_W<0> {
        TBB_BM_SF_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_bm_cg(&mut self) -> TBB_BM_CG_W<4> {
        TBB_BM_CG_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_vcm(&mut self) -> TBB_VCM_W<8> {
        TBB_VCM_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_cflt(&mut self) -> TBB_CFLT_W<12> {
        TBB_CFLT_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_iq_bias_short(&mut self) -> TBB_IQ_BIAS_SHORT_W<14> {
        TBB_IQ_BIAS_SHORT_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_atest_out_en(&mut self) -> TBB_ATEST_OUT_EN_W<15> {
        TBB_ATEST_OUT_EN_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_tosdac_q(&mut self) -> TBB_TOSDAC_Q_W<16> {
        TBB_TOSDAC_Q_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_tosdac_i(&mut self) -> TBB_TOSDAC_I_W<24> {
        TBB_TOSDAC_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tbb.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbb](index.html) module"]
pub struct TBB_SPEC;
impl crate::RegisterSpec for TBB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbb::R](R) reader structure"]
impl crate::Readable for TBB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbb::W](W) writer structure"]
impl crate::Writable for TBB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tbb to value 0"]
impl crate::Resettable for TBB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

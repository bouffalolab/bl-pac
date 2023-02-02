#[doc = "Register `tbb_gain_index2` reader"]
pub struct R(crate::R<TBB_GAIN_INDEX2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBB_GAIN_INDEX2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBB_GAIN_INDEX2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBB_GAIN_INDEX2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tbb_gain_index2` writer"]
pub struct W(crate::W<TBB_GAIN_INDEX2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBB_GAIN_INDEX2_SPEC>;
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
impl From<crate::W<TBB_GAIN_INDEX2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBB_GAIN_INDEX2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gain_ctrl2_gc_tbb` reader - "]
pub type GAIN_CTRL2_GC_TBB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl2_gc_tbb` writer - "]
pub type GAIN_CTRL2_GC_TBB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TBB_GAIN_INDEX2_SPEC, u8, u8, 5, O>;
#[doc = "Field `gain_ctrl2_gc_tmx` reader - "]
pub type GAIN_CTRL2_GC_TMX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl2_gc_tmx` writer - "]
pub type GAIN_CTRL2_GC_TMX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TBB_GAIN_INDEX2_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl2_dac_bias_sel` reader - "]
pub type GAIN_CTRL2_DAC_BIAS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl2_dac_bias_sel` writer - "]
pub type GAIN_CTRL2_DAC_BIAS_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TBB_GAIN_INDEX2_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl2_gc_tbb_boost` reader - "]
pub type GAIN_CTRL2_GC_TBB_BOOST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl2_gc_tbb_boost` writer - "]
pub type GAIN_CTRL2_GC_TBB_BOOST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TBB_GAIN_INDEX2_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl3_gc_tbb` reader - "]
pub type GAIN_CTRL3_GC_TBB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl3_gc_tbb` writer - "]
pub type GAIN_CTRL3_GC_TBB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TBB_GAIN_INDEX2_SPEC, u8, u8, 5, O>;
#[doc = "Field `gain_ctrl3_gc_tmx` reader - "]
pub type GAIN_CTRL3_GC_TMX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl3_gc_tmx` writer - "]
pub type GAIN_CTRL3_GC_TMX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TBB_GAIN_INDEX2_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl3_dac_bias_sel` reader - "]
pub type GAIN_CTRL3_DAC_BIAS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl3_dac_bias_sel` writer - "]
pub type GAIN_CTRL3_DAC_BIAS_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TBB_GAIN_INDEX2_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl3_gc_tbb_boost` reader - "]
pub type GAIN_CTRL3_GC_TBB_BOOST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl3_gc_tbb_boost` writer - "]
pub type GAIN_CTRL3_GC_TBB_BOOST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TBB_GAIN_INDEX2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_tbb(&self) -> GAIN_CTRL2_GC_TBB_R {
        GAIN_CTRL2_GC_TBB_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_tmx(&self) -> GAIN_CTRL2_GC_TMX_R {
        GAIN_CTRL2_GC_TMX_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl2_dac_bias_sel(&self) -> GAIN_CTRL2_DAC_BIAS_SEL_R {
        GAIN_CTRL2_DAC_BIAS_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_tbb_boost(&self) -> GAIN_CTRL2_GC_TBB_BOOST_R {
        GAIN_CTRL2_GC_TBB_BOOST_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_tbb(&self) -> GAIN_CTRL3_GC_TBB_R {
        GAIN_CTRL3_GC_TBB_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_tmx(&self) -> GAIN_CTRL3_GC_TMX_R {
        GAIN_CTRL3_GC_TMX_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl3_dac_bias_sel(&self) -> GAIN_CTRL3_DAC_BIAS_SEL_R {
        GAIN_CTRL3_DAC_BIAS_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_tbb_boost(&self) -> GAIN_CTRL3_GC_TBB_BOOST_R {
        GAIN_CTRL3_GC_TBB_BOOST_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl2_gc_tbb(&mut self) -> GAIN_CTRL2_GC_TBB_W<0> {
        GAIN_CTRL2_GC_TBB_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl2_gc_tmx(&mut self) -> GAIN_CTRL2_GC_TMX_W<8> {
        GAIN_CTRL2_GC_TMX_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl2_dac_bias_sel(&mut self) -> GAIN_CTRL2_DAC_BIAS_SEL_W<12> {
        GAIN_CTRL2_DAC_BIAS_SEL_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl2_gc_tbb_boost(&mut self) -> GAIN_CTRL2_GC_TBB_BOOST_W<14> {
        GAIN_CTRL2_GC_TBB_BOOST_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl3_gc_tbb(&mut self) -> GAIN_CTRL3_GC_TBB_W<16> {
        GAIN_CTRL3_GC_TBB_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl3_gc_tmx(&mut self) -> GAIN_CTRL3_GC_TMX_W<24> {
        GAIN_CTRL3_GC_TMX_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl3_dac_bias_sel(&mut self) -> GAIN_CTRL3_DAC_BIAS_SEL_W<28> {
        GAIN_CTRL3_DAC_BIAS_SEL_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl3_gc_tbb_boost(&mut self) -> GAIN_CTRL3_GC_TBB_BOOST_W<30> {
        GAIN_CTRL3_GC_TBB_BOOST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tbb_gain_index2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbb_gain_index2](index.html) module"]
pub struct TBB_GAIN_INDEX2_SPEC;
impl crate::RegisterSpec for TBB_GAIN_INDEX2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbb_gain_index2::R](R) reader structure"]
impl crate::Readable for TBB_GAIN_INDEX2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbb_gain_index2::W](W) writer structure"]
impl crate::Writable for TBB_GAIN_INDEX2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tbb_gain_index2 to value 0"]
impl crate::Resettable for TBB_GAIN_INDEX2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

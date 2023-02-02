#[doc = "Register `dfe_ctrl_16` reader"]
pub struct R(crate::R<DFE_CTRL_16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_16` writer"]
pub struct W(crate::W<DFE_CTRL_16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_16_SPEC>;
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
impl From<crate::W<DFE_CTRL_16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_tbb_ind_gc0` reader - "]
pub type RF_TBB_IND_GC0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc0` writer - "]
pub type RF_TBB_IND_GC0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_16_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc1` reader - "]
pub type RF_TBB_IND_GC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc1` writer - "]
pub type RF_TBB_IND_GC1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_16_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc2` reader - "]
pub type RF_TBB_IND_GC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc2` writer - "]
pub type RF_TBB_IND_GC2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_16_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc3` reader - "]
pub type RF_TBB_IND_GC3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc3` writer - "]
pub type RF_TBB_IND_GC3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_16_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc4` reader - "]
pub type RF_TBB_IND_GC4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc4` writer - "]
pub type RF_TBB_IND_GC4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_16_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc5` reader - "]
pub type RF_TBB_IND_GC5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc5` writer - "]
pub type RF_TBB_IND_GC5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_16_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc6` reader - "]
pub type RF_TBB_IND_GC6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc6` writer - "]
pub type RF_TBB_IND_GC6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_16_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc7` reader - "]
pub type RF_TBB_IND_GC7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc7` writer - "]
pub type RF_TBB_IND_GC7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_16_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc0(&self) -> RF_TBB_IND_GC0_R {
        RF_TBB_IND_GC0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc1(&self) -> RF_TBB_IND_GC1_R {
        RF_TBB_IND_GC1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc2(&self) -> RF_TBB_IND_GC2_R {
        RF_TBB_IND_GC2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc3(&self) -> RF_TBB_IND_GC3_R {
        RF_TBB_IND_GC3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc4(&self) -> RF_TBB_IND_GC4_R {
        RF_TBB_IND_GC4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc5(&self) -> RF_TBB_IND_GC5_R {
        RF_TBB_IND_GC5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc6(&self) -> RF_TBB_IND_GC6_R {
        RF_TBB_IND_GC6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc7(&self) -> RF_TBB_IND_GC7_R {
        RF_TBB_IND_GC7_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc0(&mut self) -> RF_TBB_IND_GC0_W<0> {
        RF_TBB_IND_GC0_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc1(&mut self) -> RF_TBB_IND_GC1_W<4> {
        RF_TBB_IND_GC1_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc2(&mut self) -> RF_TBB_IND_GC2_W<8> {
        RF_TBB_IND_GC2_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc3(&mut self) -> RF_TBB_IND_GC3_W<12> {
        RF_TBB_IND_GC3_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc4(&mut self) -> RF_TBB_IND_GC4_W<16> {
        RF_TBB_IND_GC4_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc5(&mut self) -> RF_TBB_IND_GC5_W<20> {
        RF_TBB_IND_GC5_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc6(&mut self) -> RF_TBB_IND_GC6_W<24> {
        RF_TBB_IND_GC6_W::new(self)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc7(&mut self) -> RF_TBB_IND_GC7_W<28> {
        RF_TBB_IND_GC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_16.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_16](index.html) module"]
pub struct DFE_CTRL_16_SPEC;
impl crate::RegisterSpec for DFE_CTRL_16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_16::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_16::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_16 to value 0"]
impl crate::Resettable for DFE_CTRL_16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

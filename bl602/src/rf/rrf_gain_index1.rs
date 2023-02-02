#[doc = "Register `rrf_gain_index1` reader"]
pub struct R(crate::R<RRF_GAIN_INDEX1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RRF_GAIN_INDEX1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RRF_GAIN_INDEX1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RRF_GAIN_INDEX1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rrf_gain_index1` writer"]
pub struct W(crate::W<RRF_GAIN_INDEX1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RRF_GAIN_INDEX1_SPEC>;
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
impl From<crate::W<RRF_GAIN_INDEX1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RRF_GAIN_INDEX1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gain_ctrl0_gc_rmxgm` reader - "]
pub type GAIN_CTRL0_GC_RMXGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl0_gc_rmxgm` writer - "]
pub type GAIN_CTRL0_GC_RMXGM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl0_gc_lna` reader - "]
pub type GAIN_CTRL0_GC_LNA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl0_gc_lna` writer - "]
pub type GAIN_CTRL0_GC_LNA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl1_gc_rmxgm` reader - "]
pub type GAIN_CTRL1_GC_RMXGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl1_gc_rmxgm` writer - "]
pub type GAIN_CTRL1_GC_RMXGM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl1_gc_lna` reader - "]
pub type GAIN_CTRL1_GC_LNA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl1_gc_lna` writer - "]
pub type GAIN_CTRL1_GC_LNA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl2_gc_rmxgm` reader - "]
pub type GAIN_CTRL2_GC_RMXGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl2_gc_rmxgm` writer - "]
pub type GAIN_CTRL2_GC_RMXGM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl2_gc_lna` reader - "]
pub type GAIN_CTRL2_GC_LNA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl2_gc_lna` writer - "]
pub type GAIN_CTRL2_GC_LNA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl3_gc_rmxgm` reader - "]
pub type GAIN_CTRL3_GC_RMXGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl3_gc_rmxgm` writer - "]
pub type GAIN_CTRL3_GC_RMXGM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl3_gc_lna` reader - "]
pub type GAIN_CTRL3_GC_LNA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl3_gc_lna` writer - "]
pub type GAIN_CTRL3_GC_LNA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl4_gc_rmxgm` reader - "]
pub type GAIN_CTRL4_GC_RMXGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl4_gc_rmxgm` writer - "]
pub type GAIN_CTRL4_GC_RMXGM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl4_gc_lna` reader - "]
pub type GAIN_CTRL4_GC_LNA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl4_gc_lna` writer - "]
pub type GAIN_CTRL4_GC_LNA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl5_gc_rmxgm` reader - "]
pub type GAIN_CTRL5_GC_RMXGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl5_gc_rmxgm` writer - "]
pub type GAIN_CTRL5_GC_RMXGM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl5_gc_lna` reader - "]
pub type GAIN_CTRL5_GC_LNA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl5_gc_lna` writer - "]
pub type GAIN_CTRL5_GC_LNA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RRF_GAIN_INDEX1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rmxgm(&self) -> GAIN_CTRL0_GC_RMXGM_R {
        GAIN_CTRL0_GC_RMXGM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_lna(&self) -> GAIN_CTRL0_GC_LNA_R {
        GAIN_CTRL0_GC_LNA_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rmxgm(&self) -> GAIN_CTRL1_GC_RMXGM_R {
        GAIN_CTRL1_GC_RMXGM_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_lna(&self) -> GAIN_CTRL1_GC_LNA_R {
        GAIN_CTRL1_GC_LNA_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rmxgm(&self) -> GAIN_CTRL2_GC_RMXGM_R {
        GAIN_CTRL2_GC_RMXGM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_lna(&self) -> GAIN_CTRL2_GC_LNA_R {
        GAIN_CTRL2_GC_LNA_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rmxgm(&self) -> GAIN_CTRL3_GC_RMXGM_R {
        GAIN_CTRL3_GC_RMXGM_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_lna(&self) -> GAIN_CTRL3_GC_LNA_R {
        GAIN_CTRL3_GC_LNA_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rmxgm(&self) -> GAIN_CTRL4_GC_RMXGM_R {
        GAIN_CTRL4_GC_RMXGM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_lna(&self) -> GAIN_CTRL4_GC_LNA_R {
        GAIN_CTRL4_GC_LNA_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rmxgm(&self) -> GAIN_CTRL5_GC_RMXGM_R {
        GAIN_CTRL5_GC_RMXGM_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_lna(&self) -> GAIN_CTRL5_GC_LNA_R {
        GAIN_CTRL5_GC_LNA_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl0_gc_rmxgm(&mut self) -> GAIN_CTRL0_GC_RMXGM_W<0> {
        GAIN_CTRL0_GC_RMXGM_W::new(self)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl0_gc_lna(&mut self) -> GAIN_CTRL0_GC_LNA_W<2> {
        GAIN_CTRL0_GC_LNA_W::new(self)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl1_gc_rmxgm(&mut self) -> GAIN_CTRL1_GC_RMXGM_W<5> {
        GAIN_CTRL1_GC_RMXGM_W::new(self)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl1_gc_lna(&mut self) -> GAIN_CTRL1_GC_LNA_W<7> {
        GAIN_CTRL1_GC_LNA_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl2_gc_rmxgm(&mut self) -> GAIN_CTRL2_GC_RMXGM_W<10> {
        GAIN_CTRL2_GC_RMXGM_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl2_gc_lna(&mut self) -> GAIN_CTRL2_GC_LNA_W<12> {
        GAIN_CTRL2_GC_LNA_W::new(self)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl3_gc_rmxgm(&mut self) -> GAIN_CTRL3_GC_RMXGM_W<15> {
        GAIN_CTRL3_GC_RMXGM_W::new(self)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl3_gc_lna(&mut self) -> GAIN_CTRL3_GC_LNA_W<17> {
        GAIN_CTRL3_GC_LNA_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl4_gc_rmxgm(&mut self) -> GAIN_CTRL4_GC_RMXGM_W<20> {
        GAIN_CTRL4_GC_RMXGM_W::new(self)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl4_gc_lna(&mut self) -> GAIN_CTRL4_GC_LNA_W<22> {
        GAIN_CTRL4_GC_LNA_W::new(self)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl5_gc_rmxgm(&mut self) -> GAIN_CTRL5_GC_RMXGM_W<25> {
        GAIN_CTRL5_GC_RMXGM_W::new(self)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl5_gc_lna(&mut self) -> GAIN_CTRL5_GC_LNA_W<27> {
        GAIN_CTRL5_GC_LNA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rrf_gain_index1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rrf_gain_index1](index.html) module"]
pub struct RRF_GAIN_INDEX1_SPEC;
impl crate::RegisterSpec for RRF_GAIN_INDEX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rrf_gain_index1::R](R) reader structure"]
impl crate::Readable for RRF_GAIN_INDEX1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rrf_gain_index1::W](W) writer structure"]
impl crate::Writable for RRF_GAIN_INDEX1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rrf_gain_index1 to value 0"]
impl crate::Resettable for RRF_GAIN_INDEX1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

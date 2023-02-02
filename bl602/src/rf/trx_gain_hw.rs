#[doc = "Register `trx_gain_hw` reader"]
pub struct R(crate::R<TRX_GAIN_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRX_GAIN_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRX_GAIN_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRX_GAIN_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `trx_gain_hw` writer"]
pub struct W(crate::W<TRX_GAIN_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRX_GAIN_HW_SPEC>;
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
impl From<crate::W<TRX_GAIN_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRX_GAIN_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gc_lna_hw` reader - "]
pub type GC_LNA_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_lna_hw` writer - "]
pub type GC_LNA_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN_HW_SPEC, u8, u8, 3, O>;
#[doc = "Field `gc_rmxgm_hw` reader - "]
pub type GC_RMXGM_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_rmxgm_hw` writer - "]
pub type GC_RMXGM_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_HW_SPEC, u8, u8, 2, O>;
#[doc = "Field `gc_rbb1_hw` reader - "]
pub type GC_RBB1_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_rbb1_hw` writer - "]
pub type GC_RBB1_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_HW_SPEC, u8, u8, 2, O>;
#[doc = "Field `gc_rbb2_hw` reader - "]
pub type GC_RBB2_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_rbb2_hw` writer - "]
pub type GC_RBB2_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_HW_SPEC, u8, u8, 3, O>;
#[doc = "Field `gc_tmx_hw` reader - "]
pub type GC_TMX_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_tmx_hw` writer - "]
pub type GC_TMX_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN_HW_SPEC, u8, u8, 3, O>;
#[doc = "Field `gc_tbb_hw` reader - "]
pub type GC_TBB_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_tbb_hw` writer - "]
pub type GC_TBB_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN_HW_SPEC, u8, u8, 5, O>;
#[doc = "Field `gc_tbb_boost_hw` reader - "]
pub type GC_TBB_BOOST_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_tbb_boost_hw` writer - "]
pub type GC_TBB_BOOST_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_HW_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna_hw(&self) -> GC_LNA_HW_R {
        GC_LNA_HW_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gc_rmxgm_hw(&self) -> GC_RMXGM_HW_R {
        GC_RMXGM_HW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gc_rbb1_hw(&self) -> GC_RBB1_HW_R {
        GC_RBB1_HW_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gc_rbb2_hw(&self) -> GC_RBB2_HW_R {
        GC_RBB2_HW_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gc_tmx_hw(&self) -> GC_TMX_HW_R {
        GC_TMX_HW_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gc_tbb_hw(&self) -> GC_TBB_HW_R {
        GC_TBB_HW_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gc_tbb_boost_hw(&self) -> GC_TBB_BOOST_HW_R {
        GC_TBB_BOOST_HW_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn gc_lna_hw(&mut self) -> GC_LNA_HW_W<0> {
        GC_LNA_HW_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn gc_rmxgm_hw(&mut self) -> GC_RMXGM_HW_W<4> {
        GC_RMXGM_HW_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn gc_rbb1_hw(&mut self) -> GC_RBB1_HW_W<8> {
        GC_RBB1_HW_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn gc_rbb2_hw(&mut self) -> GC_RBB2_HW_W<12> {
        GC_RBB2_HW_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn gc_tmx_hw(&mut self) -> GC_TMX_HW_W<16> {
        GC_TMX_HW_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn gc_tbb_hw(&mut self) -> GC_TBB_HW_W<20> {
        GC_TBB_HW_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn gc_tbb_boost_hw(&mut self) -> GC_TBB_BOOST_HW_W<28> {
        GC_TBB_BOOST_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "trx gain hardware readback\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trx_gain_hw](index.html) module"]
pub struct TRX_GAIN_HW_SPEC;
impl crate::RegisterSpec for TRX_GAIN_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trx_gain_hw::R](R) reader structure"]
impl crate::Readable for TRX_GAIN_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trx_gain_hw::W](W) writer structure"]
impl crate::Writable for TRX_GAIN_HW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets trx_gain_hw to value 0"]
impl crate::Resettable for TRX_GAIN_HW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

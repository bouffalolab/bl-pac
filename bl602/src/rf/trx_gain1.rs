#[doc = "Register `trx_gain1` reader"]
pub struct R(crate::R<TRX_GAIN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRX_GAIN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRX_GAIN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRX_GAIN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `trx_gain1` writer"]
pub struct W(crate::W<TRX_GAIN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRX_GAIN1_SPEC>;
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
impl From<crate::W<TRX_GAIN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRX_GAIN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gc_lna` reader - "]
pub type GC_LNA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_lna` writer - "]
pub type GC_LNA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gc_rmxgm` reader - "]
pub type GC_RMXGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_rmxgm` writer - "]
pub type GC_RMXGM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gc_rbb1` reader - "]
pub type GC_RBB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_rbb1` writer - "]
pub type GC_RBB1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gc_rbb2` reader - "]
pub type GC_RBB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_rbb2` writer - "]
pub type GC_RBB2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gc_tmx` reader - "]
pub type GC_TMX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_tmx` writer - "]
pub type GC_TMX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gc_tbb` reader - "]
pub type GC_TBB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_tbb` writer - "]
pub type GC_TBB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN1_SPEC, u8, u8, 5, O>;
#[doc = "Field `gc_tbb_boost` reader - "]
pub type GC_TBB_BOOST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_tbb_boost` writer - "]
pub type GC_TBB_BOOST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna(&self) -> GC_LNA_R {
        GC_LNA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gc_rmxgm(&self) -> GC_RMXGM_R {
        GC_RMXGM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gc_rbb1(&self) -> GC_RBB1_R {
        GC_RBB1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gc_rbb2(&self) -> GC_RBB2_R {
        GC_RBB2_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gc_tmx(&self) -> GC_TMX_R {
        GC_TMX_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gc_tbb(&self) -> GC_TBB_R {
        GC_TBB_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gc_tbb_boost(&self) -> GC_TBB_BOOST_R {
        GC_TBB_BOOST_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn gc_lna(&mut self) -> GC_LNA_W<0> {
        GC_LNA_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn gc_rmxgm(&mut self) -> GC_RMXGM_W<4> {
        GC_RMXGM_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn gc_rbb1(&mut self) -> GC_RBB1_W<8> {
        GC_RBB1_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn gc_rbb2(&mut self) -> GC_RBB2_W<12> {
        GC_RBB2_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn gc_tmx(&mut self) -> GC_TMX_W<16> {
        GC_TMX_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn gc_tbb(&mut self) -> GC_TBB_W<20> {
        GC_TBB_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn gc_tbb_boost(&mut self) -> GC_TBB_BOOST_W<28> {
        GC_TBB_BOOST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gain control1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trx_gain1](index.html) module"]
pub struct TRX_GAIN1_SPEC;
impl crate::RegisterSpec for TRX_GAIN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trx_gain1::R](R) reader structure"]
impl crate::Readable for TRX_GAIN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trx_gain1::W](W) writer structure"]
impl crate::Writable for TRX_GAIN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets trx_gain1 to value 0"]
impl crate::Resettable for TRX_GAIN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

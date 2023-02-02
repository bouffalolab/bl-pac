#[doc = "Register `rbb_gain_index1` reader"]
pub struct R(crate::R<RBB_GAIN_INDEX1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_GAIN_INDEX1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB_GAIN_INDEX1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB_GAIN_INDEX1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb_gain_index1` writer"]
pub struct W(crate::W<RBB_GAIN_INDEX1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_GAIN_INDEX1_SPEC>;
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
impl From<crate::W<RBB_GAIN_INDEX1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB_GAIN_INDEX1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gain_ctrl0_gc_rbb1` reader - "]
pub type GAIN_CTRL0_GC_RBB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl0_gc_rbb1` writer - "]
pub type GAIN_CTRL0_GC_RBB1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_INDEX1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl0_gc_rbb2` reader - "]
pub type GAIN_CTRL0_GC_RBB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl0_gc_rbb2` writer - "]
pub type GAIN_CTRL0_GC_RBB2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_INDEX1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl1_gc_rbb1` reader - "]
pub type GAIN_CTRL1_GC_RBB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl1_gc_rbb1` writer - "]
pub type GAIN_CTRL1_GC_RBB1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_INDEX1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl1_gc_rbb2` reader - "]
pub type GAIN_CTRL1_GC_RBB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl1_gc_rbb2` writer - "]
pub type GAIN_CTRL1_GC_RBB2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_INDEX1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl2_gc_rbb1` reader - "]
pub type GAIN_CTRL2_GC_RBB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl2_gc_rbb1` writer - "]
pub type GAIN_CTRL2_GC_RBB1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_INDEX1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl2_gc_rbb2` reader - "]
pub type GAIN_CTRL2_GC_RBB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl2_gc_rbb2` writer - "]
pub type GAIN_CTRL2_GC_RBB2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_INDEX1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl3_gc_rbb1` reader - "]
pub type GAIN_CTRL3_GC_RBB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl3_gc_rbb1` writer - "]
pub type GAIN_CTRL3_GC_RBB1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_INDEX1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gain_ctrl3_gc_rbb2` reader - "]
pub type GAIN_CTRL3_GC_RBB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl3_gc_rbb2` writer - "]
pub type GAIN_CTRL3_GC_RBB2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_INDEX1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rbb1(&self) -> GAIN_CTRL0_GC_RBB1_R {
        GAIN_CTRL0_GC_RBB1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rbb2(&self) -> GAIN_CTRL0_GC_RBB2_R {
        GAIN_CTRL0_GC_RBB2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rbb1(&self) -> GAIN_CTRL1_GC_RBB1_R {
        GAIN_CTRL1_GC_RBB1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rbb2(&self) -> GAIN_CTRL1_GC_RBB2_R {
        GAIN_CTRL1_GC_RBB2_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rbb1(&self) -> GAIN_CTRL2_GC_RBB1_R {
        GAIN_CTRL2_GC_RBB1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rbb2(&self) -> GAIN_CTRL2_GC_RBB2_R {
        GAIN_CTRL2_GC_RBB2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rbb1(&self) -> GAIN_CTRL3_GC_RBB1_R {
        GAIN_CTRL3_GC_RBB1_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rbb2(&self) -> GAIN_CTRL3_GC_RBB2_R {
        GAIN_CTRL3_GC_RBB2_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl0_gc_rbb1(&mut self) -> GAIN_CTRL0_GC_RBB1_W<0> {
        GAIN_CTRL0_GC_RBB1_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl0_gc_rbb2(&mut self) -> GAIN_CTRL0_GC_RBB2_W<4> {
        GAIN_CTRL0_GC_RBB2_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl1_gc_rbb1(&mut self) -> GAIN_CTRL1_GC_RBB1_W<8> {
        GAIN_CTRL1_GC_RBB1_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl1_gc_rbb2(&mut self) -> GAIN_CTRL1_GC_RBB2_W<12> {
        GAIN_CTRL1_GC_RBB2_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl2_gc_rbb1(&mut self) -> GAIN_CTRL2_GC_RBB1_W<16> {
        GAIN_CTRL2_GC_RBB1_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl2_gc_rbb2(&mut self) -> GAIN_CTRL2_GC_RBB2_W<20> {
        GAIN_CTRL2_GC_RBB2_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl3_gc_rbb1(&mut self) -> GAIN_CTRL3_GC_RBB1_W<24> {
        GAIN_CTRL3_GC_RBB1_W::new(self)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl3_gc_rbb2(&mut self) -> GAIN_CTRL3_GC_RBB2_W<28> {
        GAIN_CTRL3_GC_RBB2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb_gain_index1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_gain_index1](index.html) module"]
pub struct RBB_GAIN_INDEX1_SPEC;
impl crate::RegisterSpec for RBB_GAIN_INDEX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb_gain_index1::R](R) reader structure"]
impl crate::Readable for RBB_GAIN_INDEX1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb_gain_index1::W](W) writer structure"]
impl crate::Writable for RBB_GAIN_INDEX1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rbb_gain_index1 to value 0"]
impl crate::Resettable for RBB_GAIN_INDEX1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

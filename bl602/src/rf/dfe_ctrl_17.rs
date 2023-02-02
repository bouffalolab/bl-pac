#[doc = "Register `dfe_ctrl_17` reader"]
pub struct R(crate::R<DFE_CTRL_17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_17` writer"]
pub struct W(crate::W<DFE_CTRL_17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_17_SPEC>;
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
impl From<crate::W<DFE_CTRL_17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_tbb_ind_gc8` reader - "]
pub type RF_TBB_IND_GC8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc8` writer - "]
pub type RF_TBB_IND_GC8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_17_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc9` reader - "]
pub type RF_TBB_IND_GC9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc9` writer - "]
pub type RF_TBB_IND_GC9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_17_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc10` reader - "]
pub type RF_TBB_IND_GC10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc10` writer - "]
pub type RF_TBB_IND_GC10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_17_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc11` reader - "]
pub type RF_TBB_IND_GC11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc11` writer - "]
pub type RF_TBB_IND_GC11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_17_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc12` reader - "]
pub type RF_TBB_IND_GC12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc12` writer - "]
pub type RF_TBB_IND_GC12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_17_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc13` reader - "]
pub type RF_TBB_IND_GC13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc13` writer - "]
pub type RF_TBB_IND_GC13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_17_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc14` reader - "]
pub type RF_TBB_IND_GC14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc14` writer - "]
pub type RF_TBB_IND_GC14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_17_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_tbb_ind_gc15` reader - "]
pub type RF_TBB_IND_GC15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_tbb_ind_gc15` writer - "]
pub type RF_TBB_IND_GC15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_17_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc8(&self) -> RF_TBB_IND_GC8_R {
        RF_TBB_IND_GC8_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc9(&self) -> RF_TBB_IND_GC9_R {
        RF_TBB_IND_GC9_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc10(&self) -> RF_TBB_IND_GC10_R {
        RF_TBB_IND_GC10_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc11(&self) -> RF_TBB_IND_GC11_R {
        RF_TBB_IND_GC11_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc12(&self) -> RF_TBB_IND_GC12_R {
        RF_TBB_IND_GC12_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc13(&self) -> RF_TBB_IND_GC13_R {
        RF_TBB_IND_GC13_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc14(&self) -> RF_TBB_IND_GC14_R {
        RF_TBB_IND_GC14_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc15(&self) -> RF_TBB_IND_GC15_R {
        RF_TBB_IND_GC15_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc8(&mut self) -> RF_TBB_IND_GC8_W<0> {
        RF_TBB_IND_GC8_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc9(&mut self) -> RF_TBB_IND_GC9_W<4> {
        RF_TBB_IND_GC9_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc10(&mut self) -> RF_TBB_IND_GC10_W<8> {
        RF_TBB_IND_GC10_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc11(&mut self) -> RF_TBB_IND_GC11_W<12> {
        RF_TBB_IND_GC11_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc12(&mut self) -> RF_TBB_IND_GC12_W<16> {
        RF_TBB_IND_GC12_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc13(&mut self) -> RF_TBB_IND_GC13_W<20> {
        RF_TBB_IND_GC13_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc14(&mut self) -> RF_TBB_IND_GC14_W<24> {
        RF_TBB_IND_GC14_W::new(self)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn rf_tbb_ind_gc15(&mut self) -> RF_TBB_IND_GC15_W<28> {
        RF_TBB_IND_GC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_17.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_17](index.html) module"]
pub struct DFE_CTRL_17_SPEC;
impl crate::RegisterSpec for DFE_CTRL_17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_17::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_17::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_17_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_17 to value 0"]
impl crate::Resettable for DFE_CTRL_17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

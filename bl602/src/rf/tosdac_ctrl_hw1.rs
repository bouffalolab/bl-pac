#[doc = "Register `tosdac_ctrl_hw1` reader"]
pub struct R(crate::R<TOSDAC_CTRL_HW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOSDAC_CTRL_HW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOSDAC_CTRL_HW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOSDAC_CTRL_HW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tosdac_ctrl_hw1` writer"]
pub struct W(crate::W<TOSDAC_CTRL_HW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOSDAC_CTRL_HW1_SPEC>;
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
impl From<crate::W<TOSDAC_CTRL_HW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOSDAC_CTRL_HW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tbb_tosdac_i_gc0` reader - "]
pub type TBB_TOSDAC_I_GC0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tbb_tosdac_i_gc0` writer - "]
pub type TBB_TOSDAC_I_GC0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOSDAC_CTRL_HW1_SPEC, u8, u8, 6, O>;
#[doc = "Field `tbb_tosdac_q_gc0` reader - "]
pub type TBB_TOSDAC_Q_GC0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tbb_tosdac_q_gc0` writer - "]
pub type TBB_TOSDAC_Q_GC0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOSDAC_CTRL_HW1_SPEC, u8, u8, 6, O>;
#[doc = "Field `tbb_tosdac_i_gc1` reader - "]
pub type TBB_TOSDAC_I_GC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tbb_tosdac_i_gc1` writer - "]
pub type TBB_TOSDAC_I_GC1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOSDAC_CTRL_HW1_SPEC, u8, u8, 6, O>;
#[doc = "Field `tbb_tosdac_q_gc1` reader - "]
pub type TBB_TOSDAC_Q_GC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tbb_tosdac_q_gc1` writer - "]
pub type TBB_TOSDAC_Q_GC1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOSDAC_CTRL_HW1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc0(&self) -> TBB_TOSDAC_I_GC0_R {
        TBB_TOSDAC_I_GC0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc0(&self) -> TBB_TOSDAC_Q_GC0_R {
        TBB_TOSDAC_Q_GC0_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc1(&self) -> TBB_TOSDAC_I_GC1_R {
        TBB_TOSDAC_I_GC1_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc1(&self) -> TBB_TOSDAC_Q_GC1_R {
        TBB_TOSDAC_Q_GC1_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_tosdac_i_gc0(&mut self) -> TBB_TOSDAC_I_GC0_W<0> {
        TBB_TOSDAC_I_GC0_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_tosdac_q_gc0(&mut self) -> TBB_TOSDAC_Q_GC0_W<8> {
        TBB_TOSDAC_Q_GC0_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_tosdac_i_gc1(&mut self) -> TBB_TOSDAC_I_GC1_W<16> {
        TBB_TOSDAC_I_GC1_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn tbb_tosdac_q_gc1(&mut self) -> TBB_TOSDAC_Q_GC1_W<24> {
        TBB_TOSDAC_Q_GC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tosdac_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tosdac_ctrl_hw1](index.html) module"]
pub struct TOSDAC_CTRL_HW1_SPEC;
impl crate::RegisterSpec for TOSDAC_CTRL_HW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tosdac_ctrl_hw1::R](R) reader structure"]
impl crate::Readable for TOSDAC_CTRL_HW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tosdac_ctrl_hw1::W](W) writer structure"]
impl crate::Writable for TOSDAC_CTRL_HW1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tosdac_ctrl_hw1 to value 0"]
impl crate::Resettable for TOSDAC_CTRL_HW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

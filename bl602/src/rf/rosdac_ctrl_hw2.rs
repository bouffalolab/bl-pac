#[doc = "Register `rosdac_ctrl_hw2` reader"]
pub struct R(crate::R<ROSDAC_CTRL_HW2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROSDAC_CTRL_HW2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROSDAC_CTRL_HW2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROSDAC_CTRL_HW2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rosdac_ctrl_hw2` writer"]
pub struct W(crate::W<ROSDAC_CTRL_HW2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROSDAC_CTRL_HW2_SPEC>;
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
impl From<crate::W<ROSDAC_CTRL_HW2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROSDAC_CTRL_HW2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rosdac_i_gc2` reader - "]
pub type ROSDAC_I_GC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_i_gc2` writer - "]
pub type ROSDAC_I_GC2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROSDAC_CTRL_HW2_SPEC, u8, u8, 6, O>;
#[doc = "Field `rosdac_q_gc2` reader - "]
pub type ROSDAC_Q_GC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_q_gc2` writer - "]
pub type ROSDAC_Q_GC2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROSDAC_CTRL_HW2_SPEC, u8, u8, 6, O>;
#[doc = "Field `rosdac_i_gc3` reader - "]
pub type ROSDAC_I_GC3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_i_gc3` writer - "]
pub type ROSDAC_I_GC3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROSDAC_CTRL_HW2_SPEC, u8, u8, 6, O>;
#[doc = "Field `rosdac_q_gc3` reader - "]
pub type ROSDAC_Q_GC3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_q_gc3` writer - "]
pub type ROSDAC_Q_GC3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROSDAC_CTRL_HW2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_i_gc2(&self) -> ROSDAC_I_GC2_R {
        ROSDAC_I_GC2_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_q_gc2(&self) -> ROSDAC_Q_GC2_R {
        ROSDAC_Q_GC2_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_i_gc3(&self) -> ROSDAC_I_GC3_R {
        ROSDAC_I_GC3_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_q_gc3(&self) -> ROSDAC_Q_GC3_R {
        ROSDAC_Q_GC3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_i_gc2(&mut self) -> ROSDAC_I_GC2_W<0> {
        ROSDAC_I_GC2_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_q_gc2(&mut self) -> ROSDAC_Q_GC2_W<8> {
        ROSDAC_Q_GC2_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_i_gc3(&mut self) -> ROSDAC_I_GC3_W<16> {
        ROSDAC_I_GC3_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_q_gc3(&mut self) -> ROSDAC_Q_GC3_W<24> {
        ROSDAC_Q_GC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rosdac_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosdac_ctrl_hw2](index.html) module"]
pub struct ROSDAC_CTRL_HW2_SPEC;
impl crate::RegisterSpec for ROSDAC_CTRL_HW2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rosdac_ctrl_hw2::R](R) reader structure"]
impl crate::Readable for ROSDAC_CTRL_HW2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rosdac_ctrl_hw2::W](W) writer structure"]
impl crate::Writable for ROSDAC_CTRL_HW2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rosdac_ctrl_hw2 to value 0"]
impl crate::Resettable for ROSDAC_CTRL_HW2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

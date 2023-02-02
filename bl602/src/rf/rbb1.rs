#[doc = "Register `rbb1` reader"]
pub struct R(crate::R<RBB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb1` writer"]
pub struct W(crate::W<RBB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB1_SPEC>;
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
impl From<crate::W<RBB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rosdac_q` reader - "]
pub type ROSDAC_Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_q` writer - "]
pub type ROSDAC_Q_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB1_SPEC, u8, u8, 6, O>;
#[doc = "Field `rosdac_i` reader - "]
pub type ROSDAC_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_i` writer - "]
pub type ROSDAC_I_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB1_SPEC, u8, u8, 6, O>;
#[doc = "Field `rosdac_q_hw` reader - "]
pub type ROSDAC_Q_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_q_hw` writer - "]
pub type ROSDAC_Q_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB1_SPEC, u8, u8, 6, O>;
#[doc = "Field `rosdac_i_hw` reader - "]
pub type ROSDAC_I_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_i_hw` writer - "]
pub type ROSDAC_I_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB1_SPEC, u8, u8, 6, O>;
#[doc = "Field `rosdac_range` reader - "]
pub type ROSDAC_RANGE_R = crate::BitReader<bool>;
#[doc = "Field `rosdac_range` writer - "]
pub type ROSDAC_RANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_q(&self) -> ROSDAC_Q_R {
        ROSDAC_Q_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_i(&self) -> ROSDAC_I_R {
        ROSDAC_I_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_q_hw(&self) -> ROSDAC_Q_HW_R {
        ROSDAC_Q_HW_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_i_hw(&self) -> ROSDAC_I_HW_R {
        ROSDAC_I_HW_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rosdac_range(&self) -> ROSDAC_RANGE_R {
        ROSDAC_RANGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_q(&mut self) -> ROSDAC_Q_W<0> {
        ROSDAC_Q_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_i(&mut self) -> ROSDAC_I_W<8> {
        ROSDAC_I_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_q_hw(&mut self) -> ROSDAC_Q_HW_W<16> {
        ROSDAC_Q_HW_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_i_hw(&mut self) -> ROSDAC_I_HW_W<24> {
        ROSDAC_I_HW_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_range(&mut self) -> ROSDAC_RANGE_W<31> {
        ROSDAC_RANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb1](index.html) module"]
pub struct RBB1_SPEC;
impl crate::RegisterSpec for RBB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb1::R](R) reader structure"]
impl crate::Readable for RBB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb1::W](W) writer structure"]
impl crate::Writable for RBB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rbb1 to value 0"]
impl crate::Resettable for RBB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

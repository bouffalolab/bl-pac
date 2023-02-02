#[doc = "Register `lo_sdm_ctrl_hw8` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_SDM_CTRL_HW8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_SDM_CTRL_HW8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw8` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW8_SPEC>;
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
impl From<crate::W<LO_SDM_CTRL_HW8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_SDM_CTRL_HW8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdmin_if` reader - "]
pub type LO_SDMIN_IF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `lo_sdmin_if` writer - "]
pub type LO_SDMIN_IF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW8_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn lo_sdmin_if(&self) -> LO_SDMIN_IF_R {
        LO_SDMIN_IF_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdmin_if(&mut self) -> LO_SDMIN_IF_W<0> {
        LO_SDMIN_IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw8](index.html) module"]
pub struct LO_SDM_CTRL_HW8_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw8::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw8::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw8 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `lo_sdm_ctrl_hw6` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_SDM_CTRL_HW6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_SDM_CTRL_HW6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw6` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW6_SPEC>;
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
impl From<crate::W<LO_SDM_CTRL_HW6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_SDM_CTRL_HW6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdmin_center` reader - "]
pub type LO_SDMIN_CENTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `lo_sdmin_center` writer - "]
pub type LO_SDMIN_CENTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW6_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28"]
    #[inline(always)]
    pub fn lo_sdmin_center(&self) -> LO_SDMIN_CENTER_R {
        LO_SDMIN_CENTER_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdmin_center(&mut self) -> LO_SDMIN_CENTER_W<0> {
        LO_SDMIN_CENTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw6](index.html) module"]
pub struct LO_SDM_CTRL_HW6_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw6::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw6::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw6 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

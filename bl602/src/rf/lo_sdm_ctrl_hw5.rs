#[doc = "Register `lo_sdm_ctrl_hw5` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_SDM_CTRL_HW5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_SDM_CTRL_HW5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw5` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW5_SPEC>;
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
impl From<crate::W<LO_SDM_CTRL_HW5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_SDM_CTRL_HW5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_center_freq_mhz` reader - "]
pub type LO_CENTER_FREQ_MHZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `lo_center_freq_mhz` writer - "]
pub type LO_CENTER_FREQ_MHZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW5_SPEC, u16, u16, 12, O>;
#[doc = "Field `lo_sdm_bypass_mode` reader - "]
pub type LO_SDM_BYPASS_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_bypass_mode` writer - "]
pub type LO_SDM_BYPASS_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW5_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lo_center_freq_mhz(&self) -> LO_CENTER_FREQ_MHZ_R {
        LO_CENTER_FREQ_MHZ_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn lo_sdm_bypass_mode(&self) -> LO_SDM_BYPASS_MODE_R {
        LO_SDM_BYPASS_MODE_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn lo_center_freq_mhz(&mut self) -> LO_CENTER_FREQ_MHZ_W<0> {
        LO_CENTER_FREQ_MHZ_W::new(self)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_bypass_mode(&mut self) -> LO_SDM_BYPASS_MODE_W<12> {
        LO_SDM_BYPASS_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw5](index.html) module"]
pub struct LO_SDM_CTRL_HW5_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw5::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw5::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw5 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

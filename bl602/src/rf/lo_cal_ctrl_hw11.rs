#[doc = "Register `lo_cal_ctrl_hw11` reader"]
pub struct R(crate::R<LO_CAL_CTRL_HW11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_CAL_CTRL_HW11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_CAL_CTRL_HW11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_CAL_CTRL_HW11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_cal_ctrl_hw11` writer"]
pub struct W(crate::W<LO_CAL_CTRL_HW11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_CAL_CTRL_HW11_SPEC>;
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
impl From<crate::W<LO_CAL_CTRL_HW11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_CAL_CTRL_HW11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_vco_idac_cw_2484` reader - "]
pub type LO_VCO_IDAC_CW_2484_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_vco_idac_cw_2484` writer - "]
pub type LO_VCO_IDAC_CW_2484_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_CAL_CTRL_HW11_SPEC, u8, u8, 5, O>;
#[doc = "Field `lo_vco_freq_cw_2484` reader - "]
pub type LO_VCO_FREQ_CW_2484_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_vco_freq_cw_2484` writer - "]
pub type LO_VCO_FREQ_CW_2484_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_CAL_CTRL_HW11_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2484(&self) -> LO_VCO_IDAC_CW_2484_R {
        LO_VCO_IDAC_CW_2484_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2484(&self) -> LO_VCO_FREQ_CW_2484_R {
        LO_VCO_FREQ_CW_2484_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn lo_vco_idac_cw_2484(&mut self) -> LO_VCO_IDAC_CW_2484_W<0> {
        LO_VCO_IDAC_CW_2484_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn lo_vco_freq_cw_2484(&mut self) -> LO_VCO_FREQ_CW_2484_W<8> {
        LO_VCO_FREQ_CW_2484_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_cal_ctrl_hw11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw11](index.html) module"]
pub struct LO_CAL_CTRL_HW11_SPEC;
impl crate::RegisterSpec for LO_CAL_CTRL_HW11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_cal_ctrl_hw11::R](R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw11::W](W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw11 to value 0"]
impl crate::Resettable for LO_CAL_CTRL_HW11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

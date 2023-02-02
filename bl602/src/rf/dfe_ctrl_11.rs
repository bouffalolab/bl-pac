#[doc = "Register `dfe_ctrl_11` reader"]
pub struct R(crate::R<DFE_CTRL_11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_11` writer"]
pub struct W(crate::W<DFE_CTRL_11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_11_SPEC>;
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
impl From<crate::W<DFE_CTRL_11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dfe_adc_raw_i` reader - "]
pub type DFE_ADC_RAW_I_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dfe_adc_raw_i` writer - "]
pub type DFE_ADC_RAW_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_11_SPEC, u16, u16, 10, O>;
#[doc = "Field `dfe_adc_raw_q` reader - "]
pub type DFE_ADC_RAW_Q_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dfe_adc_raw_q` writer - "]
pub type DFE_ADC_RAW_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_11_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn dfe_adc_raw_i(&self) -> DFE_ADC_RAW_I_R {
        DFE_ADC_RAW_I_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn dfe_adc_raw_q(&self) -> DFE_ADC_RAW_Q_R {
        DFE_ADC_RAW_Q_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn dfe_adc_raw_i(&mut self) -> DFE_ADC_RAW_I_W<0> {
        DFE_ADC_RAW_I_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn dfe_adc_raw_q(&mut self) -> DFE_ADC_RAW_Q_W<16> {
        DFE_ADC_RAW_Q_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_11](index.html) module"]
pub struct DFE_CTRL_11_SPEC;
impl crate::RegisterSpec for DFE_CTRL_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_11::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_11::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_11 to value 0"]
impl crate::Resettable for DFE_CTRL_11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

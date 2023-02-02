#[doc = "Register `dfe_ctrl_3` reader"]
pub struct R(crate::R<DFE_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_3` writer"]
pub struct W(crate::W<DFE_CTRL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_3_SPEC>;
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
impl From<crate::W<DFE_CTRL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_adc_4s_i_val` reader - "]
pub type RX_ADC_4S_I_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_adc_4s_i_val` writer - "]
pub type RX_ADC_4S_I_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_3_SPEC, u16, u16, 10, O>;
#[doc = "Field `rx_adc_4s_i_en` reader - "]
pub type RX_ADC_4S_I_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_adc_4s_i_en` writer - "]
pub type RX_ADC_4S_I_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_3_SPEC, bool, O>;
#[doc = "Field `rx_adc_4s_q_val` reader - "]
pub type RX_ADC_4S_Q_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_adc_4s_q_val` writer - "]
pub type RX_ADC_4S_Q_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_3_SPEC, u16, u16, 10, O>;
#[doc = "Field `rx_adc_4s_q_en` reader - "]
pub type RX_ADC_4S_Q_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_adc_4s_q_en` writer - "]
pub type RX_ADC_4S_Q_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_4s_i_val(&self) -> RX_ADC_4S_I_VAL_R {
        RX_ADC_4S_I_VAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_adc_4s_i_en(&self) -> RX_ADC_4S_I_EN_R {
        RX_ADC_4S_I_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_4s_q_val(&self) -> RX_ADC_4S_Q_VAL_R {
        RX_ADC_4S_Q_VAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_adc_4s_q_en(&self) -> RX_ADC_4S_Q_EN_R {
        RX_ADC_4S_Q_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_adc_4s_i_val(&mut self) -> RX_ADC_4S_I_VAL_W<0> {
        RX_ADC_4S_I_VAL_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn rx_adc_4s_i_en(&mut self) -> RX_ADC_4S_I_EN_W<10> {
        RX_ADC_4S_I_EN_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn rx_adc_4s_q_val(&mut self) -> RX_ADC_4S_Q_VAL_W<16> {
        RX_ADC_4S_Q_VAL_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn rx_adc_4s_q_en(&mut self) -> RX_ADC_4S_Q_EN_W<26> {
        RX_ADC_4S_Q_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_3](index.html) module"]
pub struct DFE_CTRL_3_SPEC;
impl crate::RegisterSpec for DFE_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_3::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_3::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_3 to value 0"]
impl crate::Resettable for DFE_CTRL_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

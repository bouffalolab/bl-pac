#[doc = "Register `dfe_ctrl_2` reader"]
pub struct R(crate::R<DFE_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_2` writer"]
pub struct W(crate::W<DFE_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_2_SPEC>;
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
impl From<crate::W<DFE_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_adc_os_i` reader - "]
pub type RX_ADC_OS_I_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_adc_os_i` writer - "]
pub type RX_ADC_OS_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_2_SPEC, u16, u16, 10, O>;
#[doc = "Field `rx_adc_os_q` reader - "]
pub type RX_ADC_OS_Q_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_adc_os_q` writer - "]
pub type RX_ADC_OS_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_2_SPEC, u16, u16, 10, O>;
#[doc = "Field `rx_adc_dce_flt_en` reader - "]
pub type RX_ADC_DCE_FLT_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_adc_dce_flt_en` writer - "]
pub type RX_ADC_DCE_FLT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `rx_adc_low_pow_en` reader - "]
pub type RX_ADC_LOW_POW_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_adc_low_pow_en` writer - "]
pub type RX_ADC_LOW_POW_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `rx_adc_dat_format` reader - "]
pub type RX_ADC_DAT_FORMAT_R = crate::BitReader<bool>;
#[doc = "Field `rx_adc_dat_format` writer - "]
pub type RX_ADC_DAT_FORMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `rx_adc_iq_swap` reader - "]
pub type RX_ADC_IQ_SWAP_R = crate::BitReader<bool>;
#[doc = "Field `rx_adc_iq_swap` writer - "]
pub type RX_ADC_IQ_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_os_i(&self) -> RX_ADC_OS_I_R {
        RX_ADC_OS_I_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_os_q(&self) -> RX_ADC_OS_Q_R {
        RX_ADC_OS_Q_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_adc_dce_flt_en(&self) -> RX_ADC_DCE_FLT_EN_R {
        RX_ADC_DCE_FLT_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_adc_low_pow_en(&self) -> RX_ADC_LOW_POW_EN_R {
        RX_ADC_LOW_POW_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_adc_dat_format(&self) -> RX_ADC_DAT_FORMAT_R {
        RX_ADC_DAT_FORMAT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_adc_iq_swap(&self) -> RX_ADC_IQ_SWAP_R {
        RX_ADC_IQ_SWAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_adc_os_i(&mut self) -> RX_ADC_OS_I_W<0> {
        RX_ADC_OS_I_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn rx_adc_os_q(&mut self) -> RX_ADC_OS_Q_W<16> {
        RX_ADC_OS_Q_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn rx_adc_dce_flt_en(&mut self) -> RX_ADC_DCE_FLT_EN_W<28> {
        RX_ADC_DCE_FLT_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rx_adc_low_pow_en(&mut self) -> RX_ADC_LOW_POW_EN_W<29> {
        RX_ADC_LOW_POW_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rx_adc_dat_format(&mut self) -> RX_ADC_DAT_FORMAT_W<30> {
        RX_ADC_DAT_FORMAT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rx_adc_iq_swap(&mut self) -> RX_ADC_IQ_SWAP_W<31> {
        RX_ADC_IQ_SWAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_2](index.html) module"]
pub struct DFE_CTRL_2_SPEC;
impl crate::RegisterSpec for DFE_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_2::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_2::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_2 to value 0"]
impl crate::Resettable for DFE_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

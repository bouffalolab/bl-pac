#[doc = "Register `rf_sram_ctrl1` reader"]
pub struct R(crate::R<RF_SRAM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_SRAM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_SRAM_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_SRAM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_sram_ctrl1` writer"]
pub struct W(crate::W<RF_SRAM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_SRAM_CTRL1_SPEC>;
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
impl From<crate::W<RF_SRAM_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_SRAM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_sram_adc_done` reader - "]
pub type RF_SRAM_ADC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `rf_sram_adc_done` writer - "]
pub type RF_SRAM_ADC_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_SRAM_CTRL1_SPEC, bool, O>;
#[doc = "Field `rf_sram_adc_en` reader - "]
pub type RF_SRAM_ADC_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_sram_adc_en` writer - "]
pub type RF_SRAM_ADC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_SRAM_CTRL1_SPEC, bool, O>;
#[doc = "Field `rf_sram_adc_loop_en` reader - "]
pub type RF_SRAM_ADC_LOOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_sram_adc_loop_en` writer - "]
pub type RF_SRAM_ADC_LOOP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_SRAM_CTRL1_SPEC, bool, O>;
#[doc = "Field `rf_sram_adc_sts_clr` reader - "]
pub type RF_SRAM_ADC_STS_CLR_R = crate::BitReader<bool>;
#[doc = "Field `rf_sram_adc_sts_clr` writer - "]
pub type RF_SRAM_ADC_STS_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_SRAM_CTRL1_SPEC, bool, O>;
#[doc = "Field `rf_sram_adc_done_cnt` reader - "]
pub type RF_SRAM_ADC_DONE_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_sram_adc_done_cnt` writer - "]
pub type RF_SRAM_ADC_DONE_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SRAM_CTRL1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_sram_adc_done(&self) -> RF_SRAM_ADC_DONE_R {
        RF_SRAM_ADC_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_sram_adc_en(&self) -> RF_SRAM_ADC_EN_R {
        RF_SRAM_ADC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_sram_adc_loop_en(&self) -> RF_SRAM_ADC_LOOP_EN_R {
        RF_SRAM_ADC_LOOP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_sram_adc_sts_clr(&self) -> RF_SRAM_ADC_STS_CLR_R {
        RF_SRAM_ADC_STS_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_done_cnt(&self) -> RF_SRAM_ADC_DONE_CNT_R {
        RF_SRAM_ADC_DONE_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rf_sram_adc_done(&mut self) -> RF_SRAM_ADC_DONE_W<0> {
        RF_SRAM_ADC_DONE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rf_sram_adc_en(&mut self) -> RF_SRAM_ADC_EN_W<1> {
        RF_SRAM_ADC_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rf_sram_adc_loop_en(&mut self) -> RF_SRAM_ADC_LOOP_EN_W<2> {
        RF_SRAM_ADC_LOOP_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rf_sram_adc_sts_clr(&mut self) -> RF_SRAM_ADC_STS_CLR_W<3> {
        RF_SRAM_ADC_STS_CLR_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn rf_sram_adc_done_cnt(&mut self) -> RF_SRAM_ADC_DONE_CNT_W<16> {
        RF_SRAM_ADC_DONE_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_sram_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl1](index.html) module"]
pub struct RF_SRAM_CTRL1_SPEC;
impl crate::RegisterSpec for RF_SRAM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_sram_ctrl1::R](R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl1::W](W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_sram_ctrl1 to value 0"]
impl crate::Resettable for RF_SRAM_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

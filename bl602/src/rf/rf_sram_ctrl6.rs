#[doc = "Register `rf_sram_ctrl6` reader"]
pub struct R(crate::R<RF_SRAM_CTRL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_SRAM_CTRL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_SRAM_CTRL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_SRAM_CTRL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_sram_ctrl6` writer"]
pub struct W(crate::W<RF_SRAM_CTRL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_SRAM_CTRL6_SPEC>;
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
impl From<crate::W<RF_SRAM_CTRL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_SRAM_CTRL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_sram_dac_sts` reader - "]
pub type RF_SRAM_DAC_STS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `rf_sram_dac_sts` writer - "]
pub type RF_SRAM_DAC_STS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SRAM_CTRL6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_sram_dac_sts(&self) -> RF_SRAM_DAC_STS_R {
        RF_SRAM_DAC_STS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn rf_sram_dac_sts(&mut self) -> RF_SRAM_DAC_STS_W<0> {
        RF_SRAM_DAC_STS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_sram_ctrl6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl6](index.html) module"]
pub struct RF_SRAM_CTRL6_SPEC;
impl crate::RegisterSpec for RF_SRAM_CTRL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_sram_ctrl6::R](R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl6::W](W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_sram_ctrl6 to value 0"]
impl crate::Resettable for RF_SRAM_CTRL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

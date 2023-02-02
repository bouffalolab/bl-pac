#[doc = "Register `gpadc_config` reader"]
pub struct R(crate::R<GPADC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_config` writer"]
pub struct W(crate::W<GPADC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_CONFIG_SPEC>;
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
impl From<crate::W<GPADC_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic Analog-to-Digital Converter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_config](index.html) module"]
pub struct GPADC_CONFIG_SPEC;
impl crate::RegisterSpec for GPADC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_config::R](R) reader structure"]
impl crate::Readable for GPADC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_config::W](W) writer structure"]
impl crate::Writable for GPADC_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_config to value 0"]
impl crate::Resettable for GPADC_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

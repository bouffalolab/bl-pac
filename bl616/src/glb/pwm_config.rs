#[doc = "Register `pwm_config` reader"]
pub struct R(crate::R<PWM_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm_config` writer"]
pub struct W(crate::W<PWM_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CONFIG_SPEC>;
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
impl From<crate::W<PWM_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CONFIG_SPEC>) -> Self {
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
#[doc = "Pulse-Width configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_config](index.html) module"]
pub struct PWM_CONFIG_SPEC;
impl crate::RegisterSpec for PWM_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_config::R](R) reader structure"]
impl crate::Readable for PWM_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_config::W](W) writer structure"]
impl crate::Writable for PWM_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pwm_config to value 0"]
impl crate::Resettable for PWM_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

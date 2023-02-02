#[doc = "Register `pwm_int_config` reader"]
pub struct R(crate::R<PWM_INT_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_INT_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_INT_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_INT_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm_int_config` writer"]
pub struct W(crate::W<PWM_INT_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_INT_CONFIG_SPEC>;
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
impl From<crate::W<PWM_INT_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_INT_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_interrupt_sts` reader - "]
pub type PWM_INTERRUPT_STS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwm_int_clear` writer - "]
pub type PWM_INT_CLEAR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWM_INT_CONFIG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pwm_interrupt_sts(&self) -> PWM_INTERRUPT_STS_R {
        PWM_INTERRUPT_STS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_int_clear(&mut self) -> PWM_INT_CLEAR_W<8> {
        PWM_INT_CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm_int_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_int_config](index.html) module"]
pub struct PWM_INT_CONFIG_SPEC;
impl crate::RegisterSpec for PWM_INT_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_int_config::R](R) reader structure"]
impl crate::Readable for PWM_INT_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_int_config::W](W) writer structure"]
impl crate::Writable for PWM_INT_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pwm_int_config to value 0"]
impl crate::Resettable for PWM_INT_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

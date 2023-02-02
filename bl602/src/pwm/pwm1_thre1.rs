#[doc = "Register `pwm1_thre1` reader"]
pub struct R(crate::R<PWM1_THRE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM1_THRE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM1_THRE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM1_THRE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm1_thre1` writer"]
pub struct W(crate::W<PWM1_THRE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM1_THRE1_SPEC>;
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
impl From<crate::W<PWM1_THRE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM1_THRE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_thre1` reader - "]
pub type PWM_THRE1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwm_thre1` writer - "]
pub type PWM_THRE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWM1_THRE1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_thre1(&self) -> PWM_THRE1_R {
        PWM_THRE1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_thre1(&mut self) -> PWM_THRE1_W<0> {
        PWM_THRE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm1_thre1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm1_thre1](index.html) module"]
pub struct PWM1_THRE1_SPEC;
impl crate::RegisterSpec for PWM1_THRE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm1_thre1::R](R) reader structure"]
impl crate::Readable for PWM1_THRE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm1_thre1::W](W) writer structure"]
impl crate::Writable for PWM1_THRE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pwm1_thre1 to value 0"]
impl crate::Resettable for PWM1_THRE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

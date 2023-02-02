#[doc = "Register `pwm3_interrupt` reader"]
pub struct R(crate::R<PWM3_INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM3_INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM3_INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM3_INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm3_interrupt` writer"]
pub struct W(crate::W<PWM3_INTERRUPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM3_INTERRUPT_SPEC>;
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
impl From<crate::W<PWM3_INTERRUPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM3_INTERRUPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_int_period_cnt` reader - "]
pub type PWM_INT_PERIOD_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwm_int_period_cnt` writer - "]
pub type PWM_INT_PERIOD_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWM3_INTERRUPT_SPEC, u16, u16, 16, O>;
#[doc = "Field `pwm_int_enable` reader - "]
pub type PWM_INT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `pwm_int_enable` writer - "]
pub type PWM_INT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWM3_INTERRUPT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_int_period_cnt(&self) -> PWM_INT_PERIOD_CNT_R {
        PWM_INT_PERIOD_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pwm_int_enable(&self) -> PWM_INT_ENABLE_R {
        PWM_INT_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_int_period_cnt(&mut self) -> PWM_INT_PERIOD_CNT_W<0> {
        PWM_INT_PERIOD_CNT_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_int_enable(&mut self) -> PWM_INT_ENABLE_W<16> {
        PWM_INT_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm3_interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm3_interrupt](index.html) module"]
pub struct PWM3_INTERRUPT_SPEC;
impl crate::RegisterSpec for PWM3_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm3_interrupt::R](R) reader structure"]
impl crate::Readable for PWM3_INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm3_interrupt::W](W) writer structure"]
impl crate::Writable for PWM3_INTERRUPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pwm3_interrupt to value 0"]
impl crate::Resettable for PWM3_INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

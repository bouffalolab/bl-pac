#[doc = "Register `pwm4_clkdiv` reader"]
pub struct R(crate::R<PWM4_CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM4_CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM4_CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM4_CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm4_clkdiv` writer"]
pub struct W(crate::W<PWM4_CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM4_CLKDIV_SPEC>;
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
impl From<crate::W<PWM4_CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM4_CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_clk_div` reader - "]
pub type PWM_CLK_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwm_clk_div` writer - "]
pub type PWM_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWM4_CLKDIV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_clk_div(&self) -> PWM_CLK_DIV_R {
        PWM_CLK_DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_clk_div(&mut self) -> PWM_CLK_DIV_W<0> {
        PWM_CLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm4_clkdiv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm4_clkdiv](index.html) module"]
pub struct PWM4_CLKDIV_SPEC;
impl crate::RegisterSpec for PWM4_CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm4_clkdiv::R](R) reader structure"]
impl crate::Readable for PWM4_CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm4_clkdiv::W](W) writer structure"]
impl crate::Writable for PWM4_CLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pwm4_clkdiv to value 0"]
impl crate::Resettable for PWM4_CLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

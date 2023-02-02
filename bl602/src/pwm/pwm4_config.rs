#[doc = "Register `pwm4_config` reader"]
pub struct R(crate::R<PWM4_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM4_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM4_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM4_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm4_config` writer"]
pub struct W(crate::W<PWM4_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM4_CONFIG_SPEC>;
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
impl From<crate::W<PWM4_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM4_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_clk_sel` reader - "]
pub type REG_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_clk_sel` writer - "]
pub type REG_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWM4_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `pwm_out_inv` reader - "]
pub type PWM_OUT_INV_R = crate::BitReader<bool>;
#[doc = "Field `pwm_out_inv` writer - "]
pub type PWM_OUT_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWM4_CONFIG_SPEC, bool, O>;
#[doc = "Field `pwm_stop_mode` reader - "]
pub type PWM_STOP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `pwm_stop_mode` writer - "]
pub type PWM_STOP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWM4_CONFIG_SPEC, bool, O>;
#[doc = "Field `pwm_sw_force_val` reader - "]
pub type PWM_SW_FORCE_VAL_R = crate::BitReader<bool>;
#[doc = "Field `pwm_sw_force_val` writer - "]
pub type PWM_SW_FORCE_VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWM4_CONFIG_SPEC, bool, O>;
#[doc = "Field `pwm_sw_mode` reader - "]
pub type PWM_SW_MODE_R = crate::BitReader<bool>;
#[doc = "Field `pwm_sw_mode` writer - "]
pub type PWM_SW_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWM4_CONFIG_SPEC, bool, O>;
#[doc = "Field `pwm_stop_en` reader - "]
pub type PWM_STOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `pwm_stop_en` writer - "]
pub type PWM_STOP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWM4_CONFIG_SPEC, bool, O>;
#[doc = "Field `pwm_sts_top` reader - "]
pub type PWM_STS_TOP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_clk_sel(&self) -> REG_CLK_SEL_R {
        REG_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm_out_inv(&self) -> PWM_OUT_INV_R {
        PWM_OUT_INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm_stop_mode(&self) -> PWM_STOP_MODE_R {
        PWM_STOP_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pwm_sw_force_val(&self) -> PWM_SW_FORCE_VAL_R {
        PWM_SW_FORCE_VAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm_sw_mode(&self) -> PWM_SW_MODE_R {
        PWM_SW_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwm_stop_en(&self) -> PWM_STOP_EN_R {
        PWM_STOP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwm_sts_top(&self) -> PWM_STS_TOP_R {
        PWM_STS_TOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clk_sel(&mut self) -> REG_CLK_SEL_W<0> {
        REG_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_out_inv(&mut self) -> PWM_OUT_INV_W<2> {
        PWM_OUT_INV_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_stop_mode(&mut self) -> PWM_STOP_MODE_W<3> {
        PWM_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_sw_force_val(&mut self) -> PWM_SW_FORCE_VAL_W<4> {
        PWM_SW_FORCE_VAL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_sw_mode(&mut self) -> PWM_SW_MODE_W<5> {
        PWM_SW_MODE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_stop_en(&mut self) -> PWM_STOP_EN_W<6> {
        PWM_STOP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm4_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm4_config](index.html) module"]
pub struct PWM4_CONFIG_SPEC;
impl crate::RegisterSpec for PWM4_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm4_config::R](R) reader structure"]
impl crate::Readable for PWM4_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm4_config::W](W) writer structure"]
impl crate::Writable for PWM4_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pwm4_config to value 0x08"]
impl crate::Resettable for PWM4_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}

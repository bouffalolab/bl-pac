#[doc = "Register `cpu_clk_gate` reader"]
pub struct R(crate::R<CPU_CLK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CLK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CLK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CLK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_clk_gate` writer"]
pub struct W(crate::W<CPU_CLK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CLK_GATE_SPEC>;
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
impl From<crate::W<CPU_CLK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CLK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `force_e21_clock_on_0` reader - "]
pub type FORCE_E21_CLOCK_ON_0_R = crate::BitReader<bool>;
#[doc = "Field `force_e21_clock_on_0` writer - "]
pub type FORCE_E21_CLOCK_ON_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_CLK_GATE_SPEC, bool, O>;
#[doc = "Field `force_e21_clock_on_1` reader - "]
pub type FORCE_E21_CLOCK_ON_1_R = crate::BitReader<bool>;
#[doc = "Field `force_e21_clock_on_1` writer - "]
pub type FORCE_E21_CLOCK_ON_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_CLK_GATE_SPEC, bool, O>;
#[doc = "Field `force_e21_clock_on_2` reader - "]
pub type FORCE_E21_CLOCK_ON_2_R = crate::BitReader<bool>;
#[doc = "Field `force_e21_clock_on_2` writer - "]
pub type FORCE_E21_CLOCK_ON_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_CLK_GATE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_e21_clock_on_0(&self) -> FORCE_E21_CLOCK_ON_0_R {
        FORCE_E21_CLOCK_ON_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn force_e21_clock_on_1(&self) -> FORCE_E21_CLOCK_ON_1_R {
        FORCE_E21_CLOCK_ON_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn force_e21_clock_on_2(&self) -> FORCE_E21_CLOCK_ON_2_R {
        FORCE_E21_CLOCK_ON_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn force_e21_clock_on_0(&mut self) -> FORCE_E21_CLOCK_ON_0_W<0> {
        FORCE_E21_CLOCK_ON_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn force_e21_clock_on_1(&mut self) -> FORCE_E21_CLOCK_ON_1_W<1> {
        FORCE_E21_CLOCK_ON_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn force_e21_clock_on_2(&mut self) -> FORCE_E21_CLOCK_ON_2_W<2> {
        FORCE_E21_CLOCK_ON_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_clk_gate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_clk_gate](index.html) module"]
pub struct CPU_CLK_GATE_SPEC;
impl crate::RegisterSpec for CPU_CLK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_clk_gate::R](R) reader structure"]
impl crate::Readable for CPU_CLK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_clk_gate::W](W) writer structure"]
impl crate::Writable for CPU_CLK_GATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_clk_gate to value 0"]
impl crate::Resettable for CPU_CLK_GATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

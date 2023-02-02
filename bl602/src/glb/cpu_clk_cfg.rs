#[doc = "Register `CPU_CLK_CFG` reader"]
pub struct R(crate::R<CPU_CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_CLK_CFG` writer"]
pub struct W(crate::W<CPU_CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CLK_CFG_SPEC>;
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
impl From<crate::W<CPU_CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpu_rtc_div` reader - "]
pub type CPU_RTC_DIV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `cpu_rtc_div` writer - "]
pub type CPU_RTC_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_CLK_CFG_SPEC, u32, u32, 17, O>;
#[doc = "Field `cpu_rtc_en` reader - "]
pub type CPU_RTC_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpu_rtc_en` writer - "]
pub type CPU_RTC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CLK_CFG_SPEC, bool, O>;
#[doc = "Field `cpu_rtc_sel` reader - "]
pub type CPU_RTC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `cpu_rtc_sel` writer - "]
pub type CPU_RTC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CLK_CFG_SPEC, bool, O>;
#[doc = "Field `debug_ndreset_gate` reader - "]
pub type DEBUG_NDRESET_GATE_R = crate::BitReader<bool>;
#[doc = "Field `debug_ndreset_gate` writer - "]
pub type DEBUG_NDRESET_GATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_CLK_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn cpu_rtc_div(&self) -> CPU_RTC_DIV_R {
        CPU_RTC_DIV_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cpu_rtc_en(&self) -> CPU_RTC_EN_R {
        CPU_RTC_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cpu_rtc_sel(&self) -> CPU_RTC_SEL_R {
        CPU_RTC_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&self) -> DEBUG_NDRESET_GATE_R {
        DEBUG_NDRESET_GATE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_rtc_div(&mut self) -> CPU_RTC_DIV_W<0> {
        CPU_RTC_DIV_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_rtc_en(&mut self) -> CPU_RTC_EN_W<18> {
        CPU_RTC_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_rtc_sel(&mut self) -> CPU_RTC_SEL_W<19> {
        CPU_RTC_SEL_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn debug_ndreset_gate(&mut self) -> DEBUG_NDRESET_GATE_W<20> {
        DEBUG_NDRESET_GATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU_CLK_CFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_clk_cfg](index.html) module"]
pub struct CPU_CLK_CFG_SPEC;
impl crate::RegisterSpec for CPU_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_clk_cfg::R](R) reader structure"]
impl crate::Readable for CPU_CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_clk_cfg::W](W) writer structure"]
impl crate::Writable for CPU_CLK_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_CLK_CFG to value 0x0008_0010"]
impl crate::Resettable for CPU_CLK_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0010;
}

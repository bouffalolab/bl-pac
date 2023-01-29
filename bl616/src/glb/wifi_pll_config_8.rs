#[doc = "Register `wifi_pll_config_8` reader"]
pub struct R(crate::R<WIFI_PLL_CONFIG_8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CONFIG_8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CONFIG_8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CONFIG_8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_config_8` writer"]
pub struct W(crate::W<WIFI_PLL_CONFIG_8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CONFIG_8_SPEC>;
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
impl From<crate::W<WIFI_PLL_CONFIG_8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CONFIG_8_SPEC>) -> Self {
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_config_8](index.html) module"]
pub struct WIFI_PLL_CONFIG_8_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_config_8::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_config_8::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_8 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

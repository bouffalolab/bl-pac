#[doc = "Register `clock_config_0` reader"]
pub struct R(crate::R<CLOCK_CONFIG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CONFIG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_CONFIG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_CONFIG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clock_config_0` writer"]
pub struct W(crate::W<CLOCK_CONFIG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CONFIG_0_SPEC>;
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
impl From<crate::W<CLOCK_CONFIG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_CONFIG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pll` reader - Enable or disable Phase-Locked Loop"]
pub type PLL_R = crate::BitReader<bool>;
#[doc = "Field `pll` writer - Enable or disable Phase-Locked Loop"]
pub type PLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `fclk` reader - Enable or disable fast clock"]
pub type FCLK_R = crate::BitReader<bool>;
#[doc = "Field `fclk` writer - Enable or disable fast clock"]
pub type FCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `hclk` reader - Enable or disable hibernate clock"]
pub type HCLK_R = crate::BitReader<bool>;
#[doc = "Field `hclk` writer - Enable or disable hibernate clock"]
pub type HCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `bclk` reader - Enable or disable bus clock"]
pub type BCLK_R = crate::BitReader<bool>;
#[doc = "Field `bclk` writer - Enable or disable bus clock"]
pub type BCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `root_clk_source` reader - Set source of root clock"]
pub type ROOT_CLK_SOURCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `root_clk_source` writer - Set source of root clock"]
pub type ROOT_CLK_SOURCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLOCK_CONFIG_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `hclk_divide` reader - Set divide factor of hibernate clock"]
pub type HCLK_DIVIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hclk_divide` writer - Set divide factor of hibernate clock"]
pub type HCLK_DIVIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLOCK_CONFIG_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `bclk_divide` reader - Set divide factor of bus clock"]
pub type BCLK_DIVIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bclk_divide` writer - Set divide factor of bus clock"]
pub type BCLK_DIVIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLOCK_CONFIG_0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enable or disable Phase-Locked Loop"]
    #[inline(always)]
    pub fn pll(&self) -> PLL_R {
        PLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable fast clock"]
    #[inline(always)]
    pub fn fclk(&self) -> FCLK_R {
        FCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable hibernate clock"]
    #[inline(always)]
    pub fn hclk(&self) -> HCLK_R {
        HCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable bus clock"]
    #[inline(always)]
    pub fn bclk(&self) -> BCLK_R {
        BCLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Set source of root clock"]
    #[inline(always)]
    pub fn root_clk_source(&self) -> ROOT_CLK_SOURCE_R {
        ROOT_CLK_SOURCE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Set divide factor of hibernate clock"]
    #[inline(always)]
    pub fn hclk_divide(&self) -> HCLK_DIVIDE_R {
        HCLK_DIVIDE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Set divide factor of bus clock"]
    #[inline(always)]
    pub fn bclk_divide(&self) -> BCLK_DIVIDE_R {
        BCLK_DIVIDE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable Phase-Locked Loop"]
    #[inline(always)]
    #[must_use]
    pub fn pll(&mut self) -> PLL_W<0> {
        PLL_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable fast clock"]
    #[inline(always)]
    #[must_use]
    pub fn fclk(&mut self) -> FCLK_W<1> {
        FCLK_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable hibernate clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk(&mut self) -> HCLK_W<2> {
        HCLK_W::new(self)
    }
    #[doc = "Bit 3 - Enable or disable bus clock"]
    #[inline(always)]
    #[must_use]
    pub fn bclk(&mut self) -> BCLK_W<3> {
        BCLK_W::new(self)
    }
    #[doc = "Bits 6:7 - Set source of root clock"]
    #[inline(always)]
    #[must_use]
    pub fn root_clk_source(&mut self) -> ROOT_CLK_SOURCE_W<6> {
        ROOT_CLK_SOURCE_W::new(self)
    }
    #[doc = "Bits 8:15 - Set divide factor of hibernate clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_divide(&mut self) -> HCLK_DIVIDE_W<8> {
        HCLK_DIVIDE_W::new(self)
    }
    #[doc = "Bits 16:23 - Set divide factor of bus clock"]
    #[inline(always)]
    #[must_use]
    pub fn bclk_divide(&mut self) -> BCLK_DIVIDE_W<16> {
        BCLK_DIVIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System clock configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_config_0](index.html) module"]
pub struct CLOCK_CONFIG_0_SPEC;
impl crate::RegisterSpec for CLOCK_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_config_0::R](R) reader structure"]
impl crate::Readable for CLOCK_CONFIG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_config_0::W](W) writer structure"]
impl crate::Writable for CLOCK_CONFIG_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clock_config_0 to value 0"]
impl crate::Resettable for CLOCK_CONFIG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

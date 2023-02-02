#[doc = "Register `clkpll_fbdv` reader"]
pub struct R(crate::R<CLKPLL_FBDV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_FBDV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPLL_FBDV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPLL_FBDV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_fbdv` writer"]
pub struct W(crate::W<CLKPLL_FBDV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_FBDV_SPEC>;
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
impl From<crate::W<CLKPLL_FBDV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPLL_FBDV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_sel_sample_clk` reader - "]
pub type CLKPLL_SEL_SAMPLE_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_sel_sample_clk` writer - "]
pub type CLKPLL_SEL_SAMPLE_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_FBDV_SPEC, u8, u8, 2, O>;
#[doc = "Field `clkpll_sel_fb_clk` reader - "]
pub type CLKPLL_SEL_FB_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_sel_fb_clk` writer - "]
pub type CLKPLL_SEL_FB_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_FBDV_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clkpll_sel_sample_clk(&self) -> CLKPLL_SEL_SAMPLE_CLK_R {
        CLKPLL_SEL_SAMPLE_CLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn clkpll_sel_fb_clk(&self) -> CLKPLL_SEL_FB_CLK_R {
        CLKPLL_SEL_FB_CLK_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_sel_sample_clk(&mut self) -> CLKPLL_SEL_SAMPLE_CLK_W<0> {
        CLKPLL_SEL_SAMPLE_CLK_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_sel_fb_clk(&mut self) -> CLKPLL_SEL_FB_CLK_W<2> {
        CLKPLL_SEL_FB_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_fbdv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_fbdv](index.html) module"]
pub struct CLKPLL_FBDV_SPEC;
impl crate::RegisterSpec for CLKPLL_FBDV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_fbdv::R](R) reader structure"]
impl crate::Readable for CLKPLL_FBDV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_fbdv::W](W) writer structure"]
impl crate::Writable for CLKPLL_FBDV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clkpll_fbdv to value 0x05"]
impl crate::Resettable for CLKPLL_FBDV_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}

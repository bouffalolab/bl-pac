#[doc = "Register `clkpll_top_ctrl` reader"]
pub struct R(crate::R<CLKPLL_TOP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_TOP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPLL_TOP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPLL_TOP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_top_ctrl` writer"]
pub struct W(crate::W<CLKPLL_TOP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_TOP_CTRL_SPEC>;
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
impl From<crate::W<CLKPLL_TOP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPLL_TOP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_postdiv` reader - "]
pub type CLKPLL_POSTDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_postdiv` writer - "]
pub type CLKPLL_POSTDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_TOP_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `clkpll_refdiv_ratio` reader - "]
pub type CLKPLL_REFDIV_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_refdiv_ratio` writer - "]
pub type CLKPLL_REFDIV_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_TOP_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `clkpll_xtal_rc32m_sel` reader - "]
pub type CLKPLL_XTAL_RC32M_SEL_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_xtal_rc32m_sel` writer - "]
pub type CLKPLL_XTAL_RC32M_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TOP_CTRL_SPEC, bool, O>;
#[doc = "Field `clkpll_refclk_sel` reader - "]
pub type CLKPLL_REFCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_refclk_sel` writer - "]
pub type CLKPLL_REFCLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TOP_CTRL_SPEC, bool, O>;
#[doc = "Field `clkpll_vg11_sel` reader - "]
pub type CLKPLL_VG11_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_vg11_sel` writer - "]
pub type CLKPLL_VG11_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_TOP_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `clkpll_vg13_sel` reader - "]
pub type CLKPLL_VG13_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_vg13_sel` writer - "]
pub type CLKPLL_VG13_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_TOP_CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn clkpll_postdiv(&self) -> CLKPLL_POSTDIV_R {
        CLKPLL_POSTDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clkpll_refdiv_ratio(&self) -> CLKPLL_REFDIV_RATIO_R {
        CLKPLL_REFDIV_RATIO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clkpll_xtal_rc32m_sel(&self) -> CLKPLL_XTAL_RC32M_SEL_R {
        CLKPLL_XTAL_RC32M_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clkpll_refclk_sel(&self) -> CLKPLL_REFCLK_SEL_R {
        CLKPLL_REFCLK_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn clkpll_vg11_sel(&self) -> CLKPLL_VG11_SEL_R {
        CLKPLL_VG11_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_vg13_sel(&self) -> CLKPLL_VG13_SEL_R {
        CLKPLL_VG13_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_postdiv(&mut self) -> CLKPLL_POSTDIV_W<0> {
        CLKPLL_POSTDIV_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_refdiv_ratio(&mut self) -> CLKPLL_REFDIV_RATIO_W<8> {
        CLKPLL_REFDIV_RATIO_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_xtal_rc32m_sel(&mut self) -> CLKPLL_XTAL_RC32M_SEL_W<12> {
        CLKPLL_XTAL_RC32M_SEL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_refclk_sel(&mut self) -> CLKPLL_REFCLK_SEL_W<16> {
        CLKPLL_REFCLK_SEL_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_vg11_sel(&mut self) -> CLKPLL_VG11_SEL_W<20> {
        CLKPLL_VG11_SEL_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_vg13_sel(&mut self) -> CLKPLL_VG13_SEL_W<24> {
        CLKPLL_VG13_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_top_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_top_ctrl](index.html) module"]
pub struct CLKPLL_TOP_CTRL_SPEC;
impl crate::RegisterSpec for CLKPLL_TOP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_top_ctrl::R](R) reader structure"]
impl crate::Readable for CLKPLL_TOP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_top_ctrl::W](W) writer structure"]
impl crate::Writable for CLKPLL_TOP_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clkpll_top_ctrl to value 0x0110_0414"]
impl crate::Resettable for CLKPLL_TOP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0110_0414;
}

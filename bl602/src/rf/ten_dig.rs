#[doc = "Register `ten_dig` reader"]
pub struct R(crate::R<TEN_DIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEN_DIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEN_DIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEN_DIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ten_dig` writer"]
pub struct W(crate::W<TEN_DIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEN_DIG_SPEC>;
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
impl From<crate::W<TEN_DIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEN_DIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dten_clkpll_postdiv_clk` reader - "]
pub type DTEN_CLKPLL_POSTDIV_CLK_R = crate::BitReader<bool>;
#[doc = "Field `dten_clkpll_postdiv_clk` writer - "]
pub type DTEN_CLKPLL_POSTDIV_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TEN_DIG_SPEC, bool, O>;
#[doc = "Field `dten_clkpll_clk96m` reader - "]
pub type DTEN_CLKPLL_CLK96M_R = crate::BitReader<bool>;
#[doc = "Field `dten_clkpll_clk96m` writer - "]
pub type DTEN_CLKPLL_CLK96M_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DIG_SPEC, bool, O>;
#[doc = "Field `dten_clkpll_clk32m` reader - "]
pub type DTEN_CLKPLL_CLK32M_R = crate::BitReader<bool>;
#[doc = "Field `dten_clkpll_clk32m` writer - "]
pub type DTEN_CLKPLL_CLK32M_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DIG_SPEC, bool, O>;
#[doc = "Field `dten_clkpll_fsdm` reader - "]
pub type DTEN_CLKPLL_FSDM_R = crate::BitReader<bool>;
#[doc = "Field `dten_clkpll_fsdm` writer - "]
pub type DTEN_CLKPLL_FSDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DIG_SPEC, bool, O>;
#[doc = "Field `dten_clkpll_fref` reader - "]
pub type DTEN_CLKPLL_FREF_R = crate::BitReader<bool>;
#[doc = "Field `dten_clkpll_fref` writer - "]
pub type DTEN_CLKPLL_FREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DIG_SPEC, bool, O>;
#[doc = "Field `dten_clkpll_fin` reader - "]
pub type DTEN_CLKPLL_FIN_R = crate::BitReader<bool>;
#[doc = "Field `dten_clkpll_fin` writer - "]
pub type DTEN_CLKPLL_FIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DIG_SPEC, bool, O>;
#[doc = "Field `dten_lo_fsdm` reader - "]
pub type DTEN_LO_FSDM_R = crate::BitReader<bool>;
#[doc = "Field `dten_lo_fsdm` writer - "]
pub type DTEN_LO_FSDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DIG_SPEC, bool, O>;
#[doc = "Field `dten_lo_fref` reader - "]
pub type DTEN_LO_FREF_R = crate::BitReader<bool>;
#[doc = "Field `dten_lo_fref` writer - "]
pub type DTEN_LO_FREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DIG_SPEC, bool, O>;
#[doc = "Field `dtest_pull_down` reader - "]
pub type DTEST_PULL_DOWN_R = crate::BitReader<bool>;
#[doc = "Field `dtest_pull_down` writer - "]
pub type DTEST_PULL_DOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DIG_SPEC, bool, O>;
#[doc = "Field `rf_dtest_en` reader - "]
pub type RF_DTEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_dtest_en` writer - "]
pub type RF_DTEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dten_clkpll_postdiv_clk(&self) -> DTEN_CLKPLL_POSTDIV_CLK_R {
        DTEN_CLKPLL_POSTDIV_CLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dten_clkpll_clk96m(&self) -> DTEN_CLKPLL_CLK96M_R {
        DTEN_CLKPLL_CLK96M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dten_clkpll_clk32m(&self) -> DTEN_CLKPLL_CLK32M_R {
        DTEN_CLKPLL_CLK32M_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dten_clkpll_fsdm(&self) -> DTEN_CLKPLL_FSDM_R {
        DTEN_CLKPLL_FSDM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_clkpll_fref(&self) -> DTEN_CLKPLL_FREF_R {
        DTEN_CLKPLL_FREF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_clkpll_fin(&self) -> DTEN_CLKPLL_FIN_R {
        DTEN_CLKPLL_FIN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_lo_fsdm(&self) -> DTEN_LO_FSDM_R {
        DTEN_LO_FSDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_lo_fref(&self) -> DTEN_LO_FREF_R {
        DTEN_LO_FREF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dtest_pull_down(&self) -> DTEST_PULL_DOWN_R {
        DTEST_PULL_DOWN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rf_dtest_en(&self) -> RF_DTEST_EN_R {
        RF_DTEST_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clkpll_postdiv_clk(&mut self) -> DTEN_CLKPLL_POSTDIV_CLK_W<0> {
        DTEN_CLKPLL_POSTDIV_CLK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clkpll_clk96m(&mut self) -> DTEN_CLKPLL_CLK96M_W<1> {
        DTEN_CLKPLL_CLK96M_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clkpll_clk32m(&mut self) -> DTEN_CLKPLL_CLK32M_W<2> {
        DTEN_CLKPLL_CLK32M_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clkpll_fsdm(&mut self) -> DTEN_CLKPLL_FSDM_W<3> {
        DTEN_CLKPLL_FSDM_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clkpll_fref(&mut self) -> DTEN_CLKPLL_FREF_W<4> {
        DTEN_CLKPLL_FREF_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clkpll_fin(&mut self) -> DTEN_CLKPLL_FIN_W<5> {
        DTEN_CLKPLL_FIN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dten_lo_fsdm(&mut self) -> DTEN_LO_FSDM_W<6> {
        DTEN_LO_FSDM_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dten_lo_fref(&mut self) -> DTEN_LO_FREF_W<8> {
        DTEN_LO_FREF_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn dtest_pull_down(&mut self) -> DTEST_PULL_DOWN_W<9> {
        DTEST_PULL_DOWN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn rf_dtest_en(&mut self) -> RF_DTEST_EN_W<23> {
        RF_DTEST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ten_dig](index.html) module"]
pub struct TEN_DIG_SPEC;
impl crate::RegisterSpec for TEN_DIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ten_dig::R](R) reader structure"]
impl crate::Readable for TEN_DIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ten_dig::W](W) writer structure"]
impl crate::Writable for TEN_DIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ten_dig to value 0"]
impl crate::Resettable for TEN_DIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `pu_rst_clkpll` reader"]
pub struct R(crate::R<PU_RST_CLKPLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PU_RST_CLKPLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PU_RST_CLKPLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PU_RST_CLKPLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pu_rst_clkpll` writer"]
pub struct W(crate::W<PU_RST_CLKPLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PU_RST_CLKPLL_SPEC>;
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
impl From<crate::W<PU_RST_CLKPLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PU_RST_CLKPLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_sdm_reset` reader - "]
pub type CLKPLL_SDM_RESET_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_sdm_reset` writer - "]
pub type CLKPLL_SDM_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `clkpll_reset_postdiv` reader - "]
pub type CLKPLL_RESET_POSTDIV_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_reset_postdiv` writer - "]
pub type CLKPLL_RESET_POSTDIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `clkpll_reset_fbdv` reader - "]
pub type CLKPLL_RESET_FBDV_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_reset_fbdv` writer - "]
pub type CLKPLL_RESET_FBDV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `clkpll_reset_refdiv` reader - "]
pub type CLKPLL_RESET_REFDIV_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_reset_refdiv` writer - "]
pub type CLKPLL_RESET_REFDIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `clkpll_pu_postdiv` reader - "]
pub type CLKPLL_PU_POSTDIV_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_pu_postdiv` writer - "]
pub type CLKPLL_PU_POSTDIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `clkpll_pu_fbdv` reader - "]
pub type CLKPLL_PU_FBDV_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_pu_fbdv` writer - "]
pub type CLKPLL_PU_FBDV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `clkpll_pu_clamp_op` reader - "]
pub type CLKPLL_PU_CLAMP_OP_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_pu_clamp_op` writer - "]
pub type CLKPLL_PU_CLAMP_OP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `clkpll_pu_pfd` reader - "]
pub type CLKPLL_PU_PFD_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_pu_pfd` writer - "]
pub type CLKPLL_PU_PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `clkpll_pu_cp` reader - "]
pub type CLKPLL_PU_CP_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_pu_cp` writer - "]
pub type CLKPLL_PU_CP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `pu_clkpll_sfreg` reader - "]
pub type PU_CLKPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `pu_clkpll_sfreg` writer - "]
pub type PU_CLKPLL_SFREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `pu_clkpll` reader - "]
pub type PU_CLKPLL_R = crate::BitReader<bool>;
#[doc = "Field `pu_clkpll` writer - "]
pub type PU_CLKPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sdm_reset(&self) -> CLKPLL_SDM_RESET_R {
        CLKPLL_SDM_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_reset_postdiv(&self) -> CLKPLL_RESET_POSTDIV_R {
        CLKPLL_RESET_POSTDIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_reset_fbdv(&self) -> CLKPLL_RESET_FBDV_R {
        CLKPLL_RESET_FBDV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_reset_refdiv(&self) -> CLKPLL_RESET_REFDIV_R {
        CLKPLL_RESET_REFDIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_pu_postdiv(&self) -> CLKPLL_PU_POSTDIV_R {
        CLKPLL_PU_POSTDIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_pu_fbdv(&self) -> CLKPLL_PU_FBDV_R {
        CLKPLL_PU_FBDV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_pu_clamp_op(&self) -> CLKPLL_PU_CLAMP_OP_R {
        CLKPLL_PU_CLAMP_OP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_pu_pfd(&self) -> CLKPLL_PU_PFD_R {
        CLKPLL_PU_PFD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_pu_cp(&self) -> CLKPLL_PU_CP_R {
        CLKPLL_PU_CP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_clkpll_sfreg(&self) -> PU_CLKPLL_SFREG_R {
        PU_CLKPLL_SFREG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_clkpll(&self) -> PU_CLKPLL_R {
        PU_CLKPLL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_sdm_reset(&mut self) -> CLKPLL_SDM_RESET_W<0> {
        CLKPLL_SDM_RESET_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_reset_postdiv(&mut self) -> CLKPLL_RESET_POSTDIV_W<1> {
        CLKPLL_RESET_POSTDIV_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_reset_fbdv(&mut self) -> CLKPLL_RESET_FBDV_W<2> {
        CLKPLL_RESET_FBDV_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_reset_refdiv(&mut self) -> CLKPLL_RESET_REFDIV_W<3> {
        CLKPLL_RESET_REFDIV_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_pu_postdiv(&mut self) -> CLKPLL_PU_POSTDIV_W<4> {
        CLKPLL_PU_POSTDIV_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_pu_fbdv(&mut self) -> CLKPLL_PU_FBDV_W<5> {
        CLKPLL_PU_FBDV_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_pu_clamp_op(&mut self) -> CLKPLL_PU_CLAMP_OP_W<6> {
        CLKPLL_PU_CLAMP_OP_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_pu_pfd(&mut self) -> CLKPLL_PU_PFD_W<7> {
        CLKPLL_PU_PFD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_pu_cp(&mut self) -> CLKPLL_PU_CP_W<8> {
        CLKPLL_PU_CP_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pu_clkpll_sfreg(&mut self) -> PU_CLKPLL_SFREG_W<9> {
        PU_CLKPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_clkpll(&mut self) -> PU_CLKPLL_W<10> {
        PU_CLKPLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pu_rst_clkpll.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pu_rst_clkpll](index.html) module"]
pub struct PU_RST_CLKPLL_SPEC;
impl crate::RegisterSpec for PU_RST_CLKPLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pu_rst_clkpll::R](R) reader structure"]
impl crate::Readable for PU_RST_CLKPLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pu_rst_clkpll::W](W) writer structure"]
impl crate::Writable for PU_RST_CLKPLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pu_rst_clkpll to value 0x01f0"]
impl crate::Resettable for PU_RST_CLKPLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01f0;
}

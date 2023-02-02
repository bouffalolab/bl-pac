#[doc = "Register `clkpll_cp` reader"]
pub struct R(crate::R<CLKPLL_CP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_CP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPLL_CP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPLL_CP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_cp` writer"]
pub struct W(crate::W<CLKPLL_CP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_CP_SPEC>;
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
impl From<crate::W<CLKPLL_CP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPLL_CP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_sel_cp_bias` reader - "]
pub type CLKPLL_SEL_CP_BIAS_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_sel_cp_bias` writer - "]
pub type CLKPLL_SEL_CP_BIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKPLL_CP_SPEC, bool, O>;
#[doc = "Field `clkpll_icp_5u` reader - "]
pub type CLKPLL_ICP_5U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_icp_5u` writer - "]
pub type CLKPLL_ICP_5U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_CP_SPEC, u8, u8, 2, O>;
#[doc = "Field `clkpll_icp_1u` reader - "]
pub type CLKPLL_ICP_1U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_icp_1u` writer - "]
pub type CLKPLL_ICP_1U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_CP_SPEC, u8, u8, 2, O>;
#[doc = "Field `clkpll_int_frac_sw` reader - "]
pub type CLKPLL_INT_FRAC_SW_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_int_frac_sw` writer - "]
pub type CLKPLL_INT_FRAC_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKPLL_CP_SPEC, bool, O>;
#[doc = "Field `clkpll_cp_startup_en` reader - "]
pub type CLKPLL_CP_STARTUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_cp_startup_en` writer - "]
pub type CLKPLL_CP_STARTUP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_CP_SPEC, bool, O>;
#[doc = "Field `clkpll_cp_opamp_en` reader - "]
pub type CLKPLL_CP_OPAMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_cp_opamp_en` writer - "]
pub type CLKPLL_CP_OPAMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKPLL_CP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sel_cp_bias(&self) -> CLKPLL_SEL_CP_BIAS_R {
        CLKPLL_SEL_CP_BIAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_icp_5u(&self) -> CLKPLL_ICP_5U_R {
        CLKPLL_ICP_5U_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn clkpll_icp_1u(&self) -> CLKPLL_ICP_1U_R {
        CLKPLL_ICP_1U_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_int_frac_sw(&self) -> CLKPLL_INT_FRAC_SW_R {
        CLKPLL_INT_FRAC_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_cp_startup_en(&self) -> CLKPLL_CP_STARTUP_EN_R {
        CLKPLL_CP_STARTUP_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkpll_cp_opamp_en(&self) -> CLKPLL_CP_OPAMP_EN_R {
        CLKPLL_CP_OPAMP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_sel_cp_bias(&mut self) -> CLKPLL_SEL_CP_BIAS_W<0> {
        CLKPLL_SEL_CP_BIAS_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_icp_5u(&mut self) -> CLKPLL_ICP_5U_W<4> {
        CLKPLL_ICP_5U_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_icp_1u(&mut self) -> CLKPLL_ICP_1U_W<6> {
        CLKPLL_ICP_1U_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_int_frac_sw(&mut self) -> CLKPLL_INT_FRAC_SW_W<8> {
        CLKPLL_INT_FRAC_SW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_cp_startup_en(&mut self) -> CLKPLL_CP_STARTUP_EN_W<9> {
        CLKPLL_CP_STARTUP_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_cp_opamp_en(&mut self) -> CLKPLL_CP_OPAMP_EN_W<10> {
        CLKPLL_CP_OPAMP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_cp.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_cp](index.html) module"]
pub struct CLKPLL_CP_SPEC;
impl crate::RegisterSpec for CLKPLL_CP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_cp::R](R) reader structure"]
impl crate::Readable for CLKPLL_CP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_cp::W](W) writer structure"]
impl crate::Writable for CLKPLL_CP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clkpll_cp to value 0x0741"]
impl crate::Resettable for CLKPLL_CP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0741;
}

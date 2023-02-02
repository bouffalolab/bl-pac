#[doc = "Register `se_trng_0_ctrl_0` reader"]
pub struct R(crate::R<SE_TRNG_0_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_TRNG_0_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_TRNG_0_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_TRNG_0_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_trng_0_ctrl_0` writer"]
pub struct W(crate::W<SE_TRNG_0_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_TRNG_0_CTRL_0_SPEC>;
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
impl From<crate::W<SE_TRNG_0_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_TRNG_0_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_trng_0_busy` reader - "]
pub type SE_TRNG_0_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_trig_1t` reader - "]
pub type SE_TRNG_0_TRIG_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_trig_1t` writer - "]
pub type SE_TRNG_0_TRIG_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_trng_0_en` reader - "]
pub type SE_TRNG_0_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_en` writer - "]
pub type SE_TRNG_0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_trng_0_dout_clr_1t` reader - "]
pub type SE_TRNG_0_DOUT_CLR_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_dout_clr_1t` writer - "]
pub type SE_TRNG_0_DOUT_CLR_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_trng_0_ht_error` reader - "]
pub type SE_TRNG_0_HT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_int` reader - "]
pub type SE_TRNG_0_INT_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_int_clr_1t` reader - "]
pub type SE_TRNG_0_INT_CLR_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_int_clr_1t` writer - "]
pub type SE_TRNG_0_INT_CLR_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_trng_0_int_set_1t` reader - "]
pub type SE_TRNG_0_INT_SET_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_int_set_1t` writer - "]
pub type SE_TRNG_0_INT_SET_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_trng_0_int_mask` reader - "]
pub type SE_TRNG_0_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_int_mask` writer - "]
pub type SE_TRNG_0_INT_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_trng_0_manual_fun_sel` reader - "]
pub type SE_TRNG_0_MANUAL_FUN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_manual_fun_sel` writer - "]
pub type SE_TRNG_0_MANUAL_FUN_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_trng_0_manual_reseed` reader - "]
pub type SE_TRNG_0_MANUAL_RESEED_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_manual_reseed` writer - "]
pub type SE_TRNG_0_MANUAL_RESEED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_trng_0_manual_en` reader - "]
pub type SE_TRNG_0_MANUAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_manual_en` writer - "]
pub type SE_TRNG_0_MANUAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_busy(&self) -> SE_TRNG_0_BUSY_R {
        SE_TRNG_0_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_trig_1t(&self) -> SE_TRNG_0_TRIG_1T_R {
        SE_TRNG_0_TRIG_1T_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_en(&self) -> SE_TRNG_0_EN_R {
        SE_TRNG_0_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_dout_clr_1t(&self) -> SE_TRNG_0_DOUT_CLR_1T_R {
        SE_TRNG_0_DOUT_CLR_1T_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_trng_0_ht_error(&self) -> SE_TRNG_0_HT_ERROR_R {
        SE_TRNG_0_HT_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_0_int(&self) -> SE_TRNG_0_INT_R {
        SE_TRNG_0_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_0_int_clr_1t(&self) -> SE_TRNG_0_INT_CLR_1T_R {
        SE_TRNG_0_INT_CLR_1T_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_0_int_set_1t(&self) -> SE_TRNG_0_INT_SET_1T_R {
        SE_TRNG_0_INT_SET_1T_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_trng_0_int_mask(&self) -> SE_TRNG_0_INT_MASK_R {
        SE_TRNG_0_INT_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_trng_0_manual_fun_sel(&self) -> SE_TRNG_0_MANUAL_FUN_SEL_R {
        SE_TRNG_0_MANUAL_FUN_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_trng_0_manual_reseed(&self) -> SE_TRNG_0_MANUAL_RESEED_R {
        SE_TRNG_0_MANUAL_RESEED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_trng_0_manual_en(&self) -> SE_TRNG_0_MANUAL_EN_R {
        SE_TRNG_0_MANUAL_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_trig_1t(&mut self) -> SE_TRNG_0_TRIG_1T_W<1> {
        SE_TRNG_0_TRIG_1T_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_en(&mut self) -> SE_TRNG_0_EN_W<2> {
        SE_TRNG_0_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_dout_clr_1t(&mut self) -> SE_TRNG_0_DOUT_CLR_1T_W<3> {
        SE_TRNG_0_DOUT_CLR_1T_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_int_clr_1t(&mut self) -> SE_TRNG_0_INT_CLR_1T_W<9> {
        SE_TRNG_0_INT_CLR_1T_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_int_set_1t(&mut self) -> SE_TRNG_0_INT_SET_1T_W<10> {
        SE_TRNG_0_INT_SET_1T_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_int_mask(&mut self) -> SE_TRNG_0_INT_MASK_W<11> {
        SE_TRNG_0_INT_MASK_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_manual_fun_sel(&mut self) -> SE_TRNG_0_MANUAL_FUN_SEL_W<13> {
        SE_TRNG_0_MANUAL_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_manual_reseed(&mut self) -> SE_TRNG_0_MANUAL_RESEED_W<14> {
        SE_TRNG_0_MANUAL_RESEED_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_manual_en(&mut self) -> SE_TRNG_0_MANUAL_EN_W<15> {
        SE_TRNG_0_MANUAL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_trng_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_0](index.html) module"]
pub struct SE_TRNG_0_CTRL_0_SPEC;
impl crate::RegisterSpec for SE_TRNG_0_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_trng_0_ctrl_0::R](R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_0::W](W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_trng_0_ctrl_0 to value 0"]
impl crate::Resettable for SE_TRNG_0_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

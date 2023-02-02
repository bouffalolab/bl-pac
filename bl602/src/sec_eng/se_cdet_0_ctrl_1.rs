#[doc = "Register `se_cdet_0_ctrl_1` reader"]
pub struct R(crate::R<SE_CDET_0_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_CDET_0_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_CDET_0_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_CDET_0_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_cdet_0_ctrl_1` writer"]
pub struct W(crate::W<SE_CDET_0_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_CDET_0_CTRL_1_SPEC>;
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
impl From<crate::W<SE_CDET_0_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_CDET_0_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_cdet_0_t_loop_n` reader - "]
pub type SE_CDET_0_T_LOOP_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_cdet_0_t_loop_n` writer - "]
pub type SE_CDET_0_T_LOOP_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_CDET_0_CTRL_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `se_cdet_0_t_dly_n` reader - "]
pub type SE_CDET_0_T_DLY_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_cdet_0_t_dly_n` writer - "]
pub type SE_CDET_0_T_DLY_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_CDET_0_CTRL_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `se_cdet_0_g_slp_n` reader - "]
pub type SE_CDET_0_G_SLP_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_cdet_0_g_slp_n` writer - "]
pub type SE_CDET_0_G_SLP_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_CDET_0_CTRL_1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_cdet_0_t_loop_n(&self) -> SE_CDET_0_T_LOOP_N_R {
        SE_CDET_0_T_LOOP_N_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_cdet_0_t_dly_n(&self) -> SE_CDET_0_T_DLY_N_R {
        SE_CDET_0_T_DLY_N_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_slp_n(&self) -> SE_CDET_0_G_SLP_N_R {
        SE_CDET_0_G_SLP_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn se_cdet_0_t_loop_n(&mut self) -> SE_CDET_0_T_LOOP_N_W<0> {
        SE_CDET_0_T_LOOP_N_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn se_cdet_0_t_dly_n(&mut self) -> SE_CDET_0_T_DLY_N_W<8> {
        SE_CDET_0_T_DLY_N_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn se_cdet_0_g_slp_n(&mut self) -> SE_CDET_0_G_SLP_N_W<16> {
        SE_CDET_0_G_SLP_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_cdet_0_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_cdet_0_ctrl_1](index.html) module"]
pub struct SE_CDET_0_CTRL_1_SPEC;
impl crate::RegisterSpec for SE_CDET_0_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_cdet_0_ctrl_1::R](R) reader structure"]
impl crate::Readable for SE_CDET_0_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_cdet_0_ctrl_1::W](W) writer structure"]
impl crate::Writable for SE_CDET_0_CTRL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_cdet_0_ctrl_1 to value 0x00ff_0332"]
impl crate::Resettable for SE_CDET_0_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_0332;
}

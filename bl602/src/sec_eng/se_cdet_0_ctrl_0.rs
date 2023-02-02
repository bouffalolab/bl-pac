#[doc = "Register `se_cdet_0_ctrl_0` reader"]
pub struct R(crate::R<SE_CDET_0_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_CDET_0_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_CDET_0_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_CDET_0_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_cdet_0_ctrl_0` writer"]
pub struct W(crate::W<SE_CDET_0_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_CDET_0_CTRL_0_SPEC>;
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
impl From<crate::W<SE_CDET_0_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_CDET_0_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_cdet_0_en` reader - "]
pub type SE_CDET_0_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_cdet_0_en` writer - "]
pub type SE_CDET_0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CDET_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_cdet_0_error` reader - "]
pub type SE_CDET_0_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `se_cdet_0_status` reader - "]
pub type SE_CDET_0_STATUS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `se_cdet_0_g_loop_max` reader - "]
pub type SE_CDET_0_G_LOOP_MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_cdet_0_g_loop_max` writer - "]
pub type SE_CDET_0_G_LOOP_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_CDET_0_CTRL_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `se_cdet_0_g_loop_min` reader - "]
pub type SE_CDET_0_G_LOOP_MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_cdet_0_g_loop_min` writer - "]
pub type SE_CDET_0_G_LOOP_MIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_CDET_0_CTRL_0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_cdet_0_en(&self) -> SE_CDET_0_EN_R {
        SE_CDET_0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_cdet_0_error(&self) -> SE_CDET_0_ERROR_R {
        SE_CDET_0_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn se_cdet_0_status(&self) -> SE_CDET_0_STATUS_R {
        SE_CDET_0_STATUS_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_max(&self) -> SE_CDET_0_G_LOOP_MAX_R {
        SE_CDET_0_G_LOOP_MAX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_min(&self) -> SE_CDET_0_G_LOOP_MIN_R {
        SE_CDET_0_G_LOOP_MIN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn se_cdet_0_en(&mut self) -> SE_CDET_0_EN_W<0> {
        SE_CDET_0_EN_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn se_cdet_0_g_loop_max(&mut self) -> SE_CDET_0_G_LOOP_MAX_W<16> {
        SE_CDET_0_G_LOOP_MAX_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn se_cdet_0_g_loop_min(&mut self) -> SE_CDET_0_G_LOOP_MIN_W<24> {
        SE_CDET_0_G_LOOP_MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_cdet_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_cdet_0_ctrl_0](index.html) module"]
pub struct SE_CDET_0_CTRL_0_SPEC;
impl crate::RegisterSpec for SE_CDET_0_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_cdet_0_ctrl_0::R](R) reader structure"]
impl crate::Readable for SE_CDET_0_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_cdet_0_ctrl_0::W](W) writer structure"]
impl crate::Writable for SE_CDET_0_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_cdet_0_ctrl_0 to value 0x2164_0004"]
impl crate::Resettable for SE_CDET_0_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x2164_0004;
}

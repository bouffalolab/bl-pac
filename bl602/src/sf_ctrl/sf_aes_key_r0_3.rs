#[doc = "Register `sf_aes_key_r0_3` reader"]
pub struct R(crate::R<SF_AES_KEY_R0_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_AES_KEY_R0_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_AES_KEY_R0_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_AES_KEY_R0_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_aes_key_r0_3` writer"]
pub struct W(crate::W<SF_AES_KEY_R0_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_AES_KEY_R0_3_SPEC>;
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
impl From<crate::W<SF_AES_KEY_R0_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_AES_KEY_R0_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_aes_key_r0_3` reader - "]
pub type SF_AES_KEY_R0_3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `sf_aes_key_r0_3` writer - "]
pub type SF_AES_KEY_R0_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_AES_KEY_R0_3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r0_3(&self) -> SF_AES_KEY_R0_3_R {
        SF_AES_KEY_R0_3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_key_r0_3(&mut self) -> SF_AES_KEY_R0_3_W<0> {
        SF_AES_KEY_R0_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_aes_key_r0_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r0_3](index.html) module"]
pub struct SF_AES_KEY_R0_3_SPEC;
impl crate::RegisterSpec for SF_AES_KEY_R0_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_aes_key_r0_3::R](R) reader structure"]
impl crate::Readable for SF_AES_KEY_R0_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r0_3::W](W) writer structure"]
impl crate::Writable for SF_AES_KEY_R0_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_aes_key_r0_3 to value 0"]
impl crate::Resettable for SF_AES_KEY_R0_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

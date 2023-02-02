#[doc = "Register `se_sha_0_link` reader"]
pub struct R(crate::R<SE_SHA_0_LINK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_SHA_0_LINK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_SHA_0_LINK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_SHA_0_LINK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_sha_0_link` writer"]
pub struct W(crate::W<SE_SHA_0_LINK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_SHA_0_LINK_SPEC>;
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
impl From<crate::W<SE_SHA_0_LINK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_SHA_0_LINK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_sha_0_lca` reader - "]
pub type SE_SHA_0_LCA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `se_sha_0_lca` writer - "]
pub type SE_SHA_0_LCA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_SHA_0_LINK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_lca(&self) -> SE_SHA_0_LCA_R {
        SE_SHA_0_LCA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn se_sha_0_lca(&mut self) -> SE_SHA_0_LCA_W<0> {
        SE_SHA_0_LCA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_sha_0_link.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_link](index.html) module"]
pub struct SE_SHA_0_LINK_SPEC;
impl crate::RegisterSpec for SE_SHA_0_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_sha_0_link::R](R) reader structure"]
impl crate::Readable for SE_SHA_0_LINK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_sha_0_link::W](W) writer structure"]
impl crate::Writable for SE_SHA_0_LINK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_sha_0_link to value 0"]
impl crate::Resettable for SE_SHA_0_LINK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

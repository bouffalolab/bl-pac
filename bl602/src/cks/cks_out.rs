#[doc = "Register `cks_out` reader"]
pub struct R(crate::R<CKS_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKS_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKS_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKS_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cks_out` writer"]
pub struct W(crate::W<CKS_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKS_OUT_SPEC>;
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
impl From<crate::W<CKS_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKS_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cks_out` reader - "]
pub type CKS_OUT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cks_out(&self) -> CKS_OUT_R {
        CKS_OUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Checksum value out\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cks_out](index.html) module"]
pub struct CKS_OUT_SPEC;
impl crate::RegisterSpec for CKS_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cks_out::R](R) reader structure"]
impl crate::Readable for CKS_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cks_out::W](W) writer structure"]
impl crate::Writable for CKS_OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cks_out to value 0xffff"]
impl crate::Resettable for CKS_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}

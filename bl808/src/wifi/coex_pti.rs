#[doc = "Register `coex_pti` reader"]
pub struct R(crate::R<COEX_PTI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEX_PTI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEX_PTI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEX_PTI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `coex_pti` writer"]
pub struct W(crate::W<COEX_PTI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COEX_PTI_SPEC>;
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
impl From<crate::W<COEX_PTI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COEX_PTI_SPEC>) -> Self {
        W(writer)
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
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coex_pti](index.html) module"]
pub struct COEX_PTI_SPEC;
impl crate::RegisterSpec for COEX_PTI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coex_pti::R](R) reader structure"]
impl crate::Readable for COEX_PTI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coex_pti::W](W) writer structure"]
impl crate::Writable for COEX_PTI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets coex_pti to value 0"]
impl crate::Resettable for COEX_PTI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `terminate_state` reader"]
pub struct R(crate::R<TERMINATE_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TERMINATE_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TERMINATE_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TERMINATE_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `terminate_state` writer"]
pub struct W(crate::W<TERMINATE_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TERMINATE_STATE_SPEC>;
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
impl From<crate::W<TERMINATE_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TERMINATE_STATE_SPEC>) -> Self {
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
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [terminate_state](index.html) module"]
pub struct TERMINATE_STATE_SPEC;
impl crate::RegisterSpec for TERMINATE_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [terminate_state::R](R) reader structure"]
impl crate::Readable for TERMINATE_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [terminate_state::W](W) writer structure"]
impl crate::Writable for TERMINATE_STATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets terminate_state to value 0"]
impl crate::Resettable for TERMINATE_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

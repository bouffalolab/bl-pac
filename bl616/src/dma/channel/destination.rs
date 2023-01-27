#[doc = "Register `destination` reader"]
pub struct R(crate::R<DESTINATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESTINATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESTINATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESTINATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `destination` writer"]
pub struct W(crate::W<DESTINATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATION_SPEC>;
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
impl From<crate::W<DESTINATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATION_SPEC>) -> Self {
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
#[doc = "Destination address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destination](index.html) module"]
pub struct DESTINATION_SPEC;
impl crate::RegisterSpec for DESTINATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [destination::R](R) reader structure"]
impl crate::Readable for DESTINATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [destination::W](W) writer structure"]
impl crate::Writable for DESTINATION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets destination to value 0"]
impl crate::Resettable for DESTINATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

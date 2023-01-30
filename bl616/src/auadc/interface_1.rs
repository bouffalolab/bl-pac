#[doc = "Register `interface_1` reader"]
pub struct R(crate::R<INTERFACE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERFACE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERFACE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERFACE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `interface_1` writer"]
pub struct W(crate::W<INTERFACE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERFACE_1_SPEC>;
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
impl From<crate::W<INTERFACE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERFACE_1_SPEC>) -> Self {
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
#[doc = "Interface control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interface_1](index.html) module"]
pub struct INTERFACE_1_SPEC;
impl crate::RegisterSpec for INTERFACE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interface_1::R](R) reader structure"]
impl crate::Readable for INTERFACE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interface_1::W](W) writer structure"]
impl crate::Writable for INTERFACE_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interface_1 to value 0"]
impl crate::Resettable for INTERFACE_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `ldo18` reader"]
pub struct R(crate::R<LDO18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ldo18` writer"]
pub struct W(crate::W<LDO18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO18_SPEC>;
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
impl From<crate::W<LDO18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO18_SPEC>) -> Self {
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
#[doc = "1.8-V Low Dropout Linear Regulator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo18](index.html) module"]
pub struct LDO18_SPEC;
impl crate::RegisterSpec for LDO18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo18::R](R) reader structure"]
impl crate::Readable for LDO18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo18::W](W) writer structure"]
impl crate::Writable for LDO18_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo18 to value 0"]
impl crate::Resettable for LDO18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

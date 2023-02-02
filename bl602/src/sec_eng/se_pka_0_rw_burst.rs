#[doc = "Register `se_pka_0_rw_burst` reader"]
pub struct R(crate::R<SE_PKA_0_RW_BURST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_PKA_0_RW_BURST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_PKA_0_RW_BURST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_PKA_0_RW_BURST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_pka_0_rw_burst` writer"]
pub struct W(crate::W<SE_PKA_0_RW_BURST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_PKA_0_RW_BURST_SPEC>;
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
impl From<crate::W<SE_PKA_0_RW_BURST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_PKA_0_RW_BURST_SPEC>) -> Self {
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
#[doc = "se_pka_0_rw_burst.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_rw_burst](index.html) module"]
pub struct SE_PKA_0_RW_BURST_SPEC;
impl crate::RegisterSpec for SE_PKA_0_RW_BURST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_pka_0_rw_burst::R](R) reader structure"]
impl crate::Readable for SE_PKA_0_RW_BURST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_pka_0_rw_burst::W](W) writer structure"]
impl crate::Writable for SE_PKA_0_RW_BURST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_pka_0_rw_burst to value 0"]
impl crate::Resettable for SE_PKA_0_RW_BURST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

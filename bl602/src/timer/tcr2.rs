#[doc = "Register `TCR2` reader"]
pub struct R(crate::R<TCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tcr` reader - "]
pub type TCR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(self.bits)
    }
}
#[doc = "TCR2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr2](index.html) module"]
pub struct TCR2_SPEC;
impl crate::RegisterSpec for TCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr2::R](R) reader structure"]
impl crate::Readable for TCR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCR2 to value 0"]
impl crate::Resettable for TCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

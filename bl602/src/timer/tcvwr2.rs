#[doc = "Register `TCVWR2` reader"]
pub struct R(crate::R<TCVWR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCVWR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCVWR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCVWR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tcvwr` reader - "]
pub type TCVWR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvwr(&self) -> TCVWR_R {
        TCVWR_R::new(self.bits)
    }
}
#[doc = "TCVWR2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvwr2](index.html) module"]
pub struct TCVWR2_SPEC;
impl crate::RegisterSpec for TCVWR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcvwr2::R](R) reader structure"]
impl crate::Readable for TCVWR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCVWR2 to value 0"]
impl crate::Resettable for TCVWR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

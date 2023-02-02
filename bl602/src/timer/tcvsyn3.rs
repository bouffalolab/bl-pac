#[doc = "Register `TCVSYN3` reader"]
pub struct R(crate::R<TCVSYN3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCVSYN3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCVSYN3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCVSYN3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tcvsyn3` reader - "]
pub type TCVSYN3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn3(&self) -> TCVSYN3_R {
        TCVSYN3_R::new(self.bits)
    }
}
#[doc = "TCVSYN3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvsyn3](index.html) module"]
pub struct TCVSYN3_SPEC;
impl crate::RegisterSpec for TCVSYN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcvsyn3::R](R) reader structure"]
impl crate::Readable for TCVSYN3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCVSYN3 to value 0"]
impl crate::Resettable for TCVSYN3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

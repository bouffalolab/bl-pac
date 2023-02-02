#[doc = "Register `TCVSYN2` reader"]
pub struct R(crate::R<TCVSYN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCVSYN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCVSYN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCVSYN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tcvsyn2` reader - "]
pub type TCVSYN2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn2(&self) -> TCVSYN2_R {
        TCVSYN2_R::new(self.bits)
    }
}
#[doc = "TCVSYN2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvsyn2](index.html) module"]
pub struct TCVSYN2_SPEC;
impl crate::RegisterSpec for TCVSYN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcvsyn2::R](R) reader structure"]
impl crate::Readable for TCVSYN2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCVSYN2 to value 0"]
impl crate::Resettable for TCVSYN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

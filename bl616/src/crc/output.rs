#[doc = "Register `output` reader"]
pub struct R(crate::R<OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `checksum` reader - Read checksum from peripheral"]
pub type CHECKSUM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read checksum from peripheral"]
    #[inline(always)]
    pub fn checksum(&self) -> CHECKSUM_R {
        CHECKSUM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Checksum output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [output](index.html) module"]
pub struct OUTPUT_SPEC;
impl crate::RegisterSpec for OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [output::R](R) reader structure"]
impl crate::Readable for OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets output to value 0"]
impl crate::Resettable for OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

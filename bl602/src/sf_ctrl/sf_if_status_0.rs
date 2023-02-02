#[doc = "Register `sf_if_status_0` reader"]
pub struct R(crate::R<SF_IF_STATUS_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_IF_STATUS_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_IF_STATUS_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_IF_STATUS_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sf_if_status_0` reader - "]
pub type SF_IF_STATUS_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_status_0(&self) -> SF_IF_STATUS_0_R {
        SF_IF_STATUS_0_R::new(self.bits)
    }
}
#[doc = "sf_if_status_0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_status_0](index.html) module"]
pub struct SF_IF_STATUS_0_SPEC;
impl crate::RegisterSpec for SF_IF_STATUS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_if_status_0::R](R) reader structure"]
impl crate::Readable for SF_IF_STATUS_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sf_if_status_0 to value 0"]
impl crate::Resettable for SF_IF_STATUS_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

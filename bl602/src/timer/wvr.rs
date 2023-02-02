#[doc = "Register `WVR` reader"]
pub struct R(crate::R<WVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `wvr` reader - "]
pub type WVR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wvr(&self) -> WVR_R {
        WVR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "WVR.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvr](index.html) module"]
pub struct WVR_SPEC;
impl crate::RegisterSpec for WVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wvr::R](R) reader structure"]
impl crate::Readable for WVR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WVR to value 0"]
impl crate::Resettable for WVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

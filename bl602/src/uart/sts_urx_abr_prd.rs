#[doc = "Register `sts_urx_abr_prd` reader"]
pub struct R(crate::R<STS_URX_ABR_PRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_URX_ABR_PRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_URX_ABR_PRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_URX_ABR_PRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sts_urx_abr_prd_start` reader - "]
pub type STS_URX_ABR_PRD_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sts_urx_abr_prd_0x55` reader - "]
pub type STS_URX_ABR_PRD_0X55_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_start(&self) -> STS_URX_ABR_PRD_START_R {
        STS_URX_ABR_PRD_START_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_0x55(&self) -> STS_URX_ABR_PRD_0X55_R {
        STS_URX_ABR_PRD_0X55_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "sts_urx_abr_prd.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts_urx_abr_prd](index.html) module"]
pub struct STS_URX_ABR_PRD_SPEC;
impl crate::RegisterSpec for STS_URX_ABR_PRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts_urx_abr_prd::R](R) reader structure"]
impl crate::Readable for STS_URX_ABR_PRD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sts_urx_abr_prd to value 0"]
impl crate::Resettable for STS_URX_ABR_PRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

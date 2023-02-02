#[doc = "Register `irom1_misr_dataout_0` reader"]
pub struct R(crate::R<IROM1_MISR_DATAOUT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IROM1_MISR_DATAOUT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IROM1_MISR_DATAOUT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IROM1_MISR_DATAOUT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `irom1_misr_dataout_0` reader - "]
pub type IROM1_MISR_DATAOUT_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irom1_misr_dataout_0(&self) -> IROM1_MISR_DATAOUT_0_R {
        IROM1_MISR_DATAOUT_0_R::new(self.bits)
    }
}
#[doc = "irom1_misr_dataout_0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irom1_misr_dataout_0](index.html) module"]
pub struct IROM1_MISR_DATAOUT_0_SPEC;
impl crate::RegisterSpec for IROM1_MISR_DATAOUT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irom1_misr_dataout_0::R](R) reader structure"]
impl crate::Readable for IROM1_MISR_DATAOUT_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets irom1_misr_dataout_0 to value 0"]
impl crate::Resettable for IROM1_MISR_DATAOUT_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

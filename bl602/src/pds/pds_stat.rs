#[doc = "Register `pds_stat` reader"]
pub struct R(crate::R<PDS_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ro_pds_state` reader - "]
pub type RO_PDS_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ro_pds_rf_state` reader - "]
pub type RO_PDS_RF_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ro_pds_pll_state` reader - "]
pub type RO_PDS_PLL_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ro_pds_state(&self) -> RO_PDS_STATE_R {
        RO_PDS_STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&self) -> RO_PDS_RF_STATE_R {
        RO_PDS_RF_STATE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ro_pds_pll_state(&self) -> RO_PDS_PLL_STATE_R {
        RO_PDS_PLL_STATE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "pds_stat.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_stat](index.html) module"]
pub struct PDS_STAT_SPEC;
impl crate::RegisterSpec for PDS_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_stat::R](R) reader structure"]
impl crate::Readable for PDS_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets pds_stat to value 0"]
impl crate::Resettable for PDS_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

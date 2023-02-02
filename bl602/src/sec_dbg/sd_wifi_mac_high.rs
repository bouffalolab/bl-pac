#[doc = "Register `sd_wifi_mac_high` reader"]
pub struct R(crate::R<SD_WIFI_MAC_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_WIFI_MAC_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_WIFI_MAC_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_WIFI_MAC_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sd_wifi_mac_high` reader - "]
pub type SD_WIFI_MAC_HIGH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_wifi_mac_high(&self) -> SD_WIFI_MAC_HIGH_R {
        SD_WIFI_MAC_HIGH_R::new(self.bits)
    }
}
#[doc = "sd_wifi_mac_high.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_wifi_mac_high](index.html) module"]
pub struct SD_WIFI_MAC_HIGH_SPEC;
impl crate::RegisterSpec for SD_WIFI_MAC_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_wifi_mac_high::R](R) reader structure"]
impl crate::Readable for SD_WIFI_MAC_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sd_wifi_mac_high to value 0"]
impl crate::Resettable for SD_WIFI_MAC_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

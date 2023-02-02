#[doc = "Register `sd_wifi_mac_low` reader"]
pub struct R(crate::R<SD_WIFI_MAC_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_WIFI_MAC_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_WIFI_MAC_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_WIFI_MAC_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sd_wifi_mac_low` reader - "]
pub type SD_WIFI_MAC_LOW_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_wifi_mac_low(&self) -> SD_WIFI_MAC_LOW_R {
        SD_WIFI_MAC_LOW_R::new(self.bits)
    }
}
#[doc = "sd_wifi_mac_low.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_wifi_mac_low](index.html) module"]
pub struct SD_WIFI_MAC_LOW_SPEC;
impl crate::RegisterSpec for SD_WIFI_MAC_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_wifi_mac_low::R](R) reader structure"]
impl crate::Readable for SD_WIFI_MAC_LOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sd_wifi_mac_low to value 0"]
impl crate::Resettable for SD_WIFI_MAC_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

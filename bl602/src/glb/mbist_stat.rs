#[doc = "Register `MBIST_STAT` reader"]
pub struct R(crate::R<MBIST_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBIST_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBIST_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBIST_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `irom_mbist_done` reader - "]
pub type IROM_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `hsram_mbist_done` reader - "]
pub type HSRAM_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `tag_mbist_done` reader - "]
pub type TAG_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ocram_mbist_done` reader - "]
pub type OCRAM_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `wifi_mbist_done` reader - "]
pub type WIFI_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `irom_mbist_fail` reader - "]
pub type IROM_MBIST_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `hsram_mbist_fail` reader - "]
pub type HSRAM_MBIST_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `tag_mbist_fail` reader - "]
pub type TAG_MBIST_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `ocram_mbist_fail` reader - "]
pub type OCRAM_MBIST_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `wifi_mbist_fail` reader - "]
pub type WIFI_MBIST_FAIL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&self) -> IROM_MBIST_DONE_R {
        IROM_MBIST_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_done(&self) -> HSRAM_MBIST_DONE_R {
        HSRAM_MBIST_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_done(&self) -> TAG_MBIST_DONE_R {
        TAG_MBIST_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_done(&self) -> OCRAM_MBIST_DONE_R {
        OCRAM_MBIST_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_done(&self) -> WIFI_MBIST_DONE_R {
        WIFI_MBIST_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&self) -> IROM_MBIST_FAIL_R {
        IROM_MBIST_FAIL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mbist_fail(&self) -> HSRAM_MBIST_FAIL_R {
        HSRAM_MBIST_FAIL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tag_mbist_fail(&self) -> TAG_MBIST_FAIL_R {
        TAG_MBIST_FAIL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&self) -> OCRAM_MBIST_FAIL_R {
        OCRAM_MBIST_FAIL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_mbist_fail(&self) -> WIFI_MBIST_FAIL_R {
        WIFI_MBIST_FAIL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "MBIST_STAT.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_stat](index.html) module"]
pub struct MBIST_STAT_SPEC;
impl crate::RegisterSpec for MBIST_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbist_stat::R](R) reader structure"]
impl crate::Readable for MBIST_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MBIST_STAT to value 0"]
impl crate::Resettable for MBIST_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

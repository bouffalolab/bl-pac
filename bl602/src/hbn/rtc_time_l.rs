#[doc = "Register `RTC_TIME_L` reader"]
pub struct R(crate::R<RTC_TIME_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIME_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIME_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIME_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rtc_time_latch_l` reader - "]
pub type RTC_TIME_LATCH_L_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_time_latch_l(&self) -> RTC_TIME_LATCH_L_R {
        RTC_TIME_LATCH_L_R::new(self.bits)
    }
}
#[doc = "RTC_TIME_L.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time_l](index.html) module"]
pub struct RTC_TIME_L_SPEC;
impl crate::RegisterSpec for RTC_TIME_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_time_l::R](R) reader structure"]
impl crate::Readable for RTC_TIME_L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_TIME_L to value 0"]
impl crate::Resettable for RTC_TIME_L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

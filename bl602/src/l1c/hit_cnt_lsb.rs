#[doc = "Register `hit_cnt_lsb` reader"]
pub struct R(crate::R<HIT_CNT_LSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIT_CNT_LSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIT_CNT_LSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIT_CNT_LSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `hit_cnt_lsb` reader - "]
pub type HIT_CNT_LSB_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_lsb(&self) -> HIT_CNT_LSB_R {
        HIT_CNT_LSB_R::new(self.bits)
    }
}
#[doc = "hit_cnt_lsb.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hit_cnt_lsb](index.html) module"]
pub struct HIT_CNT_LSB_SPEC;
impl crate::RegisterSpec for HIT_CNT_LSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hit_cnt_lsb::R](R) reader structure"]
impl crate::Readable for HIT_CNT_LSB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets hit_cnt_lsb to value 0"]
impl crate::Resettable for HIT_CNT_LSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

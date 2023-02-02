#[doc = "Register `miss_cnt` reader"]
pub struct R(crate::R<MISS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `miss_cnt` reader - "]
pub type MISS_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn miss_cnt(&self) -> MISS_CNT_R {
        MISS_CNT_R::new(self.bits)
    }
}
#[doc = "miss_cnt.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miss_cnt](index.html) module"]
pub struct MISS_CNT_SPEC;
impl crate::RegisterSpec for MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miss_cnt::R](R) reader structure"]
impl crate::Readable for MISS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets miss_cnt to value 0"]
impl crate::Resettable for MISS_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

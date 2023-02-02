#[doc = "Register `se_trng_0_dout_4` reader"]
pub struct R(crate::R<SE_TRNG_0_DOUT_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_TRNG_0_DOUT_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_TRNG_0_DOUT_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_TRNG_0_DOUT_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `se_trng_0_dout_4` reader - "]
pub type SE_TRNG_0_DOUT_4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_dout_4(&self) -> SE_TRNG_0_DOUT_4_R {
        SE_TRNG_0_DOUT_4_R::new(self.bits)
    }
}
#[doc = "se_trng_0_dout_4.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_dout_4](index.html) module"]
pub struct SE_TRNG_0_DOUT_4_SPEC;
impl crate::RegisterSpec for SE_TRNG_0_DOUT_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_trng_0_dout_4::R](R) reader structure"]
impl crate::Readable for SE_TRNG_0_DOUT_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets se_trng_0_dout_4 to value 0"]
impl crate::Resettable for SE_TRNG_0_DOUT_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `bmx_dbg_out` reader"]
pub struct R(crate::R<BMX_DBG_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_DBG_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_DBG_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_DBG_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `bmx_dbg_out` reader - "]
pub type BMX_DBG_OUT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_dbg_out(&self) -> BMX_DBG_OUT_R {
        BMX_DBG_OUT_R::new(self.bits)
    }
}
#[doc = "bmx_dbg_out.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_dbg_out](index.html) module"]
pub struct BMX_DBG_OUT_SPEC;
impl crate::RegisterSpec for BMX_DBG_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_dbg_out::R](R) reader structure"]
impl crate::Readable for BMX_DBG_OUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets bmx_dbg_out to value 0"]
impl crate::Resettable for BMX_DBG_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

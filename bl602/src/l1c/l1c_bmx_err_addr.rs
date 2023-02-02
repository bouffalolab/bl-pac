#[doc = "Register `l1c_bmx_err_addr` reader"]
pub struct R(crate::R<L1C_BMX_ERR_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1C_BMX_ERR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1C_BMX_ERR_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1C_BMX_ERR_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `l1c_bmx_err_addr` reader - "]
pub type L1C_BMX_ERR_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr(&self) -> L1C_BMX_ERR_ADDR_R {
        L1C_BMX_ERR_ADDR_R::new(self.bits)
    }
}
#[doc = "l1c_bmx_err_addr.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_bmx_err_addr](index.html) module"]
pub struct L1C_BMX_ERR_ADDR_SPEC;
impl crate::RegisterSpec for L1C_BMX_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1c_bmx_err_addr::R](R) reader structure"]
impl crate::Readable for L1C_BMX_ERR_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets l1c_bmx_err_addr to value 0"]
impl crate::Resettable for L1C_BMX_ERR_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `gpadc_reg_result` reader"]
pub struct R(crate::R<GPADC_REG_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_result` writer"]
pub struct W(crate::W<GPADC_REG_RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPADC_REG_RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_data_out` reader - "]
pub type GPADC_DATA_OUT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_data_out(&self) -> GPADC_DATA_OUT_R {
        GPADC_DATA_OUT_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_result.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_result](index.html) module"]
pub struct GPADC_REG_RESULT_SPEC;
impl crate::RegisterSpec for GPADC_REG_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_result::R](R) reader structure"]
impl crate::Readable for GPADC_REG_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_result::W](W) writer structure"]
impl crate::Writable for GPADC_REG_RESULT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_result to value 0x01ef_0000"]
impl crate::Resettable for GPADC_REG_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ef_0000;
}

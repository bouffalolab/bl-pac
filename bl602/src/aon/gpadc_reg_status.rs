#[doc = "Register `gpadc_reg_status` reader"]
pub struct R(crate::R<GPADC_REG_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_status` writer"]
pub struct W(crate::W<GPADC_REG_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_STATUS_SPEC>;
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
impl From<crate::W<GPADC_REG_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_data_rdy` reader - "]
pub type GPADC_DATA_RDY_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_reserved` reader - "]
pub type GPADC_RESERVED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `gpadc_reserved` writer - "]
pub type GPADC_RESERVED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_STATUS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_data_rdy(&self) -> GPADC_DATA_RDY_R {
        GPADC_DATA_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn gpadc_reserved(&self) -> GPADC_RESERVED_R {
        GPADC_RESERVED_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_reserved(&mut self) -> GPADC_RESERVED_W<16> {
        GPADC_RESERVED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_status](index.html) module"]
pub struct GPADC_REG_STATUS_SPEC;
impl crate::RegisterSpec for GPADC_REG_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_status::R](R) reader structure"]
impl crate::Readable for GPADC_REG_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_status::W](W) writer structure"]
impl crate::Writable for GPADC_REG_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_status to value 0"]
impl crate::Resettable for GPADC_REG_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

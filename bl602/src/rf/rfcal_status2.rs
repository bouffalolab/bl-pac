#[doc = "Register `rfcal_status2` reader"]
pub struct R(crate::R<RFCAL_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCAL_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCAL_STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCAL_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfcal_status2` writer"]
pub struct W(crate::W<RFCAL_STATUS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCAL_STATUS2_SPEC>;
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
impl From<crate::W<RFCAL_STATUS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCAL_STATUS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dl_rfcal_table_status` reader - "]
pub type DL_RFCAL_TABLE_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dl_rfcal_table_status` writer - "]
pub type DL_RFCAL_TABLE_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dl_rfcal_table_status(&self) -> DL_RFCAL_TABLE_STATUS_R {
        DL_RFCAL_TABLE_STATUS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn dl_rfcal_table_status(&mut self) -> DL_RFCAL_TABLE_STATUS_W<0> {
        DL_RFCAL_TABLE_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rfcal_status2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_status2](index.html) module"]
pub struct RFCAL_STATUS2_SPEC;
impl crate::RegisterSpec for RFCAL_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcal_status2::R](R) reader structure"]
impl crate::Readable for RFCAL_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcal_status2::W](W) writer structure"]
impl crate::Writable for RFCAL_STATUS2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rfcal_status2 to value 0"]
impl crate::Resettable for RFCAL_STATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

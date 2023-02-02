#[doc = "Register `PDS_TIME1` reader"]
pub struct R(crate::R<PDS_TIME1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_TIME1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_TIME1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_TIME1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_TIME1` writer"]
pub struct W(crate::W<PDS_TIME1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_TIME1_SPEC>;
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
impl From<crate::W<PDS_TIME1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_TIME1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_sleep_duration` reader - "]
pub type CR_SLEEP_DURATION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `cr_sleep_duration` writer - "]
pub type CR_SLEEP_DURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_TIME1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_sleep_duration(&self) -> CR_SLEEP_DURATION_R {
        CR_SLEEP_DURATION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_sleep_duration(&mut self) -> CR_SLEEP_DURATION_W<0> {
        CR_SLEEP_DURATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_TIME1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_time1](index.html) module"]
pub struct PDS_TIME1_SPEC;
impl crate::RegisterSpec for PDS_TIME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_time1::R](R) reader structure"]
impl crate::Readable for PDS_TIME1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_time1::W](W) writer structure"]
impl crate::Writable for PDS_TIME1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_TIME1 to value 0x0ca8"]
impl crate::Resettable for PDS_TIME1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0ca8;
}

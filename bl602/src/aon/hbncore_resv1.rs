#[doc = "Register `hbncore_resv1` reader"]
pub struct R(crate::R<HBNCORE_RESV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBNCORE_RESV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBNCORE_RESV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBNCORE_RESV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hbncore_resv1` writer"]
pub struct W(crate::W<HBNCORE_RESV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBNCORE_RESV1_SPEC>;
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
impl From<crate::W<HBNCORE_RESV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBNCORE_RESV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hbncore_resv1_data` reader - "]
pub type HBNCORE_RESV1_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `hbncore_resv1_data` writer - "]
pub type HBNCORE_RESV1_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBNCORE_RESV1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbncore_resv1_data(&self) -> HBNCORE_RESV1_DATA_R {
        HBNCORE_RESV1_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn hbncore_resv1_data(&mut self) -> HBNCORE_RESV1_DATA_W<0> {
        HBNCORE_RESV1_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "hbncore_resv1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbncore_resv1](index.html) module"]
pub struct HBNCORE_RESV1_SPEC;
impl crate::RegisterSpec for HBNCORE_RESV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbncore_resv1::R](R) reader structure"]
impl crate::Readable for HBNCORE_RESV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbncore_resv1::W](W) writer structure"]
impl crate::Writable for HBNCORE_RESV1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hbncore_resv1 to value 0xffff_ffff"]
impl crate::Resettable for HBNCORE_RESV1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

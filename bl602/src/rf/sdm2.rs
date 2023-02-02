#[doc = "Register `sdm2` reader"]
pub struct R(crate::R<SDM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sdm2` writer"]
pub struct W(crate::W<SDM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDM2_SPEC>;
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
impl From<crate::W<SDM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdmin` reader - "]
pub type LO_SDMIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `lo_sdmin` writer - "]
pub type LO_SDMIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDM2_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin(&self) -> LO_SDMIN_R {
        LO_SDMIN_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdmin(&mut self) -> LO_SDMIN_W<0> {
        LO_SDMIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdm2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdm2](index.html) module"]
pub struct SDM2_SPEC;
impl crate::RegisterSpec for SDM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdm2::R](R) reader structure"]
impl crate::Readable for SDM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdm2::W](W) writer structure"]
impl crate::Writable for SDM2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sdm2 to value 0"]
impl crate::Resettable for SDM2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

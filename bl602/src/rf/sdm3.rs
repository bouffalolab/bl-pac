#[doc = "Register `sdm3` reader"]
pub struct R(crate::R<SDM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sdm3` writer"]
pub struct W(crate::W<SDM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDM3_SPEC>;
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
impl From<crate::W<SDM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDM3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdmin_hw` reader - "]
pub type LO_SDMIN_HW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `lo_sdmin_hw` writer - "]
pub type LO_SDMIN_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDM3_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin_hw(&self) -> LO_SDMIN_HW_R {
        LO_SDMIN_HW_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdmin_hw(&mut self) -> LO_SDMIN_HW_W<0> {
        LO_SDMIN_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdm3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdm3](index.html) module"]
pub struct SDM3_SPEC;
impl crate::RegisterSpec for SDM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdm3::R](R) reader structure"]
impl crate::Readable for SDM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdm3::W](W) writer structure"]
impl crate::Writable for SDM3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sdm3 to value 0"]
impl crate::Resettable for SDM3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

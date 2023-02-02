#[doc = "Register `TPLCR2` reader"]
pub struct R(crate::R<TPLCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPLCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPLCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPLCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPLCR2` writer"]
pub struct W(crate::W<TPLCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPLCR2_SPEC>;
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
impl From<crate::W<TPLCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPLCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tplcr` reader - "]
pub type TPLCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tplcr` writer - "]
pub type TPLCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPLCR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tplcr(&self) -> TPLCR_R {
        TPLCR_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn tplcr(&mut self) -> TPLCR_W<0> {
        TPLCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TPLCR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tplcr2](index.html) module"]
pub struct TPLCR2_SPEC;
impl crate::RegisterSpec for TPLCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tplcr2::R](R) reader structure"]
impl crate::Readable for TPLCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tplcr2::W](W) writer structure"]
impl crate::Writable for TPLCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPLCR2 to value 0"]
impl crate::Resettable for TPLCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

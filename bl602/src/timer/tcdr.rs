#[doc = "Register `TCDR` reader"]
pub struct R(crate::R<TCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCDR` writer"]
pub struct W(crate::W<TCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCDR_SPEC>;
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
impl From<crate::W<TCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tcdr2` reader - "]
pub type TCDR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tcdr2` writer - "]
pub type TCDR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `tcdr3` reader - "]
pub type TCDR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tcdr3` writer - "]
pub type TCDR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `wcdr` reader - "]
pub type WCDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wcdr` writer - "]
pub type WCDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tcdr2(&self) -> TCDR2_R {
        TCDR2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tcdr3(&self) -> TCDR3_R {
        TCDR3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn wcdr(&self) -> WCDR_R {
        WCDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn tcdr2(&mut self) -> TCDR2_W<8> {
        TCDR2_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn tcdr3(&mut self) -> TCDR3_W<16> {
        TCDR3_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn wcdr(&mut self) -> WCDR_W<24> {
        WCDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCDR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcdr](index.html) module"]
pub struct TCDR_SPEC;
impl crate::RegisterSpec for TCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcdr::R](R) reader structure"]
impl crate::Readable for TCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcdr::W](W) writer structure"]
impl crate::Writable for TCDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCDR to value 0"]
impl crate::Resettable for TCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

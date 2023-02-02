#[doc = "Register `WMR` reader"]
pub struct R(crate::R<WMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WMR` writer"]
pub struct W(crate::W<WMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WMR_SPEC>;
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
impl From<crate::W<WMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wmr` reader - "]
pub type WMR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wmr` writer - "]
pub type WMR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WMR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wmr(&self) -> WMR_R {
        WMR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn wmr(&mut self) -> WMR_W<0> {
        WMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WMR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmr](index.html) module"]
pub struct WMR_SPEC;
impl crate::RegisterSpec for WMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wmr::R](R) reader structure"]
impl crate::Readable for WMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wmr::W](W) writer structure"]
impl crate::Writable for WMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WMR to value 0xffff"]
impl crate::Resettable for WMR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}

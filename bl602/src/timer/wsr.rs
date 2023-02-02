#[doc = "Register `WSR` reader"]
pub struct R(crate::R<WSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WSR` writer"]
pub struct W(crate::W<WSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WSR_SPEC>;
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
impl From<crate::W<WSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wts` reader - "]
pub type WTS_R = crate::BitReader<bool>;
#[doc = "Field `wts` writer - "]
pub type WTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, WSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wts(&self) -> WTS_R {
        WTS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wts(&mut self) -> WTS_W<0> {
        WTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WSR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wsr](index.html) module"]
pub struct WSR_SPEC;
impl crate::RegisterSpec for WSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wsr::R](R) reader structure"]
impl crate::Readable for WSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wsr::W](W) writer structure"]
impl crate::Writable for WSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WSR to value 0"]
impl crate::Resettable for WSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

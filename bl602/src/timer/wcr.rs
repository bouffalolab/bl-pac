#[doc = "Register `WCR` writer"]
pub struct W(crate::W<WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCR_SPEC>;
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
impl From<crate::W<WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wcr` writer - "]
pub type WCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wcr(&mut self) -> WCR_W<0> {
        WCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WCR.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](index.html) module"]
pub struct WCR_SPEC;
impl crate::RegisterSpec for WCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wcr::W](W) writer structure"]
impl crate::Writable for WCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

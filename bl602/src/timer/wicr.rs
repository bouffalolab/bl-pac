#[doc = "Register `WICR` writer"]
pub struct W(crate::W<WICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WICR_SPEC>;
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
impl From<crate::W<WICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wiclr` writer - "]
pub type WICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wiclr(&mut self) -> WICLR_W<0> {
        WICLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WICR.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wicr](index.html) module"]
pub struct WICR_SPEC;
impl crate::RegisterSpec for WICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wicr::W](W) writer structure"]
impl crate::Writable for WICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WICR to value 0"]
impl crate::Resettable for WICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

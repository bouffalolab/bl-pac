#[doc = "Register `WSAR` writer"]
pub struct W(crate::W<WSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WSAR_SPEC>;
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
impl From<crate::W<WSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wsar` writer - "]
pub type WSAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WSAR_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn wsar(&mut self) -> WSAR_W<0> {
        WSAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WSAR.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wsar](index.html) module"]
pub struct WSAR_SPEC;
impl crate::RegisterSpec for WSAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wsar::W](W) writer structure"]
impl crate::Writable for WSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WSAR to value 0"]
impl crate::Resettable for WSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `TCMR` reader"]
pub struct R(crate::R<TCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCMR` writer"]
pub struct W(crate::W<TCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCMR_SPEC>;
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
impl From<crate::W<TCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer2_mode` reader - "]
pub type TIMER2_MODE_R = crate::BitReader<bool>;
#[doc = "Field `timer2_mode` writer - "]
pub type TIMER2_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCMR_SPEC, bool, O>;
#[doc = "Field `timer3_mode` reader - "]
pub type TIMER3_MODE_R = crate::BitReader<bool>;
#[doc = "Field `timer3_mode` writer - "]
pub type TIMER3_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_mode(&self) -> TIMER2_MODE_R {
        TIMER2_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_mode(&self) -> TIMER3_MODE_R {
        TIMER3_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_mode(&mut self) -> TIMER2_MODE_W<1> {
        TIMER2_MODE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer3_mode(&mut self) -> TIMER3_MODE_W<2> {
        TIMER3_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCMR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcmr](index.html) module"]
pub struct TCMR_SPEC;
impl crate::RegisterSpec for TCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcmr::R](R) reader structure"]
impl crate::Readable for TCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcmr::W](W) writer structure"]
impl crate::Writable for TCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCMR to value 0"]
impl crate::Resettable for TCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

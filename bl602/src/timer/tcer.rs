#[doc = "Register `TCER` reader"]
pub struct R(crate::R<TCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCER` writer"]
pub struct W(crate::W<TCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCER_SPEC>;
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
impl From<crate::W<TCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer2_en` reader - "]
pub type TIMER2_EN_R = crate::BitReader<bool>;
#[doc = "Field `timer2_en` writer - "]
pub type TIMER2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCER_SPEC, bool, O>;
#[doc = "Field `timer3_en` reader - "]
pub type TIMER3_EN_R = crate::BitReader<bool>;
#[doc = "Field `timer3_en` writer - "]
pub type TIMER3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_en(&self) -> TIMER2_EN_R {
        TIMER2_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_en(&self) -> TIMER3_EN_R {
        TIMER3_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_en(&mut self) -> TIMER2_EN_W<1> {
        TIMER2_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer3_en(&mut self) -> TIMER3_EN_W<2> {
        TIMER3_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCER.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcer](index.html) module"]
pub struct TCER_SPEC;
impl crate::RegisterSpec for TCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcer::R](R) reader structure"]
impl crate::Readable for TCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcer::W](W) writer structure"]
impl crate::Writable for TCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCER to value 0"]
impl crate::Resettable for TCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

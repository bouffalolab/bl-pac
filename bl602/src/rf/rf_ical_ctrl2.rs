#[doc = "Register `rf_ical_ctrl2` reader"]
pub struct R(crate::R<RF_ICAL_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ICAL_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ICAL_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ICAL_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_ical_ctrl2` writer"]
pub struct W(crate::W<RF_ICAL_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ICAL_CTRL2_SPEC>;
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
impl From<crate::W<RF_ICAL_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ICAL_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_ical_period_n` reader - "]
pub type RF_ICAL_PERIOD_N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_ical_period_n` writer - "]
pub type RF_ICAL_PERIOD_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_ICAL_CTRL2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_ical_period_n(&self) -> RF_ICAL_PERIOD_N_R {
        RF_ICAL_PERIOD_N_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ical_period_n(&mut self) -> RF_ICAL_PERIOD_N_W<0> {
        RF_ICAL_PERIOD_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_ical_ctrl2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ical_ctrl2](index.html) module"]
pub struct RF_ICAL_CTRL2_SPEC;
impl crate::RegisterSpec for RF_ICAL_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ical_ctrl2::R](R) reader structure"]
impl crate::Readable for RF_ICAL_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ical_ctrl2::W](W) writer structure"]
impl crate::Writable for RF_ICAL_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_ical_ctrl2 to value 0"]
impl crate::Resettable for RF_ICAL_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

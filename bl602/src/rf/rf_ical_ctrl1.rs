#[doc = "Register `rf_ical_ctrl1` reader"]
pub struct R(crate::R<RF_ICAL_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ICAL_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ICAL_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ICAL_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_ical_ctrl1` writer"]
pub struct W(crate::W<RF_ICAL_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ICAL_CTRL1_SPEC>;
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
impl From<crate::W<RF_ICAL_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ICAL_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_ical_r_avg_n` reader - "]
pub type RF_ICAL_R_AVG_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_ical_r_avg_n` writer - "]
pub type RF_ICAL_R_AVG_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_ICAL_CTRL1_SPEC, u8, u8, 5, O>;
#[doc = "Field `rf_ical_r_os_q` reader - "]
pub type RF_ICAL_R_OS_Q_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_ical_r_os_q` writer - "]
pub type RF_ICAL_R_OS_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_ICAL_CTRL1_SPEC, u16, u16, 10, O>;
#[doc = "Field `rf_ical_r_os_i` reader - "]
pub type RF_ICAL_R_OS_I_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_ical_r_os_i` writer - "]
pub type RF_ICAL_R_OS_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_ICAL_CTRL1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ical_r_avg_n(&self) -> RF_ICAL_R_AVG_N_R {
        RF_ICAL_R_AVG_N_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_r_os_q(&self) -> RF_ICAL_R_OS_Q_R {
        RF_ICAL_R_OS_Q_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_r_os_i(&self) -> RF_ICAL_R_OS_I_R {
        RF_ICAL_R_OS_I_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ical_r_avg_n(&mut self) -> RF_ICAL_R_AVG_N_W<0> {
        RF_ICAL_R_AVG_N_W::new(self)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ical_r_os_q(&mut self) -> RF_ICAL_R_OS_Q_W<10> {
        RF_ICAL_R_OS_Q_W::new(self)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ical_r_os_i(&mut self) -> RF_ICAL_R_OS_I_W<20> {
        RF_ICAL_R_OS_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_ical_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ical_ctrl1](index.html) module"]
pub struct RF_ICAL_CTRL1_SPEC;
impl crate::RegisterSpec for RF_ICAL_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ical_ctrl1::R](R) reader structure"]
impl crate::Readable for RF_ICAL_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ical_ctrl1::W](W) writer structure"]
impl crate::Writable for RF_ICAL_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_ical_ctrl1 to value 0"]
impl crate::Resettable for RF_ICAL_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

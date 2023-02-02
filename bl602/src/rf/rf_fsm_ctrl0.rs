#[doc = "Register `rf_fsm_ctrl0` reader"]
pub struct R(crate::R<RF_FSM_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_FSM_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_FSM_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm_ctrl0` writer"]
pub struct W(crate::W<RF_FSM_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_CTRL0_SPEC>;
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
impl From<crate::W<RF_FSM_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_FSM_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_ch_ind_wifi` reader - "]
pub type RF_CH_IND_WIFI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_ch_ind_wifi` writer - "]
pub type RF_CH_IND_WIFI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL0_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn rf_ch_ind_wifi(&self) -> RF_CH_IND_WIFI_R {
        RF_CH_IND_WIFI_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ch_ind_wifi(&mut self) -> RF_CH_IND_WIFI_W<0> {
        RF_CH_IND_WIFI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_fsm_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl0](index.html) module"]
pub struct RF_FSM_CTRL0_SPEC;
impl crate::RegisterSpec for RF_FSM_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm_ctrl0::R](R) reader structure"]
impl crate::Readable for RF_FSM_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl0::W](W) writer structure"]
impl crate::Writable for RF_FSM_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_fsm_ctrl0 to value 0"]
impl crate::Resettable for RF_FSM_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

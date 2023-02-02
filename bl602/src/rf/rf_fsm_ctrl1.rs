#[doc = "Register `rf_fsm_ctrl1` reader"]
pub struct R(crate::R<RF_FSM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_FSM_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_FSM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm_ctrl1` writer"]
pub struct W(crate::W<RF_FSM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_CTRL1_SPEC>;
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
impl From<crate::W<RF_FSM_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_FSM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_fsm_lo_time` reader - "]
pub type RF_FSM_LO_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_fsm_lo_time` writer - "]
pub type RF_FSM_LO_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL1_SPEC, u16, u16, 16, O>;
#[doc = "Field `rf_fsm_lo_rdy` reader - "]
pub type RF_FSM_LO_RDY_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_lo_rdy` writer - "]
pub type RF_FSM_LO_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_FSM_CTRL1_SPEC, bool, O>;
#[doc = "Field `rf_fsm_lo_rdy_rst` reader - "]
pub type RF_FSM_LO_RDY_RST_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_lo_rdy_rst` writer - "]
pub type RF_FSM_LO_RDY_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL1_SPEC, bool, O>;
#[doc = "Field `rf_fsm_lo_rdy_4s_1` reader - "]
pub type RF_FSM_LO_RDY_4S_1_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_lo_rdy_4s_1` writer - "]
pub type RF_FSM_LO_RDY_4S_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL1_SPEC, bool, O>;
#[doc = "Field `rf_fsm_lo_rdy_sbclr` reader - "]
pub type RF_FSM_LO_RDY_SBCLR_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_lo_rdy_sbclr` writer - "]
pub type RF_FSM_LO_RDY_SBCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL1_SPEC, bool, O>;
#[doc = "Field `rf_fsm_pu_pa_dly_n` reader - "]
pub type RF_FSM_PU_PA_DLY_N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_fsm_pu_pa_dly_n` writer - "]
pub type RF_FSM_PU_PA_DLY_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_fsm_lo_time(&self) -> RF_FSM_LO_TIME_R {
        RF_FSM_LO_TIME_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy(&self) -> RF_FSM_LO_RDY_R {
        RF_FSM_LO_RDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_rst(&self) -> RF_FSM_LO_RDY_RST_R {
        RF_FSM_LO_RDY_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_4s_1(&self) -> RF_FSM_LO_RDY_4S_1_R {
        RF_FSM_LO_RDY_4S_1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_sbclr(&self) -> RF_FSM_LO_RDY_SBCLR_R {
        RF_FSM_LO_RDY_SBCLR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_pu_pa_dly_n(&self) -> RF_FSM_PU_PA_DLY_N_R {
        RF_FSM_PU_PA_DLY_N_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_lo_time(&mut self) -> RF_FSM_LO_TIME_W<0> {
        RF_FSM_LO_TIME_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_lo_rdy(&mut self) -> RF_FSM_LO_RDY_W<16> {
        RF_FSM_LO_RDY_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_lo_rdy_rst(&mut self) -> RF_FSM_LO_RDY_RST_W<17> {
        RF_FSM_LO_RDY_RST_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_lo_rdy_4s_1(&mut self) -> RF_FSM_LO_RDY_4S_1_W<18> {
        RF_FSM_LO_RDY_4S_1_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_lo_rdy_sbclr(&mut self) -> RF_FSM_LO_RDY_SBCLR_W<19> {
        RF_FSM_LO_RDY_SBCLR_W::new(self)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_pu_pa_dly_n(&mut self) -> RF_FSM_PU_PA_DLY_N_W<20> {
        RF_FSM_PU_PA_DLY_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_fsm_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl1](index.html) module"]
pub struct RF_FSM_CTRL1_SPEC;
impl crate::RegisterSpec for RF_FSM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm_ctrl1::R](R) reader structure"]
impl crate::Readable for RF_FSM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl1::W](W) writer structure"]
impl crate::Writable for RF_FSM_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_fsm_ctrl1 to value 0"]
impl crate::Resettable for RF_FSM_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

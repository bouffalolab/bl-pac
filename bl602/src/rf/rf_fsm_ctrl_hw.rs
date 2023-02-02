#[doc = "Register `rf_fsm_ctrl_hw` reader"]
pub struct R(crate::R<RF_FSM_CTRL_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_CTRL_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_FSM_CTRL_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_FSM_CTRL_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm_ctrl_hw` writer"]
pub struct W(crate::W<RF_FSM_CTRL_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_CTRL_HW_SPEC>;
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
impl From<crate::W<RF_FSM_CTRL_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_FSM_CTRL_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_fsm_ctrl_en` reader - "]
pub type RF_FSM_CTRL_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_ctrl_en` writer - "]
pub type RF_FSM_CTRL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `rf_fsm_t2r_cal_mode` reader - "]
pub type RF_FSM_T2R_CAL_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_fsm_t2r_cal_mode` writer - "]
pub type RF_FSM_T2R_CAL_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL_HW_SPEC, u8, u8, 2, O>;
#[doc = "Field `rf_fsm_state` reader - "]
pub type RF_FSM_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_fsm_state` writer - "]
pub type RF_FSM_STATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL_HW_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_rc_state_dbg` reader - "]
pub type RF_RC_STATE_DBG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_rc_state_dbg` writer - "]
pub type RF_RC_STATE_DBG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL_HW_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_rc_state_dbg_en` reader - "]
pub type RF_RC_STATE_DBG_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_rc_state_dbg_en` writer - "]
pub type RF_RC_STATE_DBG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `rf_fsm_st_int_sel` reader - "]
pub type RF_FSM_ST_INT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_fsm_st_int_sel` writer - "]
pub type RF_FSM_ST_INT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL_HW_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_fsm_st_int` reader - "]
pub type RF_FSM_ST_INT_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_st_int` writer - "]
pub type RF_FSM_ST_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_FSM_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `rf_fsm_st_int_clr` reader - "]
pub type RF_FSM_ST_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_st_int_clr` writer - "]
pub type RF_FSM_ST_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `rf_fsm_st_int_set` reader - "]
pub type RF_FSM_ST_INT_SET_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_st_int_set` writer - "]
pub type RF_FSM_ST_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `rf_rc_state_value` reader - "]
pub type RF_RC_STATE_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_rc_state_value` writer - "]
pub type RF_RC_STATE_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL_HW_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_fsm_ctrl_en(&self) -> RF_FSM_CTRL_EN_R {
        RF_FSM_CTRL_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rf_fsm_t2r_cal_mode(&self) -> RF_FSM_T2R_CAL_MODE_R {
        RF_FSM_T2R_CAL_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_fsm_state(&self) -> RF_FSM_STATE_R {
        RF_FSM_STATE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_rc_state_dbg(&self) -> RF_RC_STATE_DBG_R {
        RF_RC_STATE_DBG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rf_rc_state_dbg_en(&self) -> RF_RC_STATE_DBG_EN_R {
        RF_RC_STATE_DBG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_fsm_st_int_sel(&self) -> RF_FSM_ST_INT_SEL_R {
        RF_FSM_ST_INT_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_st_int(&self) -> RF_FSM_ST_INT_R {
        RF_FSM_ST_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rf_fsm_st_int_clr(&self) -> RF_FSM_ST_INT_CLR_R {
        RF_FSM_ST_INT_CLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rf_fsm_st_int_set(&self) -> RF_FSM_ST_INT_SET_R {
        RF_FSM_ST_INT_SET_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_rc_state_value(&self) -> RF_RC_STATE_VALUE_R {
        RF_RC_STATE_VALUE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_ctrl_en(&mut self) -> RF_FSM_CTRL_EN_W<1> {
        RF_FSM_CTRL_EN_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_t2r_cal_mode(&mut self) -> RF_FSM_T2R_CAL_MODE_W<2> {
        RF_FSM_T2R_CAL_MODE_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_state(&mut self) -> RF_FSM_STATE_W<4> {
        RF_FSM_STATE_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn rf_rc_state_dbg(&mut self) -> RF_RC_STATE_DBG_W<8> {
        RF_RC_STATE_DBG_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rf_rc_state_dbg_en(&mut self) -> RF_RC_STATE_DBG_EN_W<11> {
        RF_RC_STATE_DBG_EN_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_st_int_sel(&mut self) -> RF_FSM_ST_INT_SEL_W<12> {
        RF_FSM_ST_INT_SEL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_st_int(&mut self) -> RF_FSM_ST_INT_W<16> {
        RF_FSM_ST_INT_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_st_int_clr(&mut self) -> RF_FSM_ST_INT_CLR_W<20> {
        RF_FSM_ST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_st_int_set(&mut self) -> RF_FSM_ST_INT_SET_W<24> {
        RF_FSM_ST_INT_SET_W::new(self)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn rf_rc_state_value(&mut self) -> RF_RC_STATE_VALUE_W<28> {
        RF_RC_STATE_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl_hw](index.html) module"]
pub struct RF_FSM_CTRL_HW_SPEC;
impl crate::RegisterSpec for RF_FSM_CTRL_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm_ctrl_hw::R](R) reader structure"]
impl crate::Readable for RF_FSM_CTRL_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl_hw::W](W) writer structure"]
impl crate::Writable for RF_FSM_CTRL_HW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_fsm_ctrl_hw to value 0"]
impl crate::Resettable for RF_FSM_CTRL_HW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `rf_fsm_ctrl_sw` reader"]
pub struct R(crate::R<RF_FSM_CTRL_SW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_CTRL_SW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_FSM_CTRL_SW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_FSM_CTRL_SW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm_ctrl_sw` writer"]
pub struct W(crate::W<RF_FSM_CTRL_SW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_CTRL_SW_SPEC>;
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
impl From<crate::W<RF_FSM_CTRL_SW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_FSM_CTRL_SW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_fsm_sw_st` reader - "]
pub type RF_FSM_SW_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_fsm_sw_st` writer - "]
pub type RF_FSM_SW_ST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL_SW_SPEC, u8, u8, 5, O>;
#[doc = "Field `rf_fsm_sw_st_vld` reader - "]
pub type RF_FSM_SW_ST_VLD_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_sw_st_vld` writer - "]
pub type RF_FSM_SW_ST_VLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL_SW_SPEC, bool, O>;
#[doc = "Field `full_cal_en` reader - "]
pub type FULL_CAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `full_cal_en` writer - "]
pub type FULL_CAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_FSM_CTRL_SW_SPEC, bool, O>;
#[doc = "Field `inc_cal_timeout` reader - "]
pub type INC_CAL_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `inc_cal_timeout` writer - "]
pub type INC_CAL_TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL_SW_SPEC, bool, O>;
#[doc = "Field `lo_unlocked` reader - "]
pub type LO_UNLOCKED_R = crate::BitReader<bool>;
#[doc = "Field `lo_unlocked` writer - "]
pub type LO_UNLOCKED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_FSM_CTRL_SW_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_fsm_sw_st(&self) -> RF_FSM_SW_ST_R {
        RF_FSM_SW_ST_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rf_fsm_sw_st_vld(&self) -> RF_FSM_SW_ST_VLD_R {
        RF_FSM_SW_ST_VLD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn full_cal_en(&self) -> FULL_CAL_EN_R {
        FULL_CAL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_cal_timeout(&self) -> INC_CAL_TIMEOUT_R {
        INC_CAL_TIMEOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_unlocked(&self) -> LO_UNLOCKED_R {
        LO_UNLOCKED_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_sw_st(&mut self) -> RF_FSM_SW_ST_W<0> {
        RF_FSM_SW_ST_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_sw_st_vld(&mut self) -> RF_FSM_SW_ST_VLD_W<8> {
        RF_FSM_SW_ST_VLD_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn full_cal_en(&mut self) -> FULL_CAL_EN_W<12> {
        FULL_CAL_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn inc_cal_timeout(&mut self) -> INC_CAL_TIMEOUT_W<16> {
        INC_CAL_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn lo_unlocked(&mut self) -> LO_UNLOCKED_W<20> {
        LO_UNLOCKED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rfsm status reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl_sw](index.html) module"]
pub struct RF_FSM_CTRL_SW_SPEC;
impl crate::RegisterSpec for RF_FSM_CTRL_SW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm_ctrl_sw::R](R) reader structure"]
impl crate::Readable for RF_FSM_CTRL_SW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl_sw::W](W) writer structure"]
impl crate::Writable for RF_FSM_CTRL_SW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_fsm_ctrl_sw to value 0"]
impl crate::Resettable for RF_FSM_CTRL_SW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

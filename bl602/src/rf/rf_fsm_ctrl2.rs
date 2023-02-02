#[doc = "Register `rf_fsm_ctrl2` reader"]
pub struct R(crate::R<RF_FSM_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_FSM_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_FSM_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm_ctrl2` writer"]
pub struct W(crate::W<RF_FSM_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_CTRL2_SPEC>;
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
impl From<crate::W<RF_FSM_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_FSM_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_fsm_st_dbg` reader - "]
pub type RF_FSM_ST_DBG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_fsm_st_dbg` writer - "]
pub type RF_FSM_ST_DBG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL2_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_fsm_st_dbg_en` reader - "]
pub type RF_FSM_ST_DBG_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_st_dbg_en` writer - "]
pub type RF_FSM_ST_DBG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL2_SPEC, bool, O>;
#[doc = "Field `rf_trx_en_ble_4s` reader - "]
pub type RF_TRX_EN_BLE_4S_R = crate::BitReader<bool>;
#[doc = "Field `rf_trx_en_ble_4s` writer - "]
pub type RF_TRX_EN_BLE_4S_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL2_SPEC, bool, O>;
#[doc = "Field `rf_trx_sw_ble_4s` reader - "]
pub type RF_TRX_SW_BLE_4S_R = crate::BitReader<bool>;
#[doc = "Field `rf_trx_sw_ble_4s` writer - "]
pub type RF_TRX_SW_BLE_4S_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL2_SPEC, bool, O>;
#[doc = "Field `rf_trx_ble_4s_en` reader - "]
pub type RF_TRX_BLE_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_trx_ble_4s_en` writer - "]
pub type RF_TRX_BLE_4S_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_FSM_CTRL2_SPEC, bool, O>;
#[doc = "Field `rf_fsm_dfe_tx_dly_n` reader - "]
pub type RF_FSM_DFE_TX_DLY_N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_fsm_dfe_tx_dly_n` writer - "]
pub type RF_FSM_DFE_TX_DLY_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL2_SPEC, u16, u16, 10, O>;
#[doc = "Field `rf_fsm_dfe_rx_dly_n` reader - "]
pub type RF_FSM_DFE_RX_DLY_N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_fsm_dfe_rx_dly_n` writer - "]
pub type RF_FSM_DFE_RX_DLY_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_CTRL2_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg(&self) -> RF_FSM_ST_DBG_R {
        RF_FSM_ST_DBG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg_en(&self) -> RF_FSM_ST_DBG_EN_R {
        RF_FSM_ST_DBG_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rf_trx_en_ble_4s(&self) -> RF_TRX_EN_BLE_4S_R {
        RF_TRX_EN_BLE_4S_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rf_trx_sw_ble_4s(&self) -> RF_TRX_SW_BLE_4S_R {
        RF_TRX_SW_BLE_4S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rf_trx_ble_4s_en(&self) -> RF_TRX_BLE_4S_EN_R {
        RF_TRX_BLE_4S_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_fsm_dfe_tx_dly_n(&self) -> RF_FSM_DFE_TX_DLY_N_R {
        RF_FSM_DFE_TX_DLY_N_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_dfe_rx_dly_n(&self) -> RF_FSM_DFE_RX_DLY_N_R {
        RF_FSM_DFE_RX_DLY_N_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_st_dbg(&mut self) -> RF_FSM_ST_DBG_W<0> {
        RF_FSM_ST_DBG_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_st_dbg_en(&mut self) -> RF_FSM_ST_DBG_EN_W<3> {
        RF_FSM_ST_DBG_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rf_trx_en_ble_4s(&mut self) -> RF_TRX_EN_BLE_4S_W<4> {
        RF_TRX_EN_BLE_4S_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rf_trx_sw_ble_4s(&mut self) -> RF_TRX_SW_BLE_4S_W<5> {
        RF_TRX_SW_BLE_4S_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rf_trx_ble_4s_en(&mut self) -> RF_TRX_BLE_4S_EN_W<6> {
        RF_TRX_BLE_4S_EN_W::new(self)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_dfe_tx_dly_n(&mut self) -> RF_FSM_DFE_TX_DLY_N_W<10> {
        RF_FSM_DFE_TX_DLY_N_W::new(self)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_dfe_rx_dly_n(&mut self) -> RF_FSM_DFE_RX_DLY_N_W<20> {
        RF_FSM_DFE_RX_DLY_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_fsm_ctrl2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl2](index.html) module"]
pub struct RF_FSM_CTRL2_SPEC;
impl crate::RegisterSpec for RF_FSM_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm_ctrl2::R](R) reader structure"]
impl crate::Readable for RF_FSM_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl2::W](W) writer structure"]
impl crate::Writable for RF_FSM_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_fsm_ctrl2 to value 0"]
impl crate::Resettable for RF_FSM_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

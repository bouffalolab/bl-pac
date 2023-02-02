#[doc = "Register `rfif_dfe_ctrl0` reader"]
pub struct R(crate::R<RFIF_DFE_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIF_DFE_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIF_DFE_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIF_DFE_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfif_dfe_ctrl0` writer"]
pub struct W(crate::W<RFIF_DFE_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIF_DFE_CTRL0_SPEC>;
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
impl From<crate::W<RFIF_DFE_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIF_DFE_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rfckg_rxclk_4s_on` reader - "]
pub type RFCKG_RXCLK_4S_ON_R = crate::BitReader<bool>;
#[doc = "Field `rfckg_rxclk_4s_on` writer - "]
pub type RFCKG_RXCLK_4S_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `rfckg_txclk_4s_on` reader - "]
pub type RFCKG_TXCLK_4S_ON_R = crate::BitReader<bool>;
#[doc = "Field `rfckg_txclk_4s_on` writer - "]
pub type RFCKG_TXCLK_4S_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `rfckg_adc_afifo_inv` reader - "]
pub type RFCKG_ADC_AFIFO_INV_R = crate::BitReader<bool>;
#[doc = "Field `rfckg_adc_afifo_inv` writer - "]
pub type RFCKG_ADC_AFIFO_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `rfckg_adc_clkout_sel` reader - "]
pub type RFCKG_ADC_CLKOUT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `rfckg_adc_clkout_sel` writer - "]
pub type RFCKG_ADC_CLKOUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `rfckg_dac_afifo_inv` reader - "]
pub type RFCKG_DAC_AFIFO_INV_R = crate::BitReader<bool>;
#[doc = "Field `rfckg_dac_afifo_inv` writer - "]
pub type RFCKG_DAC_AFIFO_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `rx_dfe_en_4s` reader - "]
pub type RX_DFE_EN_4S_R = crate::BitReader<bool>;
#[doc = "Field `rx_dfe_en_4s` writer - "]
pub type RX_DFE_EN_4S_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `rx_dfe_en_4s_en` reader - "]
pub type RX_DFE_EN_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_dfe_en_4s_en` writer - "]
pub type RX_DFE_EN_4S_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `tx_dfe_en_4s` reader - "]
pub type TX_DFE_EN_4S_R = crate::BitReader<bool>;
#[doc = "Field `tx_dfe_en_4s` writer - "]
pub type TX_DFE_EN_4S_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `tx_dfe_en_4s_en` reader - "]
pub type TX_DFE_EN_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `tx_dfe_en_4s_en` writer - "]
pub type TX_DFE_EN_4S_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `rx_test_sel` reader - "]
pub type RX_TEST_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_test_sel` writer - "]
pub type RX_TEST_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tx_test_sel` reader - "]
pub type TX_TEST_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_test_sel` writer - "]
pub type TX_TEST_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_adc_clkout_inv_en` reader - "]
pub type PAD_ADC_CLKOUT_INV_EN_R = crate::BitReader<bool>;
#[doc = "Field `pad_adc_clkout_inv_en` writer - "]
pub type PAD_ADC_CLKOUT_INV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `pad_dac_clkout_inv_en` reader - "]
pub type PAD_DAC_CLKOUT_INV_EN_R = crate::BitReader<bool>;
#[doc = "Field `pad_dac_clkout_inv_en` writer - "]
pub type PAD_DAC_CLKOUT_INV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `rf_ch_ind_ble_4s` reader - "]
pub type RF_CH_IND_BLE_4S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_ch_ind_ble_4s` writer - "]
pub type RF_CH_IND_BLE_4S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, u8, u8, 7, O>;
#[doc = "Field `rf_ch_ind_ble_4s_en` reader - "]
pub type RF_CH_IND_BLE_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_ch_ind_ble_4s_en` writer - "]
pub type RF_CH_IND_BLE_4S_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `wifimode_4s` reader - "]
pub type WIFIMODE_4S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifimode_4s` writer - "]
pub type WIFIMODE_4S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `wifimode_4s_en` reader - "]
pub type WIFIMODE_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `wifimode_4s_en` writer - "]
pub type WIFIMODE_4S_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `bbmode_4s` reader - "]
pub type BBMODE_4S_R = crate::BitReader<bool>;
#[doc = "Field `bbmode_4s` writer - "]
pub type BBMODE_4S_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `bbmode_4s_en` reader - "]
pub type BBMODE_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `bbmode_4s_en` writer - "]
pub type BBMODE_4S_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, bool, O>;
#[doc = "Field `test_sel` reader - "]
pub type TEST_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `test_sel` writer - "]
pub type TEST_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFIF_DFE_CTRL0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_rxclk_4s_on(&self) -> RFCKG_RXCLK_4S_ON_R {
        RFCKG_RXCLK_4S_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_txclk_4s_on(&self) -> RFCKG_TXCLK_4S_ON_R {
        RFCKG_TXCLK_4S_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_adc_afifo_inv(&self) -> RFCKG_ADC_AFIFO_INV_R {
        RFCKG_ADC_AFIFO_INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfckg_adc_clkout_sel(&self) -> RFCKG_ADC_CLKOUT_SEL_R {
        RFCKG_ADC_CLKOUT_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rfckg_dac_afifo_inv(&self) -> RFCKG_DAC_AFIFO_INV_R {
        RFCKG_DAC_AFIFO_INV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_dfe_en_4s(&self) -> RX_DFE_EN_4S_R {
        RX_DFE_EN_4S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_dfe_en_4s_en(&self) -> RX_DFE_EN_4S_EN_R {
        RX_DFE_EN_4S_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_dfe_en_4s(&self) -> TX_DFE_EN_4S_R {
        TX_DFE_EN_4S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_dfe_en_4s_en(&self) -> TX_DFE_EN_4S_EN_R {
        TX_DFE_EN_4S_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rx_test_sel(&self) -> RX_TEST_SEL_R {
        RX_TEST_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn tx_test_sel(&self) -> TX_TEST_SEL_R {
        TX_TEST_SEL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pad_adc_clkout_inv_en(&self) -> PAD_ADC_CLKOUT_INV_EN_R {
        PAD_ADC_CLKOUT_INV_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pad_dac_clkout_inv_en(&self) -> PAD_DAC_CLKOUT_INV_EN_R {
        PAD_DAC_CLKOUT_INV_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:21"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s(&self) -> RF_CH_IND_BLE_4S_R {
        RF_CH_IND_BLE_4S_R::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s_en(&self) -> RF_CH_IND_BLE_4S_EN_R {
        RF_CH_IND_BLE_4S_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wifimode_4s(&self) -> WIFIMODE_4S_R {
        WIFIMODE_4S_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifimode_4s_en(&self) -> WIFIMODE_4S_EN_R {
        WIFIMODE_4S_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bbmode_4s(&self) -> BBMODE_4S_R {
        BBMODE_4S_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bbmode_4s_en(&self) -> BBMODE_4S_EN_R {
        BBMODE_4S_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn test_sel(&self) -> TEST_SEL_R {
        TEST_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rfckg_rxclk_4s_on(&mut self) -> RFCKG_RXCLK_4S_ON_W<0> {
        RFCKG_RXCLK_4S_ON_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rfckg_txclk_4s_on(&mut self) -> RFCKG_TXCLK_4S_ON_W<1> {
        RFCKG_TXCLK_4S_ON_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rfckg_adc_afifo_inv(&mut self) -> RFCKG_ADC_AFIFO_INV_W<2> {
        RFCKG_ADC_AFIFO_INV_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rfckg_adc_clkout_sel(&mut self) -> RFCKG_ADC_CLKOUT_SEL_W<3> {
        RFCKG_ADC_CLKOUT_SEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rfckg_dac_afifo_inv(&mut self) -> RFCKG_DAC_AFIFO_INV_W<4> {
        RFCKG_DAC_AFIFO_INV_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dfe_en_4s(&mut self) -> RX_DFE_EN_4S_W<5> {
        RX_DFE_EN_4S_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dfe_en_4s_en(&mut self) -> RX_DFE_EN_4S_EN_W<6> {
        RX_DFE_EN_4S_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dfe_en_4s(&mut self) -> TX_DFE_EN_4S_W<7> {
        TX_DFE_EN_4S_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dfe_en_4s_en(&mut self) -> TX_DFE_EN_4S_EN_W<8> {
        TX_DFE_EN_4S_EN_W::new(self)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn rx_test_sel(&mut self) -> RX_TEST_SEL_W<9> {
        RX_TEST_SEL_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn tx_test_sel(&mut self) -> TX_TEST_SEL_W<11> {
        TX_TEST_SEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pad_adc_clkout_inv_en(&mut self) -> PAD_ADC_CLKOUT_INV_EN_W<13> {
        PAD_ADC_CLKOUT_INV_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pad_dac_clkout_inv_en(&mut self) -> PAD_DAC_CLKOUT_INV_EN_W<14> {
        PAD_DAC_CLKOUT_INV_EN_W::new(self)
    }
    #[doc = "Bits 15:21"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ch_ind_ble_4s(&mut self) -> RF_CH_IND_BLE_4S_W<15> {
        RF_CH_IND_BLE_4S_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ch_ind_ble_4s_en(&mut self) -> RF_CH_IND_BLE_4S_EN_W<22> {
        RF_CH_IND_BLE_4S_EN_W::new(self)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    #[must_use]
    pub fn wifimode_4s(&mut self) -> WIFIMODE_4S_W<23> {
        WIFIMODE_4S_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn wifimode_4s_en(&mut self) -> WIFIMODE_4S_EN_W<25> {
        WIFIMODE_4S_EN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn bbmode_4s(&mut self) -> BBMODE_4S_W<26> {
        BBMODE_4S_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn bbmode_4s_en(&mut self) -> BBMODE_4S_EN_W<27> {
        BBMODE_4S_EN_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn test_sel(&mut self) -> TEST_SEL_W<28> {
        TEST_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rfif_dfe_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfif_dfe_ctrl0](index.html) module"]
pub struct RFIF_DFE_CTRL0_SPEC;
impl crate::RegisterSpec for RFIF_DFE_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfif_dfe_ctrl0::R](R) reader structure"]
impl crate::Readable for RFIF_DFE_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfif_dfe_ctrl0::W](W) writer structure"]
impl crate::Writable for RFIF_DFE_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rfif_dfe_ctrl0 to value 0"]
impl crate::Resettable for RFIF_DFE_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

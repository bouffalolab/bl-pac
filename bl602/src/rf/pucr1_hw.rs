#[doc = "Register `pucr1_hw` reader"]
pub struct R(crate::R<PUCR1_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR1_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCR1_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCR1_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pucr1_hw` writer"]
pub struct W(crate::W<PUCR1_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR1_HW_SPEC>;
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
impl From<crate::W<PUCR1_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCR1_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_sfreg_hw` reader - "]
pub type PU_SFREG_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_sfreg_hw` writer - "]
pub type PU_SFREG_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_lna_hw` reader - "]
pub type PU_LNA_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_lna_hw` writer - "]
pub type PU_LNA_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_rmxgm_hw` reader - "]
pub type PU_RMXGM_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rmxgm_hw` writer - "]
pub type PU_RMXGM_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_rmx_hw` reader - "]
pub type PU_RMX_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rmx_hw` writer - "]
pub type PU_RMX_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_rbb_hw` reader - "]
pub type PU_RBB_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rbb_hw` writer - "]
pub type PU_RBB_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_adda_ldo_hw` reader - "]
pub type PU_ADDA_LDO_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_adda_ldo_hw` writer - "]
pub type PU_ADDA_LDO_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `adc_clk_en_hw` reader - "]
pub type ADC_CLK_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `adc_clk_en_hw` writer - "]
pub type ADC_CLK_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_adc_hw` reader - "]
pub type PU_ADC_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_adc_hw` writer - "]
pub type PU_ADC_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_pa_hw` reader - "]
pub type PU_PA_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_pa_hw` writer - "]
pub type PU_PA_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_tmx_hw` reader - "]
pub type PU_TMX_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_tmx_hw` writer - "]
pub type PU_TMX_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_tbb_hw` reader - "]
pub type PU_TBB_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_tbb_hw` writer - "]
pub type PU_TBB_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_dac_hw` reader - "]
pub type PU_DAC_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_dac_hw` writer - "]
pub type PU_DAC_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_vco_hw` reader - "]
pub type PU_VCO_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_vco_hw` writer - "]
pub type PU_VCO_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_fbdv_hw` reader - "]
pub type PU_FBDV_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_fbdv_hw` writer - "]
pub type PU_FBDV_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_pfd_hw` reader - "]
pub type PU_PFD_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_pfd_hw` writer - "]
pub type PU_PFD_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_osmx_hw` reader - "]
pub type PU_OSMX_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_osmx_hw` writer - "]
pub type PU_OSMX_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_rxbuf_hw` reader - "]
pub type PU_RXBUF_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rxbuf_hw` writer - "]
pub type PU_RXBUF_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_txbuf_hw` reader - "]
pub type PU_TXBUF_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_txbuf_hw` writer - "]
pub type PU_TXBUF_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `trsw_en_hw` reader - "]
pub type TRSW_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `trsw_en_hw` writer - "]
pub type TRSW_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_pkdet_hw` reader - "]
pub type PU_PKDET_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_pkdet_hw` writer - "]
pub type PU_PKDET_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_rosdac_hw` reader - "]
pub type PU_ROSDAC_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rosdac_hw` writer - "]
pub type PU_ROSDAC_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
#[doc = "Field `pu_tosdac_hw` reader - "]
pub type PU_TOSDAC_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_tosdac_hw` writer - "]
pub type PU_TOSDAC_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_HW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_sfreg_hw(&self) -> PU_SFREG_HW_R {
        PU_SFREG_HW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_lna_hw(&self) -> PU_LNA_HW_R {
        PU_LNA_HW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_rmxgm_hw(&self) -> PU_RMXGM_HW_R {
        PU_RMXGM_HW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_rmx_hw(&self) -> PU_RMX_HW_R {
        PU_RMX_HW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_rbb_hw(&self) -> PU_RBB_HW_R {
        PU_RBB_HW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_adda_ldo_hw(&self) -> PU_ADDA_LDO_HW_R {
        PU_ADDA_LDO_HW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en_hw(&self) -> ADC_CLK_EN_HW_R {
        ADC_CLK_EN_HW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_adc_hw(&self) -> PU_ADC_HW_R {
        PU_ADC_HW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_pa_hw(&self) -> PU_PA_HW_R {
        PU_PA_HW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_tmx_hw(&self) -> PU_TMX_HW_R {
        PU_TMX_HW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_tbb_hw(&self) -> PU_TBB_HW_R {
        PU_TBB_HW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_dac_hw(&self) -> PU_DAC_HW_R {
        PU_DAC_HW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_vco_hw(&self) -> PU_VCO_HW_R {
        PU_VCO_HW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_fbdv_hw(&self) -> PU_FBDV_HW_R {
        PU_FBDV_HW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pfd_hw(&self) -> PU_PFD_HW_R {
        PU_PFD_HW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_osmx_hw(&self) -> PU_OSMX_HW_R {
        PU_OSMX_HW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_rxbuf_hw(&self) -> PU_RXBUF_HW_R {
        PU_RXBUF_HW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pu_txbuf_hw(&self) -> PU_TXBUF_HW_R {
        PU_TXBUF_HW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn trsw_en_hw(&self) -> TRSW_EN_HW_R {
        TRSW_EN_HW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pu_pkdet_hw(&self) -> PU_PKDET_HW_R {
        PU_PKDET_HW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_rosdac_hw(&self) -> PU_ROSDAC_HW_R {
        PU_ROSDAC_HW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_tosdac_hw(&self) -> PU_TOSDAC_HW_R {
        PU_TOSDAC_HW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_sfreg_hw(&mut self) -> PU_SFREG_HW_W<0> {
        PU_SFREG_HW_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pu_lna_hw(&mut self) -> PU_LNA_HW_W<8> {
        PU_LNA_HW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rmxgm_hw(&mut self) -> PU_RMXGM_HW_W<9> {
        PU_RMXGM_HW_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rmx_hw(&mut self) -> PU_RMX_HW_W<10> {
        PU_RMX_HW_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rbb_hw(&mut self) -> PU_RBB_HW_W<11> {
        PU_RBB_HW_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pu_adda_ldo_hw(&mut self) -> PU_ADDA_LDO_HW_W<12> {
        PU_ADDA_LDO_HW_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_en_hw(&mut self) -> ADC_CLK_EN_HW_W<13> {
        ADC_CLK_EN_HW_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pu_adc_hw(&mut self) -> PU_ADC_HW_W<14> {
        PU_ADC_HW_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pu_pa_hw(&mut self) -> PU_PA_HW_W<16> {
        PU_PA_HW_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pu_tmx_hw(&mut self) -> PU_TMX_HW_W<17> {
        PU_TMX_HW_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pu_tbb_hw(&mut self) -> PU_TBB_HW_W<18> {
        PU_TBB_HW_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pu_dac_hw(&mut self) -> PU_DAC_HW_W<19> {
        PU_DAC_HW_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pu_vco_hw(&mut self) -> PU_VCO_HW_W<20> {
        PU_VCO_HW_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pu_fbdv_hw(&mut self) -> PU_FBDV_HW_W<21> {
        PU_FBDV_HW_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pu_pfd_hw(&mut self) -> PU_PFD_HW_W<22> {
        PU_PFD_HW_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pu_osmx_hw(&mut self) -> PU_OSMX_HW_W<23> {
        PU_OSMX_HW_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rxbuf_hw(&mut self) -> PU_RXBUF_HW_W<24> {
        PU_RXBUF_HW_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn pu_txbuf_hw(&mut self) -> PU_TXBUF_HW_W<25> {
        PU_TXBUF_HW_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn trsw_en_hw(&mut self) -> TRSW_EN_HW_W<26> {
        TRSW_EN_HW_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn pu_pkdet_hw(&mut self) -> PU_PKDET_HW_W<28> {
        PU_PKDET_HW_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rosdac_hw(&mut self) -> PU_ROSDAC_HW_W<29> {
        PU_ROSDAC_HW_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pu_tosdac_hw(&mut self) -> PU_TOSDAC_HW_W<31> {
        PU_TOSDAC_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "read only from hardware logic\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr1_hw](index.html) module"]
pub struct PUCR1_HW_SPEC;
impl crate::RegisterSpec for PUCR1_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr1_hw::R](R) reader structure"]
impl crate::Readable for PUCR1_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr1_hw::W](W) writer structure"]
impl crate::Writable for PUCR1_HW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pucr1_hw to value 0"]
impl crate::Resettable for PUCR1_HW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

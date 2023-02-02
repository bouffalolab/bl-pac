#[doc = "Register `pucr1` reader"]
pub struct R(crate::R<PUCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pucr1` writer"]
pub struct W(crate::W<PUCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR1_SPEC>;
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
impl From<crate::W<PUCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_sfreg` reader - "]
pub type PU_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `pu_sfreg` writer - "]
pub type PU_SFREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_lna` reader - "]
pub type PU_LNA_R = crate::BitReader<bool>;
#[doc = "Field `pu_lna` writer - "]
pub type PU_LNA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_rmxgm` reader - "]
pub type PU_RMXGM_R = crate::BitReader<bool>;
#[doc = "Field `pu_rmxgm` writer - "]
pub type PU_RMXGM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_rmx` reader - "]
pub type PU_RMX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rmx` writer - "]
pub type PU_RMX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_rbb` reader - "]
pub type PU_RBB_R = crate::BitReader<bool>;
#[doc = "Field `pu_rbb` writer - "]
pub type PU_RBB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_adda_ldo` reader - "]
pub type PU_ADDA_LDO_R = crate::BitReader<bool>;
#[doc = "Field `pu_adda_ldo` writer - "]
pub type PU_ADDA_LDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `adc_clk_en` reader - "]
pub type ADC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `adc_clk_en` writer - "]
pub type ADC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_adc` reader - "]
pub type PU_ADC_R = crate::BitReader<bool>;
#[doc = "Field `pu_adc` writer - "]
pub type PU_ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_op_atest` reader - "]
pub type PU_OP_ATEST_R = crate::BitReader<bool>;
#[doc = "Field `pu_op_atest` writer - "]
pub type PU_OP_ATEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_pa` reader - "]
pub type PU_PA_R = crate::BitReader<bool>;
#[doc = "Field `pu_pa` writer - "]
pub type PU_PA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_tmx` reader - "]
pub type PU_TMX_R = crate::BitReader<bool>;
#[doc = "Field `pu_tmx` writer - "]
pub type PU_TMX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_tbb` reader - "]
pub type PU_TBB_R = crate::BitReader<bool>;
#[doc = "Field `pu_tbb` writer - "]
pub type PU_TBB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_dac` reader - "]
pub type PU_DAC_R = crate::BitReader<bool>;
#[doc = "Field `pu_dac` writer - "]
pub type PU_DAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_vco` reader - "]
pub type PU_VCO_R = crate::BitReader<bool>;
#[doc = "Field `pu_vco` writer - "]
pub type PU_VCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_fbdv` reader - "]
pub type PU_FBDV_R = crate::BitReader<bool>;
#[doc = "Field `pu_fbdv` writer - "]
pub type PU_FBDV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_pfd` reader - "]
pub type PU_PFD_R = crate::BitReader<bool>;
#[doc = "Field `pu_pfd` writer - "]
pub type PU_PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_osmx` reader - "]
pub type PU_OSMX_R = crate::BitReader<bool>;
#[doc = "Field `pu_osmx` writer - "]
pub type PU_OSMX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_rxbuf` reader - "]
pub type PU_RXBUF_R = crate::BitReader<bool>;
#[doc = "Field `pu_rxbuf` writer - "]
pub type PU_RXBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_txbuf` reader - "]
pub type PU_TXBUF_R = crate::BitReader<bool>;
#[doc = "Field `pu_txbuf` writer - "]
pub type PU_TXBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `trsw_en` reader - "]
pub type TRSW_EN_R = crate::BitReader<bool>;
#[doc = "Field `trsw_en` writer - "]
pub type TRSW_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_pkdet` reader - "]
pub type PU_PKDET_R = crate::BitReader<bool>;
#[doc = "Field `pu_pkdet` writer - "]
pub type PU_PKDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_rosdac` reader - "]
pub type PU_ROSDAC_R = crate::BitReader<bool>;
#[doc = "Field `pu_rosdac` writer - "]
pub type PU_ROSDAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_pwrmx` reader - "]
pub type PU_PWRMX_R = crate::BitReader<bool>;
#[doc = "Field `pu_pwrmx` writer - "]
pub type PU_PWRMX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
#[doc = "Field `pu_tosdac` reader - "]
pub type PU_TOSDAC_R = crate::BitReader<bool>;
#[doc = "Field `pu_tosdac` writer - "]
pub type PU_TOSDAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_sfreg(&self) -> PU_SFREG_R {
        PU_SFREG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_lna(&self) -> PU_LNA_R {
        PU_LNA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_rmxgm(&self) -> PU_RMXGM_R {
        PU_RMXGM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_rmx(&self) -> PU_RMX_R {
        PU_RMX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_rbb(&self) -> PU_RBB_R {
        PU_RBB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_adda_ldo(&self) -> PU_ADDA_LDO_R {
        PU_ADDA_LDO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en(&self) -> ADC_CLK_EN_R {
        ADC_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_adc(&self) -> PU_ADC_R {
        PU_ADC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pu_op_atest(&self) -> PU_OP_ATEST_R {
        PU_OP_ATEST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_pa(&self) -> PU_PA_R {
        PU_PA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_tmx(&self) -> PU_TMX_R {
        PU_TMX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_tbb(&self) -> PU_TBB_R {
        PU_TBB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_dac(&self) -> PU_DAC_R {
        PU_DAC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_vco(&self) -> PU_VCO_R {
        PU_VCO_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_fbdv(&self) -> PU_FBDV_R {
        PU_FBDV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pfd(&self) -> PU_PFD_R {
        PU_PFD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_osmx(&self) -> PU_OSMX_R {
        PU_OSMX_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_rxbuf(&self) -> PU_RXBUF_R {
        PU_RXBUF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pu_txbuf(&self) -> PU_TXBUF_R {
        PU_TXBUF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn trsw_en(&self) -> TRSW_EN_R {
        TRSW_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pu_pkdet(&self) -> PU_PKDET_R {
        PU_PKDET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_rosdac(&self) -> PU_ROSDAC_R {
        PU_ROSDAC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_pwrmx(&self) -> PU_PWRMX_R {
        PU_PWRMX_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_tosdac(&self) -> PU_TOSDAC_R {
        PU_TOSDAC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_sfreg(&mut self) -> PU_SFREG_W<0> {
        PU_SFREG_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pu_lna(&mut self) -> PU_LNA_W<8> {
        PU_LNA_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rmxgm(&mut self) -> PU_RMXGM_W<9> {
        PU_RMXGM_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rmx(&mut self) -> PU_RMX_W<10> {
        PU_RMX_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rbb(&mut self) -> PU_RBB_W<11> {
        PU_RBB_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pu_adda_ldo(&mut self) -> PU_ADDA_LDO_W<12> {
        PU_ADDA_LDO_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_en(&mut self) -> ADC_CLK_EN_W<13> {
        ADC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pu_adc(&mut self) -> PU_ADC_W<14> {
        PU_ADC_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pu_op_atest(&mut self) -> PU_OP_ATEST_W<15> {
        PU_OP_ATEST_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pu_pa(&mut self) -> PU_PA_W<16> {
        PU_PA_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pu_tmx(&mut self) -> PU_TMX_W<17> {
        PU_TMX_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pu_tbb(&mut self) -> PU_TBB_W<18> {
        PU_TBB_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pu_dac(&mut self) -> PU_DAC_W<19> {
        PU_DAC_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pu_vco(&mut self) -> PU_VCO_W<20> {
        PU_VCO_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pu_fbdv(&mut self) -> PU_FBDV_W<21> {
        PU_FBDV_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pu_pfd(&mut self) -> PU_PFD_W<22> {
        PU_PFD_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pu_osmx(&mut self) -> PU_OSMX_W<23> {
        PU_OSMX_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rxbuf(&mut self) -> PU_RXBUF_W<24> {
        PU_RXBUF_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn pu_txbuf(&mut self) -> PU_TXBUF_W<25> {
        PU_TXBUF_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn trsw_en(&mut self) -> TRSW_EN_W<26> {
        TRSW_EN_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn pu_pkdet(&mut self) -> PU_PKDET_W<28> {
        PU_PKDET_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rosdac(&mut self) -> PU_ROSDAC_W<29> {
        PU_ROSDAC_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pu_pwrmx(&mut self) -> PU_PWRMX_W<30> {
        PU_PWRMX_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pu_tosdac(&mut self) -> PU_TOSDAC_W<31> {
        PU_TOSDAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pucr1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr1](index.html) module"]
pub struct PUCR1_SPEC;
impl crate::RegisterSpec for PUCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr1::R](R) reader structure"]
impl crate::Readable for PUCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr1::W](W) writer structure"]
impl crate::Writable for PUCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pucr1 to value 0"]
impl crate::Resettable for PUCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

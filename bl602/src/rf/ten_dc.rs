#[doc = "Register `ten_dc` reader"]
pub struct R(crate::R<TEN_DC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEN_DC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEN_DC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEN_DC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ten_dc` writer"]
pub struct W(crate::W<TEN_DC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEN_DC_SPEC>;
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
impl From<crate::W<TEN_DC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEN_DC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmux` reader - "]
pub type TMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tmux` writer - "]
pub type TMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEN_DC_SPEC, u8, u8, 3, O>;
#[doc = "Field `dc_tp_en` reader - "]
pub type DC_TP_EN_R = crate::BitReader<bool>;
#[doc = "Field `dc_tp_en` writer - "]
pub type DC_TP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `dc_tp_clkpll_en` reader - "]
pub type DC_TP_CLKPLL_EN_R = crate::BitReader<bool>;
#[doc = "Field `dc_tp_clkpll_en` writer - "]
pub type DC_TP_CLKPLL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_clkpll` reader - "]
pub type TEN_CLKPLL_R = crate::BitReader<bool>;
#[doc = "Field `ten_clkpll` writer - "]
pub type TEN_CLKPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_clkpll_sfreg` reader - "]
pub type TEN_CLKPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `ten_clkpll_sfreg` writer - "]
pub type TEN_CLKPLL_SFREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_rrf_0` reader - "]
pub type TEN_RRF_0_R = crate::BitReader<bool>;
#[doc = "Field `ten_rrf_0` writer - "]
pub type TEN_RRF_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_rrf_1` reader - "]
pub type TEN_RRF_1_R = crate::BitReader<bool>;
#[doc = "Field `ten_rrf_1` writer - "]
pub type TEN_RRF_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_pa` reader - "]
pub type TEN_PA_R = crate::BitReader<bool>;
#[doc = "Field `ten_pa` writer - "]
pub type TEN_PA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_tmx` reader - "]
pub type TEN_TMX_R = crate::BitReader<bool>;
#[doc = "Field `ten_tmx` writer - "]
pub type TEN_TMX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_tia` reader - "]
pub type TEN_TIA_R = crate::BitReader<bool>;
#[doc = "Field `ten_tia` writer - "]
pub type TEN_TIA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_bq` reader - "]
pub type TEN_BQ_R = crate::BitReader<bool>;
#[doc = "Field `ten_bq` writer - "]
pub type TEN_BQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_atest` reader - "]
pub type TEN_ATEST_R = crate::BitReader<bool>;
#[doc = "Field `ten_atest` writer - "]
pub type TEN_ATEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_tbb` reader - "]
pub type TEN_TBB_R = crate::BitReader<bool>;
#[doc = "Field `ten_tbb` writer - "]
pub type TEN_TBB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_adc` reader - "]
pub type TEN_ADC_R = crate::BitReader<bool>;
#[doc = "Field `ten_adc` writer - "]
pub type TEN_ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_dac_i` reader - "]
pub type TEN_DAC_I_R = crate::BitReader<bool>;
#[doc = "Field `ten_dac_i` writer - "]
pub type TEN_DAC_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_dac_q` reader - "]
pub type TEN_DAC_Q_R = crate::BitReader<bool>;
#[doc = "Field `ten_dac_q` writer - "]
pub type TEN_DAC_Q_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_vco` reader - "]
pub type TEN_VCO_R = crate::BitReader<bool>;
#[doc = "Field `ten_vco` writer - "]
pub type TEN_VCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_pfdcp` reader - "]
pub type TEN_PFDCP_R = crate::BitReader<bool>;
#[doc = "Field `ten_pfdcp` writer - "]
pub type TEN_PFDCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_lf` reader - "]
pub type TEN_LF_R = crate::BitReader<bool>;
#[doc = "Field `ten_lf` writer - "]
pub type TEN_LF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
#[doc = "Field `ten_lodist` reader - "]
pub type TEN_LODIST_R = crate::BitReader<bool>;
#[doc = "Field `ten_lodist` writer - "]
pub type TEN_LODIST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_DC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux(&self) -> TMUX_R {
        TMUX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dc_tp_en(&self) -> DC_TP_EN_R {
        DC_TP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dc_tp_clkpll_en(&self) -> DC_TP_CLKPLL_EN_R {
        DC_TP_CLKPLL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_clkpll(&self) -> TEN_CLKPLL_R {
        TEN_CLKPLL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_clkpll_sfreg(&self) -> TEN_CLKPLL_SFREG_R {
        TEN_CLKPLL_SFREG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_rrf_0(&self) -> TEN_RRF_0_R {
        TEN_RRF_0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ten_rrf_1(&self) -> TEN_RRF_1_R {
        TEN_RRF_1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ten_pa(&self) -> TEN_PA_R {
        TEN_PA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ten_tmx(&self) -> TEN_TMX_R {
        TEN_TMX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_tia(&self) -> TEN_TIA_R {
        TEN_TIA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_bq(&self) -> TEN_BQ_R {
        TEN_BQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_atest(&self) -> TEN_ATEST_R {
        TEN_ATEST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_tbb(&self) -> TEN_TBB_R {
        TEN_TBB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adc(&self) -> TEN_ADC_R {
        TEN_ADC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_dac_i(&self) -> TEN_DAC_I_R {
        TEN_DAC_I_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_dac_q(&self) -> TEN_DAC_Q_R {
        TEN_DAC_Q_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_vco(&self) -> TEN_VCO_R {
        TEN_VCO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pfdcp(&self) -> TEN_PFDCP_R {
        TEN_PFDCP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_lf(&self) -> TEN_LF_R {
        TEN_LF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&self) -> TEN_LODIST_R {
        TEN_LODIST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn tmux(&mut self) -> TMUX_W<0> {
        TMUX_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dc_tp_en(&mut self) -> DC_TP_EN_W<3> {
        DC_TP_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dc_tp_clkpll_en(&mut self) -> DC_TP_CLKPLL_EN_W<4> {
        DC_TP_CLKPLL_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ten_clkpll(&mut self) -> TEN_CLKPLL_W<8> {
        TEN_CLKPLL_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ten_clkpll_sfreg(&mut self) -> TEN_CLKPLL_SFREG_W<9> {
        TEN_CLKPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ten_rrf_0(&mut self) -> TEN_RRF_0_W<12> {
        TEN_RRF_0_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ten_rrf_1(&mut self) -> TEN_RRF_1_W<13> {
        TEN_RRF_1_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ten_pa(&mut self) -> TEN_PA_W<14> {
        TEN_PA_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ten_tmx(&mut self) -> TEN_TMX_W<15> {
        TEN_TMX_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ten_tia(&mut self) -> TEN_TIA_W<16> {
        TEN_TIA_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ten_bq(&mut self) -> TEN_BQ_W<17> {
        TEN_BQ_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ten_atest(&mut self) -> TEN_ATEST_W<18> {
        TEN_ATEST_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ten_tbb(&mut self) -> TEN_TBB_W<19> {
        TEN_TBB_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ten_adc(&mut self) -> TEN_ADC_W<20> {
        TEN_ADC_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dac_i(&mut self) -> TEN_DAC_I_W<21> {
        TEN_DAC_I_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dac_q(&mut self) -> TEN_DAC_Q_W<22> {
        TEN_DAC_Q_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn ten_vco(&mut self) -> TEN_VCO_W<24> {
        TEN_VCO_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn ten_pfdcp(&mut self) -> TEN_PFDCP_W<25> {
        TEN_PFDCP_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn ten_lf(&mut self) -> TEN_LF_W<26> {
        TEN_LF_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn ten_lodist(&mut self) -> TEN_LODIST_W<27> {
        TEN_LODIST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dc test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ten_dc](index.html) module"]
pub struct TEN_DC_SPEC;
impl crate::RegisterSpec for TEN_DC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ten_dc::R](R) reader structure"]
impl crate::Readable for TEN_DC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ten_dc::W](W) writer structure"]
impl crate::Writable for TEN_DC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ten_dc to value 0"]
impl crate::Resettable for TEN_DC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

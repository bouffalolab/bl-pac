#[doc = "Register `PDS_CTL` reader"]
pub struct R(crate::R<PDS_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_CTL` writer"]
pub struct W(crate::W<PDS_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_CTL_SPEC>;
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
impl From<crate::W<PDS_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pds_start_ps` reader - "]
pub type PDS_START_PS_R = crate::BitReader<bool>;
#[doc = "Field `pds_start_ps` writer - "]
pub type PDS_START_PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_sleep_forever` reader - "]
pub type CR_SLEEP_FOREVER_R = crate::BitReader<bool>;
#[doc = "Field `cr_sleep_forever` writer - "]
pub type CR_SLEEP_FOREVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_xtal_force_off` reader - "]
pub type CR_XTAL_FORCE_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_xtal_force_off` writer - "]
pub type CR_XTAL_FORCE_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_wifi_pds_save_state` reader - "]
pub type CR_WIFI_PDS_SAVE_STATE_R = crate::BitReader<bool>;
#[doc = "Field `cr_wifi_pds_save_state` writer - "]
pub type CR_WIFI_PDS_SAVE_STATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_pd_dcdc18` reader - "]
pub type CR_PDS_PD_DCDC18_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pd_dcdc18` writer - "]
pub type CR_PDS_PD_DCDC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_pd_bg_sys` reader - "]
pub type CR_PDS_PD_BG_SYS_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pd_bg_sys` writer - "]
pub type CR_PDS_PD_BG_SYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_gate_clk` reader - "]
pub type CR_PDS_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_gate_clk` writer - "]
pub type CR_PDS_GATE_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_mem_stby` reader - "]
pub type CR_PDS_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_mem_stby` writer - "]
pub type CR_PDS_MEM_STBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_iso_en` reader - "]
pub type CR_PDS_ISO_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_iso_en` writer - "]
pub type CR_PDS_ISO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_wait_xtal_rdy` reader - "]
pub type CR_PDS_WAIT_XTAL_RDY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_wait_xtal_rdy` writer - "]
pub type CR_PDS_WAIT_XTAL_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_pwr_off` reader - "]
pub type CR_PDS_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pwr_off` writer - "]
pub type CR_PDS_PWR_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_pd_xtal` reader - "]
pub type CR_PDS_PD_XTAL_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pd_xtal` writer - "]
pub type CR_PDS_PD_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_soc_enb_force_on` reader - "]
pub type CR_PDS_SOC_ENB_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_soc_enb_force_on` writer - "]
pub type CR_PDS_SOC_ENB_FORCE_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_rst_soc_en` reader - "]
pub type CR_PDS_RST_SOC_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_rst_soc_en` writer - "]
pub type CR_PDS_RST_SOC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_rc32m_off_dis` reader - "]
pub type CR_PDS_RC32M_OFF_DIS_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_rc32m_off_dis` writer - "]
pub type CR_PDS_RC32M_OFF_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_ldo_vsel_en` reader - "]
pub type CR_PDS_LDO_VSEL_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_ldo_vsel_en` writer - "]
pub type CR_PDS_LDO_VSEL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_np_wfi_mask` reader - "]
pub type CR_NP_WFI_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_np_wfi_mask` writer - "]
pub type CR_NP_WFI_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_pd_ldo11` reader - "]
pub type CR_PDS_PD_LDO11_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pd_ldo11` writer - "]
pub type CR_PDS_PD_LDO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL_SPEC, bool, O>;
#[doc = "Field `cr_pds_ldo_vol` reader - "]
pub type CR_PDS_LDO_VOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ldo_vol` writer - "]
pub type CR_PDS_LDO_VOL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_pds_ctrl_rf` reader - "]
pub type CR_PDS_CTRL_RF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ctrl_rf` writer - "]
pub type CR_PDS_CTRL_RF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_pds_ctrl_pll` reader - "]
pub type CR_PDS_CTRL_PLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ctrl_pll` writer - "]
pub type CR_PDS_CTRL_PLL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_CTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pds_start_ps(&self) -> PDS_START_PS_R {
        PDS_START_PS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_sleep_forever(&self) -> CR_SLEEP_FOREVER_R {
        CR_SLEEP_FOREVER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xtal_force_off(&self) -> CR_XTAL_FORCE_OFF_R {
        CR_XTAL_FORCE_OFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_wifi_pds_save_state(&self) -> CR_WIFI_PDS_SAVE_STATE_R {
        CR_WIFI_PDS_SAVE_STATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc18(&self) -> CR_PDS_PD_DCDC18_R {
        CR_PDS_PD_DCDC18_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_pd_bg_sys(&self) -> CR_PDS_PD_BG_SYS_R {
        CR_PDS_PD_BG_SYS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_gate_clk(&self) -> CR_PDS_GATE_CLK_R {
        CR_PDS_GATE_CLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_mem_stby(&self) -> CR_PDS_MEM_STBY_R {
        CR_PDS_MEM_STBY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_iso_en(&self) -> CR_PDS_ISO_EN_R {
        CR_PDS_ISO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wait_xtal_rdy(&self) -> CR_PDS_WAIT_XTAL_RDY_R {
        CR_PDS_WAIT_XTAL_RDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_pwr_off(&self) -> CR_PDS_PWR_OFF_R {
        CR_PDS_PWR_OFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_pd_xtal(&self) -> CR_PDS_PD_XTAL_R {
        CR_PDS_PD_XTAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_soc_enb_force_on(&self) -> CR_PDS_SOC_ENB_FORCE_ON_R {
        CR_PDS_SOC_ENB_FORCE_ON_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_rst_soc_en(&self) -> CR_PDS_RST_SOC_EN_R {
        CR_PDS_RST_SOC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_rc32m_off_dis(&self) -> CR_PDS_RC32M_OFF_DIS_R {
        CR_PDS_RC32M_OFF_DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_ldo_vsel_en(&self) -> CR_PDS_LDO_VSEL_EN_R {
        CR_PDS_LDO_VSEL_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_np_wfi_mask(&self) -> CR_NP_WFI_MASK_R {
        CR_NP_WFI_MASK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_pd_ldo11(&self) -> CR_PDS_PD_LDO11_R {
        CR_PDS_PD_LDO11_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_pds_ldo_vol(&self) -> CR_PDS_LDO_VOL_R {
        CR_PDS_LDO_VOL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn cr_pds_ctrl_rf(&self) -> CR_PDS_CTRL_RF_R {
        CR_PDS_CTRL_RF_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn cr_pds_ctrl_pll(&self) -> CR_PDS_CTRL_PLL_R {
        CR_PDS_CTRL_PLL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pds_start_ps(&mut self) -> PDS_START_PS_W<0> {
        PDS_START_PS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_sleep_forever(&mut self) -> CR_SLEEP_FOREVER_W<1> {
        CR_SLEEP_FOREVER_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_xtal_force_off(&mut self) -> CR_XTAL_FORCE_OFF_W<2> {
        CR_XTAL_FORCE_OFF_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wifi_pds_save_state(&mut self) -> CR_WIFI_PDS_SAVE_STATE_W<3> {
        CR_WIFI_PDS_SAVE_STATE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_dcdc18(&mut self) -> CR_PDS_PD_DCDC18_W<4> {
        CR_PDS_PD_DCDC18_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_bg_sys(&mut self) -> CR_PDS_PD_BG_SYS_W<5> {
        CR_PDS_PD_BG_SYS_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gate_clk(&mut self) -> CR_PDS_GATE_CLK_W<8> {
        CR_PDS_GATE_CLK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_mem_stby(&mut self) -> CR_PDS_MEM_STBY_W<9> {
        CR_PDS_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_iso_en(&mut self) -> CR_PDS_ISO_EN_W<11> {
        CR_PDS_ISO_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wait_xtal_rdy(&mut self) -> CR_PDS_WAIT_XTAL_RDY_W<12> {
        CR_PDS_WAIT_XTAL_RDY_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pwr_off(&mut self) -> CR_PDS_PWR_OFF_W<13> {
        CR_PDS_PWR_OFF_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_xtal(&mut self) -> CR_PDS_PD_XTAL_W<14> {
        CR_PDS_PD_XTAL_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_soc_enb_force_on(&mut self) -> CR_PDS_SOC_ENB_FORCE_ON_W<15> {
        CR_PDS_SOC_ENB_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_rst_soc_en(&mut self) -> CR_PDS_RST_SOC_EN_W<16> {
        CR_PDS_RST_SOC_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_rc32m_off_dis(&mut self) -> CR_PDS_RC32M_OFF_DIS_W<17> {
        CR_PDS_RC32M_OFF_DIS_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ldo_vsel_en(&mut self) -> CR_PDS_LDO_VSEL_EN_W<18> {
        CR_PDS_LDO_VSEL_EN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn cr_np_wfi_mask(&mut self) -> CR_NP_WFI_MASK_W<21> {
        CR_NP_WFI_MASK_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_ldo11(&mut self) -> CR_PDS_PD_LDO11_W<22> {
        CR_PDS_PD_LDO11_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ldo_vol(&mut self) -> CR_PDS_LDO_VOL_W<24> {
        CR_PDS_LDO_VOL_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_rf(&mut self) -> CR_PDS_CTRL_RF_W<28> {
        CR_PDS_CTRL_RF_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_pll(&mut self) -> CR_PDS_CTRL_PLL_W<30> {
        CR_PDS_CTRL_PLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_CTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl](index.html) module"]
pub struct PDS_CTL_SPEC;
impl crate::RegisterSpec for PDS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ctl::R](R) reader structure"]
impl crate::Readable for PDS_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ctl::W](W) writer structure"]
impl crate::Writable for PDS_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_CTL to value 0x1a00_6b00"]
impl crate::Resettable for PDS_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1a00_6b00;
}

#[doc = "Register `HBN_CTL` reader"]
pub struct R(crate::R<HBN_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_CTL` writer"]
pub struct W(crate::W<HBN_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_CTL_SPEC>;
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
impl From<crate::W<HBN_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc_ctl` reader - "]
pub type RTC_CTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rtc_ctl` writer - "]
pub type RTC_CTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HBN_CTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `hbn_mode` writer - "]
pub type HBN_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_CTL_SPEC, bool, O>;
#[doc = "Field `trap_mode` reader - "]
pub type TRAP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `pwrdn_hbn_core` reader - "]
pub type PWRDN_HBN_CORE_R = crate::BitReader<bool>;
#[doc = "Field `pwrdn_hbn_core` writer - "]
pub type PWRDN_HBN_CORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_CTL_SPEC, bool, O>;
#[doc = "Field `pwrdn_hbn_rtc` reader - "]
pub type PWRDN_HBN_RTC_R = crate::BitReader<bool>;
#[doc = "Field `pwrdn_hbn_rtc` writer - "]
pub type PWRDN_HBN_RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_CTL_SPEC, bool, O>;
#[doc = "Field `sw_rst` reader - "]
pub type SW_RST_R = crate::BitReader<bool>;
#[doc = "Field `sw_rst` writer - "]
pub type SW_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_CTL_SPEC, bool, O>;
#[doc = "Field `hbn_dis_pwr_off_ldo11` reader - "]
pub type HBN_DIS_PWR_OFF_LDO11_R = crate::BitReader<bool>;
#[doc = "Field `hbn_dis_pwr_off_ldo11` writer - "]
pub type HBN_DIS_PWR_OFF_LDO11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HBN_CTL_SPEC, bool, O>;
#[doc = "Field `hbn_dis_pwr_off_ldo11_rt` reader - "]
pub type HBN_DIS_PWR_OFF_LDO11_RT_R = crate::BitReader<bool>;
#[doc = "Field `hbn_dis_pwr_off_ldo11_rt` writer - "]
pub type HBN_DIS_PWR_OFF_LDO11_RT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HBN_CTL_SPEC, bool, O>;
#[doc = "Field `hbn_ldo11_rt_vout_sel` reader - "]
pub type HBN_LDO11_RT_VOUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_ldo11_rt_vout_sel` writer - "]
pub type HBN_LDO11_RT_VOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `hbn_ldo11_aon_vout_sel` reader - "]
pub type HBN_LDO11_AON_VOUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_ldo11_aon_vout_sel` writer - "]
pub type HBN_LDO11_AON_VOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `pu_dcdc18_aon` reader - "]
pub type PU_DCDC18_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_dcdc18_aon` writer - "]
pub type PU_DCDC18_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_CTL_SPEC, bool, O>;
#[doc = "Field `rtc_dly_option` reader - "]
pub type RTC_DLY_OPTION_R = crate::BitReader<bool>;
#[doc = "Field `rtc_dly_option` writer - "]
pub type RTC_DLY_OPTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_CTL_SPEC, bool, O>;
#[doc = "Field `pwr_on_option` reader - "]
pub type PWR_ON_OPTION_R = crate::BitReader<bool>;
#[doc = "Field `pwr_on_option` writer - "]
pub type PWR_ON_OPTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_CTL_SPEC, bool, O>;
#[doc = "Field `sram_slp_option` reader - "]
pub type SRAM_SLP_OPTION_R = crate::BitReader<bool>;
#[doc = "Field `sram_slp_option` writer - "]
pub type SRAM_SLP_OPTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_CTL_SPEC, bool, O>;
#[doc = "Field `sram_slp` reader - "]
pub type SRAM_SLP_R = crate::BitReader<bool>;
#[doc = "Field `hbn_state` reader - "]
pub type HBN_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rtc_ctl(&self) -> RTC_CTL_R {
        RTC_CTL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trap_mode(&self) -> TRAP_MODE_R {
        TRAP_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwrdn_hbn_core(&self) -> PWRDN_HBN_CORE_R {
        PWRDN_HBN_CORE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pwrdn_hbn_rtc(&self) -> PWRDN_HBN_RTC_R {
        PWRDN_HBN_RTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11(&self) -> HBN_DIS_PWR_OFF_LDO11_R {
        HBN_DIS_PWR_OFF_LDO11_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11_rt(&self) -> HBN_DIS_PWR_OFF_LDO11_RT_R {
        HBN_DIS_PWR_OFF_LDO11_RT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn hbn_ldo11_rt_vout_sel(&self) -> HBN_LDO11_RT_VOUT_SEL_R {
        HBN_LDO11_RT_VOUT_SEL_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn hbn_ldo11_aon_vout_sel(&self) -> HBN_LDO11_AON_VOUT_SEL_R {
        HBN_LDO11_AON_VOUT_SEL_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_dcdc18_aon(&self) -> PU_DCDC18_AON_R {
        PU_DCDC18_AON_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_dly_option(&self) -> RTC_DLY_OPTION_R {
        RTC_DLY_OPTION_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwr_on_option(&self) -> PWR_ON_OPTION_R {
        PWR_ON_OPTION_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sram_slp_option(&self) -> SRAM_SLP_OPTION_R {
        SRAM_SLP_OPTION_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sram_slp(&self) -> SRAM_SLP_R {
        SRAM_SLP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hbn_state(&self) -> HBN_STATE_R {
        HBN_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctl(&mut self) -> RTC_CTL_W<0> {
        RTC_CTL_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_mode(&mut self) -> HBN_MODE_W<7> {
        HBN_MODE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdn_hbn_core(&mut self) -> PWRDN_HBN_CORE_W<9> {
        PWRDN_HBN_CORE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdn_hbn_rtc(&mut self) -> PWRDN_HBN_RTC_W<11> {
        PWRDN_HBN_RTC_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst(&mut self) -> SW_RST_W<12> {
        SW_RST_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_dis_pwr_off_ldo11(&mut self) -> HBN_DIS_PWR_OFF_LDO11_W<13> {
        HBN_DIS_PWR_OFF_LDO11_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_dis_pwr_off_ldo11_rt(&mut self) -> HBN_DIS_PWR_OFF_LDO11_RT_W<14> {
        HBN_DIS_PWR_OFF_LDO11_RT_W::new(self)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_ldo11_rt_vout_sel(&mut self) -> HBN_LDO11_RT_VOUT_SEL_W<15> {
        HBN_LDO11_RT_VOUT_SEL_W::new(self)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_ldo11_aon_vout_sel(&mut self) -> HBN_LDO11_AON_VOUT_SEL_W<19> {
        HBN_LDO11_AON_VOUT_SEL_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pu_dcdc18_aon(&mut self) -> PU_DCDC18_AON_W<23> {
        PU_DCDC18_AON_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_dly_option(&mut self) -> RTC_DLY_OPTION_W<24> {
        RTC_DLY_OPTION_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_on_option(&mut self) -> PWR_ON_OPTION_W<25> {
        PWR_ON_OPTION_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn sram_slp_option(&mut self) -> SRAM_SLP_OPTION_W<26> {
        SRAM_SLP_OPTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_CTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_ctl](index.html) module"]
pub struct HBN_CTL_SPEC;
impl crate::RegisterSpec for HBN_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_ctl::R](R) reader structure"]
impl crate::Readable for HBN_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_ctl::W](W) writer structure"]
impl crate::Writable for HBN_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_CTL to value 0x00d5_0000"]
impl crate::Resettable for HBN_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x00d5_0000;
}

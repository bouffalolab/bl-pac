#[doc = "Register `aon_common` reader"]
pub struct R(crate::R<AON_COMMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AON_COMMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AON_COMMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AON_COMMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `aon_common` writer"]
pub struct W(crate::W<AON_COMMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AON_COMMON_SPEC>;
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
impl From<crate::W<AON_COMMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AON_COMMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmux_aon` reader - "]
pub type TMUX_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tmux_aon` writer - "]
pub type TMUX_AON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AON_COMMON_SPEC, u8, u8, 3, O>;
#[doc = "Field `ten_aon` reader - "]
pub type TEN_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_aon` writer - "]
pub type TEN_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `dten_xtal32k` reader - "]
pub type DTEN_XTAL32K_R = crate::BitReader<bool>;
#[doc = "Field `dten_xtal32k` writer - "]
pub type DTEN_XTAL32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_xtal32k` reader - "]
pub type TEN_XTAL32K_R = crate::BitReader<bool>;
#[doc = "Field `ten_xtal32k` writer - "]
pub type TEN_XTAL32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_vddcore_aon` reader - "]
pub type TEN_VDDCORE_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_vddcore_aon` writer - "]
pub type TEN_VDDCORE_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_ldo11soc_aon` reader - "]
pub type TEN_LDO11SOC_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_ldo11soc_aon` writer - "]
pub type TEN_LDO11SOC_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_dcdc18_0_aon` reader - "]
pub type TEN_DCDC18_0_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_dcdc18_0_aon` writer - "]
pub type TEN_DCDC18_0_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_dcdc18_1_aon` reader - "]
pub type TEN_DCDC18_1_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_dcdc18_1_aon` writer - "]
pub type TEN_DCDC18_1_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_bg_sys_aon` reader - "]
pub type TEN_BG_SYS_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_bg_sys_aon` writer - "]
pub type TEN_BG_SYS_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_ldo15rf_aon` reader - "]
pub type TEN_LDO15RF_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_ldo15rf_aon` writer - "]
pub type TEN_LDO15RF_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_xtal_aon` reader - "]
pub type TEN_XTAL_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_xtal_aon` writer - "]
pub type TEN_XTAL_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `dten_xtal_aon` reader - "]
pub type DTEN_XTAL_AON_R = crate::BitReader<bool>;
#[doc = "Field `dten_xtal_aon` writer - "]
pub type DTEN_XTAL_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_mbg_aon` reader - "]
pub type TEN_MBG_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_mbg_aon` writer - "]
pub type TEN_MBG_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_cip_misc_aon` reader - "]
pub type TEN_CIP_MISC_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_cip_misc_aon` writer - "]
pub type TEN_CIP_MISC_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&self) -> TMUX_AON_R {
        TMUX_AON_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_aon(&self) -> TEN_AON_R {
        TEN_AON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_xtal32k(&self) -> DTEN_XTAL32K_R {
        DTEN_XTAL32K_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_xtal32k(&self) -> TEN_XTAL32K_R {
        TEN_XTAL32K_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_vddcore_aon(&self) -> TEN_VDDCORE_AON_R {
        TEN_VDDCORE_AON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo11soc_aon(&self) -> TEN_LDO11SOC_AON_R {
        TEN_LDO11SOC_AON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_dcdc18_0_aon(&self) -> TEN_DCDC18_0_AON_R {
        TEN_DCDC18_0_AON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_dcdc18_1_aon(&self) -> TEN_DCDC18_1_AON_R {
        TEN_DCDC18_1_AON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&self) -> TEN_BG_SYS_AON_R {
        TEN_BG_SYS_AON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&self) -> TEN_LDO15RF_AON_R {
        TEN_LDO15RF_AON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&self) -> TEN_XTAL_AON_R {
        TEN_XTAL_AON_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&self) -> DTEN_XTAL_AON_R {
        DTEN_XTAL_AON_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&self) -> TEN_MBG_AON_R {
        TEN_MBG_AON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&self) -> TEN_CIP_MISC_AON_R {
        TEN_CIP_MISC_AON_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_aon(&mut self) -> TMUX_AON_W<0> {
        TMUX_AON_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ten_aon(&mut self) -> TEN_AON_W<4> {
        TEN_AON_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dten_xtal32k(&mut self) -> DTEN_XTAL32K_W<5> {
        DTEN_XTAL32K_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ten_xtal32k(&mut self) -> TEN_XTAL32K_W<6> {
        TEN_XTAL32K_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ten_vddcore_aon(&mut self) -> TEN_VDDCORE_AON_W<8> {
        TEN_VDDCORE_AON_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo11soc_aon(&mut self) -> TEN_LDO11SOC_AON_W<9> {
        TEN_LDO11SOC_AON_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dcdc18_0_aon(&mut self) -> TEN_DCDC18_0_AON_W<10> {
        TEN_DCDC18_0_AON_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dcdc18_1_aon(&mut self) -> TEN_DCDC18_1_AON_W<11> {
        TEN_DCDC18_1_AON_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ten_bg_sys_aon(&mut self) -> TEN_BG_SYS_AON_W<12> {
        TEN_BG_SYS_AON_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo15rf_aon(&mut self) -> TEN_LDO15RF_AON_W<16> {
        TEN_LDO15RF_AON_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ten_xtal_aon(&mut self) -> TEN_XTAL_AON_W<17> {
        TEN_XTAL_AON_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn dten_xtal_aon(&mut self) -> DTEN_XTAL_AON_W<18> {
        DTEN_XTAL_AON_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ten_mbg_aon(&mut self) -> TEN_MBG_AON_W<19> {
        TEN_MBG_AON_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ten_cip_misc_aon(&mut self) -> TEN_CIP_MISC_AON_W<20> {
        TEN_CIP_MISC_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "aon_common.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aon_common](index.html) module"]
pub struct AON_COMMON_SPEC;
impl crate::RegisterSpec for AON_COMMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aon_common::R](R) reader structure"]
impl crate::Readable for AON_COMMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aon_common::W](W) writer structure"]
impl crate::Writable for AON_COMMON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon_common to value 0"]
impl crate::Resettable for AON_COMMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

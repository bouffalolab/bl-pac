#[doc = "Register `ldo11soc_and_dctest` reader"]
pub struct R(crate::R<LDO11SOC_AND_DCTEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO11SOC_AND_DCTEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO11SOC_AND_DCTEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO11SOC_AND_DCTEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ldo11soc_and_dctest` writer"]
pub struct W(crate::W<LDO11SOC_AND_DCTEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO11SOC_AND_DCTEST_SPEC>;
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
impl From<crate::W<LDO11SOC_AND_DCTEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO11SOC_AND_DCTEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_ldo11soc_aon` reader - "]
pub type PU_LDO11SOC_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_ldo11soc_aon` writer - "]
pub type PU_LDO11SOC_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `ldo11soc_sstart_sel_aon` reader - "]
pub type LDO11SOC_SSTART_SEL_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo11soc_sstart_sel_aon` writer - "]
pub type LDO11SOC_SSTART_SEL_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `ldo11soc_sstart_delay_aon` reader - "]
pub type LDO11SOC_SSTART_DELAY_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo11soc_sstart_delay_aon` writer - "]
pub type LDO11SOC_SSTART_DELAY_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, u8, u8, 2, O>;
#[doc = "Field `ldo11soc_pulldown_aon` reader - "]
pub type LDO11SOC_PULLDOWN_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo11soc_pulldown_aon` writer - "]
pub type LDO11SOC_PULLDOWN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `ldo11soc_pulldown_sel_aon` reader - "]
pub type LDO11SOC_PULLDOWN_SEL_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo11soc_pulldown_sel_aon` writer - "]
pub type LDO11SOC_PULLDOWN_SEL_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `ldo11soc_vth_sel_aon` reader - "]
pub type LDO11SOC_VTH_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo11soc_vth_sel_aon` writer - "]
pub type LDO11SOC_VTH_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, u8, u8, 2, O>;
#[doc = "Field `ldo11soc_cc_aon` reader - "]
pub type LDO11SOC_CC_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo11soc_cc_aon` writer - "]
pub type LDO11SOC_CC_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, u8, u8, 2, O>;
#[doc = "Field `ldo11soc_rdy_aon` reader - "]
pub type LDO11SOC_RDY_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo11soc_power_good_aon` reader - "]
pub type LDO11SOC_POWER_GOOD_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_vddcore_misc_aon` reader - "]
pub type PU_VDDCORE_MISC_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_vddcore_misc_aon` writer - "]
pub type PU_VDDCORE_MISC_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `pmip_dc_tp_out_en_aon` reader - "]
pub type PMIP_DC_TP_OUT_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `pmip_dc_tp_out_en_aon` writer - "]
pub type PMIP_DC_TP_OUT_EN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo11soc_aon(&self) -> PU_LDO11SOC_AON_R {
        PU_LDO11SOC_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ldo11soc_sstart_sel_aon(&self) -> LDO11SOC_SSTART_SEL_AON_R {
        LDO11SOC_SSTART_SEL_AON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ldo11soc_sstart_delay_aon(&self) -> LDO11SOC_SSTART_DELAY_AON_R {
        LDO11SOC_SSTART_DELAY_AON_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_aon(&self) -> LDO11SOC_PULLDOWN_AON_R {
        LDO11SOC_PULLDOWN_AON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_sel_aon(&self) -> LDO11SOC_PULLDOWN_SEL_AON_R {
        LDO11SOC_PULLDOWN_SEL_AON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ldo11soc_vth_sel_aon(&self) -> LDO11SOC_VTH_SEL_AON_R {
        LDO11SOC_VTH_SEL_AON_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo11soc_cc_aon(&self) -> LDO11SOC_CC_AON_R {
        LDO11SOC_CC_AON_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo11soc_rdy_aon(&self) -> LDO11SOC_RDY_AON_R {
        LDO11SOC_RDY_AON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ldo11soc_power_good_aon(&self) -> LDO11SOC_POWER_GOOD_AON_R {
        LDO11SOC_POWER_GOOD_AON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_vddcore_misc_aon(&self) -> PU_VDDCORE_MISC_AON_R {
        PU_VDDCORE_MISC_AON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&self) -> PMIP_DC_TP_OUT_EN_AON_R {
        PMIP_DC_TP_OUT_EN_AON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ldo11soc_aon(&mut self) -> PU_LDO11SOC_AON_W<0> {
        PU_LDO11SOC_AON_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_sstart_sel_aon(&mut self) -> LDO11SOC_SSTART_SEL_AON_W<4> {
        LDO11SOC_SSTART_SEL_AON_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_sstart_delay_aon(&mut self) -> LDO11SOC_SSTART_DELAY_AON_W<8> {
        LDO11SOC_SSTART_DELAY_AON_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_pulldown_aon(&mut self) -> LDO11SOC_PULLDOWN_AON_W<10> {
        LDO11SOC_PULLDOWN_AON_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_pulldown_sel_aon(&mut self) -> LDO11SOC_PULLDOWN_SEL_AON_W<11> {
        LDO11SOC_PULLDOWN_SEL_AON_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_vth_sel_aon(&mut self) -> LDO11SOC_VTH_SEL_AON_W<12> {
        LDO11SOC_VTH_SEL_AON_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_cc_aon(&mut self) -> LDO11SOC_CC_AON_W<24> {
        LDO11SOC_CC_AON_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pu_vddcore_misc_aon(&mut self) -> PU_VDDCORE_MISC_AON_W<30> {
        PU_VDDCORE_MISC_AON_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pmip_dc_tp_out_en_aon(&mut self) -> PMIP_DC_TP_OUT_EN_AON_W<31> {
        PMIP_DC_TP_OUT_EN_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ldo11soc_and_dctest.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo11soc_and_dctest](index.html) module"]
pub struct LDO11SOC_AND_DCTEST_SPEC;
impl crate::RegisterSpec for LDO11SOC_AND_DCTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo11soc_and_dctest::R](R) reader structure"]
impl crate::Readable for LDO11SOC_AND_DCTEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo11soc_and_dctest::W](W) writer structure"]
impl crate::Writable for LDO11SOC_AND_DCTEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo11soc_and_dctest to value 0x7000_1811"]
impl crate::Resettable for LDO11SOC_AND_DCTEST_SPEC {
    const RESET_VALUE: Self::Ux = 0x7000_1811;
}

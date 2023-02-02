#[doc = "Register `dfe_ctrl_0` reader"]
pub struct R(crate::R<DFE_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_0` writer"]
pub struct W(crate::W<DFE_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_0_SPEC>;
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
impl From<crate::W<DFE_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_iqc_phase` reader - "]
pub type TX_IQC_PHASE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tx_iqc_phase` writer - "]
pub type TX_IQC_PHASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_0_SPEC, u16, u16, 10, O>;
#[doc = "Field `tx_iqc_phase_en` reader - "]
pub type TX_IQC_PHASE_EN_R = crate::BitReader<bool>;
#[doc = "Field `tx_iqc_phase_en` writer - "]
pub type TX_IQC_PHASE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_0_SPEC, bool, O>;
#[doc = "Field `tx_iqc_gain` reader - "]
pub type TX_IQC_GAIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tx_iqc_gain` writer - "]
pub type TX_IQC_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_0_SPEC, u16, u16, 11, O>;
#[doc = "Field `tx_iqc_gain_en` reader - "]
pub type TX_IQC_GAIN_EN_R = crate::BitReader<bool>;
#[doc = "Field `tx_iqc_gain_en` writer - "]
pub type TX_IQC_GAIN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_0_SPEC, bool, O>;
#[doc = "Field `tx_dvga_gain_qdb` reader - "]
pub type TX_DVGA_GAIN_QDB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_dvga_gain_qdb` writer - "]
pub type TX_DVGA_GAIN_QDB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_0_SPEC, u8, u8, 7, O>;
#[doc = "Field `tx_dvga_gain_ctrl_hw` reader - "]
pub type TX_DVGA_GAIN_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `tx_dvga_gain_ctrl_hw` writer - "]
pub type TX_DVGA_GAIN_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFE_CTRL_0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iqc_phase(&self) -> TX_IQC_PHASE_R {
        TX_IQC_PHASE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_iqc_phase_en(&self) -> TX_IQC_PHASE_EN_R {
        TX_IQC_PHASE_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    pub fn tx_iqc_gain(&self) -> TX_IQC_GAIN_R {
        TX_IQC_GAIN_R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tx_iqc_gain_en(&self) -> TX_IQC_GAIN_EN_R {
        TX_IQC_GAIN_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb(&self) -> TX_DVGA_GAIN_QDB_R {
        TX_DVGA_GAIN_QDB_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dvga_gain_ctrl_hw(&self) -> TX_DVGA_GAIN_CTRL_HW_R {
        TX_DVGA_GAIN_CTRL_HW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn tx_iqc_phase(&mut self) -> TX_IQC_PHASE_W<0> {
        TX_IQC_PHASE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tx_iqc_phase_en(&mut self) -> TX_IQC_PHASE_EN_W<10> {
        TX_IQC_PHASE_EN_W::new(self)
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    #[must_use]
    pub fn tx_iqc_gain(&mut self) -> TX_IQC_GAIN_W<12> {
        TX_IQC_GAIN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn tx_iqc_gain_en(&mut self) -> TX_IQC_GAIN_EN_W<23> {
        TX_IQC_GAIN_EN_W::new(self)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dvga_gain_qdb(&mut self) -> TX_DVGA_GAIN_QDB_W<24> {
        TX_DVGA_GAIN_QDB_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dvga_gain_ctrl_hw(&mut self) -> TX_DVGA_GAIN_CTRL_HW_W<31> {
        TX_DVGA_GAIN_CTRL_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_0](index.html) module"]
pub struct DFE_CTRL_0_SPEC;
impl crate::RegisterSpec for DFE_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_0::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_0::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_0 to value 0"]
impl crate::Resettable for DFE_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

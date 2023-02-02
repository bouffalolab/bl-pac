#[doc = "Register `dfe_ctrl_5` reader"]
pub struct R(crate::R<DFE_CTRL_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_5` writer"]
pub struct W(crate::W<DFE_CTRL_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_5_SPEC>;
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
impl From<crate::W<DFE_CTRL_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_iqc_phase` reader - "]
pub type RX_IQC_PHASE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_iqc_phase` writer - "]
pub type RX_IQC_PHASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_5_SPEC, u16, u16, 10, O>;
#[doc = "Field `rx_iqc_phase_en` reader - "]
pub type RX_IQC_PHASE_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_iqc_phase_en` writer - "]
pub type RX_IQC_PHASE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_5_SPEC, bool, O>;
#[doc = "Field `rx_iqc_gain` reader - "]
pub type RX_IQC_GAIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_iqc_gain` writer - "]
pub type RX_IQC_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_5_SPEC, u16, u16, 11, O>;
#[doc = "Field `rx_iqc_gain_en` reader - "]
pub type RX_IQC_GAIN_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_iqc_gain_en` writer - "]
pub type RX_IQC_GAIN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iqc_phase(&self) -> RX_IQC_PHASE_R {
        RX_IQC_PHASE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_iqc_phase_en(&self) -> RX_IQC_PHASE_EN_R {
        RX_IQC_PHASE_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    pub fn rx_iqc_gain(&self) -> RX_IQC_GAIN_R {
        RX_IQC_GAIN_R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_iqc_gain_en(&self) -> RX_IQC_GAIN_EN_R {
        RX_IQC_GAIN_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_iqc_phase(&mut self) -> RX_IQC_PHASE_W<0> {
        RX_IQC_PHASE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn rx_iqc_phase_en(&mut self) -> RX_IQC_PHASE_EN_W<10> {
        RX_IQC_PHASE_EN_W::new(self)
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    #[must_use]
    pub fn rx_iqc_gain(&mut self) -> RX_IQC_GAIN_W<12> {
        RX_IQC_GAIN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn rx_iqc_gain_en(&mut self) -> RX_IQC_GAIN_EN_W<23> {
        RX_IQC_GAIN_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_5](index.html) module"]
pub struct DFE_CTRL_5_SPEC;
impl crate::RegisterSpec for DFE_CTRL_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_5::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_5::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_5 to value 0"]
impl crate::Resettable for DFE_CTRL_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

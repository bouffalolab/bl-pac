#[doc = "Register `rf_base_ctrl1` reader"]
pub struct R(crate::R<RF_BASE_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_BASE_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_BASE_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_BASE_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_base_ctrl1` writer"]
pub struct W(crate::W<RF_BASE_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_BASE_CTRL1_SPEC>;
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
impl From<crate::W<RF_BASE_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_BASE_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `aupll_sdm_rst_dly` reader - "]
pub type AUPLL_SDM_RST_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `aupll_sdm_rst_dly` writer - "]
pub type AUPLL_SDM_RST_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_BASE_CTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_rst_dly` reader - "]
pub type LO_SDM_RST_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_rst_dly` writer - "]
pub type LO_SDM_RST_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_BASE_CTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ppu_lead` reader - "]
pub type PPU_LEAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ppu_lead` writer - "]
pub type PPU_LEAD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_BASE_CTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `pud_vco_dly` reader - "]
pub type PUD_VCO_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pud_vco_dly` writer - "]
pub type PUD_VCO_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_BASE_CTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `pud_iref_dly` reader - "]
pub type PUD_IREF_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pud_iref_dly` writer - "]
pub type PUD_IREF_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_BASE_CTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `pud_pa_dly` reader - "]
pub type PUD_PA_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pud_pa_dly` writer - "]
pub type PUD_PA_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_BASE_CTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `mbg_trim` reader - "]
pub type MBG_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mbg_trim` writer - "]
pub type MBG_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_BASE_CTRL1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn aupll_sdm_rst_dly(&self) -> AUPLL_SDM_RST_DLY_R {
        AUPLL_SDM_RST_DLY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_rst_dly(&self) -> LO_SDM_RST_DLY_R {
        LO_SDM_RST_DLY_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ppu_lead(&self) -> PPU_LEAD_R {
        PPU_LEAD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pud_vco_dly(&self) -> PUD_VCO_DLY_R {
        PUD_VCO_DLY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pud_iref_dly(&self) -> PUD_IREF_DLY_R {
        PUD_IREF_DLY_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pud_pa_dly(&self) -> PUD_PA_DLY_R {
        PUD_PA_DLY_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn mbg_trim(&self) -> MBG_TRIM_R {
        MBG_TRIM_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sdm_rst_dly(&mut self) -> AUPLL_SDM_RST_DLY_W<0> {
        AUPLL_SDM_RST_DLY_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_rst_dly(&mut self) -> LO_SDM_RST_DLY_W<2> {
        LO_SDM_RST_DLY_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_lead(&mut self) -> PPU_LEAD_W<8> {
        PPU_LEAD_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn pud_vco_dly(&mut self) -> PUD_VCO_DLY_W<10> {
        PUD_VCO_DLY_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn pud_iref_dly(&mut self) -> PUD_IREF_DLY_W<12> {
        PUD_IREF_DLY_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn pud_pa_dly(&mut self) -> PUD_PA_DLY_W<14> {
        PUD_PA_DLY_W::new(self)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    #[must_use]
    pub fn mbg_trim(&mut self) -> MBG_TRIM_W<27> {
        MBG_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ZRF Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_base_ctrl1](index.html) module"]
pub struct RF_BASE_CTRL1_SPEC;
impl crate::RegisterSpec for RF_BASE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_base_ctrl1::R](R) reader structure"]
impl crate::Readable for RF_BASE_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_base_ctrl1::W](W) writer structure"]
impl crate::Writable for RF_BASE_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_base_ctrl1 to value 0"]
impl crate::Resettable for RF_BASE_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

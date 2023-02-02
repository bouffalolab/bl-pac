#[doc = "Register `rfcal_ctrlen` reader"]
pub struct R(crate::R<RFCAL_CTRLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCAL_CTRLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCAL_CTRLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCAL_CTRLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfcal_ctrlen` writer"]
pub struct W(crate::W<RFCAL_CTRLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCAL_CTRLEN_SPEC>;
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
impl From<crate::W<RFCAL_CTRLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCAL_CTRLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rcal_en_resv` reader - "]
pub type RCAL_EN_RESV_R = crate::BitReader<bool>;
#[doc = "Field `rcal_en_resv` writer - "]
pub type RCAL_EN_RESV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `adc_oscal_en` reader - "]
pub type ADC_OSCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `adc_oscal_en` writer - "]
pub type ADC_OSCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `dl_rfcal_table_en` reader - "]
pub type DL_RFCAL_TABLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `dl_rfcal_table_en` writer - "]
pub type DL_RFCAL_TABLE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `fcal_en` reader - "]
pub type FCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `fcal_en` writer - "]
pub type FCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `acal_en` reader - "]
pub type ACAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `acal_en` writer - "]
pub type ACAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `fcal_inc_en` reader - "]
pub type FCAL_INC_EN_R = crate::BitReader<bool>;
#[doc = "Field `fcal_inc_en` writer - "]
pub type FCAL_INC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `acal_inc_en` reader - "]
pub type ACAL_INC_EN_R = crate::BitReader<bool>;
#[doc = "Field `acal_inc_en` writer - "]
pub type ACAL_INC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `roscal_inc_en` reader - "]
pub type ROSCAL_INC_EN_R = crate::BitReader<bool>;
#[doc = "Field `roscal_inc_en` writer - "]
pub type ROSCAL_INC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `clkpll_cal_en` reader - "]
pub type CLKPLL_CAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_cal_en` writer - "]
pub type CLKPLL_CAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `roscal_en` reader - "]
pub type ROSCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `roscal_en` writer - "]
pub type ROSCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `toscal_en` reader - "]
pub type TOSCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `toscal_en` writer - "]
pub type TOSCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `rccal_en` reader - "]
pub type RCCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `rccal_en` writer - "]
pub type RCCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `lo_leakcal_en` reader - "]
pub type LO_LEAKCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `lo_leakcal_en` writer - "]
pub type LO_LEAKCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `tiqcal_en` reader - "]
pub type TIQCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `tiqcal_en` writer - "]
pub type TIQCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `riqcal_en` reader - "]
pub type RIQCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `riqcal_en` writer - "]
pub type RIQCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `pwdet_cal_en` reader - "]
pub type PWDET_CAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `pwdet_cal_en` writer - "]
pub type PWDET_CAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `tsencal_en` reader - "]
pub type TSENCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `tsencal_en` writer - "]
pub type TSENCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
#[doc = "Field `dpd_en` reader - "]
pub type DPD_EN_R = crate::BitReader<bool>;
#[doc = "Field `dpd_en` writer - "]
pub type DPD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_CTRLEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_en_resv(&self) -> RCAL_EN_RESV_R {
        RCAL_EN_RESV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_en(&self) -> ADC_OSCAL_EN_R {
        ADC_OSCAL_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_en(&self) -> DL_RFCAL_TABLE_EN_R {
        DL_RFCAL_TABLE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_en(&self) -> FCAL_EN_R {
        FCAL_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_en(&self) -> ACAL_EN_R {
        ACAL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fcal_inc_en(&self) -> FCAL_INC_EN_R {
        FCAL_INC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn acal_inc_en(&self) -> ACAL_INC_EN_R {
        ACAL_INC_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn roscal_inc_en(&self) -> ROSCAL_INC_EN_R {
        ROSCAL_INC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_cal_en(&self) -> CLKPLL_CAL_EN_R {
        CLKPLL_CAL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn roscal_en(&self) -> ROSCAL_EN_R {
        ROSCAL_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn toscal_en(&self) -> TOSCAL_EN_R {
        TOSCAL_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rccal_en(&self) -> RCCAL_EN_R {
        RCCAL_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_leakcal_en(&self) -> LO_LEAKCAL_EN_R {
        LO_LEAKCAL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tiqcal_en(&self) -> TIQCAL_EN_R {
        TIQCAL_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn riqcal_en(&self) -> RIQCAL_EN_R {
        RIQCAL_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pwdet_cal_en(&self) -> PWDET_CAL_EN_R {
        PWDET_CAL_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tsencal_en(&self) -> TSENCAL_EN_R {
        TSENCAL_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dpd_en(&self) -> DPD_EN_R {
        DPD_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rcal_en_resv(&mut self) -> RCAL_EN_RESV_W<0> {
        RCAL_EN_RESV_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_oscal_en(&mut self) -> ADC_OSCAL_EN_W<1> {
        ADC_OSCAL_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dl_rfcal_table_en(&mut self) -> DL_RFCAL_TABLE_EN_W<2> {
        DL_RFCAL_TABLE_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_en(&mut self) -> FCAL_EN_W<3> {
        FCAL_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn acal_en(&mut self) -> ACAL_EN_W<4> {
        ACAL_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_inc_en(&mut self) -> FCAL_INC_EN_W<5> {
        FCAL_INC_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn acal_inc_en(&mut self) -> ACAL_INC_EN_W<6> {
        ACAL_INC_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn roscal_inc_en(&mut self) -> ROSCAL_INC_EN_W<7> {
        ROSCAL_INC_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_cal_en(&mut self) -> CLKPLL_CAL_EN_W<8> {
        CLKPLL_CAL_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn roscal_en(&mut self) -> ROSCAL_EN_W<9> {
        ROSCAL_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn toscal_en(&mut self) -> TOSCAL_EN_W<10> {
        TOSCAL_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rccal_en(&mut self) -> RCCAL_EN_W<11> {
        RCCAL_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn lo_leakcal_en(&mut self) -> LO_LEAKCAL_EN_W<12> {
        LO_LEAKCAL_EN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn tiqcal_en(&mut self) -> TIQCAL_EN_W<13> {
        TIQCAL_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn riqcal_en(&mut self) -> RIQCAL_EN_W<14> {
        RIQCAL_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pwdet_cal_en(&mut self) -> PWDET_CAL_EN_W<15> {
        PWDET_CAL_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tsencal_en(&mut self) -> TSENCAL_EN_W<16> {
        TSENCAL_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn dpd_en(&mut self) -> DPD_EN_W<17> {
        DPD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_ctrlen](index.html) module"]
pub struct RFCAL_CTRLEN_SPEC;
impl crate::RegisterSpec for RFCAL_CTRLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcal_ctrlen::R](R) reader structure"]
impl crate::Readable for RFCAL_CTRLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcal_ctrlen::W](W) writer structure"]
impl crate::Writable for RFCAL_CTRLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rfcal_ctrlen to value 0"]
impl crate::Resettable for RFCAL_CTRLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

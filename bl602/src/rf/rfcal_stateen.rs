#[doc = "Register `rfcal_stateen` reader"]
pub struct R(crate::R<RFCAL_STATEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCAL_STATEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCAL_STATEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCAL_STATEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfcal_stateen` writer"]
pub struct W(crate::W<RFCAL_STATEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCAL_STATEEN_SPEC>;
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
impl From<crate::W<RFCAL_STATEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCAL_STATEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rcal_sten_resv` reader - "]
pub type RCAL_STEN_RESV_R = crate::BitReader<bool>;
#[doc = "Field `rcal_sten_resv` writer - "]
pub type RCAL_STEN_RESV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `adc_oscal_sten` reader - "]
pub type ADC_OSCAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `adc_oscal_sten` writer - "]
pub type ADC_OSCAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `dl_rfcal_table_sten` reader - "]
pub type DL_RFCAL_TABLE_STEN_R = crate::BitReader<bool>;
#[doc = "Field `dl_rfcal_table_sten` writer - "]
pub type DL_RFCAL_TABLE_STEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `fcal_sten` reader - "]
pub type FCAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `fcal_sten` writer - "]
pub type FCAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `acal_sten` reader - "]
pub type ACAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `acal_sten` writer - "]
pub type ACAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `inc_fcal_sten` reader - "]
pub type INC_FCAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `inc_fcal_sten` writer - "]
pub type INC_FCAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `inc_acal_sten` reader - "]
pub type INC_ACAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `inc_acal_sten` writer - "]
pub type INC_ACAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `clkpll_cal_sten` reader - "]
pub type CLKPLL_CAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_cal_sten` writer - "]
pub type CLKPLL_CAL_STEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `roscal_sten` reader - "]
pub type ROSCAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `roscal_sten` writer - "]
pub type ROSCAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `toscal_sten_resv` reader - "]
pub type TOSCAL_STEN_RESV_R = crate::BitReader<bool>;
#[doc = "Field `toscal_sten_resv` writer - "]
pub type TOSCAL_STEN_RESV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `rccal_sten` reader - "]
pub type RCCAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `rccal_sten` writer - "]
pub type RCCAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `lo_leakcal_sten` reader - "]
pub type LO_LEAKCAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `lo_leakcal_sten` writer - "]
pub type LO_LEAKCAL_STEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `tiqcal_sten` reader - "]
pub type TIQCAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `tiqcal_sten` writer - "]
pub type TIQCAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `riqcal_sten` reader - "]
pub type RIQCAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `riqcal_sten` writer - "]
pub type RIQCAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `pwdet_cal_sten` reader - "]
pub type PWDET_CAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `pwdet_cal_sten` writer - "]
pub type PWDET_CAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `tsencal_sten` reader - "]
pub type TSENCAL_STEN_R = crate::BitReader<bool>;
#[doc = "Field `tsencal_sten` writer - "]
pub type TSENCAL_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `dpd_sten` reader - "]
pub type DPD_STEN_R = crate::BitReader<bool>;
#[doc = "Field `dpd_sten` writer - "]
pub type DPD_STEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCAL_STATEEN_SPEC, bool, O>;
#[doc = "Field `rfcal_level` reader - "]
pub type RFCAL_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rfcal_level` writer - "]
pub type RFCAL_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATEEN_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_sten_resv(&self) -> RCAL_STEN_RESV_R {
        RCAL_STEN_RESV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_sten(&self) -> ADC_OSCAL_STEN_R {
        ADC_OSCAL_STEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_sten(&self) -> DL_RFCAL_TABLE_STEN_R {
        DL_RFCAL_TABLE_STEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_sten(&self) -> FCAL_STEN_R {
        FCAL_STEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_sten(&self) -> ACAL_STEN_R {
        ACAL_STEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inc_fcal_sten(&self) -> INC_FCAL_STEN_R {
        INC_FCAL_STEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inc_acal_sten(&self) -> INC_ACAL_STEN_R {
        INC_ACAL_STEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_cal_sten(&self) -> CLKPLL_CAL_STEN_R {
        CLKPLL_CAL_STEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn roscal_sten(&self) -> ROSCAL_STEN_R {
        ROSCAL_STEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn toscal_sten_resv(&self) -> TOSCAL_STEN_RESV_R {
        TOSCAL_STEN_RESV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rccal_sten(&self) -> RCCAL_STEN_R {
        RCCAL_STEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lo_leakcal_sten(&self) -> LO_LEAKCAL_STEN_R {
        LO_LEAKCAL_STEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tiqcal_sten(&self) -> TIQCAL_STEN_R {
        TIQCAL_STEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn riqcal_sten(&self) -> RIQCAL_STEN_R {
        RIQCAL_STEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwdet_cal_sten(&self) -> PWDET_CAL_STEN_R {
        PWDET_CAL_STEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tsencal_sten(&self) -> TSENCAL_STEN_R {
        TSENCAL_STEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dpd_sten(&self) -> DPD_STEN_R {
        DPD_STEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rfcal_level(&self) -> RFCAL_LEVEL_R {
        RFCAL_LEVEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rcal_sten_resv(&mut self) -> RCAL_STEN_RESV_W<0> {
        RCAL_STEN_RESV_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_oscal_sten(&mut self) -> ADC_OSCAL_STEN_W<1> {
        ADC_OSCAL_STEN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dl_rfcal_table_sten(&mut self) -> DL_RFCAL_TABLE_STEN_W<2> {
        DL_RFCAL_TABLE_STEN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_sten(&mut self) -> FCAL_STEN_W<3> {
        FCAL_STEN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn acal_sten(&mut self) -> ACAL_STEN_W<4> {
        ACAL_STEN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn inc_fcal_sten(&mut self) -> INC_FCAL_STEN_W<5> {
        INC_FCAL_STEN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn inc_acal_sten(&mut self) -> INC_ACAL_STEN_W<6> {
        INC_ACAL_STEN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_cal_sten(&mut self) -> CLKPLL_CAL_STEN_W<7> {
        CLKPLL_CAL_STEN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn roscal_sten(&mut self) -> ROSCAL_STEN_W<8> {
        ROSCAL_STEN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn toscal_sten_resv(&mut self) -> TOSCAL_STEN_RESV_W<9> {
        TOSCAL_STEN_RESV_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn rccal_sten(&mut self) -> RCCAL_STEN_W<10> {
        RCCAL_STEN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn lo_leakcal_sten(&mut self) -> LO_LEAKCAL_STEN_W<11> {
        LO_LEAKCAL_STEN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tiqcal_sten(&mut self) -> TIQCAL_STEN_W<12> {
        TIQCAL_STEN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn riqcal_sten(&mut self) -> RIQCAL_STEN_W<13> {
        RIQCAL_STEN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pwdet_cal_sten(&mut self) -> PWDET_CAL_STEN_W<14> {
        PWDET_CAL_STEN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tsencal_sten(&mut self) -> TSENCAL_STEN_W<15> {
        TSENCAL_STEN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn dpd_sten(&mut self) -> DPD_STEN_W<16> {
        DPD_STEN_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn rfcal_level(&mut self) -> RFCAL_LEVEL_W<30> {
        RFCAL_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf calibration state enabl in full cal list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_stateen](index.html) module"]
pub struct RFCAL_STATEEN_SPEC;
impl crate::RegisterSpec for RFCAL_STATEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcal_stateen::R](R) reader structure"]
impl crate::Readable for RFCAL_STATEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcal_stateen::W](W) writer structure"]
impl crate::Writable for RFCAL_STATEEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rfcal_stateen to value 0"]
impl crate::Resettable for RFCAL_STATEEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

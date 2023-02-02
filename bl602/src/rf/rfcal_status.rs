#[doc = "Register `rfcal_status` reader"]
pub struct R(crate::R<RFCAL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCAL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCAL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCAL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfcal_status` writer"]
pub struct W(crate::W<RFCAL_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCAL_STATUS_SPEC>;
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
impl From<crate::W<RFCAL_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCAL_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rcal_status` reader - "]
pub type RCAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rcal_status` writer - "]
pub type RCAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `adc_oscal_status` reader - "]
pub type ADC_OSCAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_oscal_status` writer - "]
pub type ADC_OSCAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `fcal_status` reader - "]
pub type FCAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fcal_status` writer - "]
pub type FCAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `acal_status` reader - "]
pub type ACAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acal_status` writer - "]
pub type ACAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `inc_fcal_status` reader - "]
pub type INC_FCAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `inc_fcal_status` writer - "]
pub type INC_FCAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `inc_acal_status` reader - "]
pub type INC_ACAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `inc_acal_status` writer - "]
pub type INC_ACAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `clkpll_cal_status` reader - "]
pub type CLKPLL_CAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_cal_status` writer - "]
pub type CLKPLL_CAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `ros_status` reader - "]
pub type ROS_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ros_status` writer - "]
pub type ROS_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `tos_status` reader - "]
pub type TOS_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tos_status` writer - "]
pub type TOS_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `rccal_status` reader - "]
pub type RCCAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rccal_status` writer - "]
pub type RCCAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_leakcal_status` reader - "]
pub type LO_LEAKCAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_leakcal_status` writer - "]
pub type LO_LEAKCAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `tiqcal_status_resv` reader - "]
pub type TIQCAL_STATUS_RESV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tiqcal_status_resv` writer - "]
pub type TIQCAL_STATUS_RESV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `riqcal_status_resv` reader - "]
pub type RIQCAL_STATUS_RESV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `riqcal_status_resv` writer - "]
pub type RIQCAL_STATUS_RESV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `pwdet_cal_status` reader - "]
pub type PWDET_CAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwdet_cal_status` writer - "]
pub type PWDET_CAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `tenscal_status` reader - "]
pub type TENSCAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tenscal_status` writer - "]
pub type TENSCAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `dpd_status` reader - "]
pub type DPD_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dpd_status` writer - "]
pub type DPD_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCAL_STATUS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rcal_status(&self) -> RCAL_STATUS_R {
        RCAL_STATUS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn adc_oscal_status(&self) -> ADC_OSCAL_STATUS_R {
        ADC_OSCAL_STATUS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_status(&self) -> FCAL_STATUS_R {
        FCAL_STATUS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn acal_status(&self) -> ACAL_STATUS_R {
        ACAL_STATUS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn inc_fcal_status(&self) -> INC_FCAL_STATUS_R {
        INC_FCAL_STATUS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn inc_acal_status(&self) -> INC_ACAL_STATUS_R {
        INC_ACAL_STATUS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_cal_status(&self) -> CLKPLL_CAL_STATUS_R {
        CLKPLL_CAL_STATUS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ros_status(&self) -> ROS_STATUS_R {
        ROS_STATUS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tos_status(&self) -> TOS_STATUS_R {
        TOS_STATUS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rccal_status(&self) -> RCCAL_STATUS_R {
        RCCAL_STATUS_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_leakcal_status(&self) -> LO_LEAKCAL_STATUS_R {
        LO_LEAKCAL_STATUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tiqcal_status_resv(&self) -> TIQCAL_STATUS_RESV_R {
        TIQCAL_STATUS_RESV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn riqcal_status_resv(&self) -> RIQCAL_STATUS_RESV_R {
        RIQCAL_STATUS_RESV_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pwdet_cal_status(&self) -> PWDET_CAL_STATUS_R {
        PWDET_CAL_STATUS_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tenscal_status(&self) -> TENSCAL_STATUS_R {
        TENSCAL_STATUS_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dpd_status(&self) -> DPD_STATUS_R {
        DPD_STATUS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn rcal_status(&mut self) -> RCAL_STATUS_W<0> {
        RCAL_STATUS_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn adc_oscal_status(&mut self) -> ADC_OSCAL_STATUS_W<2> {
        ADC_OSCAL_STATUS_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_status(&mut self) -> FCAL_STATUS_W<4> {
        FCAL_STATUS_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn acal_status(&mut self) -> ACAL_STATUS_W<6> {
        ACAL_STATUS_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn inc_fcal_status(&mut self) -> INC_FCAL_STATUS_W<8> {
        INC_FCAL_STATUS_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn inc_acal_status(&mut self) -> INC_ACAL_STATUS_W<10> {
        INC_ACAL_STATUS_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_cal_status(&mut self) -> CLKPLL_CAL_STATUS_W<12> {
        CLKPLL_CAL_STATUS_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn ros_status(&mut self) -> ROS_STATUS_W<14> {
        ROS_STATUS_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn tos_status(&mut self) -> TOS_STATUS_W<16> {
        TOS_STATUS_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn rccal_status(&mut self) -> RCCAL_STATUS_W<18> {
        RCCAL_STATUS_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn lo_leakcal_status(&mut self) -> LO_LEAKCAL_STATUS_W<20> {
        LO_LEAKCAL_STATUS_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn tiqcal_status_resv(&mut self) -> TIQCAL_STATUS_RESV_W<22> {
        TIQCAL_STATUS_RESV_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn riqcal_status_resv(&mut self) -> RIQCAL_STATUS_RESV_W<24> {
        RIQCAL_STATUS_RESV_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn pwdet_cal_status(&mut self) -> PWDET_CAL_STATUS_W<26> {
        PWDET_CAL_STATUS_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn tenscal_status(&mut self) -> TENSCAL_STATUS_W<28> {
        TENSCAL_STATUS_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn dpd_status(&mut self) -> DPD_STATUS_W<30> {
        DPD_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rfcal_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_status](index.html) module"]
pub struct RFCAL_STATUS_SPEC;
impl crate::RegisterSpec for RFCAL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcal_status::R](R) reader structure"]
impl crate::Readable for RFCAL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcal_status::W](W) writer structure"]
impl crate::Writable for RFCAL_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rfcal_status to value 0"]
impl crate::Resettable for RFCAL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

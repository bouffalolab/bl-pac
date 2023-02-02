#[doc = "Register `gpadc_reg_config2` reader"]
pub struct R(crate::R<GPADC_REG_CONFIG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_CONFIG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_CONFIG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_CONFIG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_config2` writer"]
pub struct W(crate::W<GPADC_REG_CONFIG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_CONFIG2_SPEC>;
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
impl From<crate::W<GPADC_REG_CONFIG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_CONFIG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_diff_mode` reader - "]
pub type GPADC_DIFF_MODE_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_diff_mode` writer - "]
pub type GPADC_DIFF_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, bool, O>;
#[doc = "Field `gpadc_vref_sel` reader - "]
pub type GPADC_VREF_SEL_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_vref_sel` writer - "]
pub type GPADC_VREF_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, bool, O>;
#[doc = "Field `gpadc_vbat_en` reader - "]
pub type GPADC_VBAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_vbat_en` writer - "]
pub type GPADC_VBAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, bool, O>;
#[doc = "Field `gpadc_tsext_sel` reader - "]
pub type GPADC_TSEXT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_tsext_sel` writer - "]
pub type GPADC_TSEXT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, bool, O>;
#[doc = "Field `gpadc_ts_en` reader - "]
pub type GPADC_TS_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_ts_en` writer - "]
pub type GPADC_TS_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, bool, O>;
#[doc = "Field `gpadc_pga_vcm` reader - "]
pub type GPADC_PGA_VCM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_pga_vcm` writer - "]
pub type GPADC_PGA_VCM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `gpadc_pga_os_cal` reader - "]
pub type GPADC_PGA_OS_CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_pga_os_cal` writer - "]
pub type GPADC_PGA_OS_CAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, u8, u8, 4, O>;
#[doc = "Field `gpadc_pga_en` reader - "]
pub type GPADC_PGA_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_pga_en` writer - "]
pub type GPADC_PGA_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, bool, O>;
#[doc = "Field `gpadc_pga_vcmi_en` reader - "]
pub type GPADC_PGA_VCMI_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_pga_vcmi_en` writer - "]
pub type GPADC_PGA_VCMI_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, bool, O>;
#[doc = "Field `gpadc_chop_mode` reader - "]
pub type GPADC_CHOP_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_chop_mode` writer - "]
pub type GPADC_CHOP_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `gpadc_bias_sel` reader - "]
pub type GPADC_BIAS_SEL_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_bias_sel` writer - "]
pub type GPADC_BIAS_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, bool, O>;
#[doc = "Field `gpadc_test_en` reader - "]
pub type GPADC_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_test_en` writer - "]
pub type GPADC_TEST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, bool, O>;
#[doc = "Field `gpadc_test_sel` reader - "]
pub type GPADC_TEST_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_test_sel` writer - "]
pub type GPADC_TEST_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `gpadc_pga2_gain` reader - "]
pub type GPADC_PGA2_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_pga2_gain` writer - "]
pub type GPADC_PGA2_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `gpadc_pga1_gain` reader - "]
pub type GPADC_PGA1_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_pga1_gain` writer - "]
pub type GPADC_PGA1_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `gpadc_dly_sel` reader - "]
pub type GPADC_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_dly_sel` writer - "]
pub type GPADC_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `gpadc_tsvbe_low` reader - "]
pub type GPADC_TSVBE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_tsvbe_low` writer - "]
pub type GPADC_TSVBE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_diff_mode(&self) -> GPADC_DIFF_MODE_R {
        GPADC_DIFF_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_vref_sel(&self) -> GPADC_VREF_SEL_R {
        GPADC_VREF_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_vbat_en(&self) -> GPADC_VBAT_EN_R {
        GPADC_VBAT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_tsext_sel(&self) -> GPADC_TSEXT_SEL_R {
        GPADC_TSEXT_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_ts_en(&self) -> GPADC_TS_EN_R {
        GPADC_TS_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn gpadc_pga_vcm(&self) -> GPADC_PGA_VCM_R {
        GPADC_PGA_VCM_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn gpadc_pga_os_cal(&self) -> GPADC_PGA_OS_CAL_R {
        GPADC_PGA_OS_CAL_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_pga_en(&self) -> GPADC_PGA_EN_R {
        GPADC_PGA_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_pga_vcmi_en(&self) -> GPADC_PGA_VCMI_EN_R {
        GPADC_PGA_VCMI_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gpadc_chop_mode(&self) -> GPADC_CHOP_MODE_R {
        GPADC_CHOP_MODE_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_bias_sel(&self) -> GPADC_BIAS_SEL_R {
        GPADC_BIAS_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_test_en(&self) -> GPADC_TEST_EN_R {
        GPADC_TEST_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn gpadc_test_sel(&self) -> GPADC_TEST_SEL_R {
        GPADC_TEST_SEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gpadc_pga2_gain(&self) -> GPADC_PGA2_GAIN_R {
        GPADC_PGA2_GAIN_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn gpadc_pga1_gain(&self) -> GPADC_PGA1_GAIN_R {
        GPADC_PGA1_GAIN_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_dly_sel(&self) -> GPADC_DLY_SEL_R {
        GPADC_DLY_SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_tsvbe_low(&self) -> GPADC_TSVBE_LOW_R {
        GPADC_TSVBE_LOW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_diff_mode(&mut self) -> GPADC_DIFF_MODE_W<2> {
        GPADC_DIFF_MODE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_vref_sel(&mut self) -> GPADC_VREF_SEL_W<3> {
        GPADC_VREF_SEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_vbat_en(&mut self) -> GPADC_VBAT_EN_W<4> {
        GPADC_VBAT_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_tsext_sel(&mut self) -> GPADC_TSEXT_SEL_W<5> {
        GPADC_TSEXT_SEL_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_ts_en(&mut self) -> GPADC_TS_EN_W<6> {
        GPADC_TS_EN_W::new(self)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga_vcm(&mut self) -> GPADC_PGA_VCM_W<7> {
        GPADC_PGA_VCM_W::new(self)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga_os_cal(&mut self) -> GPADC_PGA_OS_CAL_W<9> {
        GPADC_PGA_OS_CAL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga_en(&mut self) -> GPADC_PGA_EN_W<13> {
        GPADC_PGA_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga_vcmi_en(&mut self) -> GPADC_PGA_VCMI_EN_W<14> {
        GPADC_PGA_VCMI_EN_W::new(self)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_chop_mode(&mut self) -> GPADC_CHOP_MODE_W<15> {
        GPADC_CHOP_MODE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_bias_sel(&mut self) -> GPADC_BIAS_SEL_W<17> {
        GPADC_BIAS_SEL_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_test_en(&mut self) -> GPADC_TEST_EN_W<18> {
        GPADC_TEST_EN_W::new(self)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_test_sel(&mut self) -> GPADC_TEST_SEL_W<19> {
        GPADC_TEST_SEL_W::new(self)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga2_gain(&mut self) -> GPADC_PGA2_GAIN_W<22> {
        GPADC_PGA2_GAIN_W::new(self)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga1_gain(&mut self) -> GPADC_PGA1_GAIN_W<25> {
        GPADC_PGA1_GAIN_W::new(self)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_dly_sel(&mut self) -> GPADC_DLY_SEL_W<28> {
        GPADC_DLY_SEL_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_tsvbe_low(&mut self) -> GPADC_TSVBE_LOW_W<31> {
        GPADC_TSVBE_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_config2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_config2](index.html) module"]
pub struct GPADC_REG_CONFIG2_SPEC;
impl crate::RegisterSpec for GPADC_REG_CONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_config2::R](R) reader structure"]
impl crate::Readable for GPADC_REG_CONFIG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_config2::W](W) writer structure"]
impl crate::Writable for GPADC_REG_CONFIG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_config2 to value 0x0001_9100"]
impl crate::Resettable for GPADC_REG_CONFIG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_9100;
}

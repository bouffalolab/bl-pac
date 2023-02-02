#[doc = "Register `gpadc_reg_cmd` reader"]
pub struct R(crate::R<GPADC_REG_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_cmd` writer"]
pub struct W(crate::W<GPADC_REG_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_CMD_SPEC>;
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
impl From<crate::W<GPADC_REG_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_global_en` reader - "]
pub type GPADC_GLOBAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_global_en` writer - "]
pub type GPADC_GLOBAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_conv_start` reader - "]
pub type GPADC_CONV_START_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_conv_start` writer - "]
pub type GPADC_CONV_START_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_soft_rst` reader - "]
pub type GPADC_SOFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_soft_rst` writer - "]
pub type GPADC_SOFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_neg_sel` reader - "]
pub type GPADC_NEG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_neg_sel` writer - "]
pub type GPADC_NEG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CMD_SPEC, u8, u8, 5, O>;
#[doc = "Field `gpadc_pos_sel` reader - "]
pub type GPADC_POS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_pos_sel` writer - "]
pub type GPADC_POS_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CMD_SPEC, u8, u8, 5, O>;
#[doc = "Field `gpadc_neg_gnd` reader - "]
pub type GPADC_NEG_GND_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_neg_gnd` writer - "]
pub type GPADC_NEG_GND_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_micbias_en` reader - "]
pub type GPADC_MICBIAS_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_micbias_en` writer - "]
pub type GPADC_MICBIAS_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_micpga_en` reader - "]
pub type GPADC_MICPGA_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_micpga_en` writer - "]
pub type GPADC_MICPGA_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_byp_micboost` reader - "]
pub type GPADC_BYP_MICBOOST_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_byp_micboost` writer - "]
pub type GPADC_BYP_MICBOOST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_dwa_en` reader - "]
pub type GPADC_DWA_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_dwa_en` writer - "]
pub type GPADC_DWA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_mic2_diff` reader - "]
pub type GPADC_MIC2_DIFF_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_mic2_diff` writer - "]
pub type GPADC_MIC2_DIFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_mic1_diff` reader - "]
pub type GPADC_MIC1_DIFF_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_mic1_diff` writer - "]
pub type GPADC_MIC1_DIFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_mic_pga2_gain` reader - "]
pub type GPADC_MIC_PGA2_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_mic_pga2_gain` writer - "]
pub type GPADC_MIC_PGA2_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `gpadc_micboost_32db_en` reader - "]
pub type GPADC_MICBOOST_32DB_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_micboost_32db_en` writer - "]
pub type GPADC_MICBOOST_32DB_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_chip_sen_pu` reader - "]
pub type GPADC_CHIP_SEN_PU_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_chip_sen_pu` writer - "]
pub type GPADC_CHIP_SEN_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
#[doc = "Field `gpadc_sen_sel` reader - "]
pub type GPADC_SEN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_sen_sel` writer - "]
pub type GPADC_SEN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `gpadc_sen_test_en` reader - "]
pub type GPADC_SEN_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_sen_test_en` writer - "]
pub type GPADC_SEN_TEST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_global_en(&self) -> GPADC_GLOBAL_EN_R {
        GPADC_GLOBAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_conv_start(&self) -> GPADC_CONV_START_R {
        GPADC_CONV_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_soft_rst(&self) -> GPADC_SOFT_RST_R {
        GPADC_SOFT_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn gpadc_neg_sel(&self) -> GPADC_NEG_SEL_R {
        GPADC_NEG_SEL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpadc_pos_sel(&self) -> GPADC_POS_SEL_R {
        GPADC_POS_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_neg_gnd(&self) -> GPADC_NEG_GND_R {
        GPADC_NEG_GND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_micbias_en(&self) -> GPADC_MICBIAS_EN_R {
        GPADC_MICBIAS_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpadc_micpga_en(&self) -> GPADC_MICPGA_EN_R {
        GPADC_MICPGA_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpadc_byp_micboost(&self) -> GPADC_BYP_MICBOOST_R {
        GPADC_BYP_MICBOOST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_dwa_en(&self) -> GPADC_DWA_EN_R {
        GPADC_DWA_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpadc_mic2_diff(&self) -> GPADC_MIC2_DIFF_R {
        GPADC_MIC2_DIFF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpadc_mic1_diff(&self) -> GPADC_MIC1_DIFF_R {
        GPADC_MIC1_DIFF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn gpadc_mic_pga2_gain(&self) -> GPADC_MIC_PGA2_GAIN_R {
        GPADC_MIC_PGA2_GAIN_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpadc_micboost_32db_en(&self) -> GPADC_MICBOOST_32DB_EN_R {
        GPADC_MICBOOST_32DB_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpadc_chip_sen_pu(&self) -> GPADC_CHIP_SEN_PU_R {
        GPADC_CHIP_SEN_PU_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gpadc_sen_sel(&self) -> GPADC_SEN_SEL_R {
        GPADC_SEN_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpadc_sen_test_en(&self) -> GPADC_SEN_TEST_EN_R {
        GPADC_SEN_TEST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_global_en(&mut self) -> GPADC_GLOBAL_EN_W<0> {
        GPADC_GLOBAL_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_conv_start(&mut self) -> GPADC_CONV_START_W<1> {
        GPADC_CONV_START_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_soft_rst(&mut self) -> GPADC_SOFT_RST_W<2> {
        GPADC_SOFT_RST_W::new(self)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_neg_sel(&mut self) -> GPADC_NEG_SEL_W<3> {
        GPADC_NEG_SEL_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pos_sel(&mut self) -> GPADC_POS_SEL_W<8> {
        GPADC_POS_SEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_neg_gnd(&mut self) -> GPADC_NEG_GND_W<13> {
        GPADC_NEG_GND_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_micbias_en(&mut self) -> GPADC_MICBIAS_EN_W<14> {
        GPADC_MICBIAS_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_micpga_en(&mut self) -> GPADC_MICPGA_EN_W<15> {
        GPADC_MICPGA_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_byp_micboost(&mut self) -> GPADC_BYP_MICBOOST_W<16> {
        GPADC_BYP_MICBOOST_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_dwa_en(&mut self) -> GPADC_DWA_EN_W<18> {
        GPADC_DWA_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_mic2_diff(&mut self) -> GPADC_MIC2_DIFF_W<19> {
        GPADC_MIC2_DIFF_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_mic1_diff(&mut self) -> GPADC_MIC1_DIFF_W<20> {
        GPADC_MIC1_DIFF_W::new(self)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_mic_pga2_gain(&mut self) -> GPADC_MIC_PGA2_GAIN_W<21> {
        GPADC_MIC_PGA2_GAIN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_micboost_32db_en(&mut self) -> GPADC_MICBOOST_32DB_EN_W<23> {
        GPADC_MICBOOST_32DB_EN_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_chip_sen_pu(&mut self) -> GPADC_CHIP_SEN_PU_W<27> {
        GPADC_CHIP_SEN_PU_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_sen_sel(&mut self) -> GPADC_SEN_SEL_W<28> {
        GPADC_SEN_SEL_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_sen_test_en(&mut self) -> GPADC_SEN_TEST_EN_W<30> {
        GPADC_SEN_TEST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_cmd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_cmd](index.html) module"]
pub struct GPADC_REG_CMD_SPEC;
impl crate::RegisterSpec for GPADC_REG_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_cmd::R](R) reader structure"]
impl crate::Readable for GPADC_REG_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_cmd::W](W) writer structure"]
impl crate::Writable for GPADC_REG_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_cmd to value 0x0f78"]
impl crate::Resettable for GPADC_REG_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f78;
}

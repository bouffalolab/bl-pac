#[doc = "Register `adda2` reader"]
pub struct R(crate::R<ADDA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adda2` writer"]
pub struct W(crate::W<ADDA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDA2_SPEC>;
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
impl From<crate::W<ADDA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_vref_sel` reader - "]
pub type ADC_VREF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_vref_sel` writer - "]
pub type ADC_VREF_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDA2_SPEC, u8, u8, 2, O>;
#[doc = "Field `adc_dly_ctl` reader - "]
pub type ADC_DLY_CTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_dly_ctl` writer - "]
pub type ADC_DLY_CTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDA2_SPEC, u8, u8, 2, O>;
#[doc = "Field `adc_dvdd_sel` reader - "]
pub type ADC_DVDD_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_dvdd_sel` writer - "]
pub type ADC_DVDD_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDA2_SPEC, u8, u8, 2, O>;
#[doc = "Field `adc_sar_ascal_en` reader - "]
pub type ADC_SAR_ASCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `adc_sar_ascal_en` writer - "]
pub type ADC_SAR_ASCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDA2_SPEC, bool, O>;
#[doc = "Field `adc_gt_rm` reader - "]
pub type ADC_GT_RM_R = crate::BitReader<bool>;
#[doc = "Field `adc_gt_rm` writer - "]
pub type ADC_GT_RM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDA2_SPEC, bool, O>;
#[doc = "Field `adc_clk_sync_inv` reader - "]
pub type ADC_CLK_SYNC_INV_R = crate::BitReader<bool>;
#[doc = "Field `adc_clk_sync_inv` writer - "]
pub type ADC_CLK_SYNC_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDA2_SPEC, bool, O>;
#[doc = "Field `adc_clk_inv` reader - "]
pub type ADC_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `adc_clk_inv` writer - "]
pub type ADC_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDA2_SPEC, bool, O>;
#[doc = "Field `adc_clk_div_sel` reader - "]
pub type ADC_CLK_DIV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adc_clk_div_sel` writer - "]
pub type ADC_CLK_DIV_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDA2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adc_vref_sel(&self) -> ADC_VREF_SEL_R {
        ADC_VREF_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc_dly_ctl(&self) -> ADC_DLY_CTL_R {
        ADC_DLY_CTL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adc_dvdd_sel(&self) -> ADC_DVDD_SEL_R {
        ADC_DVDD_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_sar_ascal_en(&self) -> ADC_SAR_ASCAL_EN_R {
        ADC_SAR_ASCAL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adc_gt_rm(&self) -> ADC_GT_RM_R {
        ADC_GT_RM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_clk_sync_inv(&self) -> ADC_CLK_SYNC_INV_R {
        ADC_CLK_SYNC_INV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_clk_inv(&self) -> ADC_CLK_INV_R {
        ADC_CLK_INV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&self) -> ADC_CLK_DIV_SEL_R {
        ADC_CLK_DIV_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_vref_sel(&mut self) -> ADC_VREF_SEL_W<0> {
        ADC_VREF_SEL_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dly_ctl(&mut self) -> ADC_DLY_CTL_W<4> {
        ADC_DLY_CTL_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dvdd_sel(&mut self) -> ADC_DVDD_SEL_W<8> {
        ADC_DVDD_SEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn adc_sar_ascal_en(&mut self) -> ADC_SAR_ASCAL_EN_W<12> {
        ADC_SAR_ASCAL_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn adc_gt_rm(&mut self) -> ADC_GT_RM_W<16> {
        ADC_GT_RM_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_sync_inv(&mut self) -> ADC_CLK_SYNC_INV_W<20> {
        ADC_CLK_SYNC_INV_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_inv(&mut self) -> ADC_CLK_INV_W<24> {
        ADC_CLK_INV_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_div_sel(&mut self) -> ADC_CLK_DIV_SEL_W<28> {
        ADC_CLK_DIV_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adda2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adda2](index.html) module"]
pub struct ADDA2_SPEC;
impl crate::RegisterSpec for ADDA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adda2::R](R) reader structure"]
impl crate::Readable for ADDA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adda2::W](W) writer structure"]
impl crate::Writable for ADDA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets adda2 to value 0"]
impl crate::Resettable for ADDA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

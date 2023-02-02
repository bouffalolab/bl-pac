#[doc = "Register `adda1` reader"]
pub struct R(crate::R<ADDA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adda1` writer"]
pub struct W(crate::W<ADDA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDA1_SPEC>;
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
impl From<crate::W<ADDA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac_dvdd_sel` reader - "]
pub type DAC_DVDD_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dac_dvdd_sel` writer - "]
pub type DAC_DVDD_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDA1_SPEC, u8, u8, 2, O>;
#[doc = "Field `dac_bias_sel` reader - "]
pub type DAC_BIAS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dac_bias_sel` writer - "]
pub type DAC_BIAS_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDA1_SPEC, u8, u8, 2, O>;
#[doc = "Field `dac_clk_sel` reader - "]
pub type DAC_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dac_clk_sel` writer - "]
pub type DAC_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDA1_SPEC, u8, u8, 2, O>;
#[doc = "Field `dac_rccalsel` reader - "]
pub type DAC_RCCALSEL_R = crate::BitReader<bool>;
#[doc = "Field `dac_rccalsel` writer - "]
pub type DAC_RCCALSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDA1_SPEC, bool, O>;
#[doc = "Field `dac_clk_sync_inv` reader - "]
pub type DAC_CLK_SYNC_INV_R = crate::BitReader<bool>;
#[doc = "Field `dac_clk_sync_inv` writer - "]
pub type DAC_CLK_SYNC_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDA1_SPEC, bool, O>;
#[doc = "Field `adda_ldo_byps` reader - "]
pub type ADDA_LDO_BYPS_R = crate::BitReader<bool>;
#[doc = "Field `adda_ldo_byps` writer - "]
pub type ADDA_LDO_BYPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDA1_SPEC, bool, O>;
#[doc = "Field `adda_ldo_dvdd_sel` reader - "]
pub type ADDA_LDO_DVDD_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adda_ldo_dvdd_sel` writer - "]
pub type ADDA_LDO_DVDD_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDA1_SPEC, u8, u8, 3, O>;
#[doc = "Field `adda_ldo_dvdd_sel_hw` reader - "]
pub type ADDA_LDO_DVDD_SEL_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adda_ldo_dvdd_sel_hw` writer - "]
pub type ADDA_LDO_DVDD_SEL_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDA1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_dvdd_sel(&self) -> DAC_DVDD_SEL_R {
        DAC_DVDD_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dac_bias_sel(&self) -> DAC_BIAS_SEL_R {
        DAC_BIAS_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dac_clk_sel(&self) -> DAC_CLK_SEL_R {
        DAC_CLK_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_rccalsel(&self) -> DAC_RCCALSEL_R {
        DAC_RCCALSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dac_clk_sync_inv(&self) -> DAC_CLK_SYNC_INV_R {
        DAC_CLK_SYNC_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adda_ldo_byps(&self) -> ADDA_LDO_BYPS_R {
        ADDA_LDO_BYPS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel(&self) -> ADDA_LDO_DVDD_SEL_R {
        ADDA_LDO_DVDD_SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_hw(&self) -> ADDA_LDO_DVDD_SEL_HW_R {
        ADDA_LDO_DVDD_SEL_HW_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn dac_dvdd_sel(&mut self) -> DAC_DVDD_SEL_W<0> {
        DAC_DVDD_SEL_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn dac_bias_sel(&mut self) -> DAC_BIAS_SEL_W<4> {
        DAC_BIAS_SEL_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_sel(&mut self) -> DAC_CLK_SEL_W<8> {
        DAC_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dac_rccalsel(&mut self) -> DAC_RCCALSEL_W<12> {
        DAC_RCCALSEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_sync_inv(&mut self) -> DAC_CLK_SYNC_INV_W<13> {
        DAC_CLK_SYNC_INV_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn adda_ldo_byps(&mut self) -> ADDA_LDO_BYPS_W<16> {
        ADDA_LDO_BYPS_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn adda_ldo_dvdd_sel(&mut self) -> ADDA_LDO_DVDD_SEL_W<20> {
        ADDA_LDO_DVDD_SEL_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn adda_ldo_dvdd_sel_hw(&mut self) -> ADDA_LDO_DVDD_SEL_HW_W<24> {
        ADDA_LDO_DVDD_SEL_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adda1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adda1](index.html) module"]
pub struct ADDA1_SPEC;
impl crate::RegisterSpec for ADDA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adda1::R](R) reader structure"]
impl crate::Readable for ADDA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adda1::W](W) writer structure"]
impl crate::Writable for ADDA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets adda1 to value 0"]
impl crate::Resettable for ADDA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

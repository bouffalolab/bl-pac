#[doc = "Register `vco2` reader"]
pub struct R(crate::R<VCO2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCO2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCO2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vco2` writer"]
pub struct W(crate::W<VCO2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCO2_SPEC>;
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
impl From<crate::W<VCO2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCO2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_vco_vbias_cw` reader - "]
pub type LO_VCO_VBIAS_CW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_vco_vbias_cw` writer - "]
pub type LO_VCO_VBIAS_CW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCO2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_vco_idac_boot` reader - "]
pub type LO_VCO_IDAC_BOOT_R = crate::BitReader<bool>;
#[doc = "Field `lo_vco_idac_boot` writer - "]
pub type LO_VCO_IDAC_BOOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO2_SPEC, bool, O>;
#[doc = "Field `lo_vco_short_vbias_filter` reader - "]
pub type LO_VCO_SHORT_VBIAS_FILTER_R = crate::BitReader<bool>;
#[doc = "Field `lo_vco_short_vbias_filter` writer - "]
pub type LO_VCO_SHORT_VBIAS_FILTER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VCO2_SPEC, bool, O>;
#[doc = "Field `lo_vco_short_idac_filter` reader - "]
pub type LO_VCO_SHORT_IDAC_FILTER_R = crate::BitReader<bool>;
#[doc = "Field `lo_vco_short_idac_filter` writer - "]
pub type LO_VCO_SHORT_IDAC_FILTER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VCO2_SPEC, bool, O>;
#[doc = "Field `acal_vref_cw` reader - "]
pub type ACAL_VREF_CW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acal_vref_cw` writer - "]
pub type ACAL_VREF_CW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCO2_SPEC, u8, u8, 3, O>;
#[doc = "Field `acal_vco_ud` reader - "]
pub type ACAL_VCO_UD_R = crate::BitReader<bool>;
#[doc = "Field `acal_vco_ud` writer - "]
pub type ACAL_VCO_UD_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO2_SPEC, bool, O>;
#[doc = "Field `acal_inc_en_hw` reader - "]
pub type ACAL_INC_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `acal_inc_en_hw` writer - "]
pub type ACAL_INC_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_vco_vbias_cw(&self) -> LO_VCO_VBIAS_CW_R {
        LO_VCO_VBIAS_CW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_vco_idac_boot(&self) -> LO_VCO_IDAC_BOOT_R {
        LO_VCO_IDAC_BOOT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lo_vco_short_vbias_filter(&self) -> LO_VCO_SHORT_VBIAS_FILTER_R {
        LO_VCO_SHORT_VBIAS_FILTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_vco_short_idac_filter(&self) -> LO_VCO_SHORT_IDAC_FILTER_R {
        LO_VCO_SHORT_IDAC_FILTER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn acal_vref_cw(&self) -> ACAL_VREF_CW_R {
        ACAL_VREF_CW_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn acal_vco_ud(&self) -> ACAL_VCO_UD_R {
        ACAL_VCO_UD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn acal_inc_en_hw(&self) -> ACAL_INC_EN_HW_R {
        ACAL_INC_EN_HW_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn lo_vco_vbias_cw(&mut self) -> LO_VCO_VBIAS_CW_W<0> {
        LO_VCO_VBIAS_CW_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn lo_vco_idac_boot(&mut self) -> LO_VCO_IDAC_BOOT_W<4> {
        LO_VCO_IDAC_BOOT_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn lo_vco_short_vbias_filter(&mut self) -> LO_VCO_SHORT_VBIAS_FILTER_W<5> {
        LO_VCO_SHORT_VBIAS_FILTER_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn lo_vco_short_idac_filter(&mut self) -> LO_VCO_SHORT_IDAC_FILTER_W<6> {
        LO_VCO_SHORT_IDAC_FILTER_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn acal_vref_cw(&mut self) -> ACAL_VREF_CW_W<8> {
        ACAL_VREF_CW_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn acal_vco_ud(&mut self) -> ACAL_VCO_UD_W<12> {
        ACAL_VCO_UD_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn acal_inc_en_hw(&mut self) -> ACAL_INC_EN_HW_W<16> {
        ACAL_INC_EN_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "vco2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco2](index.html) module"]
pub struct VCO2_SPEC;
impl crate::RegisterSpec for VCO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vco2::R](R) reader structure"]
impl crate::Readable for VCO2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vco2::W](W) writer structure"]
impl crate::Writable for VCO2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets vco2 to value 0"]
impl crate::Resettable for VCO2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

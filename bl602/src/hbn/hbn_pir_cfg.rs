#[doc = "Register `HBN_PIR_CFG` reader"]
pub struct R(crate::R<HBN_PIR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_PIR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_PIR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_PIR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_PIR_CFG` writer"]
pub struct W(crate::W<HBN_PIR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_PIR_CFG_SPEC>;
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
impl From<crate::W<HBN_PIR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_PIR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pir_hpf_sel` reader - "]
pub type PIR_HPF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pir_hpf_sel` writer - "]
pub type PIR_HPF_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_PIR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `pir_lpf_sel` reader - "]
pub type PIR_LPF_SEL_R = crate::BitReader<bool>;
#[doc = "Field `pir_lpf_sel` writer - "]
pub type PIR_LPF_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_PIR_CFG_SPEC, bool, O>;
#[doc = "Field `pir_dis` reader - "]
pub type PIR_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pir_dis` writer - "]
pub type PIR_DIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HBN_PIR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `pir_en` reader - "]
pub type PIR_EN_R = crate::BitReader<bool>;
#[doc = "Field `pir_en` writer - "]
pub type PIR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_PIR_CFG_SPEC, bool, O>;
#[doc = "Field `gpadc_cgen` reader - "]
pub type GPADC_CGEN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_cgen` writer - "]
pub type GPADC_CGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_PIR_CFG_SPEC, bool, O>;
#[doc = "Field `gpadc_nosync` reader - "]
pub type GPADC_NOSYNC_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_nosync` writer - "]
pub type GPADC_NOSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_PIR_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pir_hpf_sel(&self) -> PIR_HPF_SEL_R {
        PIR_HPF_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pir_lpf_sel(&self) -> PIR_LPF_SEL_R {
        PIR_LPF_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pir_dis(&self) -> PIR_DIS_R {
        PIR_DIS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pir_en(&self) -> PIR_EN_R {
        PIR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_cgen(&self) -> GPADC_CGEN_R {
        GPADC_CGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_nosync(&self) -> GPADC_NOSYNC_R {
        GPADC_NOSYNC_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pir_hpf_sel(&mut self) -> PIR_HPF_SEL_W<0> {
        PIR_HPF_SEL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pir_lpf_sel(&mut self) -> PIR_LPF_SEL_W<2> {
        PIR_LPF_SEL_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pir_dis(&mut self) -> PIR_DIS_W<4> {
        PIR_DIS_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pir_en(&mut self) -> PIR_EN_W<7> {
        PIR_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_cgen(&mut self) -> GPADC_CGEN_W<8> {
        GPADC_CGEN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_nosync(&mut self) -> GPADC_NOSYNC_W<9> {
        GPADC_NOSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_PIR_CFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_pir_cfg](index.html) module"]
pub struct HBN_PIR_CFG_SPEC;
impl crate::RegisterSpec for HBN_PIR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_pir_cfg::R](R) reader structure"]
impl crate::Readable for HBN_PIR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_pir_cfg::W](W) writer structure"]
impl crate::Writable for HBN_PIR_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_PIR_CFG to value 0"]
impl crate::Resettable for HBN_PIR_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

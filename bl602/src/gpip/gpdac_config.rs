#[doc = "Register `gpdac_config` reader"]
pub struct R(crate::R<GPDAC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_config` writer"]
pub struct W(crate::W<GPDAC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_CONFIG_SPEC>;
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
impl From<crate::W<GPDAC_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDAC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpdac_en` reader - "]
pub type GPDAC_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpdac_en` writer - "]
pub type GPDAC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `gpdac_en2` reader - "]
pub type GPDAC_EN2_R = crate::BitReader<bool>;
#[doc = "Field `gpdac_en2` writer - "]
pub type GPDAC_EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDAC_CONFIG_SPEC, bool, O>;
#[doc = "Field `dsm_mode` reader - "]
pub type DSM_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dsm_mode` writer - "]
pub type DSM_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDAC_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `gpdac_mode` reader - "]
pub type GPDAC_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpdac_mode` writer - "]
pub type GPDAC_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPDAC_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `gpdac_ch_a_sel` reader - "]
pub type GPDAC_CH_A_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpdac_ch_a_sel` writer - "]
pub type GPDAC_CH_A_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPDAC_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `gpdac_ch_b_sel` reader - "]
pub type GPDAC_CH_B_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpdac_ch_b_sel` writer - "]
pub type GPDAC_CH_B_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPDAC_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `rsvd_31_24` reader - "]
pub type RSVD_31_24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rsvd_31_24` writer - "]
pub type RSVD_31_24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPDAC_CONFIG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_en(&self) -> GPDAC_EN_R {
        GPDAC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_en2(&self) -> GPDAC_EN2_R {
        GPDAC_EN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dsm_mode(&self) -> DSM_MODE_R {
        DSM_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpdac_mode(&self) -> GPDAC_MODE_R {
        GPDAC_MODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gpdac_ch_a_sel(&self) -> GPDAC_CH_A_SEL_R {
        GPDAC_CH_A_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn gpdac_ch_b_sel(&self) -> GPDAC_CH_B_SEL_R {
        GPDAC_CH_B_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&self) -> RSVD_31_24_R {
        RSVD_31_24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_en(&mut self) -> GPDAC_EN_W<0> {
        GPDAC_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_en2(&mut self) -> GPDAC_EN2_W<1> {
        GPDAC_EN2_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn dsm_mode(&mut self) -> DSM_MODE_W<4> {
        DSM_MODE_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_mode(&mut self) -> GPDAC_MODE_W<8> {
        GPDAC_MODE_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_ch_a_sel(&mut self) -> GPDAC_CH_A_SEL_W<16> {
        GPDAC_CH_A_SEL_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_ch_b_sel(&mut self) -> GPDAC_CH_B_SEL_W<20> {
        GPDAC_CH_B_SEL_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_31_24(&mut self) -> RSVD_31_24_W<24> {
        RSVD_31_24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpdac_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_config](index.html) module"]
pub struct GPDAC_CONFIG_SPEC;
impl crate::RegisterSpec for GPDAC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_config::R](R) reader structure"]
impl crate::Readable for GPDAC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_config::W](W) writer structure"]
impl crate::Writable for GPDAC_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpdac_config to value 0"]
impl crate::Resettable for GPDAC_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

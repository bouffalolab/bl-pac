#[doc = "Register `bmx_cfg1` reader"]
pub struct R(crate::R<BMX_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_cfg1` writer"]
pub struct W(crate::W<BMX_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_CFG1_SPEC>;
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
impl From<crate::W<BMX_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMX_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bmx_timeout_en` reader - "]
pub type BMX_TIMEOUT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bmx_timeout_en` writer - "]
pub type BMX_TIMEOUT_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BMX_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `bmx_arb_mode` reader - "]
pub type BMX_ARB_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bmx_arb_mode` writer - "]
pub type BMX_ARB_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMX_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `bmx_err_en` reader - "]
pub type BMX_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `bmx_err_en` writer - "]
pub type BMX_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `bmx_busy_option_dis` reader - "]
pub type BMX_BUSY_OPTION_DIS_R = crate::BitReader<bool>;
#[doc = "Field `bmx_busy_option_dis` writer - "]
pub type BMX_BUSY_OPTION_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `bmx_gating_dis` reader - "]
pub type BMX_GATING_DIS_R = crate::BitReader<bool>;
#[doc = "Field `bmx_gating_dis` writer - "]
pub type BMX_GATING_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `hsel_option` reader - "]
pub type HSEL_OPTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hsel_option` writer - "]
pub type HSEL_OPTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMX_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `pds_apb_cfg` reader - "]
pub type PDS_APB_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_apb_cfg` writer - "]
pub type PDS_APB_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMX_CFG1_SPEC, u8, u8, 8, O>;
#[doc = "Field `hbn_apb_cfg` reader - "]
pub type HBN_APB_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_apb_cfg` writer - "]
pub type HBN_APB_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMX_CFG1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn bmx_timeout_en(&self) -> BMX_TIMEOUT_EN_R {
        BMX_TIMEOUT_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn bmx_arb_mode(&self) -> BMX_ARB_MODE_R {
        BMX_ARB_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bmx_err_en(&self) -> BMX_ERR_EN_R {
        BMX_ERR_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bmx_busy_option_dis(&self) -> BMX_BUSY_OPTION_DIS_R {
        BMX_BUSY_OPTION_DIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bmx_gating_dis(&self) -> BMX_GATING_DIS_R {
        BMX_GATING_DIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hsel_option(&self) -> HSEL_OPTION_R {
        HSEL_OPTION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&self) -> PDS_APB_CFG_R {
        PDS_APB_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&self) -> HBN_APB_CFG_R {
        HBN_APB_CFG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_timeout_en(&mut self) -> BMX_TIMEOUT_EN_W<0> {
        BMX_TIMEOUT_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_arb_mode(&mut self) -> BMX_ARB_MODE_W<4> {
        BMX_ARB_MODE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_err_en(&mut self) -> BMX_ERR_EN_W<8> {
        BMX_ERR_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_busy_option_dis(&mut self) -> BMX_BUSY_OPTION_DIS_W<9> {
        BMX_BUSY_OPTION_DIS_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_gating_dis(&mut self) -> BMX_GATING_DIS_W<10> {
        BMX_GATING_DIS_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn hsel_option(&mut self) -> HSEL_OPTION_W<12> {
        HSEL_OPTION_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pds_apb_cfg(&mut self) -> PDS_APB_CFG_W<16> {
        PDS_APB_CFG_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_apb_cfg(&mut self) -> HBN_APB_CFG_W<24> {
        HBN_APB_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg1](index.html) module"]
pub struct BMX_CFG1_SPEC;
impl crate::RegisterSpec for BMX_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_cfg1::R](R) reader structure"]
impl crate::Readable for BMX_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_cfg1::W](W) writer structure"]
impl crate::Writable for BMX_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg1 to value 0"]
impl crate::Resettable for BMX_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

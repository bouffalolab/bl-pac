#[doc = "Register `pa1` reader"]
pub struct R(crate::R<PA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pa1` writer"]
pub struct W(crate::W<PA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA1_SPEC>;
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
impl From<crate::W<PA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_iaq` reader - "]
pub type PA_IAQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_iaq` writer - "]
pub type PA_IAQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA1_SPEC, u8, u8, 3, O>;
#[doc = "Field `pa_etb_en` reader - "]
pub type PA_ETB_EN_R = crate::BitReader<bool>;
#[doc = "Field `pa_etb_en` writer - "]
pub type PA_ETB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA1_SPEC, bool, O>;
#[doc = "Field `pa_iet` reader - "]
pub type PA_IET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_iet` writer - "]
pub type PA_IET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA1_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vbcore` reader - "]
pub type PA_VBCORE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vbcore` writer - "]
pub type PA_VBCORE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA1_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vbcas` reader - "]
pub type PA_VBCAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vbcas` writer - "]
pub type PA_VBCAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA1_SPEC, u8, u8, 3, O>;
#[doc = "Field `pa_half_on` reader - "]
pub type PA_HALF_ON_R = crate::BitReader<bool>;
#[doc = "Field `pa_half_on` writer - "]
pub type PA_HALF_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA1_SPEC, bool, O>;
#[doc = "Field `pa_ib_fix` reader - "]
pub type PA_IB_FIX_R = crate::BitReader<bool>;
#[doc = "Field `pa_ib_fix` writer - "]
pub type PA_IB_FIX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA1_SPEC, bool, O>;
#[doc = "Field `pa_lz_bias_en` reader - "]
pub type PA_LZ_BIAS_EN_R = crate::BitReader<bool>;
#[doc = "Field `pa_lz_bias_en` writer - "]
pub type PA_LZ_BIAS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA1_SPEC, bool, O>;
#[doc = "Field `pa_pwrmx_osdac` reader - "]
pub type PA_PWRMX_OSDAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_pwrmx_osdac` writer - "]
pub type PA_PWRMX_OSDAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA1_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_pwrmx_dac_pn_switch` reader - "]
pub type PA_PWRMX_DAC_PN_SWITCH_R = crate::BitReader<bool>;
#[doc = "Field `pa_pwrmx_dac_pn_switch` writer - "]
pub type PA_PWRMX_DAC_PN_SWITCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA1_SPEC, bool, O>;
#[doc = "Field `pa_pwrmx_bm` reader - "]
pub type PA_PWRMX_BM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_pwrmx_bm` writer - "]
pub type PA_PWRMX_BM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA1_SPEC, u8, u8, 3, O>;
#[doc = "Field `pa_att_gc` reader - "]
pub type PA_ATT_GC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_att_gc` writer - "]
pub type PA_ATT_GC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pa_iaq(&self) -> PA_IAQ_R {
        PA_IAQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en(&self) -> PA_ETB_EN_R {
        PA_ETB_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet(&self) -> PA_IET_R {
        PA_IET_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore(&self) -> PA_VBCORE_R {
        PA_VBCORE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas(&self) -> PA_VBCAS_R {
        PA_VBCAS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pa_half_on(&self) -> PA_HALF_ON_R {
        PA_HALF_ON_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix(&self) -> PA_IB_FIX_R {
        PA_IB_FIX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_lz_bias_en(&self) -> PA_LZ_BIAS_EN_R {
        PA_LZ_BIAS_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn pa_pwrmx_osdac(&self) -> PA_PWRMX_OSDAC_R {
        PA_PWRMX_OSDAC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pa_pwrmx_dac_pn_switch(&self) -> PA_PWRMX_DAC_PN_SWITCH_R {
        PA_PWRMX_DAC_PN_SWITCH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_pwrmx_bm(&self) -> PA_PWRMX_BM_R {
        PA_PWRMX_BM_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pa_att_gc(&self) -> PA_ATT_GC_R {
        PA_ATT_GC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn pa_iaq(&mut self) -> PA_IAQ_W<0> {
        PA_IAQ_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pa_etb_en(&mut self) -> PA_ETB_EN_W<3> {
        PA_ETB_EN_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn pa_iet(&mut self) -> PA_IET_W<4> {
        PA_IET_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn pa_vbcore(&mut self) -> PA_VBCORE_W<8> {
        PA_VBCORE_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn pa_vbcas(&mut self) -> PA_VBCAS_W<12> {
        PA_VBCAS_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pa_half_on(&mut self) -> PA_HALF_ON_W<15> {
        PA_HALF_ON_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pa_ib_fix(&mut self) -> PA_IB_FIX_W<16> {
        PA_IB_FIX_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pa_lz_bias_en(&mut self) -> PA_LZ_BIAS_EN_W<17> {
        PA_LZ_BIAS_EN_W::new(self)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pwrmx_osdac(&mut self) -> PA_PWRMX_OSDAC_W<18> {
        PA_PWRMX_OSDAC_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pwrmx_dac_pn_switch(&mut self) -> PA_PWRMX_DAC_PN_SWITCH_W<22> {
        PA_PWRMX_DAC_PN_SWITCH_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pwrmx_bm(&mut self) -> PA_PWRMX_BM_W<24> {
        PA_PWRMX_BM_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn pa_att_gc(&mut self) -> PA_ATT_GC_W<28> {
        PA_ATT_GC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pa1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa1](index.html) module"]
pub struct PA1_SPEC;
impl crate::RegisterSpec for PA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa1::R](R) reader structure"]
impl crate::Readable for PA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa1::W](W) writer structure"]
impl crate::Writable for PA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pa1 to value 0"]
impl crate::Resettable for PA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

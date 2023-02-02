#[doc = "Register `lodist` reader"]
pub struct R(crate::R<LODIST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LODIST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LODIST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LODIST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lodist` writer"]
pub struct W(crate::W<LODIST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LODIST_SPEC>;
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
impl From<crate::W<LODIST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LODIST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_osmx_xgm_boost` reader - "]
pub type LO_OSMX_XGM_BOOST_R = crate::BitReader<bool>;
#[doc = "Field `lo_osmx_xgm_boost` writer - "]
pub type LO_OSMX_XGM_BOOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LODIST_SPEC, bool, O>;
#[doc = "Field `lo_osmx_en_xgm` reader - "]
pub type LO_OSMX_EN_XGM_R = crate::BitReader<bool>;
#[doc = "Field `lo_osmx_en_xgm` writer - "]
pub type LO_OSMX_EN_XGM_W<'a, const O: u8> = crate::BitWriter<'a, u32, LODIST_SPEC, bool, O>;
#[doc = "Field `lo_osmx_fix_cap` reader - "]
pub type LO_OSMX_FIX_CAP_R = crate::BitReader<bool>;
#[doc = "Field `lo_osmx_fix_cap` writer - "]
pub type LO_OSMX_FIX_CAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LODIST_SPEC, bool, O>;
#[doc = "Field `lo_osmx_vbuf_stre` reader - "]
pub type LO_OSMX_VBUF_STRE_R = crate::BitReader<bool>;
#[doc = "Field `lo_osmx_vbuf_stre` writer - "]
pub type LO_OSMX_VBUF_STRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LODIST_SPEC, bool, O>;
#[doc = "Field `lo_osmx_capbank_bias` reader - "]
pub type LO_OSMX_CAPBANK_BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_osmx_capbank_bias` writer - "]
pub type LO_OSMX_CAPBANK_BIAS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LODIST_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_osmx_cap` reader - "]
pub type LO_OSMX_CAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_osmx_cap` writer - "]
pub type LO_OSMX_CAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LODIST_SPEC, u8, u8, 4, O>;
#[doc = "Field `lo_lodist_txbuf_stre` reader - "]
pub type LO_LODIST_TXBUF_STRE_R = crate::BitReader<bool>;
#[doc = "Field `lo_lodist_txbuf_stre` writer - "]
pub type LO_LODIST_TXBUF_STRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LODIST_SPEC, bool, O>;
#[doc = "Field `lo_lodist_rxbuf_stre` reader - "]
pub type LO_LODIST_RXBUF_STRE_R = crate::BitReader<bool>;
#[doc = "Field `lo_lodist_rxbuf_stre` writer - "]
pub type LO_LODIST_RXBUF_STRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LODIST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_osmx_xgm_boost(&self) -> LO_OSMX_XGM_BOOST_R {
        LO_OSMX_XGM_BOOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_osmx_en_xgm(&self) -> LO_OSMX_EN_XGM_R {
        LO_OSMX_EN_XGM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_osmx_fix_cap(&self) -> LO_OSMX_FIX_CAP_R {
        LO_OSMX_FIX_CAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_osmx_vbuf_stre(&self) -> LO_OSMX_VBUF_STRE_R {
        LO_OSMX_VBUF_STRE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_osmx_capbank_bias(&self) -> LO_OSMX_CAPBANK_BIAS_R {
        LO_OSMX_CAPBANK_BIAS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lo_osmx_cap(&self) -> LO_OSMX_CAP_R {
        LO_OSMX_CAP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_lodist_txbuf_stre(&self) -> LO_LODIST_TXBUF_STRE_R {
        LO_LODIST_TXBUF_STRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_lodist_rxbuf_stre(&self) -> LO_LODIST_RXBUF_STRE_R {
        LO_LODIST_RXBUF_STRE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lo_osmx_xgm_boost(&mut self) -> LO_OSMX_XGM_BOOST_W<0> {
        LO_OSMX_XGM_BOOST_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn lo_osmx_en_xgm(&mut self) -> LO_OSMX_EN_XGM_W<4> {
        LO_OSMX_EN_XGM_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lo_osmx_fix_cap(&mut self) -> LO_OSMX_FIX_CAP_W<8> {
        LO_OSMX_FIX_CAP_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn lo_osmx_vbuf_stre(&mut self) -> LO_OSMX_VBUF_STRE_W<12> {
        LO_OSMX_VBUF_STRE_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn lo_osmx_capbank_bias(&mut self) -> LO_OSMX_CAPBANK_BIAS_W<16> {
        LO_OSMX_CAPBANK_BIAS_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn lo_osmx_cap(&mut self) -> LO_OSMX_CAP_W<20> {
        LO_OSMX_CAP_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lodist_txbuf_stre(&mut self) -> LO_LODIST_TXBUF_STRE_W<24> {
        LO_LODIST_TXBUF_STRE_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lodist_rxbuf_stre(&mut self) -> LO_LODIST_RXBUF_STRE_W<28> {
        LO_LODIST_RXBUF_STRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lodist.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lodist](index.html) module"]
pub struct LODIST_SPEC;
impl crate::RegisterSpec for LODIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lodist::R](R) reader structure"]
impl crate::Readable for LODIST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lodist::W](W) writer structure"]
impl crate::Writable for LODIST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lodist to value 0"]
impl crate::Resettable for LODIST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

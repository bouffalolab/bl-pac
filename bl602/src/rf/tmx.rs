#[doc = "Register `tmx` reader"]
pub struct R(crate::R<TMX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tmx` writer"]
pub struct W(crate::W<TMX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMX_SPEC>;
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
impl From<crate::W<TMX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmx_cs` reader - "]
pub type TMX_CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tmx_cs` writer - "]
pub type TMX_CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMX_SPEC, u8, u8, 3, O>;
#[doc = "Field `tmx_bm_sw` reader - "]
pub type TMX_BM_SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tmx_bm_sw` writer - "]
pub type TMX_BM_SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMX_SPEC, u8, u8, 3, O>;
#[doc = "Field `tmx_bm_cas` reader - "]
pub type TMX_BM_CAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tmx_bm_cas` writer - "]
pub type TMX_BM_CAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMX_SPEC, u8, u8, 3, O>;
#[doc = "Field `tmx_bm_cas_bulk` reader - "]
pub type TMX_BM_CAS_BULK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tmx_bm_cas_bulk` writer - "]
pub type TMX_BM_CAS_BULK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMX_SPEC, u8, u8, 3, O>;
#[doc = "Field `tx_tsense_en` reader - "]
pub type TX_TSENSE_EN_R = crate::BitReader<bool>;
#[doc = "Field `tx_tsense_en` writer - "]
pub type TX_TSENSE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMX_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmx_cs(&self) -> TMX_CS_R {
        TMX_CS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmx_bm_sw(&self) -> TMX_BM_SW_R {
        TMX_BM_SW_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tmx_bm_cas(&self) -> TMX_BM_CAS_R {
        TMX_BM_CAS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn tmx_bm_cas_bulk(&self) -> TMX_BM_CAS_BULK_R {
        TMX_BM_CAS_BULK_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_tsense_en(&self) -> TX_TSENSE_EN_R {
        TX_TSENSE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn tmx_cs(&mut self) -> TMX_CS_W<0> {
        TMX_CS_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn tmx_bm_sw(&mut self) -> TMX_BM_SW_W<4> {
        TMX_BM_SW_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn tmx_bm_cas(&mut self) -> TMX_BM_CAS_W<8> {
        TMX_BM_CAS_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn tmx_bm_cas_bulk(&mut self) -> TMX_BM_CAS_BULK_W<12> {
        TMX_BM_CAS_BULK_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tx_tsense_en(&mut self) -> TX_TSENSE_EN_W<16> {
        TX_TSENSE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tmx.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmx](index.html) module"]
pub struct TMX_SPEC;
impl crate::RegisterSpec for TMX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmx::R](R) reader structure"]
impl crate::Readable for TMX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmx::W](W) writer structure"]
impl crate::Writable for TMX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tmx to value 0"]
impl crate::Resettable for TMX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

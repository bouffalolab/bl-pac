#[doc = "Register `lna` reader"]
pub struct R(crate::R<LNA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LNA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LNA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LNA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lna` writer"]
pub struct W(crate::W<LNA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LNA_SPEC>;
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
impl From<crate::W<LNA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LNA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lna_bm` reader - "]
pub type LNA_BM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_bm` writer - "]
pub type LNA_BM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_SPEC, u8, u8, 4, O>;
#[doc = "Field `lna_bm_hw` reader - "]
pub type LNA_BM_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_bm_hw` writer - "]
pub type LNA_BM_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_SPEC, u8, u8, 4, O>;
#[doc = "Field `lna_load_csw` reader - "]
pub type LNA_LOAD_CSW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_load_csw` writer - "]
pub type LNA_LOAD_CSW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_SPEC, u8, u8, 4, O>;
#[doc = "Field `lna_load_csw_hw` reader - "]
pub type LNA_LOAD_CSW_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_load_csw_hw` writer - "]
pub type LNA_LOAD_CSW_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_SPEC, u8, u8, 4, O>;
#[doc = "Field `lna_rfb_match` reader - "]
pub type LNA_RFB_MATCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_rfb_match` writer - "]
pub type LNA_RFB_MATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_SPEC, u8, u8, 3, O>;
#[doc = "Field `lna_cap_lg` reader - "]
pub type LNA_CAP_LG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_cap_lg` writer - "]
pub type LNA_CAP_LG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_SPEC, u8, u8, 2, O>;
#[doc = "Field `lna_lg_gsel` reader - "]
pub type LNA_LG_GSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_lg_gsel` writer - "]
pub type LNA_LG_GSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm(&self) -> LNA_BM_R {
        LNA_BM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_hw(&self) -> LNA_BM_HW_R {
        LNA_BM_HW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&self) -> LNA_LOAD_CSW_R {
        LNA_LOAD_CSW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_hw(&self) -> LNA_LOAD_CSW_HW_R {
        LNA_LOAD_CSW_HW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn lna_rfb_match(&self) -> LNA_RFB_MATCH_R {
        LNA_RFB_MATCH_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lna_cap_lg(&self) -> LNA_CAP_LG_R {
        LNA_CAP_LG_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn lna_lg_gsel(&self) -> LNA_LG_GSEL_R {
        LNA_LG_GSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn lna_bm(&mut self) -> LNA_BM_W<0> {
        LNA_BM_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn lna_bm_hw(&mut self) -> LNA_BM_HW_W<4> {
        LNA_BM_HW_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn lna_load_csw(&mut self) -> LNA_LOAD_CSW_W<8> {
        LNA_LOAD_CSW_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn lna_load_csw_hw(&mut self) -> LNA_LOAD_CSW_HW_W<12> {
        LNA_LOAD_CSW_HW_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn lna_rfb_match(&mut self) -> LNA_RFB_MATCH_W<16> {
        LNA_RFB_MATCH_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn lna_cap_lg(&mut self) -> LNA_CAP_LG_W<20> {
        LNA_CAP_LG_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn lna_lg_gsel(&mut self) -> LNA_LG_GSEL_W<24> {
        LNA_LG_GSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lna.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lna](index.html) module"]
pub struct LNA_SPEC;
impl crate::RegisterSpec for LNA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lna::R](R) reader structure"]
impl crate::Readable for LNA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lna::W](W) writer structure"]
impl crate::Writable for LNA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lna to value 0"]
impl crate::Resettable for LNA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

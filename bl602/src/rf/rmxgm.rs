#[doc = "Register `rmxgm` reader"]
pub struct R(crate::R<RMXGM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMXGM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMXGM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMXGM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rmxgm` writer"]
pub struct W(crate::W<RMXGM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMXGM_SPEC>;
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
impl From<crate::W<RMXGM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMXGM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rmx_bm` reader - "]
pub type RMX_BM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rmx_bm` writer - "]
pub type RMX_BM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RMXGM_SPEC, u8, u8, 3, O>;
#[doc = "Field `rmxgm_bm` reader - "]
pub type RMXGM_BM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rmxgm_bm` writer - "]
pub type RMXGM_BM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RMXGM_SPEC, u8, u8, 3, O>;
#[doc = "Field `rmxgm_10m_mode_en` reader - "]
pub type RMXGM_10M_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `rmxgm_10m_mode_en` writer - "]
pub type RMXGM_10M_MODE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMXGM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&self) -> RMX_BM_R {
        RMX_BM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rmxgm_bm(&self) -> RMXGM_BM_R {
        RMXGM_BM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rmxgm_10m_mode_en(&self) -> RMXGM_10M_MODE_EN_R {
        RMXGM_10M_MODE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn rmx_bm(&mut self) -> RMX_BM_W<0> {
        RMX_BM_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn rmxgm_bm(&mut self) -> RMXGM_BM_W<4> {
        RMXGM_BM_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rmxgm_10m_mode_en(&mut self) -> RMXGM_10M_MODE_EN_W<8> {
        RMXGM_10M_MODE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rmxgm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmxgm](index.html) module"]
pub struct RMXGM_SPEC;
impl crate::RegisterSpec for RMXGM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmxgm::R](R) reader structure"]
impl crate::Readable for RMXGM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmxgm::W](W) writer structure"]
impl crate::Writable for RMXGM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rmxgm to value 0"]
impl crate::Resettable for RMXGM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

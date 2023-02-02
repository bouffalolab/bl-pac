#[doc = "Register `rbb4` reader"]
pub struct R(crate::R<RBB4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb4` writer"]
pub struct W(crate::W<RBB4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB4_SPEC>;
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
impl From<crate::W<RBB4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rbb_pkdet_vth` reader - "]
pub type RBB_PKDET_VTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_pkdet_vth` writer - "]
pub type RBB_PKDET_VTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB4_SPEC, u8, u8, 4, O>;
#[doc = "Field `rbb_pkdet_out_rstn` reader - "]
pub type RBB_PKDET_OUT_RSTN_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_out_rstn` writer - "]
pub type RBB_PKDET_OUT_RSTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB4_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_en` reader - "]
pub type RBB_PKDET_EN_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_en` writer - "]
pub type RBB_PKDET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB4_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_out_rstn_hw` reader - "]
pub type RBB_PKDET_OUT_RSTN_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_out_rstn_hw` writer - "]
pub type RBB_PKDET_OUT_RSTN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB4_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_en_hw` reader - "]
pub type RBB_PKDET_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_en_hw` writer - "]
pub type RBB_PKDET_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB4_SPEC, bool, O>;
#[doc = "Field `pkdet_out_raw` reader - "]
pub type PKDET_OUT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `pkdet_out_raw` writer - "]
pub type PKDET_OUT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB4_SPEC, bool, O>;
#[doc = "Field `pkdet_out_latch` reader - "]
pub type PKDET_OUT_LATCH_R = crate::BitReader<bool>;
#[doc = "Field `pkdet_out_latch` writer - "]
pub type PKDET_OUT_LATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rbb_pkdet_vth(&self) -> RBB_PKDET_VTH_R {
        RBB_PKDET_VTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn(&self) -> RBB_PKDET_OUT_RSTN_R {
        RBB_PKDET_OUT_RSTN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_pkdet_en(&self) -> RBB_PKDET_EN_R {
        RBB_PKDET_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_hw(&self) -> RBB_PKDET_OUT_RSTN_HW_R {
        RBB_PKDET_OUT_RSTN_HW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rbb_pkdet_en_hw(&self) -> RBB_PKDET_EN_HW_R {
        RBB_PKDET_EN_HW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pkdet_out_raw(&self) -> PKDET_OUT_RAW_R {
        PKDET_OUT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pkdet_out_latch(&self) -> PKDET_OUT_LATCH_R {
        PKDET_OUT_LATCH_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_vth(&mut self) -> RBB_PKDET_VTH_W<0> {
        RBB_PKDET_VTH_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_out_rstn(&mut self) -> RBB_PKDET_OUT_RSTN_W<4> {
        RBB_PKDET_OUT_RSTN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_en(&mut self) -> RBB_PKDET_EN_W<8> {
        RBB_PKDET_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_out_rstn_hw(&mut self) -> RBB_PKDET_OUT_RSTN_HW_W<12> {
        RBB_PKDET_OUT_RSTN_HW_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_en_hw(&mut self) -> RBB_PKDET_EN_HW_W<16> {
        RBB_PKDET_EN_HW_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pkdet_out_raw(&mut self) -> PKDET_OUT_RAW_W<20> {
        PKDET_OUT_RAW_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pkdet_out_latch(&mut self) -> PKDET_OUT_LATCH_W<24> {
        PKDET_OUT_LATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb4](index.html) module"]
pub struct RBB4_SPEC;
impl crate::RegisterSpec for RBB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb4::R](R) reader structure"]
impl crate::Readable for RBB4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb4::W](W) writer structure"]
impl crate::Writable for RBB4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rbb4 to value 0"]
impl crate::Resettable for RBB4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

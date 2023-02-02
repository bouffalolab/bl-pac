#[doc = "Register `rf_pkdet_ctrl0` reader"]
pub struct R(crate::R<RF_PKDET_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_PKDET_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_PKDET_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_PKDET_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_pkdet_ctrl0` writer"]
pub struct W(crate::W<RF_PKDET_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_PKDET_CTRL0_SPEC>;
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
impl From<crate::W<RF_PKDET_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_PKDET_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pkdet_out_cnt_sts` reader - "]
pub type PKDET_OUT_CNT_STS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pkdet_out_cnt_sts` writer - "]
pub type PKDET_OUT_CNT_STS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_PKDET_CTRL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `pkdet_out_cnt_en` reader - "]
pub type PKDET_OUT_CNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `pkdet_out_cnt_en` writer - "]
pub type PKDET_OUT_CNT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_PKDET_CTRL0_SPEC, bool, O>;
#[doc = "Field `pkdet_out_mode` reader - "]
pub type PKDET_OUT_MODE_R = crate::BitReader<bool>;
#[doc = "Field `pkdet_out_mode` writer - "]
pub type PKDET_OUT_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_PKDET_CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pkdet_out_cnt_sts(&self) -> PKDET_OUT_CNT_STS_R {
        PKDET_OUT_CNT_STS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pkdet_out_cnt_en(&self) -> PKDET_OUT_CNT_EN_R {
        PKDET_OUT_CNT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pkdet_out_mode(&self) -> PKDET_OUT_MODE_R {
        PKDET_OUT_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn pkdet_out_cnt_sts(&mut self) -> PKDET_OUT_CNT_STS_W<0> {
        PKDET_OUT_CNT_STS_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pkdet_out_cnt_en(&mut self) -> PKDET_OUT_CNT_EN_W<4> {
        PKDET_OUT_CNT_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pkdet_out_mode(&mut self) -> PKDET_OUT_MODE_W<5> {
        PKDET_OUT_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_pkdet_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_pkdet_ctrl0](index.html) module"]
pub struct RF_PKDET_CTRL0_SPEC;
impl crate::RegisterSpec for RF_PKDET_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_pkdet_ctrl0::R](R) reader structure"]
impl crate::Readable for RF_PKDET_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_pkdet_ctrl0::W](W) writer structure"]
impl crate::Writable for RF_PKDET_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_pkdet_ctrl0 to value 0"]
impl crate::Resettable for RF_PKDET_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

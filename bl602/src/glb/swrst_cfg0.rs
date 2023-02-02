#[doc = "Register `swrst_cfg0` reader"]
pub struct R(crate::R<SWRST_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRST_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRST_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `swrst_cfg0` writer"]
pub struct W(crate::W<SWRST_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRST_CFG0_SPEC>;
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
impl From<crate::W<SWRST_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRST_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `swrst_s00` reader - "]
pub type SWRST_S00_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s00` writer - "]
pub type SWRST_S00_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
#[doc = "Field `swrst_s01` reader - "]
pub type SWRST_S01_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s01` writer - "]
pub type SWRST_S01_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
#[doc = "Field `swrst_s20` reader - "]
pub type SWRST_S20_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s20` writer - "]
pub type SWRST_S20_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
#[doc = "Field `swrst_s30` reader - "]
pub type SWRST_S30_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s30` writer - "]
pub type SWRST_S30_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s00(&self) -> SWRST_S00_R {
        SWRST_S00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s01(&self) -> SWRST_S01_R {
        SWRST_S01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s20(&self) -> SWRST_S20_R {
        SWRST_S20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s30(&self) -> SWRST_S30_R {
        SWRST_S30_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s00(&mut self) -> SWRST_S00_W<0> {
        SWRST_S00_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s01(&mut self) -> SWRST_S01_W<1> {
        SWRST_S01_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s20(&mut self) -> SWRST_S20_W<4> {
        SWRST_S20_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s30(&mut self) -> SWRST_S30_W<8> {
        SWRST_S30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "swrst_cfg0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg0](index.html) module"]
pub struct SWRST_CFG0_SPEC;
impl crate::RegisterSpec for SWRST_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst_cfg0::R](R) reader structure"]
impl crate::Readable for SWRST_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrst_cfg0::W](W) writer structure"]
impl crate::Writable for SWRST_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets swrst_cfg0 to value 0"]
impl crate::Resettable for SWRST_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

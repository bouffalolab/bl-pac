#[doc = "Register `TIER2` reader"]
pub struct R(crate::R<TIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIER2` writer"]
pub struct W(crate::W<TIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIER2_SPEC>;
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
impl From<crate::W<TIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tier_0` reader - "]
pub type TIER_0_R = crate::BitReader<bool>;
#[doc = "Field `tier_0` writer - "]
pub type TIER_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIER2_SPEC, bool, O>;
#[doc = "Field `tier_1` reader - "]
pub type TIER_1_R = crate::BitReader<bool>;
#[doc = "Field `tier_1` writer - "]
pub type TIER_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIER2_SPEC, bool, O>;
#[doc = "Field `tier_2` reader - "]
pub type TIER_2_R = crate::BitReader<bool>;
#[doc = "Field `tier_2` writer - "]
pub type TIER_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIER2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tier_0(&self) -> TIER_0_R {
        TIER_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tier_1(&self) -> TIER_1_R {
        TIER_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tier_2(&self) -> TIER_2_R {
        TIER_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tier_0(&mut self) -> TIER_0_W<0> {
        TIER_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tier_1(&mut self) -> TIER_1_W<1> {
        TIER_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tier_2(&mut self) -> TIER_2_W<2> {
        TIER_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIER2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tier2](index.html) module"]
pub struct TIER2_SPEC;
impl crate::RegisterSpec for TIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tier2::R](R) reader structure"]
impl crate::Readable for TIER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tier2::W](W) writer structure"]
impl crate::Writable for TIER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIER2 to value 0"]
impl crate::Resettable for TIER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

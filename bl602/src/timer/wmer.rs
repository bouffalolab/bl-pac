#[doc = "Register `WMER` reader"]
pub struct R(crate::R<WMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WMER` writer"]
pub struct W(crate::W<WMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WMER_SPEC>;
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
impl From<crate::W<WMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `we` reader - "]
pub type WE_R = crate::BitReader<bool>;
#[doc = "Field `we` writer - "]
pub type WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WMER_SPEC, bool, O>;
#[doc = "Field `wrie` reader - "]
pub type WRIE_R = crate::BitReader<bool>;
#[doc = "Field `wrie` writer - "]
pub type WRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WMER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wrie(&self) -> WRIE_R {
        WRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn we(&mut self) -> WE_W<0> {
        WE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn wrie(&mut self) -> WRIE_W<1> {
        WRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WMER.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmer](index.html) module"]
pub struct WMER_SPEC;
impl crate::RegisterSpec for WMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wmer::R](R) reader structure"]
impl crate::Readable for WMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wmer::W](W) writer structure"]
impl crate::Writable for WMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WMER to value 0"]
impl crate::Resettable for WMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

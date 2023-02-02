#[doc = "Register `rsv2` reader"]
pub struct R(crate::R<RSV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rsv2` writer"]
pub struct W(crate::W<RSV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSV2_SPEC>;
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
impl From<crate::W<RSV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rsvd_31_0` reader - "]
pub type RSVD_31_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `rsvd_31_0` writer - "]
pub type RSVD_31_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSV2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rsvd_31_0(&self) -> RSVD_31_0_R {
        RSVD_31_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_31_0(&mut self) -> RSVD_31_0_W<0> {
        RSVD_31_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rsv2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsv2](index.html) module"]
pub struct RSV2_SPEC;
impl crate::RegisterSpec for RSV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsv2::R](R) reader structure"]
impl crate::Readable for RSV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsv2::W](W) writer structure"]
impl crate::Writable for RSV2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rsv2 to value 0"]
impl crate::Resettable for RSV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `debug` reader"]
pub struct R(crate::R<DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `debug` writer"]
pub struct W(crate::W<DEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_SPEC>;
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
impl From<crate::W<DEBUG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `debug_oe` reader - "]
pub type DEBUG_OE_R = crate::BitReader<bool>;
#[doc = "Field `debug_oe` writer - "]
pub type DEBUG_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_SPEC, bool, O>;
#[doc = "Field `debug_i` reader - "]
pub type DEBUG_I_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&self) -> DEBUG_OE_R {
        DEBUG_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn debug_i(&self) -> DEBUG_I_R {
        DEBUG_I_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn debug_oe(&mut self) -> DEBUG_OE_W<0> {
        DEBUG_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "debug.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug](index.html) module"]
pub struct DEBUG_SPEC;
impl crate::RegisterSpec for DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug::R](R) reader structure"]
impl crate::Readable for DEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug::W](W) writer structure"]
impl crate::Writable for DEBUG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets debug to value 0"]
impl crate::Resettable for DEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

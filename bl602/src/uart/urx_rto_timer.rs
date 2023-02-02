#[doc = "Register `urx_rto_timer` reader"]
pub struct R(crate::R<URX_RTO_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<URX_RTO_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<URX_RTO_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<URX_RTO_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `urx_rto_timer` writer"]
pub struct W(crate::W<URX_RTO_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<URX_RTO_TIMER_SPEC>;
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
impl From<crate::W<URX_RTO_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<URX_RTO_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_urx_rto_value` reader - "]
pub type CR_URX_RTO_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_urx_rto_value` writer - "]
pub type CR_URX_RTO_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, URX_RTO_TIMER_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_urx_rto_value(&self) -> CR_URX_RTO_VALUE_R {
        CR_URX_RTO_VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_rto_value(&mut self) -> CR_URX_RTO_VALUE_W<0> {
        CR_URX_RTO_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "urx_rto_timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [urx_rto_timer](index.html) module"]
pub struct URX_RTO_TIMER_SPEC;
impl crate::RegisterSpec for URX_RTO_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [urx_rto_timer::R](R) reader structure"]
impl crate::Readable for URX_RTO_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [urx_rto_timer::W](W) writer structure"]
impl crate::Writable for URX_RTO_TIMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets urx_rto_timer to value 0x0f"]
impl crate::Resettable for URX_RTO_TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}

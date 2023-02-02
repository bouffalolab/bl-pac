#[doc = "Register `cgen_cfg0` reader"]
pub struct R(crate::R<CGEN_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGEN_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGEN_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg0` writer"]
pub struct W(crate::W<CGEN_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG0_SPEC>;
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
impl From<crate::W<CGEN_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGEN_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cgen_m` reader - "]
pub type CGEN_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cgen_m` writer - "]
pub type CGEN_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CGEN_CFG0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cgen_m(&self) -> CGEN_M_R {
        CGEN_M_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m(&mut self) -> CGEN_M_W<0> {
        CGEN_M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_cfg0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg0](index.html) module"]
pub struct CGEN_CFG0_SPEC;
impl crate::RegisterSpec for CGEN_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg0::R](R) reader structure"]
impl crate::Readable for CGEN_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg0::W](W) writer structure"]
impl crate::Writable for CGEN_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cgen_cfg0 to value 0xff"]
impl crate::Resettable for CGEN_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}

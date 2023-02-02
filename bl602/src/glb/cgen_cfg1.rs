#[doc = "Register `cgen_cfg1` reader"]
pub struct R(crate::R<CGEN_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGEN_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGEN_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg1` writer"]
pub struct W(crate::W<CGEN_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG1_SPEC>;
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
impl From<crate::W<CGEN_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGEN_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cgen_s1` reader - "]
pub type CGEN_S1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cgen_s1` writer - "]
pub type CGEN_S1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CGEN_CFG1_SPEC, u16, u16, 16, O>;
#[doc = "Field `cgen_s1a` reader - "]
pub type CGEN_S1A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cgen_s1a` writer - "]
pub type CGEN_S1A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CGEN_CFG1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cgen_s1(&self) -> CGEN_S1_R {
        CGEN_S1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cgen_s1a(&self) -> CGEN_S1A_R {
        CGEN_S1A_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1(&mut self) -> CGEN_S1_W<0> {
        CGEN_S1_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1a(&mut self) -> CGEN_S1A_W<16> {
        CGEN_S1A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg1](index.html) module"]
pub struct CGEN_CFG1_SPEC;
impl crate::RegisterSpec for CGEN_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg1::R](R) reader structure"]
impl crate::Readable for CGEN_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg1::W](W) writer structure"]
impl crate::Writable for CGEN_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cgen_cfg1 to value 0x00ff_ffff"]
impl crate::Resettable for CGEN_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_ffff;
}

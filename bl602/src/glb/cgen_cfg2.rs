#[doc = "Register `cgen_cfg2` reader"]
pub struct R(crate::R<CGEN_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGEN_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGEN_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg2` writer"]
pub struct W(crate::W<CGEN_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG2_SPEC>;
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
impl From<crate::W<CGEN_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGEN_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cgen_s2` reader - "]
pub type CGEN_S2_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s2` writer - "]
pub type CGEN_S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s3` reader - "]
pub type CGEN_S3_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s3` writer - "]
pub type CGEN_S3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s2(&self) -> CGEN_S2_R {
        CGEN_S2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s3(&self) -> CGEN_S3_R {
        CGEN_S3_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s2(&mut self) -> CGEN_S2_W<0> {
        CGEN_S2_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s3(&mut self) -> CGEN_S3_W<4> {
        CGEN_S3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_cfg2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg2](index.html) module"]
pub struct CGEN_CFG2_SPEC;
impl crate::RegisterSpec for CGEN_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg2::R](R) reader structure"]
impl crate::Readable for CGEN_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg2::W](W) writer structure"]
impl crate::Writable for CGEN_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cgen_cfg2 to value 0x11"]
impl crate::Resettable for CGEN_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x11;
}

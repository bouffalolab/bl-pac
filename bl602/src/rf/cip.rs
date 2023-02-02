#[doc = "Register `cip` reader"]
pub struct R(crate::R<CIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cip` writer"]
pub struct W(crate::W<CIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIP_SPEC>;
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
impl From<crate::W<CIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vg11_sel` reader - "]
pub type VG11_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vg11_sel` writer - "]
pub type VG11_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIP_SPEC, u8, u8, 2, O>;
#[doc = "Field `vg13_sel` reader - "]
pub type VG13_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vg13_sel` writer - "]
pub type VG13_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIP_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vg11_sel(&self) -> VG11_SEL_R {
        VG11_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vg13_sel(&self) -> VG13_SEL_R {
        VG13_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn vg11_sel(&mut self) -> VG11_SEL_W<0> {
        VG11_SEL_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn vg13_sel(&mut self) -> VG13_SEL_W<2> {
        VG13_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX normal bias mode registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cip](index.html) module"]
pub struct CIP_SPEC;
impl crate::RegisterSpec for CIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cip::R](R) reader structure"]
impl crate::Readable for CIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cip::W](W) writer structure"]
impl crate::Writable for CIP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cip to value 0"]
impl crate::Resettable for CIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

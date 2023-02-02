#[doc = "Register `temp_comp` reader"]
pub struct R(crate::R<TEMP_COMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMP_COMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMP_COMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMP_COMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `temp_comp` writer"]
pub struct W(crate::W<TEMP_COMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMP_COMP_SPEC>;
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
impl From<crate::W<TEMP_COMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMP_COMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `const_acal` reader - "]
pub type CONST_ACAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `const_acal` writer - "]
pub type CONST_ACAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMP_COMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `const_fcal` reader - "]
pub type CONST_FCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `const_fcal` writer - "]
pub type CONST_FCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMP_COMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `temp_comp_en` reader - "]
pub type TEMP_COMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `temp_comp_en` writer - "]
pub type TEMP_COMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEMP_COMP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn const_acal(&self) -> CONST_ACAL_R {
        CONST_ACAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn const_fcal(&self) -> CONST_FCAL_R {
        CONST_FCAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn temp_comp_en(&self) -> TEMP_COMP_EN_R {
        TEMP_COMP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn const_acal(&mut self) -> CONST_ACAL_W<0> {
        CONST_ACAL_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn const_fcal(&mut self) -> CONST_FCAL_W<8> {
        CONST_FCAL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn temp_comp_en(&mut self) -> TEMP_COMP_EN_W<16> {
        TEMP_COMP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "temp_comp.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_comp](index.html) module"]
pub struct TEMP_COMP_SPEC;
impl crate::RegisterSpec for TEMP_COMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [temp_comp::R](R) reader structure"]
impl crate::Readable for TEMP_COMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [temp_comp::W](W) writer structure"]
impl crate::Writable for TEMP_COMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets temp_comp to value 0"]
impl crate::Resettable for TEMP_COMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

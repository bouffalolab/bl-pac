#[doc = "Register `rf_rev` reader"]
pub struct R(crate::R<RF_REV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_REV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_REV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_REV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_rev` writer"]
pub struct W(crate::W<RF_REV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_REV_SPEC>;
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
impl From<crate::W<RF_REV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_REV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_id` reader - "]
pub type RF_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_id` writer - "]
pub type RF_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RF_REV_SPEC, u8, u8, 8, O>;
#[doc = "Field `fw_rev` reader - "]
pub type FW_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fw_rev` writer - "]
pub type FW_REV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RF_REV_SPEC, u8, u8, 8, O>;
#[doc = "Field `hw_rev` reader - "]
pub type HW_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hw_rev` writer - "]
pub type HW_REV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RF_REV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rf_id(&self) -> RF_ID_R {
        RF_ID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn fw_rev(&self) -> FW_REV_R {
        FW_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hw_rev(&self) -> HW_REV_R {
        HW_REV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rf_id(&mut self) -> RF_ID_W<0> {
        RF_ID_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn fw_rev(&mut self) -> FW_REV_W<8> {
        FW_REV_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn hw_rev(&mut self) -> HW_REV_W<16> {
        HW_REV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Silicon revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_rev](index.html) module"]
pub struct RF_REV_SPEC;
impl crate::RegisterSpec for RF_REV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_rev::R](R) reader structure"]
impl crate::Readable for RF_REV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_rev::W](W) writer structure"]
impl crate::Writable for RF_REV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_rev to value 0"]
impl crate::Resettable for RF_REV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

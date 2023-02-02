#[doc = "Register `HBN_PIR_INTERVAL` reader"]
pub struct R(crate::R<HBN_PIR_INTERVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_PIR_INTERVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_PIR_INTERVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_PIR_INTERVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_PIR_INTERVAL` writer"]
pub struct W(crate::W<HBN_PIR_INTERVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_PIR_INTERVAL_SPEC>;
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
impl From<crate::W<HBN_PIR_INTERVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_PIR_INTERVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pir_interval` reader - "]
pub type PIR_INTERVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pir_interval` writer - "]
pub type PIR_INTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_PIR_INTERVAL_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pir_interval(&self) -> PIR_INTERVAL_R {
        PIR_INTERVAL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn pir_interval(&mut self) -> PIR_INTERVAL_W<0> {
        PIR_INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_PIR_INTERVAL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_pir_interval](index.html) module"]
pub struct HBN_PIR_INTERVAL_SPEC;
impl crate::RegisterSpec for HBN_PIR_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_pir_interval::R](R) reader structure"]
impl crate::Readable for HBN_PIR_INTERVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_pir_interval::W](W) writer structure"]
impl crate::Writable for HBN_PIR_INTERVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_PIR_INTERVAL to value 0x0a3d"]
impl crate::Resettable for HBN_PIR_INTERVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a3d;
}

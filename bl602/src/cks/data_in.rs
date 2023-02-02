#[doc = "Register `data_in` reader"]
pub struct R(crate::R<DATA_IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `data_in` writer"]
pub struct W(crate::W<DATA_IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_IN_SPEC>;
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
impl From<crate::W<DATA_IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data_in` writer - "]
pub type DATA_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA_IN_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn data_in(&mut self) -> DATA_IN_W<0> {
        DATA_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Checksum data in\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_in](index.html) module"]
pub struct DATA_IN_SPEC;
impl crate::RegisterSpec for DATA_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_in::R](R) reader structure"]
impl crate::Readable for DATA_IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_in::W](W) writer structure"]
impl crate::Writable for DATA_IN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets data_in to value 0"]
impl crate::Resettable for DATA_IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

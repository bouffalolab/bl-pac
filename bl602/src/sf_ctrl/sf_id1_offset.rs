#[doc = "Register `sf_id1_offset` reader"]
pub struct R(crate::R<SF_ID1_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_ID1_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_ID1_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_ID1_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_id1_offset` writer"]
pub struct W(crate::W<SF_ID1_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_ID1_OFFSET_SPEC>;
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
impl From<crate::W<SF_ID1_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_ID1_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_id1_offset` reader - "]
pub type SF_ID1_OFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `sf_id1_offset` writer - "]
pub type SF_ID1_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_ID1_OFFSET_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn sf_id1_offset(&self) -> SF_ID1_OFFSET_R {
        SF_ID1_OFFSET_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    #[must_use]
    pub fn sf_id1_offset(&mut self) -> SF_ID1_OFFSET_W<0> {
        SF_ID1_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_id1_offset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_id1_offset](index.html) module"]
pub struct SF_ID1_OFFSET_SPEC;
impl crate::RegisterSpec for SF_ID1_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_id1_offset::R](R) reader structure"]
impl crate::Readable for SF_ID1_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_id1_offset::W](W) writer structure"]
impl crate::Writable for SF_ID1_OFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_id1_offset to value 0"]
impl crate::Resettable for SF_ID1_OFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

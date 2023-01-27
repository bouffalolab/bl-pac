#[doc = "Register `linked_list` reader"]
pub struct R(crate::R<LINKED_LIST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINKED_LIST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINKED_LIST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINKED_LIST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `linked_list` writer"]
pub struct W(crate::W<LINKED_LIST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINKED_LIST_SPEC>;
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
impl From<crate::W<LINKED_LIST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINKED_LIST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `base_address` reader - Base address of first linked list item, must be aligned to 4 bytes"]
pub type BASE_ADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `base_address` writer - Base address of first linked list item, must be aligned to 4 bytes"]
pub type BASE_ADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LINKED_LIST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Base address of first linked list item, must be aligned to 4 bytes"]
    #[inline(always)]
    pub fn base_address(&self) -> BASE_ADDRESS_R {
        BASE_ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address of first linked list item, must be aligned to 4 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn base_address(&mut self) -> BASE_ADDRESS_W<0> {
        BASE_ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Linked list buffer base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linked_list](index.html) module"]
pub struct LINKED_LIST_SPEC;
impl crate::RegisterSpec for LINKED_LIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [linked_list::R](R) reader structure"]
impl crate::Readable for LINKED_LIST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linked_list::W](W) writer structure"]
impl crate::Writable for LINKED_LIST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets linked_list to value 0"]
impl crate::Resettable for LINKED_LIST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

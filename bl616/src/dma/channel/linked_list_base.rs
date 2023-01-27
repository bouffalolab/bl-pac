#[doc = "Register `linked_list_base` reader"]
pub struct R(crate::R<LINKED_LIST_BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINKED_LIST_BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINKED_LIST_BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINKED_LIST_BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `linked_list_base` writer"]
pub struct W(crate::W<LINKED_LIST_BASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINKED_LIST_BASE_SPEC>;
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
impl From<crate::W<LINKED_LIST_BASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINKED_LIST_BASE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Linked list buffer base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linked_list_base](index.html) module"]
pub struct LINKED_LIST_BASE_SPEC;
impl crate::RegisterSpec for LINKED_LIST_BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [linked_list_base::R](R) reader structure"]
impl crate::Readable for LINKED_LIST_BASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linked_list_base::W](W) writer structure"]
impl crate::Writable for LINKED_LIST_BASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets linked_list_base to value 0"]
impl crate::Resettable for LINKED_LIST_BASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

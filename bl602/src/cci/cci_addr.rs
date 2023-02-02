#[doc = "Register `cci_addr` reader"]
pub struct R(crate::R<CCI_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCI_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCI_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCI_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cci_addr` writer"]
pub struct W(crate::W<CCI_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCI_ADDR_SPEC>;
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
impl From<crate::W<CCI_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCI_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `apb_cci_addr` reader - "]
pub type APB_CCI_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `apb_cci_addr` writer - "]
pub type APB_CCI_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCI_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_cci_addr(&self) -> APB_CCI_ADDR_R {
        APB_CCI_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn apb_cci_addr(&mut self) -> APB_CCI_ADDR_W<0> {
        APB_CCI_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cci_addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_addr](index.html) module"]
pub struct CCI_ADDR_SPEC;
impl crate::RegisterSpec for CCI_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cci_addr::R](R) reader structure"]
impl crate::Readable for CCI_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cci_addr::W](W) writer structure"]
impl crate::Writable for CCI_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cci_addr to value 0"]
impl crate::Resettable for CCI_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

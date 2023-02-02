#[doc = "Register `uart_bit_prd` reader"]
pub struct R(crate::R<UART_BIT_PRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_BIT_PRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_BIT_PRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_BIT_PRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_bit_prd` writer"]
pub struct W(crate::W<UART_BIT_PRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_BIT_PRD_SPEC>;
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
impl From<crate::W<UART_BIT_PRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_BIT_PRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_utx_bit_prd` reader - "]
pub type CR_UTX_BIT_PRD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_utx_bit_prd` writer - "]
pub type CR_UTX_BIT_PRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_BIT_PRD_SPEC, u16, u16, 16, O>;
#[doc = "Field `cr_urx_bit_prd` reader - "]
pub type CR_URX_BIT_PRD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_urx_bit_prd` writer - "]
pub type CR_URX_BIT_PRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_BIT_PRD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_bit_prd(&self) -> CR_UTX_BIT_PRD_R {
        CR_UTX_BIT_PRD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_urx_bit_prd(&self) -> CR_URX_BIT_PRD_R {
        CR_URX_BIT_PRD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_bit_prd(&mut self) -> CR_UTX_BIT_PRD_W<0> {
        CR_UTX_BIT_PRD_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_bit_prd(&mut self) -> CR_URX_BIT_PRD_W<16> {
        CR_URX_BIT_PRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_bit_prd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_bit_prd](index.html) module"]
pub struct UART_BIT_PRD_SPEC;
impl crate::RegisterSpec for UART_BIT_PRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_bit_prd::R](R) reader structure"]
impl crate::Readable for UART_BIT_PRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_bit_prd::W](W) writer structure"]
impl crate::Writable for UART_BIT_PRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_bit_prd to value 0x00ff_00ff"]
impl crate::Resettable for UART_BIT_PRD_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_00ff;
}

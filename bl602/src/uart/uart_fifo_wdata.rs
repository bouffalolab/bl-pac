#[doc = "Register `uart_fifo_wdata` writer"]
pub struct W(crate::W<UART_FIFO_WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FIFO_WDATA_SPEC>;
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
impl From<crate::W<UART_FIFO_WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_FIFO_WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_fifo_wdata` writer - "]
pub type UART_FIFO_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_FIFO_WDATA_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn uart_fifo_wdata(&mut self) -> UART_FIFO_WDATA_W<0> {
        UART_FIFO_WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_fifo_wdata.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo_wdata](index.html) module"]
pub struct UART_FIFO_WDATA_SPEC;
impl crate::RegisterSpec for UART_FIFO_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uart_fifo_wdata::W](W) writer structure"]
impl crate::Writable for UART_FIFO_WDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_fifo_wdata to value 0"]
impl crate::Resettable for UART_FIFO_WDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `uart_int_mask` reader"]
pub struct R(crate::R<UART_INT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_INT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_INT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_int_mask` writer"]
pub struct W(crate::W<UART_INT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_INT_MASK_SPEC>;
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
impl From<crate::W<UART_INT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_INT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_utx_end_mask` reader - "]
pub type CR_UTX_END_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_end_mask` writer - "]
pub type CR_UTX_END_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_MASK_SPEC, bool, O>;
#[doc = "Field `cr_urx_end_mask` reader - "]
pub type CR_URX_END_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_end_mask` writer - "]
pub type CR_URX_END_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_MASK_SPEC, bool, O>;
#[doc = "Field `cr_utx_fifo_mask` reader - "]
pub type CR_UTX_FIFO_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_fifo_mask` writer - "]
pub type CR_UTX_FIFO_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_MASK_SPEC, bool, O>;
#[doc = "Field `cr_urx_fifo_mask` reader - "]
pub type CR_URX_FIFO_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_fifo_mask` writer - "]
pub type CR_URX_FIFO_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_MASK_SPEC, bool, O>;
#[doc = "Field `cr_urx_rto_mask` reader - "]
pub type CR_URX_RTO_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_rto_mask` writer - "]
pub type CR_URX_RTO_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_MASK_SPEC, bool, O>;
#[doc = "Field `cr_urx_pce_mask` reader - "]
pub type CR_URX_PCE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_pce_mask` writer - "]
pub type CR_URX_PCE_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_MASK_SPEC, bool, O>;
#[doc = "Field `cr_utx_fer_mask` reader - "]
pub type CR_UTX_FER_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_fer_mask` writer - "]
pub type CR_UTX_FER_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_MASK_SPEC, bool, O>;
#[doc = "Field `cr_urx_fer_mask` reader - "]
pub type CR_URX_FER_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_fer_mask` writer - "]
pub type CR_URX_FER_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_mask(&self) -> CR_UTX_END_MASK_R {
        CR_UTX_END_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_mask(&self) -> CR_URX_END_MASK_R {
        CR_URX_END_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_fifo_mask(&self) -> CR_UTX_FIFO_MASK_R {
        CR_UTX_FIFO_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_fifo_mask(&self) -> CR_URX_FIFO_MASK_R {
        CR_URX_FIFO_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_mask(&self) -> CR_URX_RTO_MASK_R {
        CR_URX_RTO_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_mask(&self) -> CR_URX_PCE_MASK_R {
        CR_URX_PCE_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_mask(&self) -> CR_UTX_FER_MASK_R {
        CR_UTX_FER_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_mask(&self) -> CR_URX_FER_MASK_R {
        CR_URX_FER_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_end_mask(&mut self) -> CR_UTX_END_MASK_W<0> {
        CR_UTX_END_MASK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_end_mask(&mut self) -> CR_URX_END_MASK_W<1> {
        CR_URX_END_MASK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_fifo_mask(&mut self) -> CR_UTX_FIFO_MASK_W<2> {
        CR_UTX_FIFO_MASK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_fifo_mask(&mut self) -> CR_URX_FIFO_MASK_W<3> {
        CR_URX_FIFO_MASK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_rto_mask(&mut self) -> CR_URX_RTO_MASK_W<4> {
        CR_URX_RTO_MASK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_pce_mask(&mut self) -> CR_URX_PCE_MASK_W<5> {
        CR_URX_PCE_MASK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_fer_mask(&mut self) -> CR_UTX_FER_MASK_W<6> {
        CR_UTX_FER_MASK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_fer_mask(&mut self) -> CR_URX_FER_MASK_W<7> {
        CR_URX_FER_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_mask](index.html) module"]
pub struct UART_INT_MASK_SPEC;
impl crate::RegisterSpec for UART_INT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_int_mask::R](R) reader structure"]
impl crate::Readable for UART_INT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_int_mask::W](W) writer structure"]
impl crate::Writable for UART_INT_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_int_mask to value 0xff"]
impl crate::Resettable for UART_INT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}

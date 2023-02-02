#[doc = "Register `uart_int_clear` reader"]
pub struct R(crate::R<UART_INT_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INT_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_INT_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_INT_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_int_clear` writer"]
pub struct W(crate::W<UART_INT_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_INT_CLEAR_SPEC>;
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
impl From<crate::W<UART_INT_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_INT_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_utx_end_clr` reader - "]
pub type CR_UTX_END_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_end_clr` writer - "]
pub type CR_UTX_END_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_CLEAR_SPEC, bool, O>;
#[doc = "Field `cr_urx_end_clr` reader - "]
pub type CR_URX_END_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_end_clr` writer - "]
pub type CR_URX_END_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_CLEAR_SPEC, bool, O>;
#[doc = "Field `rsvd_2` reader - "]
pub type RSVD_2_R = crate::BitReader<bool>;
#[doc = "Field `rsvd_2` writer - "]
pub type RSVD_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_CLEAR_SPEC, bool, O>;
#[doc = "Field `rsvd_3` reader - "]
pub type RSVD_3_R = crate::BitReader<bool>;
#[doc = "Field `rsvd_3` writer - "]
pub type RSVD_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_CLEAR_SPEC, bool, O>;
#[doc = "Field `cr_urx_rto_clr` reader - "]
pub type CR_URX_RTO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_rto_clr` writer - "]
pub type CR_URX_RTO_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_CLEAR_SPEC, bool, O>;
#[doc = "Field `cr_urx_pce_clr` reader - "]
pub type CR_URX_PCE_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_pce_clr` writer - "]
pub type CR_URX_PCE_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_CLEAR_SPEC, bool, O>;
#[doc = "Field `rsvd_6` reader - "]
pub type RSVD_6_R = crate::BitReader<bool>;
#[doc = "Field `rsvd_6` writer - "]
pub type RSVD_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_CLEAR_SPEC, bool, O>;
#[doc = "Field `rsvd_7` reader - "]
pub type RSVD_7_R = crate::BitReader<bool>;
#[doc = "Field `rsvd_7` writer - "]
pub type RSVD_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_CLEAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_clr(&self) -> CR_UTX_END_CLR_R {
        CR_UTX_END_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_clr(&self) -> CR_URX_END_CLR_R {
        CR_URX_END_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> RSVD_2_R {
        RSVD_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> RSVD_3_R {
        RSVD_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_clr(&self) -> CR_URX_RTO_CLR_R {
        CR_URX_RTO_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_clr(&self) -> CR_URX_PCE_CLR_R {
        CR_URX_PCE_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> RSVD_7_R {
        RSVD_7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_end_clr(&mut self) -> CR_UTX_END_CLR_W<0> {
        CR_UTX_END_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_end_clr(&mut self) -> CR_URX_END_CLR_W<1> {
        CR_URX_END_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_2(&mut self) -> RSVD_2_W<2> {
        RSVD_2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_3(&mut self) -> RSVD_3_W<3> {
        RSVD_3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_rto_clr(&mut self) -> CR_URX_RTO_CLR_W<4> {
        CR_URX_RTO_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_pce_clr(&mut self) -> CR_URX_PCE_CLR_W<5> {
        CR_URX_PCE_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_6(&mut self) -> RSVD_6_W<6> {
        RSVD_6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_7(&mut self) -> RSVD_7_W<7> {
        RSVD_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART interrupt clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_clear](index.html) module"]
pub struct UART_INT_CLEAR_SPEC;
impl crate::RegisterSpec for UART_INT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_int_clear::R](R) reader structure"]
impl crate::Readable for UART_INT_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_int_clear::W](W) writer structure"]
impl crate::Writable for UART_INT_CLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_int_clear to value 0"]
impl crate::Resettable for UART_INT_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

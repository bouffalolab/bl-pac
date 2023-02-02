#[doc = "Register `UART_SIG_SEL_0` reader"]
pub struct R(crate::R<UART_SIG_SEL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SIG_SEL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SIG_SEL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SIG_SEL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_SIG_SEL_0` writer"]
pub struct W(crate::W<UART_SIG_SEL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SIG_SEL_0_SPEC>;
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
impl From<crate::W<UART_SIG_SEL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SIG_SEL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_sig_0_sel` reader - "]
pub type UART_SIG_0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_0_sel` writer - "]
pub type UART_SIG_0_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_SIG_SEL_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `uart_sig_1_sel` reader - "]
pub type UART_SIG_1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_1_sel` writer - "]
pub type UART_SIG_1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_SIG_SEL_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `uart_sig_2_sel` reader - "]
pub type UART_SIG_2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_2_sel` writer - "]
pub type UART_SIG_2_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_SIG_SEL_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `uart_sig_3_sel` reader - "]
pub type UART_SIG_3_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_3_sel` writer - "]
pub type UART_SIG_3_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_SIG_SEL_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `uart_sig_4_sel` reader - "]
pub type UART_SIG_4_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_4_sel` writer - "]
pub type UART_SIG_4_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_SIG_SEL_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `uart_sig_5_sel` reader - "]
pub type UART_SIG_5_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_5_sel` writer - "]
pub type UART_SIG_5_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_SIG_SEL_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `uart_sig_6_sel` reader - "]
pub type UART_SIG_6_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_6_sel` writer - "]
pub type UART_SIG_6_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_SIG_SEL_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `uart_sig_7_sel` reader - "]
pub type UART_SIG_7_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_7_sel` writer - "]
pub type UART_SIG_7_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_SIG_SEL_0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_sig_0_sel(&self) -> UART_SIG_0_SEL_R {
        UART_SIG_0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_sig_1_sel(&self) -> UART_SIG_1_SEL_R {
        UART_SIG_1_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uart_sig_2_sel(&self) -> UART_SIG_2_SEL_R {
        UART_SIG_2_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn uart_sig_3_sel(&self) -> UART_SIG_3_SEL_R {
        UART_SIG_3_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn uart_sig_4_sel(&self) -> UART_SIG_4_SEL_R {
        UART_SIG_4_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn uart_sig_5_sel(&self) -> UART_SIG_5_SEL_R {
        UART_SIG_5_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn uart_sig_6_sel(&self) -> UART_SIG_6_SEL_R {
        UART_SIG_6_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn uart_sig_7_sel(&self) -> UART_SIG_7_SEL_R {
        UART_SIG_7_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_0_sel(&mut self) -> UART_SIG_0_SEL_W<0> {
        UART_SIG_0_SEL_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_1_sel(&mut self) -> UART_SIG_1_SEL_W<4> {
        UART_SIG_1_SEL_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_2_sel(&mut self) -> UART_SIG_2_SEL_W<8> {
        UART_SIG_2_SEL_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_3_sel(&mut self) -> UART_SIG_3_SEL_W<12> {
        UART_SIG_3_SEL_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_4_sel(&mut self) -> UART_SIG_4_SEL_W<16> {
        UART_SIG_4_SEL_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_5_sel(&mut self) -> UART_SIG_5_SEL_W<20> {
        UART_SIG_5_SEL_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_6_sel(&mut self) -> UART_SIG_6_SEL_W<24> {
        UART_SIG_6_SEL_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_7_sel(&mut self) -> UART_SIG_7_SEL_W<28> {
        UART_SIG_7_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_SIG_SEL_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sig_sel_0](index.html) module"]
pub struct UART_SIG_SEL_0_SPEC;
impl crate::RegisterSpec for UART_SIG_SEL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_sig_sel_0::R](R) reader structure"]
impl crate::Readable for UART_SIG_SEL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_sig_sel_0::W](W) writer structure"]
impl crate::Writable for UART_SIG_SEL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_SIG_SEL_0 to value 0x7654_3210"]
impl crate::Resettable for UART_SIG_SEL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x7654_3210;
}

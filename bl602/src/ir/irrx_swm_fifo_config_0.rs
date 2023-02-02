#[doc = "Register `irrx_swm_fifo_config_0` reader"]
pub struct R(crate::R<IRRX_SWM_FIFO_CONFIG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_SWM_FIFO_CONFIG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRRX_SWM_FIFO_CONFIG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRRX_SWM_FIFO_CONFIG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irrx_swm_fifo_config_0` writer"]
pub struct W(crate::W<IRRX_SWM_FIFO_CONFIG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRRX_SWM_FIFO_CONFIG_0_SPEC>;
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
impl From<crate::W<IRRX_SWM_FIFO_CONFIG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRRX_SWM_FIFO_CONFIG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_fifo_clr` reader - "]
pub type RX_FIFO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_clr` writer - "]
pub type RX_FIFO_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IRRX_SWM_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `rx_fifo_overflow` reader - "]
pub type RX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_underflow` reader - "]
pub type RX_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_cnt` reader - "]
pub type RX_FIFO_CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_clr(&self) -> RX_FIFO_CLR_R {
        RX_FIFO_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RX_FIFO_UNDERFLOW_R {
        RX_FIFO_UNDERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:10"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_clr(&mut self) -> RX_FIFO_CLR_W<0> {
        RX_FIFO_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irrx_swm_fifo_config_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_swm_fifo_config_0](index.html) module"]
pub struct IRRX_SWM_FIFO_CONFIG_0_SPEC;
impl crate::RegisterSpec for IRRX_SWM_FIFO_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_swm_fifo_config_0::R](R) reader structure"]
impl crate::Readable for IRRX_SWM_FIFO_CONFIG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irrx_swm_fifo_config_0::W](W) writer structure"]
impl crate::Writable for IRRX_SWM_FIFO_CONFIG_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irrx_swm_fifo_config_0 to value 0"]
impl crate::Resettable for IRRX_SWM_FIFO_CONFIG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

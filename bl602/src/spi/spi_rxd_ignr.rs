#[doc = "Register `spi_rxd_ignr` reader"]
pub struct R(crate::R<SPI_RXD_IGNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_RXD_IGNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_RXD_IGNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_RXD_IGNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_rxd_ignr` writer"]
pub struct W(crate::W<SPI_RXD_IGNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_RXD_IGNR_SPEC>;
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
impl From<crate::W<SPI_RXD_IGNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_RXD_IGNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_spi_rxd_ignr_p` reader - "]
pub type CR_SPI_RXD_IGNR_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_spi_rxd_ignr_p` writer - "]
pub type CR_SPI_RXD_IGNR_P_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_RXD_IGNR_SPEC, u8, u8, 5, O>;
#[doc = "Field `cr_spi_rxd_ignr_s` reader - "]
pub type CR_SPI_RXD_IGNR_S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_spi_rxd_ignr_s` writer - "]
pub type CR_SPI_RXD_IGNR_S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_RXD_IGNR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_p(&self) -> CR_SPI_RXD_IGNR_P_R {
        CR_SPI_RXD_IGNR_P_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_s(&self) -> CR_SPI_RXD_IGNR_S_R {
        CR_SPI_RXD_IGNR_S_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_rxd_ignr_p(&mut self) -> CR_SPI_RXD_IGNR_P_W<0> {
        CR_SPI_RXD_IGNR_P_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_rxd_ignr_s(&mut self) -> CR_SPI_RXD_IGNR_S_W<16> {
        CR_SPI_RXD_IGNR_S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_rxd_ignr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rxd_ignr](index.html) module"]
pub struct SPI_RXD_IGNR_SPEC;
impl crate::RegisterSpec for SPI_RXD_IGNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_rxd_ignr::R](R) reader structure"]
impl crate::Readable for SPI_RXD_IGNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_rxd_ignr::W](W) writer structure"]
impl crate::Writable for SPI_RXD_IGNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_rxd_ignr to value 0"]
impl crate::Resettable for SPI_RXD_IGNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

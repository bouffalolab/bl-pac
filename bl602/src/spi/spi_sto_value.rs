#[doc = "Register `spi_sto_value` reader"]
pub struct R(crate::R<SPI_STO_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_STO_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_STO_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_STO_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_sto_value` writer"]
pub struct W(crate::W<SPI_STO_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_STO_VALUE_SPEC>;
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
impl From<crate::W<SPI_STO_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_STO_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_spi_sto_value` reader - "]
pub type CR_SPI_STO_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_spi_sto_value` writer - "]
pub type CR_SPI_STO_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_STO_VALUE_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_spi_sto_value(&self) -> CR_SPI_STO_VALUE_R {
        CR_SPI_STO_VALUE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_sto_value(&mut self) -> CR_SPI_STO_VALUE_W<0> {
        CR_SPI_STO_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_sto_value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_sto_value](index.html) module"]
pub struct SPI_STO_VALUE_SPEC;
impl crate::RegisterSpec for SPI_STO_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_sto_value::R](R) reader structure"]
impl crate::Readable for SPI_STO_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_sto_value::W](W) writer structure"]
impl crate::Writable for SPI_STO_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_sto_value to value 0x0fff"]
impl crate::Resettable for SPI_STO_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}

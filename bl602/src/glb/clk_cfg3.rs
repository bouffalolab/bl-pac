#[doc = "Register `clk_cfg3` reader"]
pub struct R(crate::R<CLK_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg3` writer"]
pub struct W(crate::W<CLK_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG3_SPEC>;
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
impl From<crate::W<CLK_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_clk_div` reader - "]
pub type SPI_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_clk_div` writer - "]
pub type SPI_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG3_SPEC, u8, u8, 5, O>;
#[doc = "Field `spi_clk_en` reader - "]
pub type SPI_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `spi_clk_en` writer - "]
pub type SPI_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG3_SPEC, bool, O>;
#[doc = "Field `i2c_clk_div` reader - "]
pub type I2C_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `i2c_clk_div` writer - "]
pub type I2C_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG3_SPEC, u8, u8, 8, O>;
#[doc = "Field `i2c_clk_en` reader - "]
pub type I2C_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `i2c_clk_en` writer - "]
pub type I2C_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&self) -> SPI_CLK_DIV_R {
        SPI_CLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SPI_CLK_EN_R {
        SPI_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&self) -> I2C_CLK_DIV_R {
        I2C_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn spi_clk_div(&mut self) -> SPI_CLK_DIV_W<0> {
        SPI_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn spi_clk_en(&mut self) -> SPI_CLK_EN_W<8> {
        SPI_CLK_EN_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_div(&mut self) -> I2C_CLK_DIV_W<16> {
        I2C_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W<24> {
        I2C_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration for I2C and SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg3](index.html) module"]
pub struct CLK_CFG3_SPEC;
impl crate::RegisterSpec for CLK_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg3::R](R) reader structure"]
impl crate::Readable for CLK_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg3::W](W) writer structure"]
impl crate::Writable for CLK_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk_cfg3 to value 0x01ff_0103"]
impl crate::Resettable for CLK_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff_0103;
}

#[doc = "Register `i2c_config` reader"]
pub struct R(crate::R<I2C_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_config` writer"]
pub struct W(crate::W<I2C_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CONFIG_SPEC>;
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
impl From<crate::W<I2C_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clock_divide` reader - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clock_divide` writer - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `clock_enable` reader - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `clock_enable` writer - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CONFIG_SPEC, bool, O>;
#[doc = "Field `clock_source` reader - Peripheral clock source register"]
pub type CLOCK_SOURCE_R = crate::BitReader<bool>;
#[doc = "Field `clock_source` writer - Peripheral clock source register"]
pub type CLOCK_SOURCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:23 - Peripheral clock divide factor"]
    #[inline(always)]
    pub fn clock_divide(&self) -> CLOCK_DIVIDE_R {
        CLOCK_DIVIDE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Peripheral level clock gate enable"]
    #[inline(always)]
    pub fn clock_enable(&self) -> CLOCK_ENABLE_R {
        CLOCK_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral clock source register"]
    #[inline(always)]
    pub fn clock_source(&self) -> CLOCK_SOURCE_R {
        CLOCK_SOURCE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - Peripheral clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clock_divide(&mut self) -> CLOCK_DIVIDE_W<16> {
        CLOCK_DIVIDE_W::new(self)
    }
    #[doc = "Bit 24 - Peripheral level clock gate enable"]
    #[inline(always)]
    #[must_use]
    pub fn clock_enable(&mut self) -> CLOCK_ENABLE_W<24> {
        CLOCK_ENABLE_W::new(self)
    }
    #[doc = "Bit 25 - Peripheral clock source register"]
    #[inline(always)]
    #[must_use]
    pub fn clock_source(&mut self) -> CLOCK_SOURCE_W<25> {
        CLOCK_SOURCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inter-Integrated Circuit bus configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_config](index.html) module"]
pub struct I2C_CONFIG_SPEC;
impl crate::RegisterSpec for I2C_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_config::R](R) reader structure"]
impl crate::Readable for I2C_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_config::W](W) writer structure"]
impl crate::Writable for I2C_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2c_config to value 0"]
impl crate::Resettable for I2C_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

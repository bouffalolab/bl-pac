#[doc = "Register `i2c_bus_busy` reader"]
pub struct R(crate::R<I2C_BUS_BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_BUS_BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_BUS_BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_BUS_BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_bus_busy` writer"]
pub struct W(crate::W<I2C_BUS_BUSY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_BUS_BUSY_SPEC>;
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
impl From<crate::W<I2C_BUS_BUSY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_BUS_BUSY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_i2c_bus_busy` reader - "]
pub type STS_I2C_BUS_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2c_bus_busy_clr` reader - "]
pub type CR_I2C_BUS_BUSY_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2c_bus_busy_clr` writer - "]
pub type CR_I2C_BUS_BUSY_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2C_BUS_BUSY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_i2c_bus_busy(&self) -> STS_I2C_BUS_BUSY_R {
        STS_I2C_BUS_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_bus_busy_clr(&self) -> CR_I2C_BUS_BUSY_CLR_R {
        CR_I2C_BUS_BUSY_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_bus_busy_clr(&mut self) -> CR_I2C_BUS_BUSY_CLR_W<1> {
        CR_I2C_BUS_BUSY_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_bus_busy.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_bus_busy](index.html) module"]
pub struct I2C_BUS_BUSY_SPEC;
impl crate::RegisterSpec for I2C_BUS_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_bus_busy::R](R) reader structure"]
impl crate::Readable for I2C_BUS_BUSY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_bus_busy::W](W) writer structure"]
impl crate::Writable for I2C_BUS_BUSY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2c_bus_busy to value 0"]
impl crate::Resettable for I2C_BUS_BUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `i2c_fifo_wdata` writer"]
pub struct W(crate::W<I2C_FIFO_WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_FIFO_WDATA_SPEC>;
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
impl From<crate::W<I2C_FIFO_WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_FIFO_WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `i2c_fifo_wdata` writer - "]
pub type I2C_FIFO_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_FIFO_WDATA_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_fifo_wdata(&mut self) -> I2C_FIFO_WDATA_W<0> {
        I2C_FIFO_WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_fifo_wdata.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_wdata](index.html) module"]
pub struct I2C_FIFO_WDATA_SPEC;
impl crate::RegisterSpec for I2C_FIFO_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [i2c_fifo_wdata::W](W) writer structure"]
impl crate::Writable for I2C_FIFO_WDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2c_fifo_wdata to value 0"]
impl crate::Resettable for I2C_FIFO_WDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

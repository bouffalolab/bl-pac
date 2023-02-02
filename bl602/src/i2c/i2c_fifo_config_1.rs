#[doc = "Register `i2c_fifo_config_1` reader"]
pub struct R(crate::R<I2C_FIFO_CONFIG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_FIFO_CONFIG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_FIFO_CONFIG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_FIFO_CONFIG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_fifo_config_1` writer"]
pub struct W(crate::W<I2C_FIFO_CONFIG_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_FIFO_CONFIG_1_SPEC>;
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
impl From<crate::W<I2C_FIFO_CONFIG_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_FIFO_CONFIG_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_fifo_cnt` reader - "]
pub type TX_FIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_fifo_cnt` reader - "]
pub type RX_FIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_fifo_th` reader - "]
pub type TX_FIFO_TH_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_th` writer - "]
pub type TX_FIFO_TH_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_FIFO_CONFIG_1_SPEC, bool, O>;
#[doc = "Field `rx_fifo_th` reader - "]
pub type RX_FIFO_TH_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_th` writer - "]
pub type RX_FIFO_TH_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_FIFO_CONFIG_1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TX_FIFO_TH_R {
        TX_FIFO_TH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rx_fifo_th(&self) -> RX_FIFO_TH_R {
        RX_FIFO_TH_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_th(&mut self) -> TX_FIFO_TH_W<16> {
        TX_FIFO_TH_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_th(&mut self) -> RX_FIFO_TH_W<24> {
        RX_FIFO_TH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_fifo_config_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_config_1](index.html) module"]
pub struct I2C_FIFO_CONFIG_1_SPEC;
impl crate::RegisterSpec for I2C_FIFO_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_fifo_config_1::R](R) reader structure"]
impl crate::Readable for I2C_FIFO_CONFIG_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_fifo_config_1::W](W) writer structure"]
impl crate::Writable for I2C_FIFO_CONFIG_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2c_fifo_config_1 to value 0x02"]
impl crate::Resettable for I2C_FIFO_CONFIG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}

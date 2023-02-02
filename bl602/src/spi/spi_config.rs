#[doc = "Register `spi_config` reader"]
pub struct R(crate::R<SPI_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_config` writer"]
pub struct W(crate::W<SPI_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CONFIG_SPEC>;
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
impl From<crate::W<SPI_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_spi_m_en` reader - "]
pub type CR_SPI_M_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_m_en` writer - "]
pub type CR_SPI_M_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_spi_s_en` reader - "]
pub type CR_SPI_S_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_s_en` writer - "]
pub type CR_SPI_S_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_spi_frame_size` reader - "]
pub type CR_SPI_FRAME_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_spi_frame_size` writer - "]
pub type CR_SPI_FRAME_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_spi_sclk_pol` reader - "]
pub type CR_SPI_SCLK_POL_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_sclk_pol` writer - "]
pub type CR_SPI_SCLK_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_spi_sclk_ph` reader - "]
pub type CR_SPI_SCLK_PH_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_sclk_ph` writer - "]
pub type CR_SPI_SCLK_PH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_spi_bit_inv` reader - "]
pub type CR_SPI_BIT_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_bit_inv` writer - "]
pub type CR_SPI_BIT_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_spi_byte_inv` reader - "]
pub type CR_SPI_BYTE_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_byte_inv` writer - "]
pub type CR_SPI_BYTE_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_spi_rxd_ignr_en` reader - "]
pub type CR_SPI_RXD_IGNR_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_rxd_ignr_en` writer - "]
pub type CR_SPI_RXD_IGNR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_spi_m_cont_en` reader - "]
pub type CR_SPI_M_CONT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_m_cont_en` writer - "]
pub type CR_SPI_M_CONT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_spi_deg_en` reader - "]
pub type CR_SPI_DEG_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_deg_en` writer - "]
pub type CR_SPI_DEG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_spi_deg_cnt` reader - "]
pub type CR_SPI_DEG_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_spi_deg_cnt` writer - "]
pub type CR_SPI_DEG_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_CONFIG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_spi_m_en(&self) -> CR_SPI_M_EN_R {
        CR_SPI_M_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_spi_s_en(&self) -> CR_SPI_S_EN_R {
        CR_SPI_S_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_spi_frame_size(&self) -> CR_SPI_FRAME_SIZE_R {
        CR_SPI_FRAME_SIZE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_spi_sclk_pol(&self) -> CR_SPI_SCLK_POL_R {
        CR_SPI_SCLK_POL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_spi_sclk_ph(&self) -> CR_SPI_SCLK_PH_R {
        CR_SPI_SCLK_PH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_spi_bit_inv(&self) -> CR_SPI_BIT_INV_R {
        CR_SPI_BIT_INV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_spi_byte_inv(&self) -> CR_SPI_BYTE_INV_R {
        CR_SPI_BYTE_INV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_en(&self) -> CR_SPI_RXD_IGNR_EN_R {
        CR_SPI_RXD_IGNR_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_spi_m_cont_en(&self) -> CR_SPI_M_CONT_EN_R {
        CR_SPI_M_CONT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_spi_deg_en(&self) -> CR_SPI_DEG_EN_R {
        CR_SPI_DEG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_spi_deg_cnt(&self) -> CR_SPI_DEG_CNT_R {
        CR_SPI_DEG_CNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_m_en(&mut self) -> CR_SPI_M_EN_W<0> {
        CR_SPI_M_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_s_en(&mut self) -> CR_SPI_S_EN_W<1> {
        CR_SPI_S_EN_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_frame_size(&mut self) -> CR_SPI_FRAME_SIZE_W<2> {
        CR_SPI_FRAME_SIZE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_sclk_pol(&mut self) -> CR_SPI_SCLK_POL_W<4> {
        CR_SPI_SCLK_POL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_sclk_ph(&mut self) -> CR_SPI_SCLK_PH_W<5> {
        CR_SPI_SCLK_PH_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_bit_inv(&mut self) -> CR_SPI_BIT_INV_W<6> {
        CR_SPI_BIT_INV_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_byte_inv(&mut self) -> CR_SPI_BYTE_INV_W<7> {
        CR_SPI_BYTE_INV_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_rxd_ignr_en(&mut self) -> CR_SPI_RXD_IGNR_EN_W<8> {
        CR_SPI_RXD_IGNR_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_m_cont_en(&mut self) -> CR_SPI_M_CONT_EN_W<9> {
        CR_SPI_M_CONT_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_deg_en(&mut self) -> CR_SPI_DEG_EN_W<11> {
        CR_SPI_DEG_EN_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_deg_cnt(&mut self) -> CR_SPI_DEG_CNT_W<12> {
        CR_SPI_DEG_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_config](index.html) module"]
pub struct SPI_CONFIG_SPEC;
impl crate::RegisterSpec for SPI_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_config::R](R) reader structure"]
impl crate::Readable for SPI_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_config::W](W) writer structure"]
impl crate::Writable for SPI_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_config to value 0"]
impl crate::Resettable for SPI_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

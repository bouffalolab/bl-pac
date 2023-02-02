#[doc = "Register `glb_parm` reader"]
pub struct R(crate::R<GLB_PARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLB_PARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLB_PARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLB_PARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `glb_parm` writer"]
pub struct W(crate::W<GLB_PARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLB_PARM_SPEC>;
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
impl From<crate::W<GLB_PARM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLB_PARM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_bd_en` reader - "]
pub type REG_BD_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_bd_en` writer - "]
pub type REG_BD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `reg_ext_rst_smt` reader - "]
pub type REG_EXT_RST_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_ext_rst_smt` writer - "]
pub type REG_EXT_RST_SMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `jtag_swap_set` reader - "]
pub type JTAG_SWAP_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `jtag_swap_set` writer - "]
pub type JTAG_SWAP_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GLB_PARM_SPEC, u8, u8, 6, O>;
#[doc = "Field `swap_sflash_io_3_io_0` reader - "]
pub type SWAP_SFLASH_IO_3_IO_0_R = crate::BitReader<bool>;
#[doc = "Field `swap_sflash_io_3_io_0` writer - "]
pub type SWAP_SFLASH_IO_3_IO_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `sel_embedded_sflash` reader - "]
pub type SEL_EMBEDDED_SFLASH_R = crate::BitReader<bool>;
#[doc = "Field `sel_embedded_sflash` writer - "]
pub type SEL_EMBEDDED_SFLASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `reg_spi_0_master_mode` reader - "]
pub type REG_SPI_0_MASTER_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_spi_0_master_mode` writer - "]
pub type REG_SPI_0_MASTER_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `reg_spi_0_swap` reader - "]
pub type REG_SPI_0_SWAP_R = crate::BitReader<bool>;
#[doc = "Field `reg_spi_0_swap` writer - "]
pub type REG_SPI_0_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `reg_cci_use_jtag_pin` reader - "]
pub type REG_CCI_USE_JTAG_PIN_R = crate::BitReader<bool>;
#[doc = "Field `reg_cci_use_jtag_pin` writer - "]
pub type REG_CCI_USE_JTAG_PIN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `reg_cci_use_sdio_pin` reader - "]
pub type REG_CCI_USE_SDIO_PIN_R = crate::BitReader<bool>;
#[doc = "Field `reg_cci_use_sdio_pin` writer - "]
pub type REG_CCI_USE_SDIO_PIN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `p1_adc_test_with_cci` reader - "]
pub type P1_ADC_TEST_WITH_CCI_R = crate::BitReader<bool>;
#[doc = "Field `p1_adc_test_with_cci` writer - "]
pub type P1_ADC_TEST_WITH_CCI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `p2_dac_test_with_cci` reader - "]
pub type P2_DAC_TEST_WITH_CCI_R = crate::BitReader<bool>;
#[doc = "Field `p2_dac_test_with_cci` writer - "]
pub type P2_DAC_TEST_WITH_CCI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `p3_cci_use_io_2_5` reader - "]
pub type P3_CCI_USE_IO_2_5_R = crate::BitReader<bool>;
#[doc = "Field `p3_cci_use_io_2_5` writer - "]
pub type P3_CCI_USE_IO_2_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `p4_adc_test_with_jtag` reader - "]
pub type P4_ADC_TEST_WITH_JTAG_R = crate::BitReader<bool>;
#[doc = "Field `p4_adc_test_with_jtag` writer - "]
pub type P4_ADC_TEST_WITH_JTAG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `p5_dac_test_with_jtag` reader - "]
pub type P5_DAC_TEST_WITH_JTAG_R = crate::BitReader<bool>;
#[doc = "Field `p5_dac_test_with_jtag` writer - "]
pub type P5_DAC_TEST_WITH_JTAG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `p6_sdio_use_io_0_5` reader - "]
pub type P6_SDIO_USE_IO_0_5_R = crate::BitReader<bool>;
#[doc = "Field `p6_sdio_use_io_0_5` writer - "]
pub type P6_SDIO_USE_IO_0_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `p7_jtag_use_io_2_5` reader - "]
pub type P7_JTAG_USE_IO_2_5_R = crate::BitReader<bool>;
#[doc = "Field `p7_jtag_use_io_2_5` writer - "]
pub type P7_JTAG_USE_IO_2_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_PARM_SPEC, bool, O>;
#[doc = "Field `uart_swap_set` reader - "]
pub type UART_SWAP_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_swap_set` writer - "]
pub type UART_SWAP_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GLB_PARM_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bd_en(&self) -> REG_BD_EN_R {
        REG_BD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ext_rst_smt(&self) -> REG_EXT_RST_SMT_R {
        REG_EXT_RST_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn jtag_swap_set(&self) -> JTAG_SWAP_SET_R {
        JTAG_SWAP_SET_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap_sflash_io_3_io_0(&self) -> SWAP_SFLASH_IO_3_IO_0_R {
        SWAP_SFLASH_IO_3_IO_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sel_embedded_sflash(&self) -> SEL_EMBEDDED_SFLASH_R {
        SEL_EMBEDDED_SFLASH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&self) -> REG_SPI_0_MASTER_MODE_R {
        REG_SPI_0_MASTER_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&self) -> REG_SPI_0_SWAP_R {
        REG_SPI_0_SWAP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_cci_use_jtag_pin(&self) -> REG_CCI_USE_JTAG_PIN_R {
        REG_CCI_USE_JTAG_PIN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_cci_use_sdio_pin(&self) -> REG_CCI_USE_SDIO_PIN_R {
        REG_CCI_USE_SDIO_PIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&self) -> P1_ADC_TEST_WITH_CCI_R {
        P1_ADC_TEST_WITH_CCI_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&self) -> P2_DAC_TEST_WITH_CCI_R {
        P2_DAC_TEST_WITH_CCI_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_2_5(&self) -> P3_CCI_USE_IO_2_5_R {
        P3_CCI_USE_IO_2_5_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&self) -> P4_ADC_TEST_WITH_JTAG_R {
        P4_ADC_TEST_WITH_JTAG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&self) -> P5_DAC_TEST_WITH_JTAG_R {
        P5_DAC_TEST_WITH_JTAG_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn p6_sdio_use_io_0_5(&self) -> P6_SDIO_USE_IO_0_5_R {
        P6_SDIO_USE_IO_0_5_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p7_jtag_use_io_2_5(&self) -> P7_JTAG_USE_IO_2_5_R {
        P7_JTAG_USE_IO_2_5_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn uart_swap_set(&self) -> UART_SWAP_SET_R {
        UART_SWAP_SET_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bd_en(&mut self) -> REG_BD_EN_W<0> {
        REG_BD_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ext_rst_smt(&mut self) -> REG_EXT_RST_SMT_W<1> {
        REG_EXT_RST_SMT_W::new(self)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_swap_set(&mut self) -> JTAG_SWAP_SET_W<2> {
        JTAG_SWAP_SET_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn swap_sflash_io_3_io_0(&mut self) -> SWAP_SFLASH_IO_3_IO_0_W<8> {
        SWAP_SFLASH_IO_3_IO_0_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sel_embedded_sflash(&mut self) -> SEL_EMBEDDED_SFLASH_W<9> {
        SEL_EMBEDDED_SFLASH_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_spi_0_master_mode(&mut self) -> REG_SPI_0_MASTER_MODE_W<12> {
        REG_SPI_0_MASTER_MODE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_spi_0_swap(&mut self) -> REG_SPI_0_SWAP_W<13> {
        REG_SPI_0_SWAP_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cci_use_jtag_pin(&mut self) -> REG_CCI_USE_JTAG_PIN_W<15> {
        REG_CCI_USE_JTAG_PIN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cci_use_sdio_pin(&mut self) -> REG_CCI_USE_SDIO_PIN_W<16> {
        REG_CCI_USE_SDIO_PIN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn p1_adc_test_with_cci(&mut self) -> P1_ADC_TEST_WITH_CCI_W<17> {
        P1_ADC_TEST_WITH_CCI_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn p2_dac_test_with_cci(&mut self) -> P2_DAC_TEST_WITH_CCI_W<18> {
        P2_DAC_TEST_WITH_CCI_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn p3_cci_use_io_2_5(&mut self) -> P3_CCI_USE_IO_2_5_W<19> {
        P3_CCI_USE_IO_2_5_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn p4_adc_test_with_jtag(&mut self) -> P4_ADC_TEST_WITH_JTAG_W<20> {
        P4_ADC_TEST_WITH_JTAG_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn p5_dac_test_with_jtag(&mut self) -> P5_DAC_TEST_WITH_JTAG_W<21> {
        P5_DAC_TEST_WITH_JTAG_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn p6_sdio_use_io_0_5(&mut self) -> P6_SDIO_USE_IO_0_5_W<22> {
        P6_SDIO_USE_IO_0_5_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn p7_jtag_use_io_2_5(&mut self) -> P7_JTAG_USE_IO_2_5_W<23> {
        P7_JTAG_USE_IO_2_5_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn uart_swap_set(&mut self) -> UART_SWAP_SET_W<24> {
        UART_SWAP_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "glb_parm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glb_parm](index.html) module"]
pub struct GLB_PARM_SPEC;
impl crate::RegisterSpec for GLB_PARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [glb_parm::R](R) reader structure"]
impl crate::Readable for GLB_PARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glb_parm::W](W) writer structure"]
impl crate::Writable for GLB_PARM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets glb_parm to value 0x0001_8300"]
impl crate::Resettable for GLB_PARM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_8300;
}

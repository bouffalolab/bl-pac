#[doc = "Register `GPIO_CFGCTL2` reader"]
pub struct R(crate::R<GPIO_CFGCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL2` writer"]
pub struct W(crate::W<GPIO_CFGCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL2_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_4_ie` reader - Input enable for GPIO4."]
pub type REG_GPIO_4_IE_R = crate::BitReader<GPIO4INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO4INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO4INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_4_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4INPUT_ENABLED_A {
        match self.bits {
            false => GPIO4INPUT_ENABLED_A::DISABLED,
            true => GPIO4INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO4INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO4INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_4_ie` writer - Input enable for GPIO4."]
pub type REG_GPIO_4_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL2_SPEC, GPIO4INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_4_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO4INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO4INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_4_smt` reader - Schmitt trigger enabled for GPIO4."]
pub type REG_GPIO_4_SMT_R = crate::BitReader<GPIO4SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO4SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO4SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_4_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4SCHMITT_A {
        match self.bits {
            false => GPIO4SCHMITT_A::DISABLED,
            true => GPIO4SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO4SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO4SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_4_smt` writer - Schmitt trigger enabled for GPIO4."]
pub type REG_GPIO_4_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL2_SPEC, GPIO4SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_4_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO4SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO4SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_4_drv` reader - Driving control enabled for GPIO4."]
pub type REG_GPIO_4_DRV_R = crate::FieldReader<u8, GPIO4DRIVING_A>;
#[doc = "Driving control enabled for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO4DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO4DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO4DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_4_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO4DRIVING_A> {
        match self.bits {
            0 => Some(GPIO4DRIVING_A::DISABLED),
            1 => Some(GPIO4DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO4DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO4DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_4_drv` writer - Driving control enabled for GPIO4."]
pub type REG_GPIO_4_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL2_SPEC, u8, GPIO4DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_4_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO4DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO4DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_4_pu` reader - Pull Up Resistor for GPIO4."]
pub type REG_GPIO_4_PU_R = crate::BitReader<GPIO4PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO4PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO4PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_4_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO4PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO4PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO4PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO4PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_4_pu` writer - Pull Up Resistor for GPIO4."]
pub type REG_GPIO_4_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL2_SPEC, GPIO4PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_4_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO4PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO4PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_4_pd` reader - Pull Down Resistor for GPIO4."]
pub type REG_GPIO_4_PD_R = crate::BitReader<GPIO4PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO4PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO4PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_4_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO4PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO4PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO4PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO4PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_4_pd` writer - Pull Down Resistor for GPIO4."]
pub type REG_GPIO_4_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL2_SPEC, GPIO4PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_4_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO4PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO4PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_4_func_sel` reader - Function select for GPIO4."]
pub type REG_GPIO_4_FUNC_SEL_R = crate::FieldReader<u8, GPIO4FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO4FUNCTION_SELECT_A {
    #[doc = "1: `1`"]
    SDIO_DAT2 = 1,
    #[doc = "4: `100`"]
    SPI_MISO_SPI_MOSI = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG4 = 7,
    #[doc = "8: `1000`"]
    PWM_CH4 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_0 = 9,
    #[doc = "10: `1010`"]
    GPIP_CH1 = 10,
    #[doc = "11: `1011`"]
    SWGPIO_4 = 11,
    #[doc = "14: `1110`"]
    E21_TMS = 14,
}
impl From<GPIO4FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO4FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_4_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO4FUNCTION_SELECT_A> {
        match self.bits {
            1 => Some(GPIO4FUNCTION_SELECT_A::SDIO_DAT2),
            4 => Some(GPIO4FUNCTION_SELECT_A::SPI_MISO_SPI_MOSI),
            6 => Some(GPIO4FUNCTION_SELECT_A::I2C_SCL),
            7 => Some(GPIO4FUNCTION_SELECT_A::UART_SIG4),
            8 => Some(GPIO4FUNCTION_SELECT_A::PWM_CH4),
            9 => Some(GPIO4FUNCTION_SELECT_A::FEM_GPIO_0),
            10 => Some(GPIO4FUNCTION_SELECT_A::GPIP_CH1),
            11 => Some(GPIO4FUNCTION_SELECT_A::SWGPIO_4),
            14 => Some(GPIO4FUNCTION_SELECT_A::E21_TMS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO_DAT2`"]
    #[inline(always)]
    pub fn is_sdio_dat2(&self) -> bool {
        *self == GPIO4FUNCTION_SELECT_A::SDIO_DAT2
    }
    #[doc = "Checks if the value of the field is `SPI_MISO_SPI_MOSI`"]
    #[inline(always)]
    pub fn is_spi_miso_spi_mosi(&self) -> bool {
        *self == GPIO4FUNCTION_SELECT_A::SPI_MISO_SPI_MOSI
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == GPIO4FUNCTION_SELECT_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG4`"]
    #[inline(always)]
    pub fn is_uart_sig4(&self) -> bool {
        *self == GPIO4FUNCTION_SELECT_A::UART_SIG4
    }
    #[doc = "Checks if the value of the field is `PWM_CH4`"]
    #[inline(always)]
    pub fn is_pwm_ch4(&self) -> bool {
        *self == GPIO4FUNCTION_SELECT_A::PWM_CH4
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_0`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        *self == GPIO4FUNCTION_SELECT_A::FEM_GPIO_0
    }
    #[doc = "Checks if the value of the field is `GPIP_CH1`"]
    #[inline(always)]
    pub fn is_gpip_ch1(&self) -> bool {
        *self == GPIO4FUNCTION_SELECT_A::GPIP_CH1
    }
    #[doc = "Checks if the value of the field is `SWGPIO_4`"]
    #[inline(always)]
    pub fn is_swgpio_4(&self) -> bool {
        *self == GPIO4FUNCTION_SELECT_A::SWGPIO_4
    }
    #[doc = "Checks if the value of the field is `E21_TMS`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        *self == GPIO4FUNCTION_SELECT_A::E21_TMS
    }
}
#[doc = "Field `reg_gpio_4_func_sel` writer - Function select for GPIO4."]
pub type REG_GPIO_4_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL2_SPEC, u8, GPIO4FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_4_FUNC_SEL_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_dat2(self) -> &'a mut W {
        self.variant(GPIO4FUNCTION_SELECT_A::SDIO_DAT2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_miso_spi_mosi(self) -> &'a mut W {
        self.variant(GPIO4FUNCTION_SELECT_A::SPI_MISO_SPI_MOSI)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(GPIO4FUNCTION_SELECT_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig4(self) -> &'a mut W {
        self.variant(GPIO4FUNCTION_SELECT_A::UART_SIG4)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch4(self) -> &'a mut W {
        self.variant(GPIO4FUNCTION_SELECT_A::PWM_CH4)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut W {
        self.variant(GPIO4FUNCTION_SELECT_A::FEM_GPIO_0)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch1(self) -> &'a mut W {
        self.variant(GPIO4FUNCTION_SELECT_A::GPIP_CH1)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_4(self) -> &'a mut W {
        self.variant(GPIO4FUNCTION_SELECT_A::SWGPIO_4)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut W {
        self.variant(GPIO4FUNCTION_SELECT_A::E21_TMS)
    }
}
#[doc = "Field `real_gpio_4_func_sel` reader - "]
pub type REAL_GPIO_4_FUNC_SEL_R = crate::FieldReader<u8, GPIO4REAL_FUNCTION_SELECT_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO4REAL_FUNCTION_SELECT_A {
    #[doc = "0: Function select is reg_gpio_4_func_sel"]
    GLB_GPIO_REAL_MODE_REG = 0,
    #[doc = "1: `1`"]
    GLB_GPIO_REAL_MODE_SDIO = 1,
    #[doc = "12: `1100`"]
    GLB_GPIO_REAL_MODE_RF = 12,
    #[doc = "14: `1110`"]
    GLB_GPIO_REAL_MODE_JTAG = 14,
    #[doc = "15: `1111`"]
    GLB_GPIO_REAL_MODE_CCI = 15,
}
impl From<GPIO4REAL_FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO4REAL_FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REAL_GPIO_4_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO4REAL_FUNCTION_SELECT_A> {
        match self.bits {
            0 => Some(GPIO4REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG),
            1 => Some(GPIO4REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO),
            12 => Some(GPIO4REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF),
            14 => Some(GPIO4REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG),
            15 => Some(GPIO4REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_REG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == GPIO4REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_SDIO`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == GPIO4REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_RF`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == GPIO4REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_JTAG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == GPIO4REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_CCI`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == GPIO4REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI
    }
}
#[doc = "Field `reg_gpio_5_ie` reader - Input enable for GPIO5."]
pub type REG_GPIO_5_IE_R = crate::BitReader<GPIO5INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO5INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO5INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_5_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5INPUT_ENABLED_A {
        match self.bits {
            false => GPIO5INPUT_ENABLED_A::DISABLED,
            true => GPIO5INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO5INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO5INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_5_ie` writer - Input enable for GPIO5."]
pub type REG_GPIO_5_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL2_SPEC, GPIO5INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_5_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO5INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO5INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_5_smt` reader - Schmitt trigger enabled for GPIO5."]
pub type REG_GPIO_5_SMT_R = crate::BitReader<GPIO5SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO5SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO5SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_5_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5SCHMITT_A {
        match self.bits {
            false => GPIO5SCHMITT_A::DISABLED,
            true => GPIO5SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO5SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO5SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_5_smt` writer - Schmitt trigger enabled for GPIO5."]
pub type REG_GPIO_5_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL2_SPEC, GPIO5SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_5_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO5SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO5SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_5_drv` reader - Driving control enabled for GPIO5."]
pub type REG_GPIO_5_DRV_R = crate::FieldReader<u8, GPIO5DRIVING_A>;
#[doc = "Driving control enabled for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO5DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO5DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO5DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_5_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO5DRIVING_A> {
        match self.bits {
            0 => Some(GPIO5DRIVING_A::DISABLED),
            1 => Some(GPIO5DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO5DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO5DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_5_drv` writer - Driving control enabled for GPIO5."]
pub type REG_GPIO_5_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL2_SPEC, u8, GPIO5DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_5_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO5DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO5DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_5_pu` reader - Pull Up Resistor for GPIO5."]
pub type REG_GPIO_5_PU_R = crate::BitReader<GPIO5PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO5PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO5PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_5_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO5PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO5PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO5PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO5PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_5_pu` writer - Pull Up Resistor for GPIO5."]
pub type REG_GPIO_5_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL2_SPEC, GPIO5PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_5_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO5PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO5PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_5_pd` reader - Pull Down Resistor for GPIO5."]
pub type REG_GPIO_5_PD_R = crate::BitReader<GPIO5PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO5PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO5PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_5_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO5PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO5PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO5PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO5PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_5_pd` writer - Pull Down Resistor for GPIO5."]
pub type REG_GPIO_5_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL2_SPEC, GPIO5PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_5_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO5PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO5PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_5_func_sel` reader - Function select for GPIO5."]
pub type REG_GPIO_5_FUNC_SEL_R = crate::FieldReader<u8, GPIO5FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO5FUNCTION_SELECT_A {
    #[doc = "1: `1`"]
    SDIO_DAT3 = 1,
    #[doc = "4: `100`"]
    SPI_MOSI_SPI_MISO = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG5 = 7,
    #[doc = "8: `1000`"]
    PWM_CH0 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_1 = 9,
    #[doc = "10: `1010`"]
    GPIP_CH4 = 10,
    #[doc = "11: `1011`"]
    SWGPIO_5 = 11,
    #[doc = "14: `1110`"]
    E21_TDI = 14,
}
impl From<GPIO5FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO5FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_5_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO5FUNCTION_SELECT_A> {
        match self.bits {
            1 => Some(GPIO5FUNCTION_SELECT_A::SDIO_DAT3),
            4 => Some(GPIO5FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO),
            6 => Some(GPIO5FUNCTION_SELECT_A::I2C_SDA),
            7 => Some(GPIO5FUNCTION_SELECT_A::UART_SIG5),
            8 => Some(GPIO5FUNCTION_SELECT_A::PWM_CH0),
            9 => Some(GPIO5FUNCTION_SELECT_A::FEM_GPIO_1),
            10 => Some(GPIO5FUNCTION_SELECT_A::GPIP_CH4),
            11 => Some(GPIO5FUNCTION_SELECT_A::SWGPIO_5),
            14 => Some(GPIO5FUNCTION_SELECT_A::E21_TDI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO_DAT3`"]
    #[inline(always)]
    pub fn is_sdio_dat3(&self) -> bool {
        *self == GPIO5FUNCTION_SELECT_A::SDIO_DAT3
    }
    #[doc = "Checks if the value of the field is `SPI_MOSI_SPI_MISO`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == GPIO5FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == GPIO5FUNCTION_SELECT_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG5`"]
    #[inline(always)]
    pub fn is_uart_sig5(&self) -> bool {
        *self == GPIO5FUNCTION_SELECT_A::UART_SIG5
    }
    #[doc = "Checks if the value of the field is `PWM_CH0`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        *self == GPIO5FUNCTION_SELECT_A::PWM_CH0
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_1`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        *self == GPIO5FUNCTION_SELECT_A::FEM_GPIO_1
    }
    #[doc = "Checks if the value of the field is `GPIP_CH4`"]
    #[inline(always)]
    pub fn is_gpip_ch4(&self) -> bool {
        *self == GPIO5FUNCTION_SELECT_A::GPIP_CH4
    }
    #[doc = "Checks if the value of the field is `SWGPIO_5`"]
    #[inline(always)]
    pub fn is_swgpio_5(&self) -> bool {
        *self == GPIO5FUNCTION_SELECT_A::SWGPIO_5
    }
    #[doc = "Checks if the value of the field is `E21_TDI`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        *self == GPIO5FUNCTION_SELECT_A::E21_TDI
    }
}
#[doc = "Field `reg_gpio_5_func_sel` writer - Function select for GPIO5."]
pub type REG_GPIO_5_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL2_SPEC, u8, GPIO5FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_5_FUNC_SEL_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_dat3(self) -> &'a mut W {
        self.variant(GPIO5FUNCTION_SELECT_A::SDIO_DAT3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut W {
        self.variant(GPIO5FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(GPIO5FUNCTION_SELECT_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig5(self) -> &'a mut W {
        self.variant(GPIO5FUNCTION_SELECT_A::UART_SIG5)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut W {
        self.variant(GPIO5FUNCTION_SELECT_A::PWM_CH0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut W {
        self.variant(GPIO5FUNCTION_SELECT_A::FEM_GPIO_1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch4(self) -> &'a mut W {
        self.variant(GPIO5FUNCTION_SELECT_A::GPIP_CH4)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_5(self) -> &'a mut W {
        self.variant(GPIO5FUNCTION_SELECT_A::SWGPIO_5)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut W {
        self.variant(GPIO5FUNCTION_SELECT_A::E21_TDI)
    }
}
#[doc = "Field `real_gpio_5_func_sel` reader - "]
pub type REAL_GPIO_5_FUNC_SEL_R = crate::FieldReader<u8, GPIO5REAL_FUNCTION_SELECT_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO5REAL_FUNCTION_SELECT_A {
    #[doc = "0: Function select is reg_gpio_5_func_sel"]
    GLB_GPIO_REAL_MODE_REG = 0,
    #[doc = "1: `1`"]
    GLB_GPIO_REAL_MODE_SDIO = 1,
    #[doc = "12: `1100`"]
    GLB_GPIO_REAL_MODE_RF = 12,
    #[doc = "14: `1110`"]
    GLB_GPIO_REAL_MODE_JTAG = 14,
    #[doc = "15: `1111`"]
    GLB_GPIO_REAL_MODE_CCI = 15,
}
impl From<GPIO5REAL_FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO5REAL_FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REAL_GPIO_5_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO5REAL_FUNCTION_SELECT_A> {
        match self.bits {
            0 => Some(GPIO5REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG),
            1 => Some(GPIO5REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO),
            12 => Some(GPIO5REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF),
            14 => Some(GPIO5REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG),
            15 => Some(GPIO5REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_REG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == GPIO5REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_SDIO`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == GPIO5REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_RF`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == GPIO5REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_JTAG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == GPIO5REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_CCI`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == GPIO5REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_ie(&self) -> REG_GPIO_4_IE_R {
        REG_GPIO_4_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_smt(&self) -> REG_GPIO_4_SMT_R {
        REG_GPIO_4_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_drv(&self) -> REG_GPIO_4_DRV_R {
        REG_GPIO_4_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_pu(&self) -> REG_GPIO_4_PU_R {
        REG_GPIO_4_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_pd(&self) -> REG_GPIO_4_PD_R {
        REG_GPIO_4_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_func_sel(&self) -> REG_GPIO_4_FUNC_SEL_R {
        REG_GPIO_4_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_4_func_sel(&self) -> REAL_GPIO_4_FUNC_SEL_R {
        REAL_GPIO_4_FUNC_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_ie(&self) -> REG_GPIO_5_IE_R {
        REG_GPIO_5_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_smt(&self) -> REG_GPIO_5_SMT_R {
        REG_GPIO_5_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_drv(&self) -> REG_GPIO_5_DRV_R {
        REG_GPIO_5_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_pu(&self) -> REG_GPIO_5_PU_R {
        REG_GPIO_5_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_pd(&self) -> REG_GPIO_5_PD_R {
        REG_GPIO_5_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_func_sel(&self) -> REG_GPIO_5_FUNC_SEL_R {
        REG_GPIO_5_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_5_func_sel(&self) -> REAL_GPIO_5_FUNC_SEL_R {
        REAL_GPIO_5_FUNC_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_ie(&mut self) -> REG_GPIO_4_IE_W<0> {
        REG_GPIO_4_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_smt(&mut self) -> REG_GPIO_4_SMT_W<1> {
        REG_GPIO_4_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_drv(&mut self) -> REG_GPIO_4_DRV_W<2> {
        REG_GPIO_4_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_pu(&mut self) -> REG_GPIO_4_PU_W<4> {
        REG_GPIO_4_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_pd(&mut self) -> REG_GPIO_4_PD_W<5> {
        REG_GPIO_4_PD_W::new(self)
    }
    #[doc = "Bits 8:11 - Function select for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_func_sel(&mut self) -> REG_GPIO_4_FUNC_SEL_W<8> {
        REG_GPIO_4_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_ie(&mut self) -> REG_GPIO_5_IE_W<16> {
        REG_GPIO_5_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_smt(&mut self) -> REG_GPIO_5_SMT_W<17> {
        REG_GPIO_5_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_drv(&mut self) -> REG_GPIO_5_DRV_W<18> {
        REG_GPIO_5_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_pu(&mut self) -> REG_GPIO_5_PU_W<20> {
        REG_GPIO_5_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_pd(&mut self) -> REG_GPIO_5_PD_W<21> {
        REG_GPIO_5_PD_W::new(self)
    }
    #[doc = "Bits 24:27 - Function select for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_func_sel(&mut self) -> REG_GPIO_5_FUNC_SEL_W<24> {
        REG_GPIO_5_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO4, GPIO5 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl2](index.html) module"]
pub struct GPIO_CFGCTL2_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl2::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl2::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL2 to value 0x1103_1103"]
impl crate::Resettable for GPIO_CFGCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1103_1103;
}

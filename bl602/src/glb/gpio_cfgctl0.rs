#[doc = "Register `GPIO_CFGCTL0` reader"]
pub struct R(crate::R<GPIO_CFGCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL0` writer"]
pub struct W(crate::W<GPIO_CFGCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL0_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_0_ie` reader - GPIO0 input enable."]
pub type REG_GPIO_0_IE_R = crate::BitReader<GPIO0INPUT_ENABLED_A>;
#[doc = "GPIO0 input enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO0INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_0_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0INPUT_ENABLED_A {
        match self.bits {
            false => GPIO0INPUT_ENABLED_A::DISABLED,
            true => GPIO0INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO0INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO0INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_0_ie` writer - GPIO0 input enable."]
pub type REG_GPIO_0_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, GPIO0INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_0_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO0INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO0INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_0_smt` reader - Schmitt trigger enabled for GPIO0."]
pub type REG_GPIO_0_SMT_R = crate::BitReader<GPIO0SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO0SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_0_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0SCHMITT_A {
        match self.bits {
            false => GPIO0SCHMITT_A::DISABLED,
            true => GPIO0SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO0SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO0SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_0_smt` writer - Schmitt trigger enabled for GPIO0."]
pub type REG_GPIO_0_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, GPIO0SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_0_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO0SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO0SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_0_drv` reader - Driving control for GPIO0."]
pub type REG_GPIO_0_DRV_R = crate::FieldReader<u8, GPIO0DRIVING_A>;
#[doc = "Driving control for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO0DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO0DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO0DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_0_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO0DRIVING_A> {
        match self.bits {
            0 => Some(GPIO0DRIVING_A::DISABLED),
            1 => Some(GPIO0DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO0DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO0DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_0_drv` writer - Driving control for GPIO0."]
pub type REG_GPIO_0_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL0_SPEC, u8, GPIO0DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_0_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO0DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO0DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_0_pu` reader - Pull Up Resistor for GPIO0."]
pub type REG_GPIO_0_PU_R = crate::BitReader<GPIO0PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO0PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_0_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO0PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO0PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO0PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO0PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_0_pu` writer - Pull Up Resistor for GPIO0."]
pub type REG_GPIO_0_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, GPIO0PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_0_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO0PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO0PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_0_pd` reader - Pull Down Resistor for GPIO0."]
pub type REG_GPIO_0_PD_R = crate::BitReader<GPIO0PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO0PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_0_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO0PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO0PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO0PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO0PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_0_pd` writer - Pull Down Resistor for GPIO0."]
pub type REG_GPIO_0_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, GPIO0PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_0_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO0PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO0PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_0_func_sel` reader - Function select for GPIO0."]
pub type REG_GPIO_0_FUNC_SEL_R = crate::FieldReader<u8, GPIO0FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO0FUNCTION_SELECT_A {
    #[doc = "1: `1`"]
    SDIO_CLK = 1,
    #[doc = "2: `10`"]
    SF_D1 = 2,
    #[doc = "4: `100`"]
    SPI_MOSI_SPI_MISO = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG0 = 7,
    #[doc = "8: `1000`"]
    PWM_CH0 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_0 = 9,
    #[doc = "10: `1010`"]
    ATEST_IN = 10,
    #[doc = "11: `1011`"]
    SWGPIO_0 = 11,
    #[doc = "14: `1110`"]
    E21_TMS = 14,
}
impl From<GPIO0FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO0FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_0_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO0FUNCTION_SELECT_A> {
        match self.bits {
            1 => Some(GPIO0FUNCTION_SELECT_A::SDIO_CLK),
            2 => Some(GPIO0FUNCTION_SELECT_A::SF_D1),
            4 => Some(GPIO0FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO),
            6 => Some(GPIO0FUNCTION_SELECT_A::I2C_SCL),
            7 => Some(GPIO0FUNCTION_SELECT_A::UART_SIG0),
            8 => Some(GPIO0FUNCTION_SELECT_A::PWM_CH0),
            9 => Some(GPIO0FUNCTION_SELECT_A::FEM_GPIO_0),
            10 => Some(GPIO0FUNCTION_SELECT_A::ATEST_IN),
            11 => Some(GPIO0FUNCTION_SELECT_A::SWGPIO_0),
            14 => Some(GPIO0FUNCTION_SELECT_A::E21_TMS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO_CLK`"]
    #[inline(always)]
    pub fn is_sdio_clk(&self) -> bool {
        *self == GPIO0FUNCTION_SELECT_A::SDIO_CLK
    }
    #[doc = "Checks if the value of the field is `SF_D1`"]
    #[inline(always)]
    pub fn is_sf_d1(&self) -> bool {
        *self == GPIO0FUNCTION_SELECT_A::SF_D1
    }
    #[doc = "Checks if the value of the field is `SPI_MOSI_SPI_MISO`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == GPIO0FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == GPIO0FUNCTION_SELECT_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG0`"]
    #[inline(always)]
    pub fn is_uart_sig0(&self) -> bool {
        *self == GPIO0FUNCTION_SELECT_A::UART_SIG0
    }
    #[doc = "Checks if the value of the field is `PWM_CH0`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        *self == GPIO0FUNCTION_SELECT_A::PWM_CH0
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_0`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        *self == GPIO0FUNCTION_SELECT_A::FEM_GPIO_0
    }
    #[doc = "Checks if the value of the field is `ATEST_IN`"]
    #[inline(always)]
    pub fn is_atest_in(&self) -> bool {
        *self == GPIO0FUNCTION_SELECT_A::ATEST_IN
    }
    #[doc = "Checks if the value of the field is `SWGPIO_0`"]
    #[inline(always)]
    pub fn is_swgpio_0(&self) -> bool {
        *self == GPIO0FUNCTION_SELECT_A::SWGPIO_0
    }
    #[doc = "Checks if the value of the field is `E21_TMS`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        *self == GPIO0FUNCTION_SELECT_A::E21_TMS
    }
}
#[doc = "Field `reg_gpio_0_func_sel` writer - Function select for GPIO0."]
pub type REG_GPIO_0_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL0_SPEC, u8, GPIO0FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_0_FUNC_SEL_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_clk(self) -> &'a mut W {
        self.variant(GPIO0FUNCTION_SELECT_A::SDIO_CLK)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d1(self) -> &'a mut W {
        self.variant(GPIO0FUNCTION_SELECT_A::SF_D1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut W {
        self.variant(GPIO0FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(GPIO0FUNCTION_SELECT_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig0(self) -> &'a mut W {
        self.variant(GPIO0FUNCTION_SELECT_A::UART_SIG0)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut W {
        self.variant(GPIO0FUNCTION_SELECT_A::PWM_CH0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut W {
        self.variant(GPIO0FUNCTION_SELECT_A::FEM_GPIO_0)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn atest_in(self) -> &'a mut W {
        self.variant(GPIO0FUNCTION_SELECT_A::ATEST_IN)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_0(self) -> &'a mut W {
        self.variant(GPIO0FUNCTION_SELECT_A::SWGPIO_0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut W {
        self.variant(GPIO0FUNCTION_SELECT_A::E21_TMS)
    }
}
#[doc = "Field `real_gpio_0_func_sel` reader - "]
pub type REAL_GPIO_0_FUNC_SEL_R = crate::FieldReader<u8, GPIO0REAL_FUNCTION_SELECT_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO0REAL_FUNCTION_SELECT_A {
    #[doc = "0: Function select is reg_gpio_0_func_sel"]
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
impl From<GPIO0REAL_FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO0REAL_FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REAL_GPIO_0_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO0REAL_FUNCTION_SELECT_A> {
        match self.bits {
            0 => Some(GPIO0REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG),
            1 => Some(GPIO0REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO),
            12 => Some(GPIO0REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF),
            14 => Some(GPIO0REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG),
            15 => Some(GPIO0REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_REG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == GPIO0REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_SDIO`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == GPIO0REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_RF`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == GPIO0REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_JTAG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == GPIO0REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_CCI`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == GPIO0REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI
    }
}
#[doc = "Field `reg_gpio_1_ie` reader - Input enable for GPIO1."]
pub type REG_GPIO_1_IE_R = crate::BitReader<GPIO1INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO1INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_1_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1INPUT_ENABLED_A {
        match self.bits {
            false => GPIO1INPUT_ENABLED_A::DISABLED,
            true => GPIO1INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO1INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO1INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_1_ie` writer - Input enable for GPIO1."]
pub type REG_GPIO_1_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, GPIO1INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_1_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO1INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO1INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_1_smt` reader - Schmitt trigger enabled for GPIO1."]
pub type REG_GPIO_1_SMT_R = crate::BitReader<GPIO1SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO1SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_1_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1SCHMITT_A {
        match self.bits {
            false => GPIO1SCHMITT_A::DISABLED,
            true => GPIO1SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO1SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO1SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_1_smt` writer - Schmitt trigger enabled for GPIO1."]
pub type REG_GPIO_1_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, GPIO1SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_1_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO1SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO1SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_1_drv` reader - Driving control enabled for GPIO1."]
pub type REG_GPIO_1_DRV_R = crate::FieldReader<u8, GPIO1DRIVING_A>;
#[doc = "Driving control enabled for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO1DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO1DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO1DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_1_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO1DRIVING_A> {
        match self.bits {
            0 => Some(GPIO1DRIVING_A::DISABLED),
            1 => Some(GPIO1DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO1DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO1DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_1_drv` writer - Driving control enabled for GPIO1."]
pub type REG_GPIO_1_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL0_SPEC, u8, GPIO1DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_1_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO1DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO1DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_1_pu` reader - Pull Up Resistor for GPIO1."]
pub type REG_GPIO_1_PU_R = crate::BitReader<GPIO1PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO1PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_1_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO1PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO1PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO1PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO1PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_1_pu` writer - Pull Up Resistor for GPIO1."]
pub type REG_GPIO_1_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, GPIO1PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_1_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO1PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO1PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_1_pd` reader - Pull Down Resistor for GPIO1."]
pub type REG_GPIO_1_PD_R = crate::BitReader<GPIO1PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO1PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_1_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO1PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO1PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO1PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO1PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_1_pd` writer - Pull Down Resistor for GPIO1."]
pub type REG_GPIO_1_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, GPIO1PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_1_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO1PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO1PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_1_func_sel` reader - Function select for GPIO1."]
pub type REG_GPIO_1_FUNC_SEL_R = crate::FieldReader<u8, GPIO1FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO1FUNCTION_SELECT_A {
    #[doc = "1: `1`"]
    SDIO_CMD = 1,
    #[doc = "2: `10`"]
    SF_D2 = 2,
    #[doc = "4: `100`"]
    SPI_MOSI_SPI_MISO = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG1 = 7,
    #[doc = "8: `1000`"]
    PWM_CH1 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_1 = 9,
    #[doc = "10: `1010`"]
    ATEST_IP = 10,
    #[doc = "11: `1011`"]
    SWGPIO_1 = 11,
    #[doc = "14: `1110`"]
    E21_TDI = 14,
}
impl From<GPIO1FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO1FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_1_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO1FUNCTION_SELECT_A> {
        match self.bits {
            1 => Some(GPIO1FUNCTION_SELECT_A::SDIO_CMD),
            2 => Some(GPIO1FUNCTION_SELECT_A::SF_D2),
            4 => Some(GPIO1FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO),
            6 => Some(GPIO1FUNCTION_SELECT_A::I2C_SDA),
            7 => Some(GPIO1FUNCTION_SELECT_A::UART_SIG1),
            8 => Some(GPIO1FUNCTION_SELECT_A::PWM_CH1),
            9 => Some(GPIO1FUNCTION_SELECT_A::FEM_GPIO_1),
            10 => Some(GPIO1FUNCTION_SELECT_A::ATEST_IP),
            11 => Some(GPIO1FUNCTION_SELECT_A::SWGPIO_1),
            14 => Some(GPIO1FUNCTION_SELECT_A::E21_TDI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO_CMD`"]
    #[inline(always)]
    pub fn is_sdio_cmd(&self) -> bool {
        *self == GPIO1FUNCTION_SELECT_A::SDIO_CMD
    }
    #[doc = "Checks if the value of the field is `SF_D2`"]
    #[inline(always)]
    pub fn is_sf_d2(&self) -> bool {
        *self == GPIO1FUNCTION_SELECT_A::SF_D2
    }
    #[doc = "Checks if the value of the field is `SPI_MOSI_SPI_MISO`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == GPIO1FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == GPIO1FUNCTION_SELECT_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG1`"]
    #[inline(always)]
    pub fn is_uart_sig1(&self) -> bool {
        *self == GPIO1FUNCTION_SELECT_A::UART_SIG1
    }
    #[doc = "Checks if the value of the field is `PWM_CH1`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        *self == GPIO1FUNCTION_SELECT_A::PWM_CH1
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_1`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        *self == GPIO1FUNCTION_SELECT_A::FEM_GPIO_1
    }
    #[doc = "Checks if the value of the field is `ATEST_IP`"]
    #[inline(always)]
    pub fn is_atest_ip(&self) -> bool {
        *self == GPIO1FUNCTION_SELECT_A::ATEST_IP
    }
    #[doc = "Checks if the value of the field is `SWGPIO_1`"]
    #[inline(always)]
    pub fn is_swgpio_1(&self) -> bool {
        *self == GPIO1FUNCTION_SELECT_A::SWGPIO_1
    }
    #[doc = "Checks if the value of the field is `E21_TDI`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        *self == GPIO1FUNCTION_SELECT_A::E21_TDI
    }
}
#[doc = "Field `reg_gpio_1_func_sel` writer - Function select for GPIO1."]
pub type REG_GPIO_1_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL0_SPEC, u8, GPIO1FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_1_FUNC_SEL_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_cmd(self) -> &'a mut W {
        self.variant(GPIO1FUNCTION_SELECT_A::SDIO_CMD)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d2(self) -> &'a mut W {
        self.variant(GPIO1FUNCTION_SELECT_A::SF_D2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut W {
        self.variant(GPIO1FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(GPIO1FUNCTION_SELECT_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig1(self) -> &'a mut W {
        self.variant(GPIO1FUNCTION_SELECT_A::UART_SIG1)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut W {
        self.variant(GPIO1FUNCTION_SELECT_A::PWM_CH1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut W {
        self.variant(GPIO1FUNCTION_SELECT_A::FEM_GPIO_1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn atest_ip(self) -> &'a mut W {
        self.variant(GPIO1FUNCTION_SELECT_A::ATEST_IP)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_1(self) -> &'a mut W {
        self.variant(GPIO1FUNCTION_SELECT_A::SWGPIO_1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut W {
        self.variant(GPIO1FUNCTION_SELECT_A::E21_TDI)
    }
}
#[doc = "Field `real_gpio_1_func_sel` reader - "]
pub type REAL_GPIO_1_FUNC_SEL_R = crate::FieldReader<u8, GPIO1REAL_FUNCTION_SELECT_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO1REAL_FUNCTION_SELECT_A {
    #[doc = "0: Function select is reg_gpio_1_func_sel"]
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
impl From<GPIO1REAL_FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO1REAL_FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REAL_GPIO_1_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO1REAL_FUNCTION_SELECT_A> {
        match self.bits {
            0 => Some(GPIO1REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG),
            1 => Some(GPIO1REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO),
            12 => Some(GPIO1REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF),
            14 => Some(GPIO1REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG),
            15 => Some(GPIO1REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_REG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == GPIO1REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_SDIO`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == GPIO1REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_RF`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == GPIO1REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_JTAG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == GPIO1REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_CCI`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == GPIO1REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI
    }
}
impl R {
    #[doc = "Bit 0 - GPIO0 input enable."]
    #[inline(always)]
    pub fn reg_gpio_0_ie(&self) -> REG_GPIO_0_IE_R {
        REG_GPIO_0_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_smt(&self) -> REG_GPIO_0_SMT_R {
        REG_GPIO_0_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_drv(&self) -> REG_GPIO_0_DRV_R {
        REG_GPIO_0_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_pu(&self) -> REG_GPIO_0_PU_R {
        REG_GPIO_0_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_pd(&self) -> REG_GPIO_0_PD_R {
        REG_GPIO_0_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_func_sel(&self) -> REG_GPIO_0_FUNC_SEL_R {
        REG_GPIO_0_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_0_func_sel(&self) -> REAL_GPIO_0_FUNC_SEL_R {
        REAL_GPIO_0_FUNC_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_ie(&self) -> REG_GPIO_1_IE_R {
        REG_GPIO_1_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_smt(&self) -> REG_GPIO_1_SMT_R {
        REG_GPIO_1_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_drv(&self) -> REG_GPIO_1_DRV_R {
        REG_GPIO_1_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_pu(&self) -> REG_GPIO_1_PU_R {
        REG_GPIO_1_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_pd(&self) -> REG_GPIO_1_PD_R {
        REG_GPIO_1_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_func_sel(&self) -> REG_GPIO_1_FUNC_SEL_R {
        REG_GPIO_1_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_1_func_sel(&self) -> REAL_GPIO_1_FUNC_SEL_R {
        REAL_GPIO_1_FUNC_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO0 input enable."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_ie(&mut self) -> REG_GPIO_0_IE_W<0> {
        REG_GPIO_0_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_smt(&mut self) -> REG_GPIO_0_SMT_W<1> {
        REG_GPIO_0_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control for GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_drv(&mut self) -> REG_GPIO_0_DRV_W<2> {
        REG_GPIO_0_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_pu(&mut self) -> REG_GPIO_0_PU_W<4> {
        REG_GPIO_0_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_pd(&mut self) -> REG_GPIO_0_PD_W<5> {
        REG_GPIO_0_PD_W::new(self)
    }
    #[doc = "Bits 8:11 - Function select for GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_func_sel(&mut self) -> REG_GPIO_0_FUNC_SEL_W<8> {
        REG_GPIO_0_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_ie(&mut self) -> REG_GPIO_1_IE_W<16> {
        REG_GPIO_1_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_smt(&mut self) -> REG_GPIO_1_SMT_W<17> {
        REG_GPIO_1_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_drv(&mut self) -> REG_GPIO_1_DRV_W<18> {
        REG_GPIO_1_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_pu(&mut self) -> REG_GPIO_1_PU_W<20> {
        REG_GPIO_1_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_pd(&mut self) -> REG_GPIO_1_PD_W<21> {
        REG_GPIO_1_PD_W::new(self)
    }
    #[doc = "Bits 24:27 - Function select for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_func_sel(&mut self) -> REG_GPIO_1_FUNC_SEL_W<24> {
        REG_GPIO_1_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO0, GPIO1 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl0](index.html) module"]
pub struct GPIO_CFGCTL0_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl0::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl0::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL0 to value 0x1103_1103"]
impl crate::Resettable for GPIO_CFGCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1103_1103;
}

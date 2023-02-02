#[doc = "Register `GPIO_CFGCTL1` reader"]
pub struct R(crate::R<GPIO_CFGCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL1` writer"]
pub struct W(crate::W<GPIO_CFGCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL1_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_2_ie` reader - Input enable for GPIO2."]
pub type REG_GPIO_2_IE_R = crate::BitReader<GPIO2INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO2INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_2_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2INPUT_ENABLED_A {
        match self.bits {
            false => GPIO2INPUT_ENABLED_A::DISABLED,
            true => GPIO2INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO2INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO2INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_2_ie` writer - Input enable for GPIO2."]
pub type REG_GPIO_2_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL1_SPEC, GPIO2INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_2_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO2INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO2INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_2_smt` reader - Schmitt trigger enabled for GPIO2."]
pub type REG_GPIO_2_SMT_R = crate::BitReader<GPIO2SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO2SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_2_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2SCHMITT_A {
        match self.bits {
            false => GPIO2SCHMITT_A::DISABLED,
            true => GPIO2SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO2SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO2SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_2_smt` writer - Schmitt trigger enabled for GPIO2."]
pub type REG_GPIO_2_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL1_SPEC, GPIO2SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_2_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO2SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO2SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_2_drv` reader - Driving control enabled for GPIO2."]
pub type REG_GPIO_2_DRV_R = crate::FieldReader<u8, GPIO2DRIVING_A>;
#[doc = "Driving control enabled for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO2DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO2DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO2DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_2_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO2DRIVING_A> {
        match self.bits {
            0 => Some(GPIO2DRIVING_A::DISABLED),
            1 => Some(GPIO2DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO2DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO2DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_2_drv` writer - Driving control enabled for GPIO2."]
pub type REG_GPIO_2_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL1_SPEC, u8, GPIO2DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_2_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO2DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO2DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_2_pu` reader - Pull Up Resistor for GPIO2."]
pub type REG_GPIO_2_PU_R = crate::BitReader<GPIO2PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO2PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_2_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO2PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO2PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO2PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO2PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_2_pu` writer - Pull Up Resistor for GPIO2."]
pub type REG_GPIO_2_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL1_SPEC, GPIO2PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_2_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO2PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO2PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_2_pd` reader - Pull Down Resistor for GPIO2."]
pub type REG_GPIO_2_PD_R = crate::BitReader<GPIO2PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO2PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_2_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO2PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO2PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO2PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO2PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_2_pd` writer - Pull Down Resistor for GPIO2."]
pub type REG_GPIO_2_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL1_SPEC, GPIO2PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_2_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO2PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO2PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_2_func_sel` reader - Function select for GPIO2."]
pub type REG_GPIO_2_FUNC_SEL_R = crate::FieldReader<u8, GPIO2FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO2FUNCTION_SELECT_A {
    #[doc = "1: `1`"]
    SDIO_DAT0 = 1,
    #[doc = "2: `10`"]
    SF_D3 = 2,
    #[doc = "4: `100`"]
    SPI_SS = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG2 = 7,
    #[doc = "8: `1000`"]
    PWM_CH2 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_2 = 9,
    #[doc = "10: `1010`"]
    ATEST_QN = 10,
    #[doc = "11: `1011`"]
    SWGPIO_2 = 11,
    #[doc = "14: `1110`"]
    E21_TCK = 14,
}
impl From<GPIO2FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO2FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_2_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO2FUNCTION_SELECT_A> {
        match self.bits {
            1 => Some(GPIO2FUNCTION_SELECT_A::SDIO_DAT0),
            2 => Some(GPIO2FUNCTION_SELECT_A::SF_D3),
            4 => Some(GPIO2FUNCTION_SELECT_A::SPI_SS),
            6 => Some(GPIO2FUNCTION_SELECT_A::I2C_SCL),
            7 => Some(GPIO2FUNCTION_SELECT_A::UART_SIG2),
            8 => Some(GPIO2FUNCTION_SELECT_A::PWM_CH2),
            9 => Some(GPIO2FUNCTION_SELECT_A::FEM_GPIO_2),
            10 => Some(GPIO2FUNCTION_SELECT_A::ATEST_QN),
            11 => Some(GPIO2FUNCTION_SELECT_A::SWGPIO_2),
            14 => Some(GPIO2FUNCTION_SELECT_A::E21_TCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO_DAT0`"]
    #[inline(always)]
    pub fn is_sdio_dat0(&self) -> bool {
        *self == GPIO2FUNCTION_SELECT_A::SDIO_DAT0
    }
    #[doc = "Checks if the value of the field is `SF_D3`"]
    #[inline(always)]
    pub fn is_sf_d3(&self) -> bool {
        *self == GPIO2FUNCTION_SELECT_A::SF_D3
    }
    #[doc = "Checks if the value of the field is `SPI_SS`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == GPIO2FUNCTION_SELECT_A::SPI_SS
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == GPIO2FUNCTION_SELECT_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG2`"]
    #[inline(always)]
    pub fn is_uart_sig2(&self) -> bool {
        *self == GPIO2FUNCTION_SELECT_A::UART_SIG2
    }
    #[doc = "Checks if the value of the field is `PWM_CH2`"]
    #[inline(always)]
    pub fn is_pwm_ch2(&self) -> bool {
        *self == GPIO2FUNCTION_SELECT_A::PWM_CH2
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_2`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == GPIO2FUNCTION_SELECT_A::FEM_GPIO_2
    }
    #[doc = "Checks if the value of the field is `ATEST_QN`"]
    #[inline(always)]
    pub fn is_atest_qn(&self) -> bool {
        *self == GPIO2FUNCTION_SELECT_A::ATEST_QN
    }
    #[doc = "Checks if the value of the field is `SWGPIO_2`"]
    #[inline(always)]
    pub fn is_swgpio_2(&self) -> bool {
        *self == GPIO2FUNCTION_SELECT_A::SWGPIO_2
    }
    #[doc = "Checks if the value of the field is `E21_TCK`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == GPIO2FUNCTION_SELECT_A::E21_TCK
    }
}
#[doc = "Field `reg_gpio_2_func_sel` writer - Function select for GPIO2."]
pub type REG_GPIO_2_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL1_SPEC, u8, GPIO2FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_2_FUNC_SEL_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_dat0(self) -> &'a mut W {
        self.variant(GPIO2FUNCTION_SELECT_A::SDIO_DAT0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d3(self) -> &'a mut W {
        self.variant(GPIO2FUNCTION_SELECT_A::SF_D3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut W {
        self.variant(GPIO2FUNCTION_SELECT_A::SPI_SS)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(GPIO2FUNCTION_SELECT_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig2(self) -> &'a mut W {
        self.variant(GPIO2FUNCTION_SELECT_A::UART_SIG2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch2(self) -> &'a mut W {
        self.variant(GPIO2FUNCTION_SELECT_A::PWM_CH2)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut W {
        self.variant(GPIO2FUNCTION_SELECT_A::FEM_GPIO_2)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn atest_qn(self) -> &'a mut W {
        self.variant(GPIO2FUNCTION_SELECT_A::ATEST_QN)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_2(self) -> &'a mut W {
        self.variant(GPIO2FUNCTION_SELECT_A::SWGPIO_2)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut W {
        self.variant(GPIO2FUNCTION_SELECT_A::E21_TCK)
    }
}
#[doc = "Field `real_gpio_2_func_sel` reader - "]
pub type REAL_GPIO_2_FUNC_SEL_R = crate::FieldReader<u8, GPIO2REAL_FUNCTION_SELECT_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO2REAL_FUNCTION_SELECT_A {
    #[doc = "0: Function select is reg_gpio_2_func_sel"]
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
impl From<GPIO2REAL_FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO2REAL_FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REAL_GPIO_2_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO2REAL_FUNCTION_SELECT_A> {
        match self.bits {
            0 => Some(GPIO2REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG),
            1 => Some(GPIO2REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO),
            12 => Some(GPIO2REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF),
            14 => Some(GPIO2REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG),
            15 => Some(GPIO2REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_REG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == GPIO2REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_SDIO`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == GPIO2REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_RF`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == GPIO2REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_JTAG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == GPIO2REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_CCI`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == GPIO2REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI
    }
}
#[doc = "Field `reg_gpio_3_ie` reader - Input enable for GPIO3."]
pub type REG_GPIO_3_IE_R = crate::BitReader<GPIO3INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO3INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO3INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_3_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3INPUT_ENABLED_A {
        match self.bits {
            false => GPIO3INPUT_ENABLED_A::DISABLED,
            true => GPIO3INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO3INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO3INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_3_ie` writer - Input enable for GPIO3."]
pub type REG_GPIO_3_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL1_SPEC, GPIO3INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_3_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO3INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO3INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_3_smt` reader - Schmitt trigger enabled for GPIO3."]
pub type REG_GPIO_3_SMT_R = crate::BitReader<GPIO3SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO3SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO3SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_3_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3SCHMITT_A {
        match self.bits {
            false => GPIO3SCHMITT_A::DISABLED,
            true => GPIO3SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO3SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO3SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_3_smt` writer - Schmitt trigger enabled for GPIO3."]
pub type REG_GPIO_3_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL1_SPEC, GPIO3SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_3_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO3SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO3SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_3_drv` reader - Driving control enabled for GPIO3."]
pub type REG_GPIO_3_DRV_R = crate::FieldReader<u8, GPIO3DRIVING_A>;
#[doc = "Driving control enabled for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO3DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO3DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO3DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_3_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO3DRIVING_A> {
        match self.bits {
            0 => Some(GPIO3DRIVING_A::DISABLED),
            1 => Some(GPIO3DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO3DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO3DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_3_drv` writer - Driving control enabled for GPIO3."]
pub type REG_GPIO_3_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL1_SPEC, u8, GPIO3DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_3_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO3DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO3DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_3_pu` reader - Pull Up Resistor for GPIO3."]
pub type REG_GPIO_3_PU_R = crate::BitReader<GPIO3PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO3PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO3PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_3_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO3PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO3PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO3PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO3PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_3_pu` writer - Pull Up Resistor for GPIO3."]
pub type REG_GPIO_3_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL1_SPEC, GPIO3PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_3_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO3PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO3PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_3_pd` reader - Pull Down Resistor for GPIO3."]
pub type REG_GPIO_3_PD_R = crate::BitReader<GPIO3PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO3PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO3PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_3_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO3PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO3PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO3PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO3PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_3_pd` writer - Pull Down Resistor for GPIO3."]
pub type REG_GPIO_3_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL1_SPEC, GPIO3PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_3_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO3PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO3PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_3_func_sel` reader - Function select for GPIO3."]
pub type REG_GPIO_3_FUNC_SEL_R = crate::FieldReader<u8, GPIO3FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO3FUNCTION_SELECT_A {
    #[doc = "1: `1`"]
    SDIO_DAT1 = 1,
    #[doc = "4: `100`"]
    SPI_SCLK = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG3 = 7,
    #[doc = "8: `1000`"]
    PWM_CH3 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_3 = 9,
    #[doc = "10: `1010`"]
    ATEST_QP = 10,
    #[doc = "11: `1011`"]
    SWGPIO_3 = 11,
    #[doc = "14: `1110`"]
    E21_TDO = 14,
}
impl From<GPIO3FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO3FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_3_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO3FUNCTION_SELECT_A> {
        match self.bits {
            1 => Some(GPIO3FUNCTION_SELECT_A::SDIO_DAT1),
            4 => Some(GPIO3FUNCTION_SELECT_A::SPI_SCLK),
            6 => Some(GPIO3FUNCTION_SELECT_A::I2C_SDA),
            7 => Some(GPIO3FUNCTION_SELECT_A::UART_SIG3),
            8 => Some(GPIO3FUNCTION_SELECT_A::PWM_CH3),
            9 => Some(GPIO3FUNCTION_SELECT_A::FEM_GPIO_3),
            10 => Some(GPIO3FUNCTION_SELECT_A::ATEST_QP),
            11 => Some(GPIO3FUNCTION_SELECT_A::SWGPIO_3),
            14 => Some(GPIO3FUNCTION_SELECT_A::E21_TDO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO_DAT1`"]
    #[inline(always)]
    pub fn is_sdio_dat1(&self) -> bool {
        *self == GPIO3FUNCTION_SELECT_A::SDIO_DAT1
    }
    #[doc = "Checks if the value of the field is `SPI_SCLK`"]
    #[inline(always)]
    pub fn is_spi_sclk(&self) -> bool {
        *self == GPIO3FUNCTION_SELECT_A::SPI_SCLK
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == GPIO3FUNCTION_SELECT_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG3`"]
    #[inline(always)]
    pub fn is_uart_sig3(&self) -> bool {
        *self == GPIO3FUNCTION_SELECT_A::UART_SIG3
    }
    #[doc = "Checks if the value of the field is `PWM_CH3`"]
    #[inline(always)]
    pub fn is_pwm_ch3(&self) -> bool {
        *self == GPIO3FUNCTION_SELECT_A::PWM_CH3
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_3`"]
    #[inline(always)]
    pub fn is_fem_gpio_3(&self) -> bool {
        *self == GPIO3FUNCTION_SELECT_A::FEM_GPIO_3
    }
    #[doc = "Checks if the value of the field is `ATEST_QP`"]
    #[inline(always)]
    pub fn is_atest_qp(&self) -> bool {
        *self == GPIO3FUNCTION_SELECT_A::ATEST_QP
    }
    #[doc = "Checks if the value of the field is `SWGPIO_3`"]
    #[inline(always)]
    pub fn is_swgpio_3(&self) -> bool {
        *self == GPIO3FUNCTION_SELECT_A::SWGPIO_3
    }
    #[doc = "Checks if the value of the field is `E21_TDO`"]
    #[inline(always)]
    pub fn is_e21_tdo(&self) -> bool {
        *self == GPIO3FUNCTION_SELECT_A::E21_TDO
    }
}
#[doc = "Field `reg_gpio_3_func_sel` writer - Function select for GPIO3."]
pub type REG_GPIO_3_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL1_SPEC, u8, GPIO3FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_3_FUNC_SEL_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_dat1(self) -> &'a mut W {
        self.variant(GPIO3FUNCTION_SELECT_A::SDIO_DAT1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_sclk(self) -> &'a mut W {
        self.variant(GPIO3FUNCTION_SELECT_A::SPI_SCLK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(GPIO3FUNCTION_SELECT_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig3(self) -> &'a mut W {
        self.variant(GPIO3FUNCTION_SELECT_A::UART_SIG3)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch3(self) -> &'a mut W {
        self.variant(GPIO3FUNCTION_SELECT_A::PWM_CH3)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_3(self) -> &'a mut W {
        self.variant(GPIO3FUNCTION_SELECT_A::FEM_GPIO_3)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn atest_qp(self) -> &'a mut W {
        self.variant(GPIO3FUNCTION_SELECT_A::ATEST_QP)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_3(self) -> &'a mut W {
        self.variant(GPIO3FUNCTION_SELECT_A::SWGPIO_3)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdo(self) -> &'a mut W {
        self.variant(GPIO3FUNCTION_SELECT_A::E21_TDO)
    }
}
#[doc = "Field `real_gpio_3_func_sel` reader - "]
pub type REAL_GPIO_3_FUNC_SEL_R = crate::FieldReader<u8, GPIO3REAL_FUNCTION_SELECT_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO3REAL_FUNCTION_SELECT_A {
    #[doc = "0: Function select is reg_gpio_3_func_sel"]
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
impl From<GPIO3REAL_FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO3REAL_FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REAL_GPIO_3_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO3REAL_FUNCTION_SELECT_A> {
        match self.bits {
            0 => Some(GPIO3REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG),
            1 => Some(GPIO3REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO),
            12 => Some(GPIO3REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF),
            14 => Some(GPIO3REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG),
            15 => Some(GPIO3REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_REG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == GPIO3REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_REG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_SDIO`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == GPIO3REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_SDIO
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_RF`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == GPIO3REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_RF
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_JTAG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == GPIO3REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_JTAG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_CCI`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == GPIO3REAL_FUNCTION_SELECT_A::GLB_GPIO_REAL_MODE_CCI
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_ie(&self) -> REG_GPIO_2_IE_R {
        REG_GPIO_2_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_smt(&self) -> REG_GPIO_2_SMT_R {
        REG_GPIO_2_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_drv(&self) -> REG_GPIO_2_DRV_R {
        REG_GPIO_2_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_pu(&self) -> REG_GPIO_2_PU_R {
        REG_GPIO_2_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_pd(&self) -> REG_GPIO_2_PD_R {
        REG_GPIO_2_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_func_sel(&self) -> REG_GPIO_2_FUNC_SEL_R {
        REG_GPIO_2_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_2_func_sel(&self) -> REAL_GPIO_2_FUNC_SEL_R {
        REAL_GPIO_2_FUNC_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_ie(&self) -> REG_GPIO_3_IE_R {
        REG_GPIO_3_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_smt(&self) -> REG_GPIO_3_SMT_R {
        REG_GPIO_3_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_drv(&self) -> REG_GPIO_3_DRV_R {
        REG_GPIO_3_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_pu(&self) -> REG_GPIO_3_PU_R {
        REG_GPIO_3_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_pd(&self) -> REG_GPIO_3_PD_R {
        REG_GPIO_3_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_func_sel(&self) -> REG_GPIO_3_FUNC_SEL_R {
        REG_GPIO_3_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_3_func_sel(&self) -> REAL_GPIO_3_FUNC_SEL_R {
        REAL_GPIO_3_FUNC_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_ie(&mut self) -> REG_GPIO_2_IE_W<0> {
        REG_GPIO_2_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_smt(&mut self) -> REG_GPIO_2_SMT_W<1> {
        REG_GPIO_2_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_drv(&mut self) -> REG_GPIO_2_DRV_W<2> {
        REG_GPIO_2_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_pu(&mut self) -> REG_GPIO_2_PU_W<4> {
        REG_GPIO_2_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_pd(&mut self) -> REG_GPIO_2_PD_W<5> {
        REG_GPIO_2_PD_W::new(self)
    }
    #[doc = "Bits 8:11 - Function select for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_func_sel(&mut self) -> REG_GPIO_2_FUNC_SEL_W<8> {
        REG_GPIO_2_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_ie(&mut self) -> REG_GPIO_3_IE_W<16> {
        REG_GPIO_3_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_smt(&mut self) -> REG_GPIO_3_SMT_W<17> {
        REG_GPIO_3_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_drv(&mut self) -> REG_GPIO_3_DRV_W<18> {
        REG_GPIO_3_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_pu(&mut self) -> REG_GPIO_3_PU_W<20> {
        REG_GPIO_3_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_pd(&mut self) -> REG_GPIO_3_PD_W<21> {
        REG_GPIO_3_PD_W::new(self)
    }
    #[doc = "Bits 24:27 - Function select for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_func_sel(&mut self) -> REG_GPIO_3_FUNC_SEL_W<24> {
        REG_GPIO_3_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO2, GPIO3 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl1](index.html) module"]
pub struct GPIO_CFGCTL1_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl1::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl1::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL1 to value 0x1103_1103"]
impl crate::Resettable for GPIO_CFGCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1103_1103;
}

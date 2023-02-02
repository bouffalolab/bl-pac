#[doc = "Register `GPIO_CFGCTL10` reader"]
pub struct R(crate::R<GPIO_CFGCTL10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL10` writer"]
pub struct W(crate::W<GPIO_CFGCTL10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL10_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_20_ie` reader - Input enable for GPIO20."]
pub type REG_GPIO_20_IE_R = crate::BitReader<GPIO20INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO20.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO20INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO20INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_20_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20INPUT_ENABLED_A {
        match self.bits {
            false => GPIO20INPUT_ENABLED_A::DISABLED,
            true => GPIO20INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO20INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO20INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_20_ie` writer - Input enable for GPIO20."]
pub type REG_GPIO_20_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL10_SPEC, GPIO20INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_20_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO20INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO20INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_20_smt` reader - Schmitt trigger enabled for GPIO20."]
pub type REG_GPIO_20_SMT_R = crate::BitReader<GPIO20SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO20.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO20SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO20SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_20_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20SCHMITT_A {
        match self.bits {
            false => GPIO20SCHMITT_A::DISABLED,
            true => GPIO20SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO20SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO20SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_20_smt` writer - Schmitt trigger enabled for GPIO20."]
pub type REG_GPIO_20_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL10_SPEC, GPIO20SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_20_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO20SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO20SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_20_drv` reader - Driving control enabled for GPIO20."]
pub type REG_GPIO_20_DRV_R = crate::FieldReader<u8, GPIO20DRIVING_A>;
#[doc = "Driving control enabled for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO20DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO20DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO20DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_20_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO20DRIVING_A> {
        match self.bits {
            0 => Some(GPIO20DRIVING_A::DISABLED),
            1 => Some(GPIO20DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO20DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO20DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_20_drv` writer - Driving control enabled for GPIO20."]
pub type REG_GPIO_20_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL10_SPEC, u8, GPIO20DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_20_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO20DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO20DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_20_pu` reader - Pull Up Resistor for GPIO20."]
pub type REG_GPIO_20_PU_R = crate::BitReader<GPIO20PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO20PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO20PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_20_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO20PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO20PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO20PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO20PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_20_pu` writer - Pull Up Resistor for GPIO20."]
pub type REG_GPIO_20_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL10_SPEC, GPIO20PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_20_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO20PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO20PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_20_pd` reader - Pull Down Resistor for GPIO20."]
pub type REG_GPIO_20_PD_R = crate::BitReader<GPIO20PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO20PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO20PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_20_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO20PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO20PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO20PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO20PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_20_pd` writer - Pull Down Resistor for GPIO20."]
pub type REG_GPIO_20_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL10_SPEC, GPIO20PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_20_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO20PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO20PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_20_func_sel` reader - Function select for GPIO20."]
pub type REG_GPIO_20_FUNC_SEL_R = crate::FieldReader<u8, GPIO20FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO20.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO20FUNCTION_SELECT_A {
    #[doc = "2: `10`"]
    SF_D0 = 2,
    #[doc = "4: `100`"]
    SPI_MISO_SPI_MOSI = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG4 = 7,
    #[doc = "8: `1000`"]
    PWM_CH0 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_0 = 9,
    #[doc = "11: `1011`"]
    SWGPIO_20 = 11,
    #[doc = "14: `1110`"]
    E21_TMS = 14,
}
impl From<GPIO20FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO20FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_20_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO20FUNCTION_SELECT_A> {
        match self.bits {
            2 => Some(GPIO20FUNCTION_SELECT_A::SF_D0),
            4 => Some(GPIO20FUNCTION_SELECT_A::SPI_MISO_SPI_MOSI),
            6 => Some(GPIO20FUNCTION_SELECT_A::I2C_SCL),
            7 => Some(GPIO20FUNCTION_SELECT_A::UART_SIG4),
            8 => Some(GPIO20FUNCTION_SELECT_A::PWM_CH0),
            9 => Some(GPIO20FUNCTION_SELECT_A::FEM_GPIO_0),
            11 => Some(GPIO20FUNCTION_SELECT_A::SWGPIO_20),
            14 => Some(GPIO20FUNCTION_SELECT_A::E21_TMS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SF_D0`"]
    #[inline(always)]
    pub fn is_sf_d0(&self) -> bool {
        *self == GPIO20FUNCTION_SELECT_A::SF_D0
    }
    #[doc = "Checks if the value of the field is `SPI_MISO_SPI_MOSI`"]
    #[inline(always)]
    pub fn is_spi_miso_spi_mosi(&self) -> bool {
        *self == GPIO20FUNCTION_SELECT_A::SPI_MISO_SPI_MOSI
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == GPIO20FUNCTION_SELECT_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG4`"]
    #[inline(always)]
    pub fn is_uart_sig4(&self) -> bool {
        *self == GPIO20FUNCTION_SELECT_A::UART_SIG4
    }
    #[doc = "Checks if the value of the field is `PWM_CH0`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        *self == GPIO20FUNCTION_SELECT_A::PWM_CH0
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_0`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        *self == GPIO20FUNCTION_SELECT_A::FEM_GPIO_0
    }
    #[doc = "Checks if the value of the field is `SWGPIO_20`"]
    #[inline(always)]
    pub fn is_swgpio_20(&self) -> bool {
        *self == GPIO20FUNCTION_SELECT_A::SWGPIO_20
    }
    #[doc = "Checks if the value of the field is `E21_TMS`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        *self == GPIO20FUNCTION_SELECT_A::E21_TMS
    }
}
#[doc = "Field `reg_gpio_20_func_sel` writer - Function select for GPIO20."]
pub type REG_GPIO_20_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL10_SPEC, u8, GPIO20FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_20_FUNC_SEL_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d0(self) -> &'a mut W {
        self.variant(GPIO20FUNCTION_SELECT_A::SF_D0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_miso_spi_mosi(self) -> &'a mut W {
        self.variant(GPIO20FUNCTION_SELECT_A::SPI_MISO_SPI_MOSI)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(GPIO20FUNCTION_SELECT_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig4(self) -> &'a mut W {
        self.variant(GPIO20FUNCTION_SELECT_A::UART_SIG4)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut W {
        self.variant(GPIO20FUNCTION_SELECT_A::PWM_CH0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut W {
        self.variant(GPIO20FUNCTION_SELECT_A::FEM_GPIO_0)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_20(self) -> &'a mut W {
        self.variant(GPIO20FUNCTION_SELECT_A::SWGPIO_20)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut W {
        self.variant(GPIO20FUNCTION_SELECT_A::E21_TMS)
    }
}
#[doc = "Field `reg_gpio_21_ie` reader - Input enable for GPIO21."]
pub type REG_GPIO_21_IE_R = crate::BitReader<GPIO21INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO21.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO21INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO21INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_21_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21INPUT_ENABLED_A {
        match self.bits {
            false => GPIO21INPUT_ENABLED_A::DISABLED,
            true => GPIO21INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO21INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO21INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_21_ie` writer - Input enable for GPIO21."]
pub type REG_GPIO_21_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL10_SPEC, GPIO21INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_21_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO21INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO21INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_21_smt` reader - Schmitt trigger enabled for GPIO21."]
pub type REG_GPIO_21_SMT_R = crate::BitReader<GPIO21SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO21.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO21SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO21SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_21_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21SCHMITT_A {
        match self.bits {
            false => GPIO21SCHMITT_A::DISABLED,
            true => GPIO21SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO21SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO21SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_21_smt` writer - Schmitt trigger enabled for GPIO21."]
pub type REG_GPIO_21_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL10_SPEC, GPIO21SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_21_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO21SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO21SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_21_drv` reader - Driving control enabled for GPIO21."]
pub type REG_GPIO_21_DRV_R = crate::FieldReader<u8, GPIO21DRIVING_A>;
#[doc = "Driving control enabled for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO21DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO21DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO21DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_21_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO21DRIVING_A> {
        match self.bits {
            0 => Some(GPIO21DRIVING_A::DISABLED),
            1 => Some(GPIO21DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO21DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO21DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_21_drv` writer - Driving control enabled for GPIO21."]
pub type REG_GPIO_21_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL10_SPEC, u8, GPIO21DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_21_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO21DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO21DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_21_pu` reader - Pull Up Resistor for GPIO21."]
pub type REG_GPIO_21_PU_R = crate::BitReader<GPIO21PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO21PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO21PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_21_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO21PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO21PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO21PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO21PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_21_pu` writer - Pull Up Resistor for GPIO21."]
pub type REG_GPIO_21_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL10_SPEC, GPIO21PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_21_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO21PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO21PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_21_pd` reader - Pull Down Resistor for GPIO21."]
pub type REG_GPIO_21_PD_R = crate::BitReader<GPIO21PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO21PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO21PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_21_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO21PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO21PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO21PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO21PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_21_pd` writer - Pull Down Resistor for GPIO21."]
pub type REG_GPIO_21_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL10_SPEC, GPIO21PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_21_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO21PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO21PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_21_func_sel` reader - Function select for GPIO21."]
pub type REG_GPIO_21_FUNC_SEL_R = crate::FieldReader<u8, GPIO21FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO21.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO21FUNCTION_SELECT_A {
    #[doc = "2: `10`"]
    SF_CS = 2,
    #[doc = "4: `100`"]
    SPI_MOSI_SPI_MISO = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG5 = 7,
    #[doc = "8: `1000`"]
    PWM_CH1 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_1 = 9,
    #[doc = "11: `1011`"]
    SWGPIO_21 = 11,
    #[doc = "14: `1110`"]
    E21_TDI = 14,
}
impl From<GPIO21FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO21FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_21_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO21FUNCTION_SELECT_A> {
        match self.bits {
            2 => Some(GPIO21FUNCTION_SELECT_A::SF_CS),
            4 => Some(GPIO21FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO),
            6 => Some(GPIO21FUNCTION_SELECT_A::I2C_SDA),
            7 => Some(GPIO21FUNCTION_SELECT_A::UART_SIG5),
            8 => Some(GPIO21FUNCTION_SELECT_A::PWM_CH1),
            9 => Some(GPIO21FUNCTION_SELECT_A::FEM_GPIO_1),
            11 => Some(GPIO21FUNCTION_SELECT_A::SWGPIO_21),
            14 => Some(GPIO21FUNCTION_SELECT_A::E21_TDI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SF_CS`"]
    #[inline(always)]
    pub fn is_sf_cs(&self) -> bool {
        *self == GPIO21FUNCTION_SELECT_A::SF_CS
    }
    #[doc = "Checks if the value of the field is `SPI_MOSI_SPI_MISO`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == GPIO21FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == GPIO21FUNCTION_SELECT_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG5`"]
    #[inline(always)]
    pub fn is_uart_sig5(&self) -> bool {
        *self == GPIO21FUNCTION_SELECT_A::UART_SIG5
    }
    #[doc = "Checks if the value of the field is `PWM_CH1`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        *self == GPIO21FUNCTION_SELECT_A::PWM_CH1
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_1`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        *self == GPIO21FUNCTION_SELECT_A::FEM_GPIO_1
    }
    #[doc = "Checks if the value of the field is `SWGPIO_21`"]
    #[inline(always)]
    pub fn is_swgpio_21(&self) -> bool {
        *self == GPIO21FUNCTION_SELECT_A::SWGPIO_21
    }
    #[doc = "Checks if the value of the field is `E21_TDI`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        *self == GPIO21FUNCTION_SELECT_A::E21_TDI
    }
}
#[doc = "Field `reg_gpio_21_func_sel` writer - Function select for GPIO21."]
pub type REG_GPIO_21_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL10_SPEC, u8, GPIO21FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_21_FUNC_SEL_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_cs(self) -> &'a mut W {
        self.variant(GPIO21FUNCTION_SELECT_A::SF_CS)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut W {
        self.variant(GPIO21FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(GPIO21FUNCTION_SELECT_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig5(self) -> &'a mut W {
        self.variant(GPIO21FUNCTION_SELECT_A::UART_SIG5)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut W {
        self.variant(GPIO21FUNCTION_SELECT_A::PWM_CH1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut W {
        self.variant(GPIO21FUNCTION_SELECT_A::FEM_GPIO_1)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_21(self) -> &'a mut W {
        self.variant(GPIO21FUNCTION_SELECT_A::SWGPIO_21)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut W {
        self.variant(GPIO21FUNCTION_SELECT_A::E21_TDI)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_ie(&self) -> REG_GPIO_20_IE_R {
        REG_GPIO_20_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_smt(&self) -> REG_GPIO_20_SMT_R {
        REG_GPIO_20_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_drv(&self) -> REG_GPIO_20_DRV_R {
        REG_GPIO_20_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_pu(&self) -> REG_GPIO_20_PU_R {
        REG_GPIO_20_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_pd(&self) -> REG_GPIO_20_PD_R {
        REG_GPIO_20_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_func_sel(&self) -> REG_GPIO_20_FUNC_SEL_R {
        REG_GPIO_20_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_ie(&self) -> REG_GPIO_21_IE_R {
        REG_GPIO_21_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_smt(&self) -> REG_GPIO_21_SMT_R {
        REG_GPIO_21_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_drv(&self) -> REG_GPIO_21_DRV_R {
        REG_GPIO_21_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_pu(&self) -> REG_GPIO_21_PU_R {
        REG_GPIO_21_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_pd(&self) -> REG_GPIO_21_PD_R {
        REG_GPIO_21_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_func_sel(&self) -> REG_GPIO_21_FUNC_SEL_R {
        REG_GPIO_21_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_ie(&mut self) -> REG_GPIO_20_IE_W<0> {
        REG_GPIO_20_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_smt(&mut self) -> REG_GPIO_20_SMT_W<1> {
        REG_GPIO_20_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_drv(&mut self) -> REG_GPIO_20_DRV_W<2> {
        REG_GPIO_20_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_pu(&mut self) -> REG_GPIO_20_PU_W<4> {
        REG_GPIO_20_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_pd(&mut self) -> REG_GPIO_20_PD_W<5> {
        REG_GPIO_20_PD_W::new(self)
    }
    #[doc = "Bits 8:11 - Function select for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_func_sel(&mut self) -> REG_GPIO_20_FUNC_SEL_W<8> {
        REG_GPIO_20_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_ie(&mut self) -> REG_GPIO_21_IE_W<16> {
        REG_GPIO_21_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_smt(&mut self) -> REG_GPIO_21_SMT_W<17> {
        REG_GPIO_21_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_drv(&mut self) -> REG_GPIO_21_DRV_W<18> {
        REG_GPIO_21_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_pu(&mut self) -> REG_GPIO_21_PU_W<20> {
        REG_GPIO_21_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_pd(&mut self) -> REG_GPIO_21_PD_W<21> {
        REG_GPIO_21_PD_W::new(self)
    }
    #[doc = "Bits 24:27 - Function select for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_func_sel(&mut self) -> REG_GPIO_21_FUNC_SEL_W<24> {
        REG_GPIO_21_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO20, GPIO21 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl10](index.html) module"]
pub struct GPIO_CFGCTL10_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl10::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl10::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL10 to value 0x0b03_0b03"]
impl crate::Resettable for GPIO_CFGCTL10_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b03_0b03;
}

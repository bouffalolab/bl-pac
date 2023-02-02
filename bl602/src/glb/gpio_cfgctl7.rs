#[doc = "Register `GPIO_CFGCTL7` reader"]
pub struct R(crate::R<GPIO_CFGCTL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL7` writer"]
pub struct W(crate::W<GPIO_CFGCTL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL7_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_14_ie` reader - Input enable for GPIO14."]
pub type REG_GPIO_14_IE_R = crate::BitReader<GPIO14INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO14.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO14INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO14INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_14_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14INPUT_ENABLED_A {
        match self.bits {
            false => GPIO14INPUT_ENABLED_A::DISABLED,
            true => GPIO14INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO14INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO14INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_14_ie` writer - Input enable for GPIO14."]
pub type REG_GPIO_14_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, GPIO14INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_14_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO14INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO14INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_14_smt` reader - Schmitt trigger enabled for GPIO14."]
pub type REG_GPIO_14_SMT_R = crate::BitReader<GPIO14SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO14.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO14SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO14SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_14_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14SCHMITT_A {
        match self.bits {
            false => GPIO14SCHMITT_A::DISABLED,
            true => GPIO14SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO14SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO14SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_14_smt` writer - Schmitt trigger enabled for GPIO14."]
pub type REG_GPIO_14_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, GPIO14SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_14_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO14SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO14SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_14_drv` reader - Driving control enabled for GPIO14."]
pub type REG_GPIO_14_DRV_R = crate::FieldReader<u8, GPIO14DRIVING_A>;
#[doc = "Driving control enabled for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO14DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO14DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO14DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_14_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO14DRIVING_A> {
        match self.bits {
            0 => Some(GPIO14DRIVING_A::DISABLED),
            1 => Some(GPIO14DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO14DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO14DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_14_drv` writer - Driving control enabled for GPIO14."]
pub type REG_GPIO_14_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL7_SPEC, u8, GPIO14DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_14_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO14DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO14DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_14_pu` reader - Pull Up Resistor for GPIO14."]
pub type REG_GPIO_14_PU_R = crate::BitReader<GPIO14PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO14PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO14PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_14_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO14PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO14PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO14PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO14PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_14_pu` writer - Pull Up Resistor for GPIO14."]
pub type REG_GPIO_14_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, GPIO14PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_14_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO14PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO14PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_14_pd` reader - Pull Down Resistor for GPIO14."]
pub type REG_GPIO_14_PD_R = crate::BitReader<GPIO14PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO14PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO14PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_14_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO14PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO14PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO14PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO14PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_14_pd` writer - Pull Down Resistor for GPIO14."]
pub type REG_GPIO_14_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, GPIO14PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_14_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO14PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO14PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_14_func_sel` reader - Function select for GPIO14."]
pub type REG_GPIO_14_FUNC_SEL_R = crate::FieldReader<u8, GPIO14FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO14.\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO14FUNCTION_SELECT_A {
    #[doc = "4: `100`"]
    SPI_SS = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG6 = 7,
    #[doc = "8: `1000`"]
    PWM_CH4 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_2 = 9,
    #[doc = "10: `1010`"]
    GPIP_CH2 = 10,
    #[doc = "11: `1011`"]
    SWGPIO_14 = 11,
    #[doc = "14: `1110`"]
    E21_TCK = 14,
}
impl From<GPIO14FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO14FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_14_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO14FUNCTION_SELECT_A> {
        match self.bits {
            4 => Some(GPIO14FUNCTION_SELECT_A::SPI_SS),
            6 => Some(GPIO14FUNCTION_SELECT_A::I2C_SCL),
            7 => Some(GPIO14FUNCTION_SELECT_A::UART_SIG6),
            8 => Some(GPIO14FUNCTION_SELECT_A::PWM_CH4),
            9 => Some(GPIO14FUNCTION_SELECT_A::FEM_GPIO_2),
            10 => Some(GPIO14FUNCTION_SELECT_A::GPIP_CH2),
            11 => Some(GPIO14FUNCTION_SELECT_A::SWGPIO_14),
            14 => Some(GPIO14FUNCTION_SELECT_A::E21_TCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_SS`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == GPIO14FUNCTION_SELECT_A::SPI_SS
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == GPIO14FUNCTION_SELECT_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG6`"]
    #[inline(always)]
    pub fn is_uart_sig6(&self) -> bool {
        *self == GPIO14FUNCTION_SELECT_A::UART_SIG6
    }
    #[doc = "Checks if the value of the field is `PWM_CH4`"]
    #[inline(always)]
    pub fn is_pwm_ch4(&self) -> bool {
        *self == GPIO14FUNCTION_SELECT_A::PWM_CH4
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_2`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == GPIO14FUNCTION_SELECT_A::FEM_GPIO_2
    }
    #[doc = "Checks if the value of the field is `GPIP_CH2`"]
    #[inline(always)]
    pub fn is_gpip_ch2(&self) -> bool {
        *self == GPIO14FUNCTION_SELECT_A::GPIP_CH2
    }
    #[doc = "Checks if the value of the field is `SWGPIO_14`"]
    #[inline(always)]
    pub fn is_swgpio_14(&self) -> bool {
        *self == GPIO14FUNCTION_SELECT_A::SWGPIO_14
    }
    #[doc = "Checks if the value of the field is `E21_TCK`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == GPIO14FUNCTION_SELECT_A::E21_TCK
    }
}
#[doc = "Field `reg_gpio_14_func_sel` writer - Function select for GPIO14."]
pub type REG_GPIO_14_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL7_SPEC, u8, GPIO14FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_14_FUNC_SEL_W<'a, O> {
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut W {
        self.variant(GPIO14FUNCTION_SELECT_A::SPI_SS)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(GPIO14FUNCTION_SELECT_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig6(self) -> &'a mut W {
        self.variant(GPIO14FUNCTION_SELECT_A::UART_SIG6)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch4(self) -> &'a mut W {
        self.variant(GPIO14FUNCTION_SELECT_A::PWM_CH4)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut W {
        self.variant(GPIO14FUNCTION_SELECT_A::FEM_GPIO_2)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch2(self) -> &'a mut W {
        self.variant(GPIO14FUNCTION_SELECT_A::GPIP_CH2)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_14(self) -> &'a mut W {
        self.variant(GPIO14FUNCTION_SELECT_A::SWGPIO_14)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut W {
        self.variant(GPIO14FUNCTION_SELECT_A::E21_TCK)
    }
}
#[doc = "Field `reg_gpio_15_ie` reader - Input enable for GPIO15."]
pub type REG_GPIO_15_IE_R = crate::BitReader<GPIO15INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO15.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO15INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO15INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_15_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15INPUT_ENABLED_A {
        match self.bits {
            false => GPIO15INPUT_ENABLED_A::DISABLED,
            true => GPIO15INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO15INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO15INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_15_ie` writer - Input enable for GPIO15."]
pub type REG_GPIO_15_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, GPIO15INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_15_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO15INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO15INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_15_smt` reader - Schmitt trigger enabled for GPIO15."]
pub type REG_GPIO_15_SMT_R = crate::BitReader<GPIO15SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO15.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO15SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO15SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_15_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15SCHMITT_A {
        match self.bits {
            false => GPIO15SCHMITT_A::DISABLED,
            true => GPIO15SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO15SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO15SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_15_smt` writer - Schmitt trigger enabled for GPIO15."]
pub type REG_GPIO_15_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, GPIO15SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_15_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO15SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO15SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_15_drv` reader - Driving control enabled for GPIO15."]
pub type REG_GPIO_15_DRV_R = crate::FieldReader<u8, GPIO15DRIVING_A>;
#[doc = "Driving control enabled for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO15DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO15DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO15DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_15_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO15DRIVING_A> {
        match self.bits {
            0 => Some(GPIO15DRIVING_A::DISABLED),
            1 => Some(GPIO15DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO15DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO15DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_15_drv` writer - Driving control enabled for GPIO15."]
pub type REG_GPIO_15_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL7_SPEC, u8, GPIO15DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_15_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO15DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO15DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_15_pu` reader - Pull Up Resistor for GPIO15."]
pub type REG_GPIO_15_PU_R = crate::BitReader<GPIO15PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO15PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO15PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_15_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO15PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO15PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO15PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO15PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_15_pu` writer - Pull Up Resistor for GPIO15."]
pub type REG_GPIO_15_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, GPIO15PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_15_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO15PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO15PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_15_pd` reader - Pull Down Resistor for GPIO15."]
pub type REG_GPIO_15_PD_R = crate::BitReader<GPIO15PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO15PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO15PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_15_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO15PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO15PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO15PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO15PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_15_pd` writer - Pull Down Resistor for GPIO15."]
pub type REG_GPIO_15_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, GPIO15PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_15_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO15PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO15PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_15_func_sel` reader - Function select for GPIO15."]
pub type REG_GPIO_15_FUNC_SEL_R = crate::FieldReader<u8, GPIO15FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO15.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO15FUNCTION_SELECT_A {
    #[doc = "4: `100`"]
    SPI_SCLK = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG7 = 7,
    #[doc = "8: `1000`"]
    PWM_CH0 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_3 = 9,
    #[doc = "10: `1010`"]
    PSW_IRRCV_OUT_GPIP_CH11 = 10,
    #[doc = "11: `1011`"]
    SWGPIO_15 = 11,
    #[doc = "14: `1110`"]
    E21_TDO = 14,
}
impl From<GPIO15FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO15FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_15_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO15FUNCTION_SELECT_A> {
        match self.bits {
            4 => Some(GPIO15FUNCTION_SELECT_A::SPI_SCLK),
            6 => Some(GPIO15FUNCTION_SELECT_A::I2C_SDA),
            7 => Some(GPIO15FUNCTION_SELECT_A::UART_SIG7),
            8 => Some(GPIO15FUNCTION_SELECT_A::PWM_CH0),
            9 => Some(GPIO15FUNCTION_SELECT_A::FEM_GPIO_3),
            10 => Some(GPIO15FUNCTION_SELECT_A::PSW_IRRCV_OUT_GPIP_CH11),
            11 => Some(GPIO15FUNCTION_SELECT_A::SWGPIO_15),
            14 => Some(GPIO15FUNCTION_SELECT_A::E21_TDO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_SCLK`"]
    #[inline(always)]
    pub fn is_spi_sclk(&self) -> bool {
        *self == GPIO15FUNCTION_SELECT_A::SPI_SCLK
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == GPIO15FUNCTION_SELECT_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG7`"]
    #[inline(always)]
    pub fn is_uart_sig7(&self) -> bool {
        *self == GPIO15FUNCTION_SELECT_A::UART_SIG7
    }
    #[doc = "Checks if the value of the field is `PWM_CH0`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        *self == GPIO15FUNCTION_SELECT_A::PWM_CH0
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_3`"]
    #[inline(always)]
    pub fn is_fem_gpio_3(&self) -> bool {
        *self == GPIO15FUNCTION_SELECT_A::FEM_GPIO_3
    }
    #[doc = "Checks if the value of the field is `PSW_IRRCV_OUT_GPIP_CH11`"]
    #[inline(always)]
    pub fn is_psw_irrcv_out_gpip_ch11(&self) -> bool {
        *self == GPIO15FUNCTION_SELECT_A::PSW_IRRCV_OUT_GPIP_CH11
    }
    #[doc = "Checks if the value of the field is `SWGPIO_15`"]
    #[inline(always)]
    pub fn is_swgpio_15(&self) -> bool {
        *self == GPIO15FUNCTION_SELECT_A::SWGPIO_15
    }
    #[doc = "Checks if the value of the field is `E21_TDO`"]
    #[inline(always)]
    pub fn is_e21_tdo(&self) -> bool {
        *self == GPIO15FUNCTION_SELECT_A::E21_TDO
    }
}
#[doc = "Field `reg_gpio_15_func_sel` writer - Function select for GPIO15."]
pub type REG_GPIO_15_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL7_SPEC, u8, GPIO15FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_15_FUNC_SEL_W<'a, O> {
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_sclk(self) -> &'a mut W {
        self.variant(GPIO15FUNCTION_SELECT_A::SPI_SCLK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(GPIO15FUNCTION_SELECT_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig7(self) -> &'a mut W {
        self.variant(GPIO15FUNCTION_SELECT_A::UART_SIG7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut W {
        self.variant(GPIO15FUNCTION_SELECT_A::PWM_CH0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_3(self) -> &'a mut W {
        self.variant(GPIO15FUNCTION_SELECT_A::FEM_GPIO_3)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn psw_irrcv_out_gpip_ch11(self) -> &'a mut W {
        self.variant(GPIO15FUNCTION_SELECT_A::PSW_IRRCV_OUT_GPIP_CH11)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_15(self) -> &'a mut W {
        self.variant(GPIO15FUNCTION_SELECT_A::SWGPIO_15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdo(self) -> &'a mut W {
        self.variant(GPIO15FUNCTION_SELECT_A::E21_TDO)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_ie(&self) -> REG_GPIO_14_IE_R {
        REG_GPIO_14_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_smt(&self) -> REG_GPIO_14_SMT_R {
        REG_GPIO_14_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_drv(&self) -> REG_GPIO_14_DRV_R {
        REG_GPIO_14_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_pu(&self) -> REG_GPIO_14_PU_R {
        REG_GPIO_14_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_pd(&self) -> REG_GPIO_14_PD_R {
        REG_GPIO_14_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_func_sel(&self) -> REG_GPIO_14_FUNC_SEL_R {
        REG_GPIO_14_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_ie(&self) -> REG_GPIO_15_IE_R {
        REG_GPIO_15_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_smt(&self) -> REG_GPIO_15_SMT_R {
        REG_GPIO_15_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_drv(&self) -> REG_GPIO_15_DRV_R {
        REG_GPIO_15_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_pu(&self) -> REG_GPIO_15_PU_R {
        REG_GPIO_15_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_pd(&self) -> REG_GPIO_15_PD_R {
        REG_GPIO_15_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_func_sel(&self) -> REG_GPIO_15_FUNC_SEL_R {
        REG_GPIO_15_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_ie(&mut self) -> REG_GPIO_14_IE_W<0> {
        REG_GPIO_14_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_smt(&mut self) -> REG_GPIO_14_SMT_W<1> {
        REG_GPIO_14_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_drv(&mut self) -> REG_GPIO_14_DRV_W<2> {
        REG_GPIO_14_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_pu(&mut self) -> REG_GPIO_14_PU_W<4> {
        REG_GPIO_14_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_pd(&mut self) -> REG_GPIO_14_PD_W<5> {
        REG_GPIO_14_PD_W::new(self)
    }
    #[doc = "Bits 8:11 - Function select for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_func_sel(&mut self) -> REG_GPIO_14_FUNC_SEL_W<8> {
        REG_GPIO_14_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_ie(&mut self) -> REG_GPIO_15_IE_W<16> {
        REG_GPIO_15_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_smt(&mut self) -> REG_GPIO_15_SMT_W<17> {
        REG_GPIO_15_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_drv(&mut self) -> REG_GPIO_15_DRV_W<18> {
        REG_GPIO_15_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_pu(&mut self) -> REG_GPIO_15_PU_W<20> {
        REG_GPIO_15_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_pd(&mut self) -> REG_GPIO_15_PD_W<21> {
        REG_GPIO_15_PD_W::new(self)
    }
    #[doc = "Bits 24:27 - Function select for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_func_sel(&mut self) -> REG_GPIO_15_FUNC_SEL_W<24> {
        REG_GPIO_15_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO14, GPIO15 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl7](index.html) module"]
pub struct GPIO_CFGCTL7_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl7::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl7::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL7 to value 0x0b03_0e03"]
impl crate::Resettable for GPIO_CFGCTL7_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b03_0e03;
}

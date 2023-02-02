#[doc = "Register `GPIO_CFGCTL3` reader"]
pub struct R(crate::R<GPIO_CFGCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL3` writer"]
pub struct W(crate::W<GPIO_CFGCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL3_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_6_ie` reader - Input enable for GPIO6."]
pub type REG_GPIO_6_IE_R = crate::BitReader<GPIO6INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO6.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO6INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO6INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_6_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6INPUT_ENABLED_A {
        match self.bits {
            false => GPIO6INPUT_ENABLED_A::DISABLED,
            true => GPIO6INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO6INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO6INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_6_ie` writer - Input enable for GPIO6."]
pub type REG_GPIO_6_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL3_SPEC, GPIO6INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_6_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO6INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO6INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_6_smt` reader - Schmitt trigger enabled for GPIO6."]
pub type REG_GPIO_6_SMT_R = crate::BitReader<GPIO6SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO6.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO6SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO6SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_6_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6SCHMITT_A {
        match self.bits {
            false => GPIO6SCHMITT_A::DISABLED,
            true => GPIO6SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO6SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO6SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_6_smt` writer - Schmitt trigger enabled for GPIO6."]
pub type REG_GPIO_6_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL3_SPEC, GPIO6SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_6_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO6SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO6SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_6_drv` reader - Driving control enabled for GPIO6."]
pub type REG_GPIO_6_DRV_R = crate::FieldReader<u8, GPIO6DRIVING_A>;
#[doc = "Driving control enabled for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO6DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO6DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO6DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_6_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO6DRIVING_A> {
        match self.bits {
            0 => Some(GPIO6DRIVING_A::DISABLED),
            1 => Some(GPIO6DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO6DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO6DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_6_drv` writer - Driving control enabled for GPIO6."]
pub type REG_GPIO_6_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL3_SPEC, u8, GPIO6DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_6_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO6DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO6DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_6_pu` reader - Pull Up Resistor for GPIO6."]
pub type REG_GPIO_6_PU_R = crate::BitReader<GPIO6PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO6PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO6PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_6_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO6PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO6PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO6PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO6PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_6_pu` writer - Pull Up Resistor for GPIO6."]
pub type REG_GPIO_6_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL3_SPEC, GPIO6PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_6_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO6PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO6PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_6_pd` reader - Pull Down Resistor for GPIO6."]
pub type REG_GPIO_6_PD_R = crate::BitReader<GPIO6PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO6PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO6PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_6_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO6PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO6PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO6PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO6PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_6_pd` writer - Pull Down Resistor for GPIO6."]
pub type REG_GPIO_6_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL3_SPEC, GPIO6PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_6_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO6PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO6PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_6_func_sel` reader - Function select for GPIO6."]
pub type REG_GPIO_6_FUNC_SEL_R = crate::FieldReader<u8, GPIO6FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO6.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO6FUNCTION_SELECT_A {
    #[doc = "4: `100`"]
    SPI_SS = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG6 = 7,
    #[doc = "8: `1000`"]
    PWM_CH1 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_2 = 9,
    #[doc = "10: `1010`"]
    GPIP_CH5 = 10,
    #[doc = "11: `1011`"]
    SWGPIO_6 = 11,
    #[doc = "14: `1110`"]
    E21_TCK = 14,
}
impl From<GPIO6FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO6FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_6_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO6FUNCTION_SELECT_A> {
        match self.bits {
            4 => Some(GPIO6FUNCTION_SELECT_A::SPI_SS),
            6 => Some(GPIO6FUNCTION_SELECT_A::I2C_SCL),
            7 => Some(GPIO6FUNCTION_SELECT_A::UART_SIG6),
            8 => Some(GPIO6FUNCTION_SELECT_A::PWM_CH1),
            9 => Some(GPIO6FUNCTION_SELECT_A::FEM_GPIO_2),
            10 => Some(GPIO6FUNCTION_SELECT_A::GPIP_CH5),
            11 => Some(GPIO6FUNCTION_SELECT_A::SWGPIO_6),
            14 => Some(GPIO6FUNCTION_SELECT_A::E21_TCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_SS`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == GPIO6FUNCTION_SELECT_A::SPI_SS
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == GPIO6FUNCTION_SELECT_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG6`"]
    #[inline(always)]
    pub fn is_uart_sig6(&self) -> bool {
        *self == GPIO6FUNCTION_SELECT_A::UART_SIG6
    }
    #[doc = "Checks if the value of the field is `PWM_CH1`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        *self == GPIO6FUNCTION_SELECT_A::PWM_CH1
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_2`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == GPIO6FUNCTION_SELECT_A::FEM_GPIO_2
    }
    #[doc = "Checks if the value of the field is `GPIP_CH5`"]
    #[inline(always)]
    pub fn is_gpip_ch5(&self) -> bool {
        *self == GPIO6FUNCTION_SELECT_A::GPIP_CH5
    }
    #[doc = "Checks if the value of the field is `SWGPIO_6`"]
    #[inline(always)]
    pub fn is_swgpio_6(&self) -> bool {
        *self == GPIO6FUNCTION_SELECT_A::SWGPIO_6
    }
    #[doc = "Checks if the value of the field is `E21_TCK`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == GPIO6FUNCTION_SELECT_A::E21_TCK
    }
}
#[doc = "Field `reg_gpio_6_func_sel` writer - Function select for GPIO6."]
pub type REG_GPIO_6_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL3_SPEC, u8, GPIO6FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_6_FUNC_SEL_W<'a, O> {
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut W {
        self.variant(GPIO6FUNCTION_SELECT_A::SPI_SS)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(GPIO6FUNCTION_SELECT_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig6(self) -> &'a mut W {
        self.variant(GPIO6FUNCTION_SELECT_A::UART_SIG6)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut W {
        self.variant(GPIO6FUNCTION_SELECT_A::PWM_CH1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut W {
        self.variant(GPIO6FUNCTION_SELECT_A::FEM_GPIO_2)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch5(self) -> &'a mut W {
        self.variant(GPIO6FUNCTION_SELECT_A::GPIP_CH5)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_6(self) -> &'a mut W {
        self.variant(GPIO6FUNCTION_SELECT_A::SWGPIO_6)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut W {
        self.variant(GPIO6FUNCTION_SELECT_A::E21_TCK)
    }
}
#[doc = "Field `reg_gpio_7_ie` reader - Input enable for GPIO7."]
pub type REG_GPIO_7_IE_R = crate::BitReader<GPIO7INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO7INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO7INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_7_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7INPUT_ENABLED_A {
        match self.bits {
            false => GPIO7INPUT_ENABLED_A::DISABLED,
            true => GPIO7INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO7INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO7INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_7_ie` writer - Input enable for GPIO7."]
pub type REG_GPIO_7_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL3_SPEC, GPIO7INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_7_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO7INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO7INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_7_smt` reader - Schmitt trigger enabled for GPIO7."]
pub type REG_GPIO_7_SMT_R = crate::BitReader<GPIO7SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO7SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO7SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_7_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7SCHMITT_A {
        match self.bits {
            false => GPIO7SCHMITT_A::DISABLED,
            true => GPIO7SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO7SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO7SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_7_smt` writer - Schmitt trigger enabled for GPIO7."]
pub type REG_GPIO_7_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL3_SPEC, GPIO7SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_7_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO7SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO7SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_7_drv` reader - Driving control enabled for GPIO7."]
pub type REG_GPIO_7_DRV_R = crate::FieldReader<u8, GPIO7DRIVING_A>;
#[doc = "Driving control enabled for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO7DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO7DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO7DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_7_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO7DRIVING_A> {
        match self.bits {
            0 => Some(GPIO7DRIVING_A::DISABLED),
            1 => Some(GPIO7DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO7DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO7DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_7_drv` writer - Driving control enabled for GPIO7."]
pub type REG_GPIO_7_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL3_SPEC, u8, GPIO7DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_7_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO7DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO7DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_7_pu` reader - Pull Up Resistor for GPIO7."]
pub type REG_GPIO_7_PU_R = crate::BitReader<GPIO7PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO7PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO7PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_7_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO7PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO7PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO7PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO7PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_7_pu` writer - Pull Up Resistor for GPIO7."]
pub type REG_GPIO_7_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL3_SPEC, GPIO7PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_7_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO7PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO7PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_7_pd` reader - Pull Down Resistor for GPIO7."]
pub type REG_GPIO_7_PD_R = crate::BitReader<GPIO7PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO7PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO7PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_7_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO7PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO7PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO7PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO7PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_7_pd` writer - Pull Down Resistor for GPIO7."]
pub type REG_GPIO_7_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL3_SPEC, GPIO7PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_7_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO7PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO7PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_7_func_sel` reader - Function select for GPIO7."]
pub type REG_GPIO_7_FUNC_SEL_R = crate::FieldReader<u8, GPIO7FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO7.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO7FUNCTION_SELECT_A {
    #[doc = "4: `100`"]
    SPI_SCLK = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG7 = 7,
    #[doc = "8: `1000`"]
    PWM_CH2 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_3 = 9,
    #[doc = "11: `1011`"]
    SWGPIO_7 = 11,
    #[doc = "14: `1110`"]
    E21_TDO = 14,
}
impl From<GPIO7FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO7FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_7_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO7FUNCTION_SELECT_A> {
        match self.bits {
            4 => Some(GPIO7FUNCTION_SELECT_A::SPI_SCLK),
            6 => Some(GPIO7FUNCTION_SELECT_A::I2C_SDA),
            7 => Some(GPIO7FUNCTION_SELECT_A::UART_SIG7),
            8 => Some(GPIO7FUNCTION_SELECT_A::PWM_CH2),
            9 => Some(GPIO7FUNCTION_SELECT_A::FEM_GPIO_3),
            11 => Some(GPIO7FUNCTION_SELECT_A::SWGPIO_7),
            14 => Some(GPIO7FUNCTION_SELECT_A::E21_TDO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_SCLK`"]
    #[inline(always)]
    pub fn is_spi_sclk(&self) -> bool {
        *self == GPIO7FUNCTION_SELECT_A::SPI_SCLK
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == GPIO7FUNCTION_SELECT_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG7`"]
    #[inline(always)]
    pub fn is_uart_sig7(&self) -> bool {
        *self == GPIO7FUNCTION_SELECT_A::UART_SIG7
    }
    #[doc = "Checks if the value of the field is `PWM_CH2`"]
    #[inline(always)]
    pub fn is_pwm_ch2(&self) -> bool {
        *self == GPIO7FUNCTION_SELECT_A::PWM_CH2
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_3`"]
    #[inline(always)]
    pub fn is_fem_gpio_3(&self) -> bool {
        *self == GPIO7FUNCTION_SELECT_A::FEM_GPIO_3
    }
    #[doc = "Checks if the value of the field is `SWGPIO_7`"]
    #[inline(always)]
    pub fn is_swgpio_7(&self) -> bool {
        *self == GPIO7FUNCTION_SELECT_A::SWGPIO_7
    }
    #[doc = "Checks if the value of the field is `E21_TDO`"]
    #[inline(always)]
    pub fn is_e21_tdo(&self) -> bool {
        *self == GPIO7FUNCTION_SELECT_A::E21_TDO
    }
}
#[doc = "Field `reg_gpio_7_func_sel` writer - Function select for GPIO7."]
pub type REG_GPIO_7_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL3_SPEC, u8, GPIO7FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_7_FUNC_SEL_W<'a, O> {
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_sclk(self) -> &'a mut W {
        self.variant(GPIO7FUNCTION_SELECT_A::SPI_SCLK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(GPIO7FUNCTION_SELECT_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig7(self) -> &'a mut W {
        self.variant(GPIO7FUNCTION_SELECT_A::UART_SIG7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch2(self) -> &'a mut W {
        self.variant(GPIO7FUNCTION_SELECT_A::PWM_CH2)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_3(self) -> &'a mut W {
        self.variant(GPIO7FUNCTION_SELECT_A::FEM_GPIO_3)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_7(self) -> &'a mut W {
        self.variant(GPIO7FUNCTION_SELECT_A::SWGPIO_7)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdo(self) -> &'a mut W {
        self.variant(GPIO7FUNCTION_SELECT_A::E21_TDO)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_ie(&self) -> REG_GPIO_6_IE_R {
        REG_GPIO_6_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_smt(&self) -> REG_GPIO_6_SMT_R {
        REG_GPIO_6_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_drv(&self) -> REG_GPIO_6_DRV_R {
        REG_GPIO_6_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_pu(&self) -> REG_GPIO_6_PU_R {
        REG_GPIO_6_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_pd(&self) -> REG_GPIO_6_PD_R {
        REG_GPIO_6_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_func_sel(&self) -> REG_GPIO_6_FUNC_SEL_R {
        REG_GPIO_6_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_ie(&self) -> REG_GPIO_7_IE_R {
        REG_GPIO_7_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_smt(&self) -> REG_GPIO_7_SMT_R {
        REG_GPIO_7_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_drv(&self) -> REG_GPIO_7_DRV_R {
        REG_GPIO_7_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_pu(&self) -> REG_GPIO_7_PU_R {
        REG_GPIO_7_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_pd(&self) -> REG_GPIO_7_PD_R {
        REG_GPIO_7_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_func_sel(&self) -> REG_GPIO_7_FUNC_SEL_R {
        REG_GPIO_7_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_ie(&mut self) -> REG_GPIO_6_IE_W<0> {
        REG_GPIO_6_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_smt(&mut self) -> REG_GPIO_6_SMT_W<1> {
        REG_GPIO_6_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_drv(&mut self) -> REG_GPIO_6_DRV_W<2> {
        REG_GPIO_6_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_pu(&mut self) -> REG_GPIO_6_PU_W<4> {
        REG_GPIO_6_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_pd(&mut self) -> REG_GPIO_6_PD_W<5> {
        REG_GPIO_6_PD_W::new(self)
    }
    #[doc = "Bits 8:11 - Function select for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_func_sel(&mut self) -> REG_GPIO_6_FUNC_SEL_W<8> {
        REG_GPIO_6_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_ie(&mut self) -> REG_GPIO_7_IE_W<16> {
        REG_GPIO_7_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_smt(&mut self) -> REG_GPIO_7_SMT_W<17> {
        REG_GPIO_7_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_drv(&mut self) -> REG_GPIO_7_DRV_W<18> {
        REG_GPIO_7_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_pu(&mut self) -> REG_GPIO_7_PU_W<20> {
        REG_GPIO_7_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_pd(&mut self) -> REG_GPIO_7_PD_W<21> {
        REG_GPIO_7_PD_W::new(self)
    }
    #[doc = "Bits 24:27 - Function select for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_func_sel(&mut self) -> REG_GPIO_7_FUNC_SEL_W<24> {
        REG_GPIO_7_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO6, GPIO7 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl3](index.html) module"]
pub struct GPIO_CFGCTL3_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl3::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl3::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL3 to value 0x0b03_0b03"]
impl crate::Resettable for GPIO_CFGCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b03_0b03;
}

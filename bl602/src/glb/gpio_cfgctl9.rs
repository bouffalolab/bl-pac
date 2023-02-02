#[doc = "Register `GPIO_CFGCTL9` reader"]
pub struct R(crate::R<GPIO_CFGCTL9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL9` writer"]
pub struct W(crate::W<GPIO_CFGCTL9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL9_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_18_ie` reader - Input enable for GPIO18."]
pub type REG_GPIO_18_IE_R = crate::BitReader<GPIO18INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO18.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO18INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO18INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_18_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18INPUT_ENABLED_A {
        match self.bits {
            false => GPIO18INPUT_ENABLED_A::DISABLED,
            true => GPIO18INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO18INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO18INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_18_ie` writer - Input enable for GPIO18."]
pub type REG_GPIO_18_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL9_SPEC, GPIO18INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_18_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO18INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO18INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_18_smt` reader - Schmitt trigger enabled for GPIO18."]
pub type REG_GPIO_18_SMT_R = crate::BitReader<GPIO18SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO18.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO18SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO18SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_18_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18SCHMITT_A {
        match self.bits {
            false => GPIO18SCHMITT_A::DISABLED,
            true => GPIO18SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO18SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO18SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_18_smt` writer - Schmitt trigger enabled for GPIO18."]
pub type REG_GPIO_18_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL9_SPEC, GPIO18SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_18_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO18SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO18SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_18_drv` reader - Driving control enabled for GPIO18."]
pub type REG_GPIO_18_DRV_R = crate::FieldReader<u8, GPIO18DRIVING_A>;
#[doc = "Driving control enabled for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO18DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO18DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO18DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_18_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO18DRIVING_A> {
        match self.bits {
            0 => Some(GPIO18DRIVING_A::DISABLED),
            1 => Some(GPIO18DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO18DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO18DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_18_drv` writer - Driving control enabled for GPIO18."]
pub type REG_GPIO_18_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL9_SPEC, u8, GPIO18DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_18_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO18DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO18DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_18_pu` reader - Pull Up Resistor for GPIO18."]
pub type REG_GPIO_18_PU_R = crate::BitReader<GPIO18PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO18PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO18PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_18_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO18PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO18PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO18PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO18PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_18_pu` writer - Pull Up Resistor for GPIO18."]
pub type REG_GPIO_18_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL9_SPEC, GPIO18PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_18_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO18PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO18PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_18_pd` reader - Pull Down Resistor for GPIO18."]
pub type REG_GPIO_18_PD_R = crate::BitReader<GPIO18PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO18PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO18PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_18_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO18PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO18PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO18PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO18PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_18_pd` writer - Pull Down Resistor for GPIO18."]
pub type REG_GPIO_18_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL9_SPEC, GPIO18PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_18_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO18PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO18PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_18_func_sel` reader - Function select for GPIO18."]
pub type REG_GPIO_18_FUNC_SEL_R = crate::FieldReader<u8, GPIO18FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO18.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO18FUNCTION_SELECT_A {
    #[doc = "2: `10`"]
    SF_D2 = 2,
    #[doc = "4: `100`"]
    SPI_SS = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG2 = 7,
    #[doc = "8: `1000`"]
    PWM_CH3 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_2 = 9,
    #[doc = "11: `1011`"]
    SWGPIO_18 = 11,
    #[doc = "14: `1110`"]
    E21_TCK = 14,
}
impl From<GPIO18FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO18FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_18_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO18FUNCTION_SELECT_A> {
        match self.bits {
            2 => Some(GPIO18FUNCTION_SELECT_A::SF_D2),
            4 => Some(GPIO18FUNCTION_SELECT_A::SPI_SS),
            6 => Some(GPIO18FUNCTION_SELECT_A::I2C_SCL),
            7 => Some(GPIO18FUNCTION_SELECT_A::UART_SIG2),
            8 => Some(GPIO18FUNCTION_SELECT_A::PWM_CH3),
            9 => Some(GPIO18FUNCTION_SELECT_A::FEM_GPIO_2),
            11 => Some(GPIO18FUNCTION_SELECT_A::SWGPIO_18),
            14 => Some(GPIO18FUNCTION_SELECT_A::E21_TCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SF_D2`"]
    #[inline(always)]
    pub fn is_sf_d2(&self) -> bool {
        *self == GPIO18FUNCTION_SELECT_A::SF_D2
    }
    #[doc = "Checks if the value of the field is `SPI_SS`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == GPIO18FUNCTION_SELECT_A::SPI_SS
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == GPIO18FUNCTION_SELECT_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG2`"]
    #[inline(always)]
    pub fn is_uart_sig2(&self) -> bool {
        *self == GPIO18FUNCTION_SELECT_A::UART_SIG2
    }
    #[doc = "Checks if the value of the field is `PWM_CH3`"]
    #[inline(always)]
    pub fn is_pwm_ch3(&self) -> bool {
        *self == GPIO18FUNCTION_SELECT_A::PWM_CH3
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_2`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == GPIO18FUNCTION_SELECT_A::FEM_GPIO_2
    }
    #[doc = "Checks if the value of the field is `SWGPIO_18`"]
    #[inline(always)]
    pub fn is_swgpio_18(&self) -> bool {
        *self == GPIO18FUNCTION_SELECT_A::SWGPIO_18
    }
    #[doc = "Checks if the value of the field is `E21_TCK`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == GPIO18FUNCTION_SELECT_A::E21_TCK
    }
}
#[doc = "Field `reg_gpio_18_func_sel` writer - Function select for GPIO18."]
pub type REG_GPIO_18_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL9_SPEC, u8, GPIO18FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_18_FUNC_SEL_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d2(self) -> &'a mut W {
        self.variant(GPIO18FUNCTION_SELECT_A::SF_D2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut W {
        self.variant(GPIO18FUNCTION_SELECT_A::SPI_SS)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(GPIO18FUNCTION_SELECT_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig2(self) -> &'a mut W {
        self.variant(GPIO18FUNCTION_SELECT_A::UART_SIG2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch3(self) -> &'a mut W {
        self.variant(GPIO18FUNCTION_SELECT_A::PWM_CH3)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut W {
        self.variant(GPIO18FUNCTION_SELECT_A::FEM_GPIO_2)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_18(self) -> &'a mut W {
        self.variant(GPIO18FUNCTION_SELECT_A::SWGPIO_18)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut W {
        self.variant(GPIO18FUNCTION_SELECT_A::E21_TCK)
    }
}
#[doc = "Field `reg_gpio_19_ie` reader - Input enable for GPIO19."]
pub type REG_GPIO_19_IE_R = crate::BitReader<GPIO19INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO19.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO19INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO19INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_19_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19INPUT_ENABLED_A {
        match self.bits {
            false => GPIO19INPUT_ENABLED_A::DISABLED,
            true => GPIO19INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO19INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO19INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_19_ie` writer - Input enable for GPIO19."]
pub type REG_GPIO_19_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL9_SPEC, GPIO19INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_19_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO19INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO19INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_19_smt` reader - Schmitt trigger enabled for GPIO19."]
pub type REG_GPIO_19_SMT_R = crate::BitReader<GPIO19SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO19.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO19SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO19SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_19_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19SCHMITT_A {
        match self.bits {
            false => GPIO19SCHMITT_A::DISABLED,
            true => GPIO19SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO19SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO19SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_19_smt` writer - Schmitt trigger enabled for GPIO19."]
pub type REG_GPIO_19_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL9_SPEC, GPIO19SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_19_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO19SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO19SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_19_drv` reader - Driving control enabled for GPIO19."]
pub type REG_GPIO_19_DRV_R = crate::FieldReader<u8, GPIO19DRIVING_A>;
#[doc = "Driving control enabled for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO19DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO19DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO19DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_19_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO19DRIVING_A> {
        match self.bits {
            0 => Some(GPIO19DRIVING_A::DISABLED),
            1 => Some(GPIO19DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO19DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO19DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_19_drv` writer - Driving control enabled for GPIO19."]
pub type REG_GPIO_19_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL9_SPEC, u8, GPIO19DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_19_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO19DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO19DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_19_pu` reader - Pull Up Resistor for GPIO19."]
pub type REG_GPIO_19_PU_R = crate::BitReader<GPIO19PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO19PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO19PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_19_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO19PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO19PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO19PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO19PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_19_pu` writer - Pull Up Resistor for GPIO19."]
pub type REG_GPIO_19_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL9_SPEC, GPIO19PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_19_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO19PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO19PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_19_pd` reader - Pull Down Resistor for GPIO19."]
pub type REG_GPIO_19_PD_R = crate::BitReader<GPIO19PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO19PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO19PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_19_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO19PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO19PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO19PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO19PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_19_pd` writer - Pull Down Resistor for GPIO19."]
pub type REG_GPIO_19_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL9_SPEC, GPIO19PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_19_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO19PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO19PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_19_func_sel` reader - Function select for GPIO19."]
pub type REG_GPIO_19_FUNC_SEL_R = crate::FieldReader<u8, GPIO19FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO19.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO19FUNCTION_SELECT_A {
    #[doc = "2: `10`"]
    SF_D1 = 2,
    #[doc = "4: `100`"]
    SPI_SCLK = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG3 = 7,
    #[doc = "8: `1000`"]
    PWM_CH4 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_3 = 9,
    #[doc = "11: `1011`"]
    SWGPIO_19 = 11,
    #[doc = "14: `1110`"]
    E21_TDO = 14,
}
impl From<GPIO19FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO19FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_19_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO19FUNCTION_SELECT_A> {
        match self.bits {
            2 => Some(GPIO19FUNCTION_SELECT_A::SF_D1),
            4 => Some(GPIO19FUNCTION_SELECT_A::SPI_SCLK),
            6 => Some(GPIO19FUNCTION_SELECT_A::I2C_SDA),
            7 => Some(GPIO19FUNCTION_SELECT_A::UART_SIG3),
            8 => Some(GPIO19FUNCTION_SELECT_A::PWM_CH4),
            9 => Some(GPIO19FUNCTION_SELECT_A::FEM_GPIO_3),
            11 => Some(GPIO19FUNCTION_SELECT_A::SWGPIO_19),
            14 => Some(GPIO19FUNCTION_SELECT_A::E21_TDO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SF_D1`"]
    #[inline(always)]
    pub fn is_sf_d1(&self) -> bool {
        *self == GPIO19FUNCTION_SELECT_A::SF_D1
    }
    #[doc = "Checks if the value of the field is `SPI_SCLK`"]
    #[inline(always)]
    pub fn is_spi_sclk(&self) -> bool {
        *self == GPIO19FUNCTION_SELECT_A::SPI_SCLK
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == GPIO19FUNCTION_SELECT_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG3`"]
    #[inline(always)]
    pub fn is_uart_sig3(&self) -> bool {
        *self == GPIO19FUNCTION_SELECT_A::UART_SIG3
    }
    #[doc = "Checks if the value of the field is `PWM_CH4`"]
    #[inline(always)]
    pub fn is_pwm_ch4(&self) -> bool {
        *self == GPIO19FUNCTION_SELECT_A::PWM_CH4
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_3`"]
    #[inline(always)]
    pub fn is_fem_gpio_3(&self) -> bool {
        *self == GPIO19FUNCTION_SELECT_A::FEM_GPIO_3
    }
    #[doc = "Checks if the value of the field is `SWGPIO_19`"]
    #[inline(always)]
    pub fn is_swgpio_19(&self) -> bool {
        *self == GPIO19FUNCTION_SELECT_A::SWGPIO_19
    }
    #[doc = "Checks if the value of the field is `E21_TDO`"]
    #[inline(always)]
    pub fn is_e21_tdo(&self) -> bool {
        *self == GPIO19FUNCTION_SELECT_A::E21_TDO
    }
}
#[doc = "Field `reg_gpio_19_func_sel` writer - Function select for GPIO19."]
pub type REG_GPIO_19_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL9_SPEC, u8, GPIO19FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_19_FUNC_SEL_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d1(self) -> &'a mut W {
        self.variant(GPIO19FUNCTION_SELECT_A::SF_D1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_sclk(self) -> &'a mut W {
        self.variant(GPIO19FUNCTION_SELECT_A::SPI_SCLK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(GPIO19FUNCTION_SELECT_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig3(self) -> &'a mut W {
        self.variant(GPIO19FUNCTION_SELECT_A::UART_SIG3)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch4(self) -> &'a mut W {
        self.variant(GPIO19FUNCTION_SELECT_A::PWM_CH4)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_3(self) -> &'a mut W {
        self.variant(GPIO19FUNCTION_SELECT_A::FEM_GPIO_3)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_19(self) -> &'a mut W {
        self.variant(GPIO19FUNCTION_SELECT_A::SWGPIO_19)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdo(self) -> &'a mut W {
        self.variant(GPIO19FUNCTION_SELECT_A::E21_TDO)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_ie(&self) -> REG_GPIO_18_IE_R {
        REG_GPIO_18_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_smt(&self) -> REG_GPIO_18_SMT_R {
        REG_GPIO_18_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_drv(&self) -> REG_GPIO_18_DRV_R {
        REG_GPIO_18_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_pu(&self) -> REG_GPIO_18_PU_R {
        REG_GPIO_18_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_pd(&self) -> REG_GPIO_18_PD_R {
        REG_GPIO_18_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_func_sel(&self) -> REG_GPIO_18_FUNC_SEL_R {
        REG_GPIO_18_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_ie(&self) -> REG_GPIO_19_IE_R {
        REG_GPIO_19_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_smt(&self) -> REG_GPIO_19_SMT_R {
        REG_GPIO_19_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_drv(&self) -> REG_GPIO_19_DRV_R {
        REG_GPIO_19_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_pu(&self) -> REG_GPIO_19_PU_R {
        REG_GPIO_19_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_pd(&self) -> REG_GPIO_19_PD_R {
        REG_GPIO_19_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_func_sel(&self) -> REG_GPIO_19_FUNC_SEL_R {
        REG_GPIO_19_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_ie(&mut self) -> REG_GPIO_18_IE_W<0> {
        REG_GPIO_18_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_smt(&mut self) -> REG_GPIO_18_SMT_W<1> {
        REG_GPIO_18_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_drv(&mut self) -> REG_GPIO_18_DRV_W<2> {
        REG_GPIO_18_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_pu(&mut self) -> REG_GPIO_18_PU_W<4> {
        REG_GPIO_18_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_pd(&mut self) -> REG_GPIO_18_PD_W<5> {
        REG_GPIO_18_PD_W::new(self)
    }
    #[doc = "Bits 8:11 - Function select for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_func_sel(&mut self) -> REG_GPIO_18_FUNC_SEL_W<8> {
        REG_GPIO_18_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_ie(&mut self) -> REG_GPIO_19_IE_W<16> {
        REG_GPIO_19_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_smt(&mut self) -> REG_GPIO_19_SMT_W<17> {
        REG_GPIO_19_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_drv(&mut self) -> REG_GPIO_19_DRV_W<18> {
        REG_GPIO_19_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_pu(&mut self) -> REG_GPIO_19_PU_W<20> {
        REG_GPIO_19_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_pd(&mut self) -> REG_GPIO_19_PD_W<21> {
        REG_GPIO_19_PD_W::new(self)
    }
    #[doc = "Bits 24:27 - Function select for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_func_sel(&mut self) -> REG_GPIO_19_FUNC_SEL_W<24> {
        REG_GPIO_19_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO18, GPIO19 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl9](index.html) module"]
pub struct GPIO_CFGCTL9_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl9::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl9::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL9 to value 0x0b03_0b03"]
impl crate::Resettable for GPIO_CFGCTL9_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b03_0b03;
}

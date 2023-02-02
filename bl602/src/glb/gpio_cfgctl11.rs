#[doc = "Register `GPIO_CFGCTL11` reader"]
pub struct R(crate::R<GPIO_CFGCTL11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL11` writer"]
pub struct W(crate::W<GPIO_CFGCTL11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL11_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_22_ie` reader - Input enable for GPIO22."]
pub type REG_GPIO_22_IE_R = crate::BitReader<GPIO22INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO22.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO22INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO22INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_22_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22INPUT_ENABLED_A {
        match self.bits {
            false => GPIO22INPUT_ENABLED_A::DISABLED,
            true => GPIO22INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO22INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO22INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_22_ie` writer - Input enable for GPIO22."]
pub type REG_GPIO_22_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL11_SPEC, GPIO22INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_22_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO22INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO22INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_22_smt` reader - Schmitt trigger enabled for GPIO22."]
pub type REG_GPIO_22_SMT_R = crate::BitReader<GPIO22SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO22.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO22SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO22SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_22_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22SCHMITT_A {
        match self.bits {
            false => GPIO22SCHMITT_A::DISABLED,
            true => GPIO22SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO22SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO22SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_22_smt` writer - Schmitt trigger enabled for GPIO22."]
pub type REG_GPIO_22_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL11_SPEC, GPIO22SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_22_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO22SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO22SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_22_drv` reader - Driving control enabled for GPIO22."]
pub type REG_GPIO_22_DRV_R = crate::FieldReader<u8, GPIO22DRIVING_A>;
#[doc = "Driving control enabled for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO22DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO22DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO22DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_22_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO22DRIVING_A> {
        match self.bits {
            0 => Some(GPIO22DRIVING_A::DISABLED),
            1 => Some(GPIO22DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO22DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO22DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_22_drv` writer - Driving control enabled for GPIO22."]
pub type REG_GPIO_22_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL11_SPEC, u8, GPIO22DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_22_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO22DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO22DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_22_pu` reader - Pull Up Resistor for GPIO22."]
pub type REG_GPIO_22_PU_R = crate::BitReader<GPIO22PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO22PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO22PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_22_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO22PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO22PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO22PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO22PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_22_pu` writer - Pull Up Resistor for GPIO22."]
pub type REG_GPIO_22_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL11_SPEC, GPIO22PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_22_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO22PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO22PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_22_pd` reader - Pull Down Resistor for GPIO22."]
pub type REG_GPIO_22_PD_R = crate::BitReader<GPIO22PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO22PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO22PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_22_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO22PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO22PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO22PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO22PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_22_pd` writer - Pull Down Resistor for GPIO22."]
pub type REG_GPIO_22_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL11_SPEC, GPIO22PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_22_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO22PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO22PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_22_func_sel` reader - Function select for GPIO22."]
pub type REG_GPIO_22_FUNC_SEL_R = crate::FieldReader<u8, GPIO22FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO22.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO22FUNCTION_SELECT_A {
    #[doc = "2: `10`"]
    SF_CLK_OUT = 2,
    #[doc = "4: `100`"]
    SPI_SS = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG6 = 7,
    #[doc = "8: `1000`"]
    PWM_CH2 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_2 = 9,
    #[doc = "11: `1011`"]
    SWGPIO_22 = 11,
    #[doc = "14: `1110`"]
    E21_TCK = 14,
}
impl From<GPIO22FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO22FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_22_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO22FUNCTION_SELECT_A> {
        match self.bits {
            2 => Some(GPIO22FUNCTION_SELECT_A::SF_CLK_OUT),
            4 => Some(GPIO22FUNCTION_SELECT_A::SPI_SS),
            6 => Some(GPIO22FUNCTION_SELECT_A::I2C_SCL),
            7 => Some(GPIO22FUNCTION_SELECT_A::UART_SIG6),
            8 => Some(GPIO22FUNCTION_SELECT_A::PWM_CH2),
            9 => Some(GPIO22FUNCTION_SELECT_A::FEM_GPIO_2),
            11 => Some(GPIO22FUNCTION_SELECT_A::SWGPIO_22),
            14 => Some(GPIO22FUNCTION_SELECT_A::E21_TCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SF_CLK_OUT`"]
    #[inline(always)]
    pub fn is_sf_clk_out(&self) -> bool {
        *self == GPIO22FUNCTION_SELECT_A::SF_CLK_OUT
    }
    #[doc = "Checks if the value of the field is `SPI_SS`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == GPIO22FUNCTION_SELECT_A::SPI_SS
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == GPIO22FUNCTION_SELECT_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG6`"]
    #[inline(always)]
    pub fn is_uart_sig6(&self) -> bool {
        *self == GPIO22FUNCTION_SELECT_A::UART_SIG6
    }
    #[doc = "Checks if the value of the field is `PWM_CH2`"]
    #[inline(always)]
    pub fn is_pwm_ch2(&self) -> bool {
        *self == GPIO22FUNCTION_SELECT_A::PWM_CH2
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_2`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == GPIO22FUNCTION_SELECT_A::FEM_GPIO_2
    }
    #[doc = "Checks if the value of the field is `SWGPIO_22`"]
    #[inline(always)]
    pub fn is_swgpio_22(&self) -> bool {
        *self == GPIO22FUNCTION_SELECT_A::SWGPIO_22
    }
    #[doc = "Checks if the value of the field is `E21_TCK`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == GPIO22FUNCTION_SELECT_A::E21_TCK
    }
}
#[doc = "Field `reg_gpio_22_func_sel` writer - Function select for GPIO22."]
pub type REG_GPIO_22_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL11_SPEC, u8, GPIO22FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_22_FUNC_SEL_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_clk_out(self) -> &'a mut W {
        self.variant(GPIO22FUNCTION_SELECT_A::SF_CLK_OUT)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut W {
        self.variant(GPIO22FUNCTION_SELECT_A::SPI_SS)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(GPIO22FUNCTION_SELECT_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig6(self) -> &'a mut W {
        self.variant(GPIO22FUNCTION_SELECT_A::UART_SIG6)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch2(self) -> &'a mut W {
        self.variant(GPIO22FUNCTION_SELECT_A::PWM_CH2)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut W {
        self.variant(GPIO22FUNCTION_SELECT_A::FEM_GPIO_2)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_22(self) -> &'a mut W {
        self.variant(GPIO22FUNCTION_SELECT_A::SWGPIO_22)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut W {
        self.variant(GPIO22FUNCTION_SELECT_A::E21_TCK)
    }
}
#[doc = "Field `reg_gpio_23_ie` reader - Input enable for GPIO23."]
pub type REG_GPIO_23_IE_R = crate::BitReader<GPIO23INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO23.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO23INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO23INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_23_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23INPUT_ENABLED_A {
        match self.bits {
            false => GPIO23INPUT_ENABLED_A::DISABLED,
            true => GPIO23INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO23INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO23INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_23_ie` writer - Input enable for GPIO23."]
pub type REG_GPIO_23_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL11_SPEC, GPIO23INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_23_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO23INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO23INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_23_smt` reader - Schmitt trigger enabled for GPIO23."]
pub type REG_GPIO_23_SMT_R = crate::BitReader<GPIO23SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO23.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO23SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO23SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_23_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23SCHMITT_A {
        match self.bits {
            false => GPIO23SCHMITT_A::DISABLED,
            true => GPIO23SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO23SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO23SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_23_smt` writer - Schmitt trigger enabled for GPIO23."]
pub type REG_GPIO_23_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL11_SPEC, GPIO23SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_23_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO23SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO23SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_23_drv` reader - Driving control enabled for GPIO23."]
pub type REG_GPIO_23_DRV_R = crate::FieldReader<u8, GPIO23DRIVING_A>;
#[doc = "Driving control enabled for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO23DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO23DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO23DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_23_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO23DRIVING_A> {
        match self.bits {
            0 => Some(GPIO23DRIVING_A::DISABLED),
            1 => Some(GPIO23DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO23DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO23DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_23_drv` writer - Driving control enabled for GPIO23."]
pub type REG_GPIO_23_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL11_SPEC, u8, GPIO23DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_23_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO23DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO23DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_23_pu` reader - Pull Up Resistor for GPIO23."]
pub type REG_GPIO_23_PU_R = crate::BitReader<GPIO23PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO23PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO23PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_23_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO23PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO23PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO23PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO23PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_23_pu` writer - Pull Up Resistor for GPIO23."]
pub type REG_GPIO_23_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL11_SPEC, GPIO23PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_23_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO23PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO23PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_23_pd` reader - Pull Down Resistor for GPIO23."]
pub type REG_GPIO_23_PD_R = crate::BitReader<GPIO23PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO23PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO23PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_23_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO23PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO23PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO23PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO23PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_23_pd` writer - Pull Down Resistor for GPIO23."]
pub type REG_GPIO_23_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL11_SPEC, GPIO23PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_23_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO23PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO23PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_ie(&self) -> REG_GPIO_22_IE_R {
        REG_GPIO_22_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_smt(&self) -> REG_GPIO_22_SMT_R {
        REG_GPIO_22_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_drv(&self) -> REG_GPIO_22_DRV_R {
        REG_GPIO_22_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_pu(&self) -> REG_GPIO_22_PU_R {
        REG_GPIO_22_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_pd(&self) -> REG_GPIO_22_PD_R {
        REG_GPIO_22_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_func_sel(&self) -> REG_GPIO_22_FUNC_SEL_R {
        REG_GPIO_22_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_ie(&self) -> REG_GPIO_23_IE_R {
        REG_GPIO_23_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_smt(&self) -> REG_GPIO_23_SMT_R {
        REG_GPIO_23_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_drv(&self) -> REG_GPIO_23_DRV_R {
        REG_GPIO_23_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_pu(&self) -> REG_GPIO_23_PU_R {
        REG_GPIO_23_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_pd(&self) -> REG_GPIO_23_PD_R {
        REG_GPIO_23_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_ie(&mut self) -> REG_GPIO_22_IE_W<0> {
        REG_GPIO_22_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_smt(&mut self) -> REG_GPIO_22_SMT_W<1> {
        REG_GPIO_22_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_drv(&mut self) -> REG_GPIO_22_DRV_W<2> {
        REG_GPIO_22_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_pu(&mut self) -> REG_GPIO_22_PU_W<4> {
        REG_GPIO_22_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_pd(&mut self) -> REG_GPIO_22_PD_W<5> {
        REG_GPIO_22_PD_W::new(self)
    }
    #[doc = "Bits 8:11 - Function select for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_func_sel(&mut self) -> REG_GPIO_22_FUNC_SEL_W<8> {
        REG_GPIO_22_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO23."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_ie(&mut self) -> REG_GPIO_23_IE_W<16> {
        REG_GPIO_23_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO23."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_smt(&mut self) -> REG_GPIO_23_SMT_W<17> {
        REG_GPIO_23_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO23."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_drv(&mut self) -> REG_GPIO_23_DRV_W<18> {
        REG_GPIO_23_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO23."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_pu(&mut self) -> REG_GPIO_23_PU_W<20> {
        REG_GPIO_23_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO23."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_pd(&mut self) -> REG_GPIO_23_PD_W<21> {
        REG_GPIO_23_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO22, GPIO23 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl11](index.html) module"]
pub struct GPIO_CFGCTL11_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl11::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl11::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL11 to value 0x0003_0b03"]
impl crate::Resettable for GPIO_CFGCTL11_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0b03;
}

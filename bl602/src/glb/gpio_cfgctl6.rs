#[doc = "Register `GPIO_CFGCTL6` reader"]
pub struct R(crate::R<GPIO_CFGCTL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL6` writer"]
pub struct W(crate::W<GPIO_CFGCTL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL6_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_12_ie` reader - Input enable for GPIO12."]
pub type REG_GPIO_12_IE_R = crate::BitReader<GPIO12INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO12.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO12INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO12INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_12_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12INPUT_ENABLED_A {
        match self.bits {
            false => GPIO12INPUT_ENABLED_A::DISABLED,
            true => GPIO12INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO12INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO12INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_12_ie` writer - Input enable for GPIO12."]
pub type REG_GPIO_12_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL6_SPEC, GPIO12INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_12_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO12INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO12INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_12_smt` reader - Schmitt trigger enabled for GPIO12."]
pub type REG_GPIO_12_SMT_R = crate::BitReader<GPIO12SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO12.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO12SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO12SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_12_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12SCHMITT_A {
        match self.bits {
            false => GPIO12SCHMITT_A::DISABLED,
            true => GPIO12SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO12SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO12SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_12_smt` writer - Schmitt trigger enabled for GPIO12."]
pub type REG_GPIO_12_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL6_SPEC, GPIO12SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_12_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO12SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO12SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_12_drv` reader - Driving control enabled for GPIO12."]
pub type REG_GPIO_12_DRV_R = crate::FieldReader<u8, GPIO12DRIVING_A>;
#[doc = "Driving control enabled for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO12DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO12DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO12DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_12_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO12DRIVING_A> {
        match self.bits {
            0 => Some(GPIO12DRIVING_A::DISABLED),
            1 => Some(GPIO12DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO12DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO12DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_12_drv` writer - Driving control enabled for GPIO12."]
pub type REG_GPIO_12_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL6_SPEC, u8, GPIO12DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_12_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO12DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO12DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_12_pu` reader - Pull Up Resistor for GPIO12."]
pub type REG_GPIO_12_PU_R = crate::BitReader<GPIO12PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO12PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO12PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_12_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO12PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO12PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO12PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO12PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_12_pu` writer - Pull Up Resistor for GPIO12."]
pub type REG_GPIO_12_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL6_SPEC, GPIO12PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_12_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO12PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO12PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_12_pd` reader - Pull Down Resistor for GPIO12."]
pub type REG_GPIO_12_PD_R = crate::BitReader<GPIO12PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO12PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO12PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_12_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO12PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO12PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO12PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO12PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_12_pd` writer - Pull Down Resistor for GPIO12."]
pub type REG_GPIO_12_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL6_SPEC, GPIO12PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_12_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO12PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO12PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_12_func_sel` reader - Function select for GPIO12."]
pub type REG_GPIO_12_FUNC_SEL_R = crate::FieldReader<u8, GPIO12FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO12.\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO12FUNCTION_SELECT_A {
    #[doc = "4: `100`"]
    SPI_MISO_SPI_MOSI = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG4 = 7,
    #[doc = "8: `1000`"]
    PWM_CH2 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_0 = 9,
    #[doc = "10: `1010`"]
    GPIP_CH0_GPADC_VREF_EXT = 10,
    #[doc = "11: `1011`"]
    SWGPIO_12 = 11,
    #[doc = "14: `1110`"]
    E21_TMS = 14,
}
impl From<GPIO12FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO12FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_12_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO12FUNCTION_SELECT_A> {
        match self.bits {
            4 => Some(GPIO12FUNCTION_SELECT_A::SPI_MISO_SPI_MOSI),
            6 => Some(GPIO12FUNCTION_SELECT_A::I2C_SCL),
            7 => Some(GPIO12FUNCTION_SELECT_A::UART_SIG4),
            8 => Some(GPIO12FUNCTION_SELECT_A::PWM_CH2),
            9 => Some(GPIO12FUNCTION_SELECT_A::FEM_GPIO_0),
            10 => Some(GPIO12FUNCTION_SELECT_A::GPIP_CH0_GPADC_VREF_EXT),
            11 => Some(GPIO12FUNCTION_SELECT_A::SWGPIO_12),
            14 => Some(GPIO12FUNCTION_SELECT_A::E21_TMS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MISO_SPI_MOSI`"]
    #[inline(always)]
    pub fn is_spi_miso_spi_mosi(&self) -> bool {
        *self == GPIO12FUNCTION_SELECT_A::SPI_MISO_SPI_MOSI
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == GPIO12FUNCTION_SELECT_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG4`"]
    #[inline(always)]
    pub fn is_uart_sig4(&self) -> bool {
        *self == GPIO12FUNCTION_SELECT_A::UART_SIG4
    }
    #[doc = "Checks if the value of the field is `PWM_CH2`"]
    #[inline(always)]
    pub fn is_pwm_ch2(&self) -> bool {
        *self == GPIO12FUNCTION_SELECT_A::PWM_CH2
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_0`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        *self == GPIO12FUNCTION_SELECT_A::FEM_GPIO_0
    }
    #[doc = "Checks if the value of the field is `GPIP_CH0_GPADC_VREF_EXT`"]
    #[inline(always)]
    pub fn is_gpip_ch0_gpadc_vref_ext(&self) -> bool {
        *self == GPIO12FUNCTION_SELECT_A::GPIP_CH0_GPADC_VREF_EXT
    }
    #[doc = "Checks if the value of the field is `SWGPIO_12`"]
    #[inline(always)]
    pub fn is_swgpio_12(&self) -> bool {
        *self == GPIO12FUNCTION_SELECT_A::SWGPIO_12
    }
    #[doc = "Checks if the value of the field is `E21_TMS`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        *self == GPIO12FUNCTION_SELECT_A::E21_TMS
    }
}
#[doc = "Field `reg_gpio_12_func_sel` writer - Function select for GPIO12."]
pub type REG_GPIO_12_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL6_SPEC, u8, GPIO12FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_12_FUNC_SEL_W<'a, O> {
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_miso_spi_mosi(self) -> &'a mut W {
        self.variant(GPIO12FUNCTION_SELECT_A::SPI_MISO_SPI_MOSI)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(GPIO12FUNCTION_SELECT_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig4(self) -> &'a mut W {
        self.variant(GPIO12FUNCTION_SELECT_A::UART_SIG4)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch2(self) -> &'a mut W {
        self.variant(GPIO12FUNCTION_SELECT_A::PWM_CH2)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut W {
        self.variant(GPIO12FUNCTION_SELECT_A::FEM_GPIO_0)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch0_gpadc_vref_ext(self) -> &'a mut W {
        self.variant(GPIO12FUNCTION_SELECT_A::GPIP_CH0_GPADC_VREF_EXT)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_12(self) -> &'a mut W {
        self.variant(GPIO12FUNCTION_SELECT_A::SWGPIO_12)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut W {
        self.variant(GPIO12FUNCTION_SELECT_A::E21_TMS)
    }
}
#[doc = "Field `reg_gpio_13_ie` reader - Input enable for GPIO13."]
pub type REG_GPIO_13_IE_R = crate::BitReader<GPIO13INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO13.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO13INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO13INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_13_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13INPUT_ENABLED_A {
        match self.bits {
            false => GPIO13INPUT_ENABLED_A::DISABLED,
            true => GPIO13INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO13INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO13INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_13_ie` writer - Input enable for GPIO13."]
pub type REG_GPIO_13_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL6_SPEC, GPIO13INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_13_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO13INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO13INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_13_smt` reader - Schmitt trigger enabled for GPIO13."]
pub type REG_GPIO_13_SMT_R = crate::BitReader<GPIO13SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO13.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO13SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO13SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_13_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13SCHMITT_A {
        match self.bits {
            false => GPIO13SCHMITT_A::DISABLED,
            true => GPIO13SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO13SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO13SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_13_smt` writer - Schmitt trigger enabled for GPIO13."]
pub type REG_GPIO_13_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL6_SPEC, GPIO13SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_13_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO13SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO13SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_13_drv` reader - Driving control enabled for GPIO13."]
pub type REG_GPIO_13_DRV_R = crate::FieldReader<u8, GPIO13DRIVING_A>;
#[doc = "Driving control enabled for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO13DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO13DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO13DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_13_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO13DRIVING_A> {
        match self.bits {
            0 => Some(GPIO13DRIVING_A::DISABLED),
            1 => Some(GPIO13DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO13DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO13DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_13_drv` writer - Driving control enabled for GPIO13."]
pub type REG_GPIO_13_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL6_SPEC, u8, GPIO13DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_13_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO13DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO13DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_13_pu` reader - Pull Up Resistor for GPIO13."]
pub type REG_GPIO_13_PU_R = crate::BitReader<GPIO13PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO13PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO13PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_13_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO13PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO13PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO13PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO13PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_13_pu` writer - Pull Up Resistor for GPIO13."]
pub type REG_GPIO_13_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL6_SPEC, GPIO13PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_13_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO13PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO13PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_13_pd` reader - Pull Down Resistor for GPIO13."]
pub type REG_GPIO_13_PD_R = crate::BitReader<GPIO13PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO13PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO13PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_13_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO13PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO13PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO13PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO13PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_13_pd` writer - Pull Down Resistor for GPIO13."]
pub type REG_GPIO_13_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL6_SPEC, GPIO13PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_13_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO13PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO13PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_13_func_sel` reader - Function select for GPIO13."]
pub type REG_GPIO_13_FUNC_SEL_R = crate::FieldReader<u8, GPIO13FUNCTION_SELECT_A>;
#[doc = "Function select for GPIO13.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO13FUNCTION_SELECT_A {
    #[doc = "4: `100`"]
    SPI_MOSI_SPI_MISO = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG5 = 7,
    #[doc = "8: `1000`"]
    PWM_CH3 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_1 = 9,
    #[doc = "10: `1010`"]
    GPIP_CH3 = 10,
    #[doc = "11: `1011`"]
    SWGPIO_13 = 11,
    #[doc = "14: `1110`"]
    E21_TDI = 14,
}
impl From<GPIO13FUNCTION_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO13FUNCTION_SELECT_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_13_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO13FUNCTION_SELECT_A> {
        match self.bits {
            4 => Some(GPIO13FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO),
            6 => Some(GPIO13FUNCTION_SELECT_A::I2C_SDA),
            7 => Some(GPIO13FUNCTION_SELECT_A::UART_SIG5),
            8 => Some(GPIO13FUNCTION_SELECT_A::PWM_CH3),
            9 => Some(GPIO13FUNCTION_SELECT_A::FEM_GPIO_1),
            10 => Some(GPIO13FUNCTION_SELECT_A::GPIP_CH3),
            11 => Some(GPIO13FUNCTION_SELECT_A::SWGPIO_13),
            14 => Some(GPIO13FUNCTION_SELECT_A::E21_TDI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MOSI_SPI_MISO`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == GPIO13FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == GPIO13FUNCTION_SELECT_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG5`"]
    #[inline(always)]
    pub fn is_uart_sig5(&self) -> bool {
        *self == GPIO13FUNCTION_SELECT_A::UART_SIG5
    }
    #[doc = "Checks if the value of the field is `PWM_CH3`"]
    #[inline(always)]
    pub fn is_pwm_ch3(&self) -> bool {
        *self == GPIO13FUNCTION_SELECT_A::PWM_CH3
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_1`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        *self == GPIO13FUNCTION_SELECT_A::FEM_GPIO_1
    }
    #[doc = "Checks if the value of the field is `GPIP_CH3`"]
    #[inline(always)]
    pub fn is_gpip_ch3(&self) -> bool {
        *self == GPIO13FUNCTION_SELECT_A::GPIP_CH3
    }
    #[doc = "Checks if the value of the field is `SWGPIO_13`"]
    #[inline(always)]
    pub fn is_swgpio_13(&self) -> bool {
        *self == GPIO13FUNCTION_SELECT_A::SWGPIO_13
    }
    #[doc = "Checks if the value of the field is `E21_TDI`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        *self == GPIO13FUNCTION_SELECT_A::E21_TDI
    }
}
#[doc = "Field `reg_gpio_13_func_sel` writer - Function select for GPIO13."]
pub type REG_GPIO_13_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL6_SPEC, u8, GPIO13FUNCTION_SELECT_A, 4, O>;
impl<'a, const O: u8> REG_GPIO_13_FUNC_SEL_W<'a, O> {
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut W {
        self.variant(GPIO13FUNCTION_SELECT_A::SPI_MOSI_SPI_MISO)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(GPIO13FUNCTION_SELECT_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig5(self) -> &'a mut W {
        self.variant(GPIO13FUNCTION_SELECT_A::UART_SIG5)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch3(self) -> &'a mut W {
        self.variant(GPIO13FUNCTION_SELECT_A::PWM_CH3)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut W {
        self.variant(GPIO13FUNCTION_SELECT_A::FEM_GPIO_1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch3(self) -> &'a mut W {
        self.variant(GPIO13FUNCTION_SELECT_A::GPIP_CH3)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_13(self) -> &'a mut W {
        self.variant(GPIO13FUNCTION_SELECT_A::SWGPIO_13)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut W {
        self.variant(GPIO13FUNCTION_SELECT_A::E21_TDI)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_ie(&self) -> REG_GPIO_12_IE_R {
        REG_GPIO_12_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_smt(&self) -> REG_GPIO_12_SMT_R {
        REG_GPIO_12_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_drv(&self) -> REG_GPIO_12_DRV_R {
        REG_GPIO_12_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_pu(&self) -> REG_GPIO_12_PU_R {
        REG_GPIO_12_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_pd(&self) -> REG_GPIO_12_PD_R {
        REG_GPIO_12_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_func_sel(&self) -> REG_GPIO_12_FUNC_SEL_R {
        REG_GPIO_12_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_ie(&self) -> REG_GPIO_13_IE_R {
        REG_GPIO_13_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_smt(&self) -> REG_GPIO_13_SMT_R {
        REG_GPIO_13_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_drv(&self) -> REG_GPIO_13_DRV_R {
        REG_GPIO_13_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_pu(&self) -> REG_GPIO_13_PU_R {
        REG_GPIO_13_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_pd(&self) -> REG_GPIO_13_PD_R {
        REG_GPIO_13_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_func_sel(&self) -> REG_GPIO_13_FUNC_SEL_R {
        REG_GPIO_13_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_ie(&mut self) -> REG_GPIO_12_IE_W<0> {
        REG_GPIO_12_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_smt(&mut self) -> REG_GPIO_12_SMT_W<1> {
        REG_GPIO_12_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_drv(&mut self) -> REG_GPIO_12_DRV_W<2> {
        REG_GPIO_12_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_pu(&mut self) -> REG_GPIO_12_PU_W<4> {
        REG_GPIO_12_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_pd(&mut self) -> REG_GPIO_12_PD_W<5> {
        REG_GPIO_12_PD_W::new(self)
    }
    #[doc = "Bits 8:11 - Function select for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_func_sel(&mut self) -> REG_GPIO_12_FUNC_SEL_W<8> {
        REG_GPIO_12_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_ie(&mut self) -> REG_GPIO_13_IE_W<16> {
        REG_GPIO_13_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_smt(&mut self) -> REG_GPIO_13_SMT_W<17> {
        REG_GPIO_13_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_drv(&mut self) -> REG_GPIO_13_DRV_W<18> {
        REG_GPIO_13_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_pu(&mut self) -> REG_GPIO_13_PU_W<20> {
        REG_GPIO_13_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_pd(&mut self) -> REG_GPIO_13_PD_W<21> {
        REG_GPIO_13_PD_W::new(self)
    }
    #[doc = "Bits 24:27 - Function select for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_func_sel(&mut self) -> REG_GPIO_13_FUNC_SEL_W<24> {
        REG_GPIO_13_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO12, GPIO13 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl6](index.html) module"]
pub struct GPIO_CFGCTL6_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl6::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl6::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL6 to value 0x0b03_0e03"]
impl crate::Resettable for GPIO_CFGCTL6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b03_0e03;
}

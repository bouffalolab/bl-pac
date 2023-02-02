#[doc = "Register `GPIO_CFGCTL30` reader"]
pub struct R(crate::R<GPIO_CFGCTL30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `reg_gpio_0_i` reader - "]
pub type REG_GPIO_0_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_1_i` reader - Input register for GPIO1."]
pub type REG_GPIO_1_I_R = crate::BitReader<GPIO1INPUT_A>;
#[doc = "Input register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO1INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_1_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1INPUT_A {
        match self.bits {
            false => GPIO1INPUT_A::DISABLED,
            true => GPIO1INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO1INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO1INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_2_i` reader - Input register for GPIO2."]
pub type REG_GPIO_2_I_R = crate::BitReader<GPIO2INPUT_A>;
#[doc = "Input register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO2INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_2_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2INPUT_A {
        match self.bits {
            false => GPIO2INPUT_A::DISABLED,
            true => GPIO2INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO2INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO2INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_3_i` reader - Input register for GPIO3."]
pub type REG_GPIO_3_I_R = crate::BitReader<GPIO3INPUT_A>;
#[doc = "Input register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO3INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO3INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_3_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3INPUT_A {
        match self.bits {
            false => GPIO3INPUT_A::DISABLED,
            true => GPIO3INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO3INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO3INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_4_i` reader - Input register for GPIO4."]
pub type REG_GPIO_4_I_R = crate::BitReader<GPIO4INPUT_A>;
#[doc = "Input register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO4INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO4INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_4_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4INPUT_A {
        match self.bits {
            false => GPIO4INPUT_A::DISABLED,
            true => GPIO4INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO4INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO4INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_5_i` reader - Input register for GPIO5."]
pub type REG_GPIO_5_I_R = crate::BitReader<GPIO5INPUT_A>;
#[doc = "Input register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO5INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO5INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_5_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5INPUT_A {
        match self.bits {
            false => GPIO5INPUT_A::DISABLED,
            true => GPIO5INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO5INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO5INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_6_i` reader - Input register for GPIO6."]
pub type REG_GPIO_6_I_R = crate::BitReader<GPIO6INPUT_A>;
#[doc = "Input register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO6INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO6INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_6_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6INPUT_A {
        match self.bits {
            false => GPIO6INPUT_A::DISABLED,
            true => GPIO6INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO6INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO6INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_7_i` reader - Input register for GPIO7."]
pub type REG_GPIO_7_I_R = crate::BitReader<GPIO7INPUT_A>;
#[doc = "Input register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO7INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO7INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_7_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7INPUT_A {
        match self.bits {
            false => GPIO7INPUT_A::DISABLED,
            true => GPIO7INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO7INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO7INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_8_i` reader - Input register for GPIO8."]
pub type REG_GPIO_8_I_R = crate::BitReader<GPIO8INPUT_A>;
#[doc = "Input register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO8INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO8INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO8INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_8_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8INPUT_A {
        match self.bits {
            false => GPIO8INPUT_A::DISABLED,
            true => GPIO8INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO8INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO8INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_9_i` reader - Input register for GPIO9."]
pub type REG_GPIO_9_I_R = crate::BitReader<GPIO9INPUT_A>;
#[doc = "Input register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO9INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO9INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO9INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_9_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9INPUT_A {
        match self.bits {
            false => GPIO9INPUT_A::DISABLED,
            true => GPIO9INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO9INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO9INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_10_i` reader - Input register for GPIO10."]
pub type REG_GPIO_10_I_R = crate::BitReader<GPIO10INPUT_A>;
#[doc = "Input register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO10INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO10INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO10INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_10_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10INPUT_A {
        match self.bits {
            false => GPIO10INPUT_A::DISABLED,
            true => GPIO10INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO10INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO10INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_11_i` reader - Input register for GPIO11."]
pub type REG_GPIO_11_I_R = crate::BitReader<GPIO11INPUT_A>;
#[doc = "Input register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO11INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO11INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO11INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_11_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11INPUT_A {
        match self.bits {
            false => GPIO11INPUT_A::DISABLED,
            true => GPIO11INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO11INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO11INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_12_i` reader - Input register for GPIO12."]
pub type REG_GPIO_12_I_R = crate::BitReader<GPIO12INPUT_A>;
#[doc = "Input register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO12INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO12INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_12_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12INPUT_A {
        match self.bits {
            false => GPIO12INPUT_A::DISABLED,
            true => GPIO12INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO12INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO12INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_13_i` reader - Input register for GPIO13."]
pub type REG_GPIO_13_I_R = crate::BitReader<GPIO13INPUT_A>;
#[doc = "Input register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO13INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO13INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_13_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13INPUT_A {
        match self.bits {
            false => GPIO13INPUT_A::DISABLED,
            true => GPIO13INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO13INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO13INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_14_i` reader - Input register for GPIO14."]
pub type REG_GPIO_14_I_R = crate::BitReader<GPIO14INPUT_A>;
#[doc = "Input register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO14INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO14INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_14_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14INPUT_A {
        match self.bits {
            false => GPIO14INPUT_A::DISABLED,
            true => GPIO14INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO14INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO14INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_15_i` reader - Input register for GPIO15."]
pub type REG_GPIO_15_I_R = crate::BitReader<GPIO15INPUT_A>;
#[doc = "Input register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO15INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO15INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_15_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15INPUT_A {
        match self.bits {
            false => GPIO15INPUT_A::DISABLED,
            true => GPIO15INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO15INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO15INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_16_i` reader - Input register for GPIO16."]
pub type REG_GPIO_16_I_R = crate::BitReader<GPIO16INPUT_A>;
#[doc = "Input register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO16INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO16INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO16INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_16_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16INPUT_A {
        match self.bits {
            false => GPIO16INPUT_A::DISABLED,
            true => GPIO16INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO16INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO16INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_17_i` reader - Input register for GPIO17."]
pub type REG_GPIO_17_I_R = crate::BitReader<GPIO17INPUT_A>;
#[doc = "Input register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO17INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO17INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO17INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_17_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17INPUT_A {
        match self.bits {
            false => GPIO17INPUT_A::DISABLED,
            true => GPIO17INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO17INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO17INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_18_i` reader - Input register for GPIO18."]
pub type REG_GPIO_18_I_R = crate::BitReader<GPIO18INPUT_A>;
#[doc = "Input register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO18INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO18INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_18_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18INPUT_A {
        match self.bits {
            false => GPIO18INPUT_A::DISABLED,
            true => GPIO18INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO18INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO18INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_19_i` reader - Input register for GPIO19."]
pub type REG_GPIO_19_I_R = crate::BitReader<GPIO19INPUT_A>;
#[doc = "Input register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO19INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO19INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_19_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19INPUT_A {
        match self.bits {
            false => GPIO19INPUT_A::DISABLED,
            true => GPIO19INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO19INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO19INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_20_i` reader - Input register for GPIO20."]
pub type REG_GPIO_20_I_R = crate::BitReader<GPIO20INPUT_A>;
#[doc = "Input register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO20INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO20INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_20_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20INPUT_A {
        match self.bits {
            false => GPIO20INPUT_A::DISABLED,
            true => GPIO20INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO20INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO20INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_21_i` reader - Input register for GPIO21."]
pub type REG_GPIO_21_I_R = crate::BitReader<GPIO21INPUT_A>;
#[doc = "Input register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO21INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO21INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_21_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21INPUT_A {
        match self.bits {
            false => GPIO21INPUT_A::DISABLED,
            true => GPIO21INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO21INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO21INPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_22_i` reader - Input register for GPIO22."]
pub type REG_GPIO_22_I_R = crate::BitReader<GPIO22INPUT_A>;
#[doc = "Input register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO22INPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO22INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_22_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22INPUT_A {
        match self.bits {
            false => GPIO22INPUT_A::DISABLED,
            true => GPIO22INPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO22INPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO22INPUT_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_i(&self) -> REG_GPIO_0_I_R {
        REG_GPIO_0_I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_i(&self) -> REG_GPIO_1_I_R {
        REG_GPIO_1_I_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_i(&self) -> REG_GPIO_2_I_R {
        REG_GPIO_2_I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_i(&self) -> REG_GPIO_3_I_R {
        REG_GPIO_3_I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_i(&self) -> REG_GPIO_4_I_R {
        REG_GPIO_4_I_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_i(&self) -> REG_GPIO_5_I_R {
        REG_GPIO_5_I_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_i(&self) -> REG_GPIO_6_I_R {
        REG_GPIO_6_I_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_i(&self) -> REG_GPIO_7_I_R {
        REG_GPIO_7_I_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_i(&self) -> REG_GPIO_8_I_R {
        REG_GPIO_8_I_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_i(&self) -> REG_GPIO_9_I_R {
        REG_GPIO_9_I_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_i(&self) -> REG_GPIO_10_I_R {
        REG_GPIO_10_I_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_i(&self) -> REG_GPIO_11_I_R {
        REG_GPIO_11_I_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_i(&self) -> REG_GPIO_12_I_R {
        REG_GPIO_12_I_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_i(&self) -> REG_GPIO_13_I_R {
        REG_GPIO_13_I_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_i(&self) -> REG_GPIO_14_I_R {
        REG_GPIO_14_I_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_i(&self) -> REG_GPIO_15_I_R {
        REG_GPIO_15_I_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_i(&self) -> REG_GPIO_16_I_R {
        REG_GPIO_16_I_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_i(&self) -> REG_GPIO_17_I_R {
        REG_GPIO_17_I_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_i(&self) -> REG_GPIO_18_I_R {
        REG_GPIO_18_I_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_i(&self) -> REG_GPIO_19_I_R {
        REG_GPIO_19_I_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_i(&self) -> REG_GPIO_20_I_R {
        REG_GPIO_20_I_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_i(&self) -> REG_GPIO_21_I_R {
        REG_GPIO_21_I_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_i(&self) -> REG_GPIO_22_I_R {
        REG_GPIO_22_I_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Input register for all GPIO pins. Input Enabled bit must be set in configuration register to work.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl30](index.html) module"]
pub struct GPIO_CFGCTL30_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl30::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL30_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_CFGCTL30 to value 0"]
impl crate::Resettable for GPIO_CFGCTL30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GPIO_CFGCTL32` reader"]
pub struct R(crate::R<GPIO_CFGCTL32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL32` writer"]
pub struct W(crate::W<GPIO_CFGCTL32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL32_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_0_o` reader - Output register for GPIO0."]
pub type REG_GPIO_0_O_R = crate::BitReader<GPIO0OUTPUT_A>;
#[doc = "Output register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO0OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_0_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0OUTPUT_A {
        match self.bits {
            false => GPIO0OUTPUT_A::DISABLED,
            true => GPIO0OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO0OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO0OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_0_o` writer - Output register for GPIO0."]
pub type REG_GPIO_0_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO0OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_0_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO0OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO0OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_1_o` reader - Output register for GPIO1."]
pub type REG_GPIO_1_O_R = crate::BitReader<GPIO1OUTPUT_A>;
#[doc = "Output register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO1OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_1_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1OUTPUT_A {
        match self.bits {
            false => GPIO1OUTPUT_A::DISABLED,
            true => GPIO1OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO1OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO1OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_1_o` writer - Output register for GPIO1."]
pub type REG_GPIO_1_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO1OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_1_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO1OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO1OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_2_o` reader - Output register for GPIO2."]
pub type REG_GPIO_2_O_R = crate::BitReader<GPIO2OUTPUT_A>;
#[doc = "Output register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO2OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_2_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2OUTPUT_A {
        match self.bits {
            false => GPIO2OUTPUT_A::DISABLED,
            true => GPIO2OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO2OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO2OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_2_o` writer - Output register for GPIO2."]
pub type REG_GPIO_2_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO2OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_2_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO2OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO2OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_3_o` reader - Output register for GPIO3."]
pub type REG_GPIO_3_O_R = crate::BitReader<GPIO3OUTPUT_A>;
#[doc = "Output register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO3OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO3OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_3_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3OUTPUT_A {
        match self.bits {
            false => GPIO3OUTPUT_A::DISABLED,
            true => GPIO3OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO3OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO3OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_3_o` writer - Output register for GPIO3."]
pub type REG_GPIO_3_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO3OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_3_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO3OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO3OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_4_o` reader - Output register for GPIO4."]
pub type REG_GPIO_4_O_R = crate::BitReader<GPIO4OUTPUT_A>;
#[doc = "Output register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO4OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO4OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_4_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4OUTPUT_A {
        match self.bits {
            false => GPIO4OUTPUT_A::DISABLED,
            true => GPIO4OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO4OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO4OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_4_o` writer - Output register for GPIO4."]
pub type REG_GPIO_4_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO4OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_4_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO4OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO4OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_5_o` reader - Output register for GPIO5."]
pub type REG_GPIO_5_O_R = crate::BitReader<GPIO5OUTPUT_A>;
#[doc = "Output register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO5OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO5OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_5_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5OUTPUT_A {
        match self.bits {
            false => GPIO5OUTPUT_A::DISABLED,
            true => GPIO5OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO5OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO5OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_5_o` writer - Output register for GPIO5."]
pub type REG_GPIO_5_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO5OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_5_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO5OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO5OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_6_o` reader - Output register for GPIO6."]
pub type REG_GPIO_6_O_R = crate::BitReader<GPIO6OUTPUT_A>;
#[doc = "Output register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO6OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO6OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_6_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6OUTPUT_A {
        match self.bits {
            false => GPIO6OUTPUT_A::DISABLED,
            true => GPIO6OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO6OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO6OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_6_o` writer - Output register for GPIO6."]
pub type REG_GPIO_6_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO6OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_6_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO6OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO6OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_7_o` reader - Output register for GPIO7."]
pub type REG_GPIO_7_O_R = crate::BitReader<GPIO7OUTPUT_A>;
#[doc = "Output register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO7OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO7OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_7_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7OUTPUT_A {
        match self.bits {
            false => GPIO7OUTPUT_A::DISABLED,
            true => GPIO7OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO7OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO7OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_7_o` writer - Output register for GPIO7."]
pub type REG_GPIO_7_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO7OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_7_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO7OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO7OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_8_o` reader - Output register for GPIO8."]
pub type REG_GPIO_8_O_R = crate::BitReader<GPIO8OUTPUT_A>;
#[doc = "Output register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO8OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO8OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO8OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_8_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8OUTPUT_A {
        match self.bits {
            false => GPIO8OUTPUT_A::DISABLED,
            true => GPIO8OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO8OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO8OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_8_o` writer - Output register for GPIO8."]
pub type REG_GPIO_8_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO8OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_8_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO8OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO8OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_9_o` reader - Output register for GPIO9."]
pub type REG_GPIO_9_O_R = crate::BitReader<GPIO9OUTPUT_A>;
#[doc = "Output register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO9OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO9OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO9OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_9_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9OUTPUT_A {
        match self.bits {
            false => GPIO9OUTPUT_A::DISABLED,
            true => GPIO9OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO9OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO9OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_9_o` writer - Output register for GPIO9."]
pub type REG_GPIO_9_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO9OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_9_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO9OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO9OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_10_o` reader - Output register for GPIO10."]
pub type REG_GPIO_10_O_R = crate::BitReader<GPIO10OUTPUT_A>;
#[doc = "Output register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO10OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO10OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO10OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_10_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10OUTPUT_A {
        match self.bits {
            false => GPIO10OUTPUT_A::DISABLED,
            true => GPIO10OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO10OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO10OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_10_o` writer - Output register for GPIO10."]
pub type REG_GPIO_10_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO10OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_10_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO10OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO10OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_11_o` reader - Output register for GPIO11."]
pub type REG_GPIO_11_O_R = crate::BitReader<GPIO11OUTPUT_A>;
#[doc = "Output register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO11OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO11OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO11OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_11_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11OUTPUT_A {
        match self.bits {
            false => GPIO11OUTPUT_A::DISABLED,
            true => GPIO11OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO11OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO11OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_11_o` writer - Output register for GPIO11."]
pub type REG_GPIO_11_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO11OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_11_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO11OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO11OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_12_o` reader - Output register for GPIO12."]
pub type REG_GPIO_12_O_R = crate::BitReader<GPIO12OUTPUT_A>;
#[doc = "Output register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO12OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO12OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_12_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12OUTPUT_A {
        match self.bits {
            false => GPIO12OUTPUT_A::DISABLED,
            true => GPIO12OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO12OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO12OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_12_o` writer - Output register for GPIO12."]
pub type REG_GPIO_12_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO12OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_12_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO12OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO12OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_13_o` reader - Output register for GPIO13."]
pub type REG_GPIO_13_O_R = crate::BitReader<GPIO13OUTPUT_A>;
#[doc = "Output register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO13OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO13OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_13_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13OUTPUT_A {
        match self.bits {
            false => GPIO13OUTPUT_A::DISABLED,
            true => GPIO13OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO13OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO13OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_13_o` writer - Output register for GPIO13."]
pub type REG_GPIO_13_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO13OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_13_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO13OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO13OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_14_o` reader - Output register for GPIO14."]
pub type REG_GPIO_14_O_R = crate::BitReader<GPIO14OUTPUT_A>;
#[doc = "Output register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO14OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO14OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_14_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14OUTPUT_A {
        match self.bits {
            false => GPIO14OUTPUT_A::DISABLED,
            true => GPIO14OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO14OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO14OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_14_o` writer - Output register for GPIO14."]
pub type REG_GPIO_14_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO14OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_14_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO14OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO14OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_15_o` reader - Output register for GPIO15."]
pub type REG_GPIO_15_O_R = crate::BitReader<GPIO15OUTPUT_A>;
#[doc = "Output register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO15OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO15OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_15_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15OUTPUT_A {
        match self.bits {
            false => GPIO15OUTPUT_A::DISABLED,
            true => GPIO15OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO15OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO15OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_15_o` writer - Output register for GPIO15."]
pub type REG_GPIO_15_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO15OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_15_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO15OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO15OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_16_o` reader - Output register for GPIO16."]
pub type REG_GPIO_16_O_R = crate::BitReader<GPIO16OUTPUT_A>;
#[doc = "Output register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO16OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO16OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO16OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_16_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16OUTPUT_A {
        match self.bits {
            false => GPIO16OUTPUT_A::DISABLED,
            true => GPIO16OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO16OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO16OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_16_o` writer - Output register for GPIO16."]
pub type REG_GPIO_16_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO16OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_16_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO16OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO16OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_17_o` reader - Output register for GPIO17."]
pub type REG_GPIO_17_O_R = crate::BitReader<GPIO17OUTPUT_A>;
#[doc = "Output register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO17OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO17OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO17OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_17_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17OUTPUT_A {
        match self.bits {
            false => GPIO17OUTPUT_A::DISABLED,
            true => GPIO17OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO17OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO17OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_17_o` writer - Output register for GPIO17."]
pub type REG_GPIO_17_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO17OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_17_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO17OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO17OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_18_o` reader - Output register for GPIO18."]
pub type REG_GPIO_18_O_R = crate::BitReader<GPIO18OUTPUT_A>;
#[doc = "Output register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO18OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO18OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_18_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18OUTPUT_A {
        match self.bits {
            false => GPIO18OUTPUT_A::DISABLED,
            true => GPIO18OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO18OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO18OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_18_o` writer - Output register for GPIO18."]
pub type REG_GPIO_18_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO18OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_18_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO18OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO18OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_19_o` reader - Output register for GPIO19."]
pub type REG_GPIO_19_O_R = crate::BitReader<GPIO19OUTPUT_A>;
#[doc = "Output register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO19OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO19OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_19_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19OUTPUT_A {
        match self.bits {
            false => GPIO19OUTPUT_A::DISABLED,
            true => GPIO19OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO19OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO19OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_19_o` writer - Output register for GPIO19."]
pub type REG_GPIO_19_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO19OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_19_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO19OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO19OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_20_o` reader - Output register for GPIO20."]
pub type REG_GPIO_20_O_R = crate::BitReader<GPIO20OUTPUT_A>;
#[doc = "Output register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO20OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO20OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_20_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20OUTPUT_A {
        match self.bits {
            false => GPIO20OUTPUT_A::DISABLED,
            true => GPIO20OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO20OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO20OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_20_o` writer - Output register for GPIO20."]
pub type REG_GPIO_20_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO20OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_20_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO20OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO20OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_21_o` reader - Output register for GPIO21."]
pub type REG_GPIO_21_O_R = crate::BitReader<GPIO21OUTPUT_A>;
#[doc = "Output register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO21OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO21OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_21_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21OUTPUT_A {
        match self.bits {
            false => GPIO21OUTPUT_A::DISABLED,
            true => GPIO21OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO21OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO21OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_21_o` writer - Output register for GPIO21."]
pub type REG_GPIO_21_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO21OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_21_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO21OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO21OUTPUT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_22_o` reader - Output register for GPIO22."]
pub type REG_GPIO_22_O_R = crate::BitReader<GPIO22OUTPUT_A>;
#[doc = "Output register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO22OUTPUT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO22OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_22_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22OUTPUT_A {
        match self.bits {
            false => GPIO22OUTPUT_A::DISABLED,
            true => GPIO22OUTPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO22OUTPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO22OUTPUT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_22_o` writer - Output register for GPIO22."]
pub type REG_GPIO_22_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, GPIO22OUTPUT_A, O>;
impl<'a, const O: u8> REG_GPIO_22_O_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO22OUTPUT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO22OUTPUT_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Output register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_o(&self) -> REG_GPIO_0_O_R {
        REG_GPIO_0_O_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_o(&self) -> REG_GPIO_1_O_R {
        REG_GPIO_1_O_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_o(&self) -> REG_GPIO_2_O_R {
        REG_GPIO_2_O_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_o(&self) -> REG_GPIO_3_O_R {
        REG_GPIO_3_O_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_o(&self) -> REG_GPIO_4_O_R {
        REG_GPIO_4_O_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_o(&self) -> REG_GPIO_5_O_R {
        REG_GPIO_5_O_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_o(&self) -> REG_GPIO_6_O_R {
        REG_GPIO_6_O_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_o(&self) -> REG_GPIO_7_O_R {
        REG_GPIO_7_O_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_o(&self) -> REG_GPIO_8_O_R {
        REG_GPIO_8_O_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_o(&self) -> REG_GPIO_9_O_R {
        REG_GPIO_9_O_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_o(&self) -> REG_GPIO_10_O_R {
        REG_GPIO_10_O_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_o(&self) -> REG_GPIO_11_O_R {
        REG_GPIO_11_O_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_o(&self) -> REG_GPIO_12_O_R {
        REG_GPIO_12_O_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_o(&self) -> REG_GPIO_13_O_R {
        REG_GPIO_13_O_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_o(&self) -> REG_GPIO_14_O_R {
        REG_GPIO_14_O_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_o(&self) -> REG_GPIO_15_O_R {
        REG_GPIO_15_O_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_o(&self) -> REG_GPIO_16_O_R {
        REG_GPIO_16_O_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_o(&self) -> REG_GPIO_17_O_R {
        REG_GPIO_17_O_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_o(&self) -> REG_GPIO_18_O_R {
        REG_GPIO_18_O_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_o(&self) -> REG_GPIO_19_O_R {
        REG_GPIO_19_O_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_o(&self) -> REG_GPIO_20_O_R {
        REG_GPIO_20_O_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_o(&self) -> REG_GPIO_21_O_R {
        REG_GPIO_21_O_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_o(&self) -> REG_GPIO_22_O_R {
        REG_GPIO_22_O_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output register for GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_o(&mut self) -> REG_GPIO_0_O_W<0> {
        REG_GPIO_0_O_W::new(self)
    }
    #[doc = "Bit 1 - Output register for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_o(&mut self) -> REG_GPIO_1_O_W<1> {
        REG_GPIO_1_O_W::new(self)
    }
    #[doc = "Bit 2 - Output register for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_o(&mut self) -> REG_GPIO_2_O_W<2> {
        REG_GPIO_2_O_W::new(self)
    }
    #[doc = "Bit 3 - Output register for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_o(&mut self) -> REG_GPIO_3_O_W<3> {
        REG_GPIO_3_O_W::new(self)
    }
    #[doc = "Bit 4 - Output register for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_o(&mut self) -> REG_GPIO_4_O_W<4> {
        REG_GPIO_4_O_W::new(self)
    }
    #[doc = "Bit 5 - Output register for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_o(&mut self) -> REG_GPIO_5_O_W<5> {
        REG_GPIO_5_O_W::new(self)
    }
    #[doc = "Bit 6 - Output register for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_o(&mut self) -> REG_GPIO_6_O_W<6> {
        REG_GPIO_6_O_W::new(self)
    }
    #[doc = "Bit 7 - Output register for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_o(&mut self) -> REG_GPIO_7_O_W<7> {
        REG_GPIO_7_O_W::new(self)
    }
    #[doc = "Bit 8 - Output register for GPIO8."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_8_o(&mut self) -> REG_GPIO_8_O_W<8> {
        REG_GPIO_8_O_W::new(self)
    }
    #[doc = "Bit 9 - Output register for GPIO9."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_9_o(&mut self) -> REG_GPIO_9_O_W<9> {
        REG_GPIO_9_O_W::new(self)
    }
    #[doc = "Bit 10 - Output register for GPIO10."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_10_o(&mut self) -> REG_GPIO_10_O_W<10> {
        REG_GPIO_10_O_W::new(self)
    }
    #[doc = "Bit 11 - Output register for GPIO11."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_11_o(&mut self) -> REG_GPIO_11_O_W<11> {
        REG_GPIO_11_O_W::new(self)
    }
    #[doc = "Bit 12 - Output register for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_o(&mut self) -> REG_GPIO_12_O_W<12> {
        REG_GPIO_12_O_W::new(self)
    }
    #[doc = "Bit 13 - Output register for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_o(&mut self) -> REG_GPIO_13_O_W<13> {
        REG_GPIO_13_O_W::new(self)
    }
    #[doc = "Bit 14 - Output register for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_o(&mut self) -> REG_GPIO_14_O_W<14> {
        REG_GPIO_14_O_W::new(self)
    }
    #[doc = "Bit 15 - Output register for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_o(&mut self) -> REG_GPIO_15_O_W<15> {
        REG_GPIO_15_O_W::new(self)
    }
    #[doc = "Bit 16 - Output register for GPIO16."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_16_o(&mut self) -> REG_GPIO_16_O_W<16> {
        REG_GPIO_16_O_W::new(self)
    }
    #[doc = "Bit 17 - Output register for GPIO17."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_17_o(&mut self) -> REG_GPIO_17_O_W<17> {
        REG_GPIO_17_O_W::new(self)
    }
    #[doc = "Bit 18 - Output register for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_o(&mut self) -> REG_GPIO_18_O_W<18> {
        REG_GPIO_18_O_W::new(self)
    }
    #[doc = "Bit 19 - Output register for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_o(&mut self) -> REG_GPIO_19_O_W<19> {
        REG_GPIO_19_O_W::new(self)
    }
    #[doc = "Bit 20 - Output register for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_o(&mut self) -> REG_GPIO_20_O_W<20> {
        REG_GPIO_20_O_W::new(self)
    }
    #[doc = "Bit 21 - Output register for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_o(&mut self) -> REG_GPIO_21_O_W<21> {
        REG_GPIO_21_O_W::new(self)
    }
    #[doc = "Bit 22 - Output register for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_o(&mut self) -> REG_GPIO_22_O_W<22> {
        REG_GPIO_22_O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output register for all GPIO pins. Output Enabled bit must be set in Output Enable register to work.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl32](index.html) module"]
pub struct GPIO_CFGCTL32_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl32::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl32::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL32 to value 0"]
impl crate::Resettable for GPIO_CFGCTL32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

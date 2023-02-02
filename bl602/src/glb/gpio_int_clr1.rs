#[doc = "Register `GPIO_INT_CLR1` reader"]
pub struct R(crate::R<GPIO_INT_CLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_CLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_CLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_CLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_CLR1` writer"]
pub struct W(crate::W<GPIO_INT_CLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_CLR1_SPEC>;
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
impl From<crate::W<GPIO_INT_CLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_CLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_0_interrupt_clear` reader - Interrupt clearing register for GPIO0."]
pub type REG_GPIO_0_INTERRUPT_CLEAR_R = crate::BitReader<GPIO0INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO0INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_0_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO0INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO0INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO0INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO0INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_0_interrupt_clear` writer - Interrupt clearing register for GPIO0."]
pub type REG_GPIO_0_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO0INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_0_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO0INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO0INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_1_interrupt_clear` reader - Interrupt clearing register for GPIO1."]
pub type REG_GPIO_1_INTERRUPT_CLEAR_R = crate::BitReader<GPIO1INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO1INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_1_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO1INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO1INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO1INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO1INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_1_interrupt_clear` writer - Interrupt clearing register for GPIO1."]
pub type REG_GPIO_1_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO1INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_1_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO1INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO1INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_2_interrupt_clear` reader - Interrupt clearing register for GPIO2."]
pub type REG_GPIO_2_INTERRUPT_CLEAR_R = crate::BitReader<GPIO2INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO2INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_2_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO2INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO2INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO2INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO2INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_2_interrupt_clear` writer - Interrupt clearing register for GPIO2."]
pub type REG_GPIO_2_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO2INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_2_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO2INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO2INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_3_interrupt_clear` reader - Interrupt clearing register for GPIO3."]
pub type REG_GPIO_3_INTERRUPT_CLEAR_R = crate::BitReader<GPIO3INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO3INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO3INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_3_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO3INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO3INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO3INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO3INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_3_interrupt_clear` writer - Interrupt clearing register for GPIO3."]
pub type REG_GPIO_3_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO3INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_3_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO3INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO3INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_4_interrupt_clear` reader - Interrupt clearing register for GPIO4."]
pub type REG_GPIO_4_INTERRUPT_CLEAR_R = crate::BitReader<GPIO4INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO4INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO4INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_4_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO4INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO4INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO4INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO4INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_4_interrupt_clear` writer - Interrupt clearing register for GPIO4."]
pub type REG_GPIO_4_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO4INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_4_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO4INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO4INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_5_interrupt_clear` reader - Interrupt clearing register for GPIO5."]
pub type REG_GPIO_5_INTERRUPT_CLEAR_R = crate::BitReader<GPIO5INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO5INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO5INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_5_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO5INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO5INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO5INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO5INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_5_interrupt_clear` writer - Interrupt clearing register for GPIO5."]
pub type REG_GPIO_5_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO5INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_5_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO5INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO5INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_6_interrupt_clear` reader - Interrupt clearing register for GPIO6."]
pub type REG_GPIO_6_INTERRUPT_CLEAR_R = crate::BitReader<GPIO6INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO6INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO6INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_6_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO6INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO6INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO6INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO6INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_6_interrupt_clear` writer - Interrupt clearing register for GPIO6."]
pub type REG_GPIO_6_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO6INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_6_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO6INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO6INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_7_interrupt_clear` reader - Interrupt clearing register for GPIO7."]
pub type REG_GPIO_7_INTERRUPT_CLEAR_R = crate::BitReader<GPIO7INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO7INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO7INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_7_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO7INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO7INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO7INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO7INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_7_interrupt_clear` writer - Interrupt clearing register for GPIO7."]
pub type REG_GPIO_7_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO7INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_7_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO7INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO7INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_8_interrupt_clear` reader - Interrupt clearing register for GPIO8."]
pub type REG_GPIO_8_INTERRUPT_CLEAR_R = crate::BitReader<GPIO8INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO8INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO8INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO8INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_8_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO8INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO8INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO8INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO8INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_8_interrupt_clear` writer - Interrupt clearing register for GPIO8."]
pub type REG_GPIO_8_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO8INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_8_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO8INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO8INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_9_interrupt_clear` reader - Interrupt clearing register for GPIO9."]
pub type REG_GPIO_9_INTERRUPT_CLEAR_R = crate::BitReader<GPIO9INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO9INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO9INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO9INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_9_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO9INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO9INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO9INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO9INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_9_interrupt_clear` writer - Interrupt clearing register for GPIO9."]
pub type REG_GPIO_9_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO9INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_9_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO9INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO9INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_10_interrupt_clear` reader - Interrupt clearing register for GPIO10."]
pub type REG_GPIO_10_INTERRUPT_CLEAR_R = crate::BitReader<GPIO10INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO10INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO10INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO10INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_10_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO10INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO10INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO10INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO10INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_10_interrupt_clear` writer - Interrupt clearing register for GPIO10."]
pub type REG_GPIO_10_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO10INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_10_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO10INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO10INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_11_interrupt_clear` reader - Interrupt clearing register for GPIO11."]
pub type REG_GPIO_11_INTERRUPT_CLEAR_R = crate::BitReader<GPIO11INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO11INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO11INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO11INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_11_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO11INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO11INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO11INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO11INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_11_interrupt_clear` writer - Interrupt clearing register for GPIO11."]
pub type REG_GPIO_11_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO11INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_11_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO11INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO11INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_12_interrupt_clear` reader - Interrupt clearing register for GPIO12."]
pub type REG_GPIO_12_INTERRUPT_CLEAR_R = crate::BitReader<GPIO12INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO12INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO12INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_12_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO12INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO12INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO12INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO12INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_12_interrupt_clear` writer - Interrupt clearing register for GPIO12."]
pub type REG_GPIO_12_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO12INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_12_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO12INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO12INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_13_interrupt_clear` reader - Interrupt clearing register for GPIO13."]
pub type REG_GPIO_13_INTERRUPT_CLEAR_R = crate::BitReader<GPIO13INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO13INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO13INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_13_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO13INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO13INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO13INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO13INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_13_interrupt_clear` writer - Interrupt clearing register for GPIO13."]
pub type REG_GPIO_13_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO13INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_13_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO13INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO13INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_14_interrupt_clear` reader - Interrupt clearing register for GPIO14."]
pub type REG_GPIO_14_INTERRUPT_CLEAR_R = crate::BitReader<GPIO14INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO14INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO14INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_14_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO14INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO14INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO14INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO14INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_14_interrupt_clear` writer - Interrupt clearing register for GPIO14."]
pub type REG_GPIO_14_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO14INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_14_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO14INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO14INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_15_interrupt_clear` reader - Interrupt clearing register for GPIO15."]
pub type REG_GPIO_15_INTERRUPT_CLEAR_R = crate::BitReader<GPIO15INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO15INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO15INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_15_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO15INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO15INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO15INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO15INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_15_interrupt_clear` writer - Interrupt clearing register for GPIO15."]
pub type REG_GPIO_15_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO15INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_15_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO15INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO15INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_16_interrupt_clear` reader - Interrupt clearing register for GPIO16."]
pub type REG_GPIO_16_INTERRUPT_CLEAR_R = crate::BitReader<GPIO16INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO16INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO16INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO16INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_16_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO16INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO16INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO16INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO16INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_16_interrupt_clear` writer - Interrupt clearing register for GPIO16."]
pub type REG_GPIO_16_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO16INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_16_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO16INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO16INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_17_interrupt_clear` reader - Interrupt clearing register for GPIO17."]
pub type REG_GPIO_17_INTERRUPT_CLEAR_R = crate::BitReader<GPIO17INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO17INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO17INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO17INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_17_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO17INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO17INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO17INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO17INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_17_interrupt_clear` writer - Interrupt clearing register for GPIO17."]
pub type REG_GPIO_17_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO17INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_17_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO17INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO17INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_18_interrupt_clear` reader - Interrupt clearing register for GPIO18."]
pub type REG_GPIO_18_INTERRUPT_CLEAR_R = crate::BitReader<GPIO18INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO18INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO18INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_18_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO18INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO18INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO18INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO18INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_18_interrupt_clear` writer - Interrupt clearing register for GPIO18."]
pub type REG_GPIO_18_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO18INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_18_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO18INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO18INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_19_interrupt_clear` reader - Interrupt clearing register for GPIO19."]
pub type REG_GPIO_19_INTERRUPT_CLEAR_R = crate::BitReader<GPIO19INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO19INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO19INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_19_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO19INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO19INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO19INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO19INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_19_interrupt_clear` writer - Interrupt clearing register for GPIO19."]
pub type REG_GPIO_19_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO19INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_19_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO19INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO19INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_20_interrupt_clear` reader - Interrupt clearing register for GPIO20."]
pub type REG_GPIO_20_INTERRUPT_CLEAR_R = crate::BitReader<GPIO20INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO20INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO20INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_20_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO20INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO20INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO20INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO20INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_20_interrupt_clear` writer - Interrupt clearing register for GPIO20."]
pub type REG_GPIO_20_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO20INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_20_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO20INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO20INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_21_interrupt_clear` reader - Interrupt clearing register for GPIO21."]
pub type REG_GPIO_21_INTERRUPT_CLEAR_R = crate::BitReader<GPIO21INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO21INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO21INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_21_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO21INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO21INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO21INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO21INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_21_interrupt_clear` writer - Interrupt clearing register for GPIO21."]
pub type REG_GPIO_21_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO21INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_21_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO21INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO21INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_22_interrupt_clear` reader - Interrupt clearing register for GPIO22."]
pub type REG_GPIO_22_INTERRUPT_CLEAR_R = crate::BitReader<GPIO22INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO22INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO22INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_22_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO22INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO22INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO22INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO22INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_22_interrupt_clear` writer - Interrupt clearing register for GPIO22."]
pub type REG_GPIO_22_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO22INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_22_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO22INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO22INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_23_interrupt_clear` reader - Interrupt clearing register for GPIO23."]
pub type REG_GPIO_23_INTERRUPT_CLEAR_R = crate::BitReader<GPIO23INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO23INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO23INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_23_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO23INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO23INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO23INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO23INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_23_interrupt_clear` writer - Interrupt clearing register for GPIO23."]
pub type REG_GPIO_23_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO23INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_23_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO23INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO23INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_24_interrupt_clear` reader - Interrupt clearing register for GPIO24."]
pub type REG_GPIO_24_INTERRUPT_CLEAR_R = crate::BitReader<GPIO24INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO24INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO24INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO24INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_24_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO24INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO24INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO24INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO24INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_24_interrupt_clear` writer - Interrupt clearing register for GPIO24."]
pub type REG_GPIO_24_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO24INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_24_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO24INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO24INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_25_interrupt_clear` reader - Interrupt clearing register for GPIO25."]
pub type REG_GPIO_25_INTERRUPT_CLEAR_R = crate::BitReader<GPIO25INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO25INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO25INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO25INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_25_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO25INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO25INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO25INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO25INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_25_interrupt_clear` writer - Interrupt clearing register for GPIO25."]
pub type REG_GPIO_25_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO25INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_25_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO25INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO25INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_26_interrupt_clear` reader - Interrupt clearing register for GPIO26."]
pub type REG_GPIO_26_INTERRUPT_CLEAR_R = crate::BitReader<GPIO26INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO26INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO26INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO26INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_26_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO26INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO26INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO26INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO26INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_26_interrupt_clear` writer - Interrupt clearing register for GPIO26."]
pub type REG_GPIO_26_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO26INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_26_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO26INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO26INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_27_interrupt_clear` reader - Interrupt clearing register for GPIO27."]
pub type REG_GPIO_27_INTERRUPT_CLEAR_R = crate::BitReader<GPIO27INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO27INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO27INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO27INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_27_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO27INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO27INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO27INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO27INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_27_interrupt_clear` writer - Interrupt clearing register for GPIO27."]
pub type REG_GPIO_27_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO27INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_27_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO27INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO27INTERRUPT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `reg_gpio_28_interrupt_clear` reader - Interrupt clearing register for GPIO28."]
pub type REG_GPIO_28_INTERRUPT_CLEAR_R = crate::BitReader<GPIO28INTERRUPT_CLEAR_A>;
#[doc = "Interrupt clearing register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO28INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<GPIO28INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO28INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_28_INTERRUPT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28INTERRUPT_CLEAR_A {
        match self.bits {
            false => GPIO28INTERRUPT_CLEAR_A::NO_CLEAR,
            true => GPIO28INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == GPIO28INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO28INTERRUPT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `reg_gpio_28_interrupt_clear` writer - Interrupt clearing register for GPIO28."]
pub type REG_GPIO_28_INTERRUPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_CLR1_SPEC, GPIO28INTERRUPT_CLEAR_A, O>;
impl<'a, const O: u8> REG_GPIO_28_INTERRUPT_CLEAR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(GPIO28INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO28INTERRUPT_CLEAR_A::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt clearing register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_clear(&self) -> REG_GPIO_0_INTERRUPT_CLEAR_R {
        REG_GPIO_0_INTERRUPT_CLEAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt clearing register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_clear(&self) -> REG_GPIO_1_INTERRUPT_CLEAR_R {
        REG_GPIO_1_INTERRUPT_CLEAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt clearing register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_clear(&self) -> REG_GPIO_2_INTERRUPT_CLEAR_R {
        REG_GPIO_2_INTERRUPT_CLEAR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt clearing register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_clear(&self) -> REG_GPIO_3_INTERRUPT_CLEAR_R {
        REG_GPIO_3_INTERRUPT_CLEAR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt clearing register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_clear(&self) -> REG_GPIO_4_INTERRUPT_CLEAR_R {
        REG_GPIO_4_INTERRUPT_CLEAR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt clearing register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_clear(&self) -> REG_GPIO_5_INTERRUPT_CLEAR_R {
        REG_GPIO_5_INTERRUPT_CLEAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt clearing register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_clear(&self) -> REG_GPIO_6_INTERRUPT_CLEAR_R {
        REG_GPIO_6_INTERRUPT_CLEAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt clearing register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_clear(&self) -> REG_GPIO_7_INTERRUPT_CLEAR_R {
        REG_GPIO_7_INTERRUPT_CLEAR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt clearing register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_clear(&self) -> REG_GPIO_8_INTERRUPT_CLEAR_R {
        REG_GPIO_8_INTERRUPT_CLEAR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt clearing register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_clear(&self) -> REG_GPIO_9_INTERRUPT_CLEAR_R {
        REG_GPIO_9_INTERRUPT_CLEAR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt clearing register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_clear(&self) -> REG_GPIO_10_INTERRUPT_CLEAR_R {
        REG_GPIO_10_INTERRUPT_CLEAR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt clearing register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_clear(&self) -> REG_GPIO_11_INTERRUPT_CLEAR_R {
        REG_GPIO_11_INTERRUPT_CLEAR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt clearing register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_clear(&self) -> REG_GPIO_12_INTERRUPT_CLEAR_R {
        REG_GPIO_12_INTERRUPT_CLEAR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt clearing register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_clear(&self) -> REG_GPIO_13_INTERRUPT_CLEAR_R {
        REG_GPIO_13_INTERRUPT_CLEAR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt clearing register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_clear(&self) -> REG_GPIO_14_INTERRUPT_CLEAR_R {
        REG_GPIO_14_INTERRUPT_CLEAR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt clearing register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_clear(&self) -> REG_GPIO_15_INTERRUPT_CLEAR_R {
        REG_GPIO_15_INTERRUPT_CLEAR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt clearing register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_clear(&self) -> REG_GPIO_16_INTERRUPT_CLEAR_R {
        REG_GPIO_16_INTERRUPT_CLEAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt clearing register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_clear(&self) -> REG_GPIO_17_INTERRUPT_CLEAR_R {
        REG_GPIO_17_INTERRUPT_CLEAR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt clearing register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_clear(&self) -> REG_GPIO_18_INTERRUPT_CLEAR_R {
        REG_GPIO_18_INTERRUPT_CLEAR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt clearing register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_clear(&self) -> REG_GPIO_19_INTERRUPT_CLEAR_R {
        REG_GPIO_19_INTERRUPT_CLEAR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt clearing register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_clear(&self) -> REG_GPIO_20_INTERRUPT_CLEAR_R {
        REG_GPIO_20_INTERRUPT_CLEAR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt clearing register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_clear(&self) -> REG_GPIO_21_INTERRUPT_CLEAR_R {
        REG_GPIO_21_INTERRUPT_CLEAR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt clearing register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_clear(&self) -> REG_GPIO_22_INTERRUPT_CLEAR_R {
        REG_GPIO_22_INTERRUPT_CLEAR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt clearing register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_clear(&self) -> REG_GPIO_23_INTERRUPT_CLEAR_R {
        REG_GPIO_23_INTERRUPT_CLEAR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt clearing register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_clear(&self) -> REG_GPIO_24_INTERRUPT_CLEAR_R {
        REG_GPIO_24_INTERRUPT_CLEAR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt clearing register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_clear(&self) -> REG_GPIO_25_INTERRUPT_CLEAR_R {
        REG_GPIO_25_INTERRUPT_CLEAR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt clearing register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_clear(&self) -> REG_GPIO_26_INTERRUPT_CLEAR_R {
        REG_GPIO_26_INTERRUPT_CLEAR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt clearing register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_clear(&self) -> REG_GPIO_27_INTERRUPT_CLEAR_R {
        REG_GPIO_27_INTERRUPT_CLEAR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt clearing register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_clear(&self) -> REG_GPIO_28_INTERRUPT_CLEAR_R {
        REG_GPIO_28_INTERRUPT_CLEAR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt clearing register for GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_interrupt_clear(&mut self) -> REG_GPIO_0_INTERRUPT_CLEAR_W<0> {
        REG_GPIO_0_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt clearing register for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_interrupt_clear(&mut self) -> REG_GPIO_1_INTERRUPT_CLEAR_W<1> {
        REG_GPIO_1_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt clearing register for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_interrupt_clear(&mut self) -> REG_GPIO_2_INTERRUPT_CLEAR_W<2> {
        REG_GPIO_2_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt clearing register for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_interrupt_clear(&mut self) -> REG_GPIO_3_INTERRUPT_CLEAR_W<3> {
        REG_GPIO_3_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt clearing register for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_interrupt_clear(&mut self) -> REG_GPIO_4_INTERRUPT_CLEAR_W<4> {
        REG_GPIO_4_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt clearing register for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_interrupt_clear(&mut self) -> REG_GPIO_5_INTERRUPT_CLEAR_W<5> {
        REG_GPIO_5_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt clearing register for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_interrupt_clear(&mut self) -> REG_GPIO_6_INTERRUPT_CLEAR_W<6> {
        REG_GPIO_6_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt clearing register for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_interrupt_clear(&mut self) -> REG_GPIO_7_INTERRUPT_CLEAR_W<7> {
        REG_GPIO_7_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt clearing register for GPIO8."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_8_interrupt_clear(&mut self) -> REG_GPIO_8_INTERRUPT_CLEAR_W<8> {
        REG_GPIO_8_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt clearing register for GPIO9."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_9_interrupt_clear(&mut self) -> REG_GPIO_9_INTERRUPT_CLEAR_W<9> {
        REG_GPIO_9_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt clearing register for GPIO10."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_10_interrupt_clear(&mut self) -> REG_GPIO_10_INTERRUPT_CLEAR_W<10> {
        REG_GPIO_10_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt clearing register for GPIO11."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_11_interrupt_clear(&mut self) -> REG_GPIO_11_INTERRUPT_CLEAR_W<11> {
        REG_GPIO_11_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt clearing register for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_interrupt_clear(&mut self) -> REG_GPIO_12_INTERRUPT_CLEAR_W<12> {
        REG_GPIO_12_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt clearing register for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_interrupt_clear(&mut self) -> REG_GPIO_13_INTERRUPT_CLEAR_W<13> {
        REG_GPIO_13_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt clearing register for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_interrupt_clear(&mut self) -> REG_GPIO_14_INTERRUPT_CLEAR_W<14> {
        REG_GPIO_14_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt clearing register for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_interrupt_clear(&mut self) -> REG_GPIO_15_INTERRUPT_CLEAR_W<15> {
        REG_GPIO_15_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt clearing register for GPIO16."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_16_interrupt_clear(&mut self) -> REG_GPIO_16_INTERRUPT_CLEAR_W<16> {
        REG_GPIO_16_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt clearing register for GPIO17."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_17_interrupt_clear(&mut self) -> REG_GPIO_17_INTERRUPT_CLEAR_W<17> {
        REG_GPIO_17_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt clearing register for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_interrupt_clear(&mut self) -> REG_GPIO_18_INTERRUPT_CLEAR_W<18> {
        REG_GPIO_18_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt clearing register for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_interrupt_clear(&mut self) -> REG_GPIO_19_INTERRUPT_CLEAR_W<19> {
        REG_GPIO_19_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt clearing register for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_interrupt_clear(&mut self) -> REG_GPIO_20_INTERRUPT_CLEAR_W<20> {
        REG_GPIO_20_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt clearing register for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_interrupt_clear(&mut self) -> REG_GPIO_21_INTERRUPT_CLEAR_W<21> {
        REG_GPIO_21_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt clearing register for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_interrupt_clear(&mut self) -> REG_GPIO_22_INTERRUPT_CLEAR_W<22> {
        REG_GPIO_22_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt clearing register for GPIO23."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_interrupt_clear(&mut self) -> REG_GPIO_23_INTERRUPT_CLEAR_W<23> {
        REG_GPIO_23_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt clearing register for GPIO24."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_interrupt_clear(&mut self) -> REG_GPIO_24_INTERRUPT_CLEAR_W<24> {
        REG_GPIO_24_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt clearing register for GPIO25."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_interrupt_clear(&mut self) -> REG_GPIO_25_INTERRUPT_CLEAR_W<25> {
        REG_GPIO_25_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt clearing register for GPIO26."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_interrupt_clear(&mut self) -> REG_GPIO_26_INTERRUPT_CLEAR_W<26> {
        REG_GPIO_26_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt clearing register for GPIO27."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_interrupt_clear(&mut self) -> REG_GPIO_27_INTERRUPT_CLEAR_W<27> {
        REG_GPIO_27_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt clearing register for GPIO28."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_interrupt_clear(&mut self) -> REG_GPIO_28_INTERRUPT_CLEAR_W<28> {
        REG_GPIO_28_INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clearing register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_clr1](index.html) module"]
pub struct GPIO_INT_CLR1_SPEC;
impl crate::RegisterSpec for GPIO_INT_CLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_clr1::R](R) reader structure"]
impl crate::Readable for GPIO_INT_CLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_clr1::W](W) writer structure"]
impl crate::Writable for GPIO_INT_CLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_INT_CLR1 to value 0"]
impl crate::Resettable for GPIO_INT_CLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

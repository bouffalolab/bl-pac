#[doc = "Register `GPIO_INT_MODE_SET1` reader"]
pub struct R(crate::R<GPIO_INT_MODE_SET1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_MODE_SET1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_MODE_SET1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_MODE_SET1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_MODE_SET1` writer"]
pub struct W(crate::W<GPIO_INT_MODE_SET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_MODE_SET1_SPEC>;
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
impl From<crate::W<GPIO_INT_MODE_SET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_MODE_SET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_0_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO0."]
pub type REG_GPIO_0_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO0TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO0TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO0TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO0TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_0_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO0TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO0TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO0TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO0TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO0TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO0TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO0TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO0TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_0_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO0."]
pub type REG_GPIO_0_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET1_SPEC, u8, GPIO0TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_0_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO0TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO0TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO0TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO0TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_0_interrupt_control_mode` reader - Interrupt control mode register for GPIO0."]
pub type REG_GPIO_0_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO0CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO0CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_0_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0CONTROL_MODE_A {
        match self.bits {
            false => GPIO0CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO0CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO0CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO0CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_0_interrupt_control_mode` writer - Interrupt control mode register for GPIO0."]
pub type REG_GPIO_0_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET1_SPEC, GPIO0CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_0_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO0CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO0CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_1_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO1."]
pub type REG_GPIO_1_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO1TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO1TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO1TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO1TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_1_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO1TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO1TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO1TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO1TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO1TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO1TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO1TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO1TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_1_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO1."]
pub type REG_GPIO_1_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET1_SPEC, u8, GPIO1TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_1_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO1TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO1TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO1TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO1TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_1_interrupt_control_mode` reader - Interrupt control mode register for GPIO1."]
pub type REG_GPIO_1_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO1CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO1CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_1_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1CONTROL_MODE_A {
        match self.bits {
            false => GPIO1CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO1CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO1CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO1CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_1_interrupt_control_mode` writer - Interrupt control mode register for GPIO1."]
pub type REG_GPIO_1_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET1_SPEC, GPIO1CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_1_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO1CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO1CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_2_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO2."]
pub type REG_GPIO_2_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO2TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO2TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO2TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO2TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_2_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO2TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO2TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO2TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO2TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO2TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO2TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO2TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO2TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_2_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO2."]
pub type REG_GPIO_2_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET1_SPEC, u8, GPIO2TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_2_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO2TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO2TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO2TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO2TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_2_interrupt_control_mode` reader - Interrupt control mode register for GPIO2."]
pub type REG_GPIO_2_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO2CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO2CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_2_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2CONTROL_MODE_A {
        match self.bits {
            false => GPIO2CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO2CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO2CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO2CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_2_interrupt_control_mode` writer - Interrupt control mode register for GPIO2."]
pub type REG_GPIO_2_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET1_SPEC, GPIO2CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_2_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO2CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO2CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_3_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO3."]
pub type REG_GPIO_3_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO3TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO3TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO3TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO3TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_3_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO3TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO3TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO3TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO3TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO3TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO3TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO3TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO3TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_3_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO3."]
pub type REG_GPIO_3_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET1_SPEC, u8, GPIO3TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_3_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO3TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO3TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO3TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO3TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_3_interrupt_control_mode` reader - Interrupt control mode register for GPIO3."]
pub type REG_GPIO_3_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO3CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO3CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO3CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_3_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3CONTROL_MODE_A {
        match self.bits {
            false => GPIO3CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO3CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO3CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO3CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_3_interrupt_control_mode` writer - Interrupt control mode register for GPIO3."]
pub type REG_GPIO_3_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET1_SPEC, GPIO3CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_3_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO3CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO3CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_4_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO4."]
pub type REG_GPIO_4_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO4TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO4TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO4TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO4TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_4_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO4TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO4TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO4TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO4TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO4TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO4TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO4TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO4TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_4_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO4."]
pub type REG_GPIO_4_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET1_SPEC, u8, GPIO4TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_4_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO4TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO4TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO4TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO4TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_4_interrupt_control_mode` reader - Interrupt control mode register for GPIO4."]
pub type REG_GPIO_4_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO4CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO4CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO4CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_4_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4CONTROL_MODE_A {
        match self.bits {
            false => GPIO4CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO4CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO4CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO4CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_4_interrupt_control_mode` writer - Interrupt control mode register for GPIO4."]
pub type REG_GPIO_4_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET1_SPEC, GPIO4CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_4_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO4CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO4CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_5_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO5."]
pub type REG_GPIO_5_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO5TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO5TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO5TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO5TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_5_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO5TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO5TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO5TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO5TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO5TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO5TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO5TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO5TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_5_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO5."]
pub type REG_GPIO_5_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET1_SPEC, u8, GPIO5TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_5_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO5TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO5TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO5TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO5TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_5_interrupt_control_mode` reader - Interrupt control mode register for GPIO5."]
pub type REG_GPIO_5_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO5CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO5CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO5CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_5_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5CONTROL_MODE_A {
        match self.bits {
            false => GPIO5CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO5CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO5CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO5CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_5_interrupt_control_mode` writer - Interrupt control mode register for GPIO5."]
pub type REG_GPIO_5_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET1_SPEC, GPIO5CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_5_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO5CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO5CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_6_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO6."]
pub type REG_GPIO_6_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO6TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO6TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO6TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO6TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_6_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO6TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO6TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO6TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO6TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO6TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO6TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO6TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO6TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_6_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO6."]
pub type REG_GPIO_6_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET1_SPEC, u8, GPIO6TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_6_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO6TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO6TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO6TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO6TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_6_interrupt_control_mode` reader - Interrupt control mode register for GPIO6."]
pub type REG_GPIO_6_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO6CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO6CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO6CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_6_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6CONTROL_MODE_A {
        match self.bits {
            false => GPIO6CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO6CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO6CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO6CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_6_interrupt_control_mode` writer - Interrupt control mode register for GPIO6."]
pub type REG_GPIO_6_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET1_SPEC, GPIO6CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_6_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO6CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO6CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_7_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO7."]
pub type REG_GPIO_7_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO7TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO7TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO7TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO7TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_7_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO7TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO7TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO7TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO7TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO7TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO7TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO7TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO7TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_7_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO7."]
pub type REG_GPIO_7_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET1_SPEC, u8, GPIO7TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_7_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO7TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO7TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO7TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO7TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_7_interrupt_control_mode` reader - Interrupt control mode register for GPIO7."]
pub type REG_GPIO_7_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO7CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO7CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO7CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_7_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7CONTROL_MODE_A {
        match self.bits {
            false => GPIO7CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO7CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO7CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO7CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_7_interrupt_control_mode` writer - Interrupt control mode register for GPIO7."]
pub type REG_GPIO_7_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET1_SPEC, GPIO7CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_7_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO7CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO7CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_8_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO8."]
pub type REG_GPIO_8_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO8TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO8TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO8TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO8TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_8_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO8TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO8TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO8TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO8TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO8TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO8TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO8TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO8TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_8_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO8."]
pub type REG_GPIO_8_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET1_SPEC, u8, GPIO8TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_8_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO8TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO8TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO8TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO8TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_8_interrupt_control_mode` reader - Interrupt control mode register for GPIO8."]
pub type REG_GPIO_8_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO8CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO8CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO8CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO8CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_8_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8CONTROL_MODE_A {
        match self.bits {
            false => GPIO8CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO8CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO8CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO8CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_8_interrupt_control_mode` writer - Interrupt control mode register for GPIO8."]
pub type REG_GPIO_8_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET1_SPEC, GPIO8CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_8_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO8CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO8CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_9_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO9."]
pub type REG_GPIO_9_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO9TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO9TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO9TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO9TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_9_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO9TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO9TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO9TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO9TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO9TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO9TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO9TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO9TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_9_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO9."]
pub type REG_GPIO_9_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET1_SPEC, u8, GPIO9TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_9_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO9TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO9TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO9TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO9TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_9_interrupt_control_mode` reader - Interrupt control mode register for GPIO9."]
pub type REG_GPIO_9_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO9CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO9CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO9CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO9CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_9_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9CONTROL_MODE_A {
        match self.bits {
            false => GPIO9CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO9CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO9CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO9CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_9_interrupt_control_mode` writer - Interrupt control mode register for GPIO9."]
pub type REG_GPIO_9_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET1_SPEC, GPIO9CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_9_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO9CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO9CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_trigger_mode(&self) -> REG_GPIO_0_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_0_INTERRUPT_TRIGGER_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_control_mode(&self) -> REG_GPIO_0_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_0_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_trigger_mode(&self) -> REG_GPIO_1_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_1_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_control_mode(&self) -> REG_GPIO_1_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_1_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_trigger_mode(&self) -> REG_GPIO_2_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_2_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_control_mode(&self) -> REG_GPIO_2_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_2_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_trigger_mode(&self) -> REG_GPIO_3_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_3_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_control_mode(&self) -> REG_GPIO_3_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_3_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_trigger_mode(&self) -> REG_GPIO_4_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_4_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_control_mode(&self) -> REG_GPIO_4_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_4_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_trigger_mode(&self) -> REG_GPIO_5_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_5_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_control_mode(&self) -> REG_GPIO_5_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_5_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_trigger_mode(&self) -> REG_GPIO_6_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_6_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_control_mode(&self) -> REG_GPIO_6_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_6_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_trigger_mode(&self) -> REG_GPIO_7_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_7_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_control_mode(&self) -> REG_GPIO_7_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_7_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_trigger_mode(&self) -> REG_GPIO_8_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_8_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_control_mode(&self) -> REG_GPIO_8_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_8_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_trigger_mode(&self) -> REG_GPIO_9_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_9_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_control_mode(&self) -> REG_GPIO_9_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_9_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_interrupt_trigger_mode(&mut self) -> REG_GPIO_0_INTERRUPT_TRIGGER_MODE_W<0> {
        REG_GPIO_0_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_interrupt_control_mode(&mut self) -> REG_GPIO_0_INTERRUPT_CONTROL_MODE_W<2> {
        REG_GPIO_0_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_interrupt_trigger_mode(&mut self) -> REG_GPIO_1_INTERRUPT_TRIGGER_MODE_W<3> {
        REG_GPIO_1_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_interrupt_control_mode(&mut self) -> REG_GPIO_1_INTERRUPT_CONTROL_MODE_W<5> {
        REG_GPIO_1_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_interrupt_trigger_mode(&mut self) -> REG_GPIO_2_INTERRUPT_TRIGGER_MODE_W<6> {
        REG_GPIO_2_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_interrupt_control_mode(&mut self) -> REG_GPIO_2_INTERRUPT_CONTROL_MODE_W<8> {
        REG_GPIO_2_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_interrupt_trigger_mode(&mut self) -> REG_GPIO_3_INTERRUPT_TRIGGER_MODE_W<9> {
        REG_GPIO_3_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_interrupt_control_mode(&mut self) -> REG_GPIO_3_INTERRUPT_CONTROL_MODE_W<11> {
        REG_GPIO_3_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_interrupt_trigger_mode(&mut self) -> REG_GPIO_4_INTERRUPT_TRIGGER_MODE_W<12> {
        REG_GPIO_4_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_interrupt_control_mode(&mut self) -> REG_GPIO_4_INTERRUPT_CONTROL_MODE_W<14> {
        REG_GPIO_4_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_interrupt_trigger_mode(&mut self) -> REG_GPIO_5_INTERRUPT_TRIGGER_MODE_W<15> {
        REG_GPIO_5_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_interrupt_control_mode(&mut self) -> REG_GPIO_5_INTERRUPT_CONTROL_MODE_W<17> {
        REG_GPIO_5_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_interrupt_trigger_mode(&mut self) -> REG_GPIO_6_INTERRUPT_TRIGGER_MODE_W<18> {
        REG_GPIO_6_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_interrupt_control_mode(&mut self) -> REG_GPIO_6_INTERRUPT_CONTROL_MODE_W<20> {
        REG_GPIO_6_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_interrupt_trigger_mode(&mut self) -> REG_GPIO_7_INTERRUPT_TRIGGER_MODE_W<21> {
        REG_GPIO_7_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_interrupt_control_mode(&mut self) -> REG_GPIO_7_INTERRUPT_CONTROL_MODE_W<23> {
        REG_GPIO_7_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO8."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_8_interrupt_trigger_mode(&mut self) -> REG_GPIO_8_INTERRUPT_TRIGGER_MODE_W<24> {
        REG_GPIO_8_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO8."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_8_interrupt_control_mode(&mut self) -> REG_GPIO_8_INTERRUPT_CONTROL_MODE_W<26> {
        REG_GPIO_8_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO9."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_9_interrupt_trigger_mode(&mut self) -> REG_GPIO_9_INTERRUPT_TRIGGER_MODE_W<27> {
        REG_GPIO_9_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO9."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_9_interrupt_control_mode(&mut self) -> REG_GPIO_9_INTERRUPT_CONTROL_MODE_W<29> {
        REG_GPIO_9_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt trigger and control register for GPIO0-GPIO9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mode_set1](index.html) module"]
pub struct GPIO_INT_MODE_SET1_SPEC;
impl crate::RegisterSpec for GPIO_INT_MODE_SET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_mode_set1::R](R) reader structure"]
impl crate::Readable for GPIO_INT_MODE_SET1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_mode_set1::W](W) writer structure"]
impl crate::Writable for GPIO_INT_MODE_SET1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_INT_MODE_SET1 to value 0"]
impl crate::Resettable for GPIO_INT_MODE_SET1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GPIO_INT_MODE_SET2` reader"]
pub struct R(crate::R<GPIO_INT_MODE_SET2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_MODE_SET2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_MODE_SET2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_MODE_SET2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_MODE_SET2` writer"]
pub struct W(crate::W<GPIO_INT_MODE_SET2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_MODE_SET2_SPEC>;
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
impl From<crate::W<GPIO_INT_MODE_SET2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_MODE_SET2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_10_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO10."]
pub type REG_GPIO_10_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO10TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO10TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO10TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO10TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_10_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO10TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO10TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO10TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO10TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO10TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO10TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO10TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO10TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_10_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO10."]
pub type REG_GPIO_10_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET2_SPEC, u8, GPIO10TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_10_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO10TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO10TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO10TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO10TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_10_interrupt_control_mode` reader - Interrupt control mode register for GPIO10."]
pub type REG_GPIO_10_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO10CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO10CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO10CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO10CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_10_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10CONTROL_MODE_A {
        match self.bits {
            false => GPIO10CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO10CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO10CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO10CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_10_interrupt_control_mode` writer - Interrupt control mode register for GPIO10."]
pub type REG_GPIO_10_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET2_SPEC, GPIO10CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_10_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO10CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO10CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_11_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO11."]
pub type REG_GPIO_11_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO11TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO11TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO11TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO11TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_11_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO11TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO11TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO11TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO11TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO11TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO11TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO11TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO11TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_11_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO11."]
pub type REG_GPIO_11_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET2_SPEC, u8, GPIO11TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_11_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO11TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO11TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO11TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO11TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_11_interrupt_control_mode` reader - Interrupt control mode register for GPIO11."]
pub type REG_GPIO_11_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO11CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO11CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO11CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO11CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_11_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11CONTROL_MODE_A {
        match self.bits {
            false => GPIO11CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO11CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO11CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO11CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_11_interrupt_control_mode` writer - Interrupt control mode register for GPIO11."]
pub type REG_GPIO_11_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET2_SPEC, GPIO11CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_11_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO11CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO11CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_12_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO12."]
pub type REG_GPIO_12_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO12TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO12TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO12TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO12TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_12_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO12TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO12TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO12TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO12TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO12TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO12TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO12TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO12TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_12_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO12."]
pub type REG_GPIO_12_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET2_SPEC, u8, GPIO12TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_12_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO12TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO12TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO12TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO12TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_12_interrupt_control_mode` reader - Interrupt control mode register for GPIO12."]
pub type REG_GPIO_12_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO12CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO12CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO12CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_12_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12CONTROL_MODE_A {
        match self.bits {
            false => GPIO12CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO12CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO12CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO12CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_12_interrupt_control_mode` writer - Interrupt control mode register for GPIO12."]
pub type REG_GPIO_12_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET2_SPEC, GPIO12CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_12_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO12CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO12CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_13_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO13."]
pub type REG_GPIO_13_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO13TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO13TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO13TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO13TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_13_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO13TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO13TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO13TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO13TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO13TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO13TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO13TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO13TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_13_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO13."]
pub type REG_GPIO_13_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET2_SPEC, u8, GPIO13TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_13_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO13TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO13TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO13TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO13TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_13_interrupt_control_mode` reader - Interrupt control mode register for GPIO13."]
pub type REG_GPIO_13_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO13CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO13CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO13CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_13_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13CONTROL_MODE_A {
        match self.bits {
            false => GPIO13CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO13CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO13CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO13CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_13_interrupt_control_mode` writer - Interrupt control mode register for GPIO13."]
pub type REG_GPIO_13_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET2_SPEC, GPIO13CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_13_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO13CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO13CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_14_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO14."]
pub type REG_GPIO_14_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO14TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO14TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO14TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO14TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_14_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO14TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO14TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO14TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO14TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO14TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO14TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO14TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO14TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_14_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO14."]
pub type REG_GPIO_14_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET2_SPEC, u8, GPIO14TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_14_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO14TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO14TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO14TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO14TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_14_interrupt_control_mode` reader - Interrupt control mode register for GPIO14."]
pub type REG_GPIO_14_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO14CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO14CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO14CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_14_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14CONTROL_MODE_A {
        match self.bits {
            false => GPIO14CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO14CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO14CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO14CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_14_interrupt_control_mode` writer - Interrupt control mode register for GPIO14."]
pub type REG_GPIO_14_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET2_SPEC, GPIO14CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_14_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO14CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO14CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_15_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO15."]
pub type REG_GPIO_15_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO15TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO15TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO15TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO15TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_15_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO15TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO15TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO15TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO15TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO15TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO15TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO15TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO15TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_15_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO15."]
pub type REG_GPIO_15_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET2_SPEC, u8, GPIO15TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_15_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO15TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO15TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO15TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO15TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_15_interrupt_control_mode` reader - Interrupt control mode register for GPIO15."]
pub type REG_GPIO_15_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO15CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO15CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO15CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_15_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15CONTROL_MODE_A {
        match self.bits {
            false => GPIO15CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO15CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO15CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO15CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_15_interrupt_control_mode` writer - Interrupt control mode register for GPIO15."]
pub type REG_GPIO_15_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET2_SPEC, GPIO15CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_15_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO15CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO15CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_16_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO16."]
pub type REG_GPIO_16_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO16TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO16TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO16TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO16TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_16_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO16TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO16TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO16TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO16TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO16TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO16TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO16TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO16TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_16_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO16."]
pub type REG_GPIO_16_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET2_SPEC, u8, GPIO16TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_16_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO16TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO16TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO16TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO16TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_16_interrupt_control_mode` reader - Interrupt control mode register for GPIO16."]
pub type REG_GPIO_16_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO16CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO16CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO16CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO16CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_16_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16CONTROL_MODE_A {
        match self.bits {
            false => GPIO16CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO16CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO16CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO16CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_16_interrupt_control_mode` writer - Interrupt control mode register for GPIO16."]
pub type REG_GPIO_16_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET2_SPEC, GPIO16CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_16_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO16CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO16CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_17_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO17."]
pub type REG_GPIO_17_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO17TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO17TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO17TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO17TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_17_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO17TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO17TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO17TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO17TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO17TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO17TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO17TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO17TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_17_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO17."]
pub type REG_GPIO_17_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET2_SPEC, u8, GPIO17TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_17_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO17TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO17TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO17TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO17TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_17_interrupt_control_mode` reader - Interrupt control mode register for GPIO17."]
pub type REG_GPIO_17_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO17CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO17CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO17CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO17CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_17_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17CONTROL_MODE_A {
        match self.bits {
            false => GPIO17CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO17CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO17CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO17CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_17_interrupt_control_mode` writer - Interrupt control mode register for GPIO17."]
pub type REG_GPIO_17_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET2_SPEC, GPIO17CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_17_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO17CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO17CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_18_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO18."]
pub type REG_GPIO_18_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO18TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO18TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO18TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO18TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_18_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO18TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO18TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO18TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO18TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO18TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO18TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO18TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO18TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_18_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO18."]
pub type REG_GPIO_18_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET2_SPEC, u8, GPIO18TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_18_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO18TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO18TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO18TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO18TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_18_interrupt_control_mode` reader - Interrupt control mode register for GPIO18."]
pub type REG_GPIO_18_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO18CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO18CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO18CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_18_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18CONTROL_MODE_A {
        match self.bits {
            false => GPIO18CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO18CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO18CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO18CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_18_interrupt_control_mode` writer - Interrupt control mode register for GPIO18."]
pub type REG_GPIO_18_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET2_SPEC, GPIO18CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_18_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO18CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO18CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_19_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO19."]
pub type REG_GPIO_19_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO19TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO19TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO19TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO19TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_19_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO19TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO19TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO19TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO19TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO19TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO19TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO19TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO19TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_19_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO19."]
pub type REG_GPIO_19_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET2_SPEC, u8, GPIO19TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_19_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO19TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO19TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO19TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO19TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_19_interrupt_control_mode` reader - Interrupt control mode register for GPIO19."]
pub type REG_GPIO_19_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO19CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO19CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO19CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_19_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19CONTROL_MODE_A {
        match self.bits {
            false => GPIO19CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO19CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO19CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO19CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_19_interrupt_control_mode` writer - Interrupt control mode register for GPIO19."]
pub type REG_GPIO_19_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET2_SPEC, GPIO19CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_19_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO19CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO19CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_trigger_mode(&self) -> REG_GPIO_10_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_10_INTERRUPT_TRIGGER_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_control_mode(&self) -> REG_GPIO_10_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_10_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_trigger_mode(&self) -> REG_GPIO_11_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_11_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_control_mode(&self) -> REG_GPIO_11_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_11_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_trigger_mode(&self) -> REG_GPIO_12_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_12_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_control_mode(&self) -> REG_GPIO_12_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_12_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_trigger_mode(&self) -> REG_GPIO_13_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_13_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_control_mode(&self) -> REG_GPIO_13_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_13_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_trigger_mode(&self) -> REG_GPIO_14_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_14_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_control_mode(&self) -> REG_GPIO_14_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_14_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_trigger_mode(&self) -> REG_GPIO_15_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_15_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_control_mode(&self) -> REG_GPIO_15_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_15_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_trigger_mode(&self) -> REG_GPIO_16_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_16_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_control_mode(&self) -> REG_GPIO_16_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_16_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_trigger_mode(&self) -> REG_GPIO_17_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_17_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_control_mode(&self) -> REG_GPIO_17_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_17_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_trigger_mode(&self) -> REG_GPIO_18_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_18_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_control_mode(&self) -> REG_GPIO_18_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_18_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_trigger_mode(&self) -> REG_GPIO_19_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_19_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_control_mode(&self) -> REG_GPIO_19_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_19_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO10."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_10_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_10_INTERRUPT_TRIGGER_MODE_W<0> {
        REG_GPIO_10_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO10."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_10_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_10_INTERRUPT_CONTROL_MODE_W<2> {
        REG_GPIO_10_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO11."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_11_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_11_INTERRUPT_TRIGGER_MODE_W<3> {
        REG_GPIO_11_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO11."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_11_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_11_INTERRUPT_CONTROL_MODE_W<5> {
        REG_GPIO_11_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_12_INTERRUPT_TRIGGER_MODE_W<6> {
        REG_GPIO_12_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_12_INTERRUPT_CONTROL_MODE_W<8> {
        REG_GPIO_12_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_13_INTERRUPT_TRIGGER_MODE_W<9> {
        REG_GPIO_13_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_13_INTERRUPT_CONTROL_MODE_W<11> {
        REG_GPIO_13_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_14_INTERRUPT_TRIGGER_MODE_W<12> {
        REG_GPIO_14_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_14_INTERRUPT_CONTROL_MODE_W<14> {
        REG_GPIO_14_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_15_INTERRUPT_TRIGGER_MODE_W<15> {
        REG_GPIO_15_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_15_INTERRUPT_CONTROL_MODE_W<17> {
        REG_GPIO_15_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO16."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_16_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_16_INTERRUPT_TRIGGER_MODE_W<18> {
        REG_GPIO_16_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO16."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_16_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_16_INTERRUPT_CONTROL_MODE_W<20> {
        REG_GPIO_16_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO17."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_17_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_17_INTERRUPT_TRIGGER_MODE_W<21> {
        REG_GPIO_17_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO17."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_17_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_17_INTERRUPT_CONTROL_MODE_W<23> {
        REG_GPIO_17_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_18_INTERRUPT_TRIGGER_MODE_W<24> {
        REG_GPIO_18_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_18_INTERRUPT_CONTROL_MODE_W<26> {
        REG_GPIO_18_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_19_INTERRUPT_TRIGGER_MODE_W<27> {
        REG_GPIO_19_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_19_INTERRUPT_CONTROL_MODE_W<29> {
        REG_GPIO_19_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt trigger and control register for GPIO10-GPIO19.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mode_set2](index.html) module"]
pub struct GPIO_INT_MODE_SET2_SPEC;
impl crate::RegisterSpec for GPIO_INT_MODE_SET2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_mode_set2::R](R) reader structure"]
impl crate::Readable for GPIO_INT_MODE_SET2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_mode_set2::W](W) writer structure"]
impl crate::Writable for GPIO_INT_MODE_SET2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_INT_MODE_SET2 to value 0"]
impl crate::Resettable for GPIO_INT_MODE_SET2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

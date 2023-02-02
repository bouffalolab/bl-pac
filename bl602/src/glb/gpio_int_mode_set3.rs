#[doc = "Register `GPIO_INT_MODE_SET3` reader"]
pub struct R(crate::R<GPIO_INT_MODE_SET3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_MODE_SET3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_MODE_SET3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_MODE_SET3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_MODE_SET3` writer"]
pub struct W(crate::W<GPIO_INT_MODE_SET3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_MODE_SET3_SPEC>;
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
impl From<crate::W<GPIO_INT_MODE_SET3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_MODE_SET3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_20_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO20."]
pub type REG_GPIO_20_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO20TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO20TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO20TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO20TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_20_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO20TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO20TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO20TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO20TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO20TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO20TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO20TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO20TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_20_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO20."]
pub type REG_GPIO_20_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET3_SPEC, u8, GPIO20TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_20_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO20TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO20TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO20TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO20TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_20_interrupt_control_mode` reader - Interrupt control mode register for GPIO20."]
pub type REG_GPIO_20_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO20CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO20CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO20CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_20_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20CONTROL_MODE_A {
        match self.bits {
            false => GPIO20CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO20CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO20CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO20CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_20_interrupt_control_mode` writer - Interrupt control mode register for GPIO20."]
pub type REG_GPIO_20_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET3_SPEC, GPIO20CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_20_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO20CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO20CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_21_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO21."]
pub type REG_GPIO_21_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO21TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO21TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO21TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO21TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_21_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO21TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO21TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO21TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO21TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO21TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO21TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO21TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO21TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_21_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO21."]
pub type REG_GPIO_21_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET3_SPEC, u8, GPIO21TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_21_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO21TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO21TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO21TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO21TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_21_interrupt_control_mode` reader - Interrupt control mode register for GPIO21."]
pub type REG_GPIO_21_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO21CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO21CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO21CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_21_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21CONTROL_MODE_A {
        match self.bits {
            false => GPIO21CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO21CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO21CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO21CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_21_interrupt_control_mode` writer - Interrupt control mode register for GPIO21."]
pub type REG_GPIO_21_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET3_SPEC, GPIO21CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_21_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO21CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO21CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_22_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO22."]
pub type REG_GPIO_22_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO22TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO22TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO22TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO22TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_22_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO22TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO22TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO22TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO22TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO22TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO22TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO22TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO22TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_22_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO22."]
pub type REG_GPIO_22_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET3_SPEC, u8, GPIO22TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_22_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO22TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO22TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO22TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO22TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_22_interrupt_control_mode` reader - Interrupt control mode register for GPIO22."]
pub type REG_GPIO_22_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO22CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO22CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO22CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_22_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22CONTROL_MODE_A {
        match self.bits {
            false => GPIO22CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO22CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO22CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO22CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_22_interrupt_control_mode` writer - Interrupt control mode register for GPIO22."]
pub type REG_GPIO_22_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET3_SPEC, GPIO22CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_22_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO22CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO22CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_23_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO23."]
pub type REG_GPIO_23_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO23TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO23TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO23TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO23TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_23_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO23TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO23TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO23TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO23TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO23TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO23TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO23TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO23TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_23_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO23."]
pub type REG_GPIO_23_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET3_SPEC, u8, GPIO23TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_23_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO23TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO23TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO23TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO23TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_23_interrupt_control_mode` reader - Interrupt control mode register for GPIO23."]
pub type REG_GPIO_23_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO23CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO23CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO23CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_23_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23CONTROL_MODE_A {
        match self.bits {
            false => GPIO23CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO23CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO23CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO23CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_23_interrupt_control_mode` writer - Interrupt control mode register for GPIO23."]
pub type REG_GPIO_23_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET3_SPEC, GPIO23CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_23_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO23CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO23CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_24_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO24."]
pub type REG_GPIO_24_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO24TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO24TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO24TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO24TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_24_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO24TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO24TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO24TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO24TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO24TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO24TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO24TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO24TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_24_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO24."]
pub type REG_GPIO_24_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET3_SPEC, u8, GPIO24TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_24_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO24TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO24TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO24TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO24TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_24_interrupt_control_mode` reader - Interrupt control mode register for GPIO24."]
pub type REG_GPIO_24_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO24CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO24CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO24CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO24CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_24_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24CONTROL_MODE_A {
        match self.bits {
            false => GPIO24CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO24CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO24CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO24CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_24_interrupt_control_mode` writer - Interrupt control mode register for GPIO24."]
pub type REG_GPIO_24_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET3_SPEC, GPIO24CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_24_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO24CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO24CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_25_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO25."]
pub type REG_GPIO_25_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO25TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO25TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO25TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO25TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_25_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO25TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO25TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO25TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO25TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO25TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO25TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO25TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO25TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_25_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO25."]
pub type REG_GPIO_25_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET3_SPEC, u8, GPIO25TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_25_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO25TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO25TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO25TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO25TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_25_interrupt_control_mode` reader - Interrupt control mode register for GPIO25."]
pub type REG_GPIO_25_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO25CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO25CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO25CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO25CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_25_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25CONTROL_MODE_A {
        match self.bits {
            false => GPIO25CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO25CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO25CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO25CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_25_interrupt_control_mode` writer - Interrupt control mode register for GPIO25."]
pub type REG_GPIO_25_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET3_SPEC, GPIO25CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_25_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO25CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO25CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_26_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO26."]
pub type REG_GPIO_26_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO26TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO26TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO26TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO26TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_26_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO26TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO26TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO26TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO26TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO26TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO26TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO26TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO26TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_26_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO26."]
pub type REG_GPIO_26_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET3_SPEC, u8, GPIO26TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_26_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO26TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO26TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO26TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO26TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_26_interrupt_control_mode` reader - Interrupt control mode register for GPIO26."]
pub type REG_GPIO_26_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO26CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO26CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO26CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO26CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_26_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26CONTROL_MODE_A {
        match self.bits {
            false => GPIO26CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO26CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO26CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO26CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_26_interrupt_control_mode` writer - Interrupt control mode register for GPIO26."]
pub type REG_GPIO_26_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET3_SPEC, GPIO26CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_26_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO26CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO26CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_27_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO27."]
pub type REG_GPIO_27_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO27TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO27TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO27TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO27TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_27_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO27TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO27TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO27TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO27TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO27TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO27TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO27TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO27TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_27_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO27."]
pub type REG_GPIO_27_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET3_SPEC, u8, GPIO27TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_27_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO27TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO27TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO27TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO27TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_27_interrupt_control_mode` reader - Interrupt control mode register for GPIO27."]
pub type REG_GPIO_27_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO27CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO27CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO27CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO27CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_27_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27CONTROL_MODE_A {
        match self.bits {
            false => GPIO27CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO27CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO27CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO27CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_27_interrupt_control_mode` writer - Interrupt control mode register for GPIO27."]
pub type REG_GPIO_27_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET3_SPEC, GPIO27CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_27_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO27CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO27CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_28_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO28."]
pub type REG_GPIO_28_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO28TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO28TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO28TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO28TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_28_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO28TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO28TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO28TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO28TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO28TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO28TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO28TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO28TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_28_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO28."]
pub type REG_GPIO_28_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET3_SPEC, u8, GPIO28TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_28_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO28TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO28TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO28TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO28TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_28_interrupt_control_mode` reader - Interrupt control mode register for GPIO28."]
pub type REG_GPIO_28_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO28CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO28CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO28CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO28CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_28_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28CONTROL_MODE_A {
        match self.bits {
            false => GPIO28CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO28CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO28CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO28CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_28_interrupt_control_mode` writer - Interrupt control mode register for GPIO28."]
pub type REG_GPIO_28_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET3_SPEC, GPIO28CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_28_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO28CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO28CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
#[doc = "Field `reg_gpio_29_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO29."]
pub type REG_GPIO_29_INTERRUPT_TRIGGER_MODE_R = crate::FieldReader<u8, GPIO29TRIGGER_MODE_A>;
#[doc = "Interrupt trigger mode register for GPIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO29TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<GPIO29TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO29TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_29_INTERRUPT_TRIGGER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO29TRIGGER_MODE_A {
        match self.bits {
            0 => GPIO29TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => GPIO29TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => GPIO29TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => GPIO29TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == GPIO29TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == GPIO29TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == GPIO29TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == GPIO29TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
#[doc = "Field `reg_gpio_29_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO29."]
pub type REG_GPIO_29_INTERRUPT_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_INT_MODE_SET3_SPEC, u8, GPIO29TRIGGER_MODE_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_29_INTERRUPT_TRIGGER_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(GPIO29TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(GPIO29TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(GPIO29TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(GPIO29TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
}
#[doc = "Field `reg_gpio_29_interrupt_control_mode` reader - Interrupt control mode register for GPIO29."]
pub type REG_GPIO_29_INTERRUPT_CONTROL_MODE_R = crate::BitReader<GPIO29CONTROL_MODE_A>;
#[doc = "Interrupt control mode register for GPIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO29CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<GPIO29CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO29CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_29_INTERRUPT_CONTROL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO29CONTROL_MODE_A {
        match self.bits {
            false => GPIO29CONTROL_MODE_A::SYNCHRONOUS,
            true => GPIO29CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == GPIO29CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == GPIO29CONTROL_MODE_A::ASYNCHRONOUS
    }
}
#[doc = "Field `reg_gpio_29_interrupt_control_mode` writer - Interrupt control mode register for GPIO29."]
pub type REG_GPIO_29_INTERRUPT_CONTROL_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MODE_SET3_SPEC, GPIO29CONTROL_MODE_A, O>;
impl<'a, const O: u8> REG_GPIO_29_INTERRUPT_CONTROL_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(GPIO29CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(GPIO29CONTROL_MODE_A::ASYNCHRONOUS)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_trigger_mode(&self) -> REG_GPIO_20_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_20_INTERRUPT_TRIGGER_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_control_mode(&self) -> REG_GPIO_20_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_20_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_trigger_mode(&self) -> REG_GPIO_21_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_21_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_control_mode(&self) -> REG_GPIO_21_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_21_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_trigger_mode(&self) -> REG_GPIO_22_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_22_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_control_mode(&self) -> REG_GPIO_22_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_22_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_trigger_mode(&self) -> REG_GPIO_23_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_23_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_control_mode(&self) -> REG_GPIO_23_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_23_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_trigger_mode(&self) -> REG_GPIO_24_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_24_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_control_mode(&self) -> REG_GPIO_24_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_24_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_trigger_mode(&self) -> REG_GPIO_25_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_25_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_control_mode(&self) -> REG_GPIO_25_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_25_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_trigger_mode(&self) -> REG_GPIO_26_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_26_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_control_mode(&self) -> REG_GPIO_26_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_26_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_trigger_mode(&self) -> REG_GPIO_27_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_27_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_control_mode(&self) -> REG_GPIO_27_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_27_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_trigger_mode(&self) -> REG_GPIO_28_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_28_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_control_mode(&self) -> REG_GPIO_28_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_28_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO29."]
    #[inline(always)]
    pub fn reg_gpio_29_interrupt_trigger_mode(&self) -> REG_GPIO_29_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_29_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO29."]
    #[inline(always)]
    pub fn reg_gpio_29_interrupt_control_mode(&self) -> REG_GPIO_29_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_29_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_20_INTERRUPT_TRIGGER_MODE_W<0> {
        REG_GPIO_20_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_20_INTERRUPT_CONTROL_MODE_W<2> {
        REG_GPIO_20_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_21_INTERRUPT_TRIGGER_MODE_W<3> {
        REG_GPIO_21_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_21_INTERRUPT_CONTROL_MODE_W<5> {
        REG_GPIO_21_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_22_INTERRUPT_TRIGGER_MODE_W<6> {
        REG_GPIO_22_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_22_INTERRUPT_CONTROL_MODE_W<8> {
        REG_GPIO_22_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO23."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_23_INTERRUPT_TRIGGER_MODE_W<9> {
        REG_GPIO_23_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO23."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_23_INTERRUPT_CONTROL_MODE_W<11> {
        REG_GPIO_23_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO24."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_24_INTERRUPT_TRIGGER_MODE_W<12> {
        REG_GPIO_24_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO24."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_24_INTERRUPT_CONTROL_MODE_W<14> {
        REG_GPIO_24_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO25."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_25_INTERRUPT_TRIGGER_MODE_W<15> {
        REG_GPIO_25_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO25."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_25_INTERRUPT_CONTROL_MODE_W<17> {
        REG_GPIO_25_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO26."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_26_INTERRUPT_TRIGGER_MODE_W<18> {
        REG_GPIO_26_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO26."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_26_INTERRUPT_CONTROL_MODE_W<20> {
        REG_GPIO_26_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO27."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_27_INTERRUPT_TRIGGER_MODE_W<21> {
        REG_GPIO_27_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO27."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_27_INTERRUPT_CONTROL_MODE_W<23> {
        REG_GPIO_27_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO28."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_28_INTERRUPT_TRIGGER_MODE_W<24> {
        REG_GPIO_28_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO28."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_28_INTERRUPT_CONTROL_MODE_W<26> {
        REG_GPIO_28_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO29."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_29_interrupt_trigger_mode(
        &mut self,
    ) -> REG_GPIO_29_INTERRUPT_TRIGGER_MODE_W<27> {
        REG_GPIO_29_INTERRUPT_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO29."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_29_interrupt_control_mode(
        &mut self,
    ) -> REG_GPIO_29_INTERRUPT_CONTROL_MODE_W<29> {
        REG_GPIO_29_INTERRUPT_CONTROL_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt trigger and control register for GPIO20-GPIO29.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mode_set3](index.html) module"]
pub struct GPIO_INT_MODE_SET3_SPEC;
impl crate::RegisterSpec for GPIO_INT_MODE_SET3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_mode_set3::R](R) reader structure"]
impl crate::Readable for GPIO_INT_MODE_SET3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_mode_set3::W](W) writer structure"]
impl crate::Writable for GPIO_INT_MODE_SET3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_INT_MODE_SET3 to value 0"]
impl crate::Resettable for GPIO_INT_MODE_SET3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

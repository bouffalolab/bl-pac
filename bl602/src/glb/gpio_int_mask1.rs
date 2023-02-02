#[doc = "Register `GPIO_INT_MASK1` reader"]
pub struct R(crate::R<GPIO_INT_MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_MASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_MASK1` writer"]
pub struct W(crate::W<GPIO_INT_MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_MASK1_SPEC>;
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
impl From<crate::W<GPIO_INT_MASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_0_mask` reader - Mask register for GPIO0."]
pub type REG_GPIO_0_MASK_R = crate::BitReader<GPIO0MASK_A>;
#[doc = "Mask register for GPIO0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO0MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_0_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0MASK_A {
        match self.bits {
            false => GPIO0MASK_A::UNMASKED,
            true => GPIO0MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO0MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO0MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_0_mask` writer - Mask register for GPIO0."]
pub type REG_GPIO_0_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO0MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_0_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO0MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO0MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_1_mask` reader - Mask register for GPIO1."]
pub type REG_GPIO_1_MASK_R = crate::BitReader<GPIO1MASK_A>;
#[doc = "Mask register for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO1MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_1_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1MASK_A {
        match self.bits {
            false => GPIO1MASK_A::UNMASKED,
            true => GPIO1MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO1MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO1MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_1_mask` writer - Mask register for GPIO1."]
pub type REG_GPIO_1_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO1MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_1_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO1MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO1MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_2_mask` reader - Mask register for GPIO2."]
pub type REG_GPIO_2_MASK_R = crate::BitReader<GPIO2MASK_A>;
#[doc = "Mask register for GPIO2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO2MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_2_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2MASK_A {
        match self.bits {
            false => GPIO2MASK_A::UNMASKED,
            true => GPIO2MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO2MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO2MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_2_mask` writer - Mask register for GPIO2."]
pub type REG_GPIO_2_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO2MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_2_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO2MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO2MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_3_mask` reader - Mask register for GPIO3."]
pub type REG_GPIO_3_MASK_R = crate::BitReader<GPIO3MASK_A>;
#[doc = "Mask register for GPIO3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO3MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO3MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_3_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3MASK_A {
        match self.bits {
            false => GPIO3MASK_A::UNMASKED,
            true => GPIO3MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO3MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO3MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_3_mask` writer - Mask register for GPIO3."]
pub type REG_GPIO_3_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO3MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_3_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO3MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO3MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_4_mask` reader - Mask register for GPIO4."]
pub type REG_GPIO_4_MASK_R = crate::BitReader<GPIO4MASK_A>;
#[doc = "Mask register for GPIO4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO4MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO4MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_4_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4MASK_A {
        match self.bits {
            false => GPIO4MASK_A::UNMASKED,
            true => GPIO4MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO4MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO4MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_4_mask` writer - Mask register for GPIO4."]
pub type REG_GPIO_4_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO4MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_4_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO4MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO4MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_5_mask` reader - Mask register for GPIO5."]
pub type REG_GPIO_5_MASK_R = crate::BitReader<GPIO5MASK_A>;
#[doc = "Mask register for GPIO5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO5MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO5MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_5_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5MASK_A {
        match self.bits {
            false => GPIO5MASK_A::UNMASKED,
            true => GPIO5MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO5MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO5MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_5_mask` writer - Mask register for GPIO5."]
pub type REG_GPIO_5_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO5MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_5_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO5MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO5MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_6_mask` reader - Mask register for GPIO6."]
pub type REG_GPIO_6_MASK_R = crate::BitReader<GPIO6MASK_A>;
#[doc = "Mask register for GPIO6.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO6MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO6MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_6_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6MASK_A {
        match self.bits {
            false => GPIO6MASK_A::UNMASKED,
            true => GPIO6MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO6MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO6MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_6_mask` writer - Mask register for GPIO6."]
pub type REG_GPIO_6_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO6MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_6_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO6MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO6MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_7_mask` reader - Mask register for GPIO7."]
pub type REG_GPIO_7_MASK_R = crate::BitReader<GPIO7MASK_A>;
#[doc = "Mask register for GPIO7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO7MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO7MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_7_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7MASK_A {
        match self.bits {
            false => GPIO7MASK_A::UNMASKED,
            true => GPIO7MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO7MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO7MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_7_mask` writer - Mask register for GPIO7."]
pub type REG_GPIO_7_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO7MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_7_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO7MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO7MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_8_mask` reader - Mask register for GPIO8."]
pub type REG_GPIO_8_MASK_R = crate::BitReader<GPIO8MASK_A>;
#[doc = "Mask register for GPIO8.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO8MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO8MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO8MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_8_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8MASK_A {
        match self.bits {
            false => GPIO8MASK_A::UNMASKED,
            true => GPIO8MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO8MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO8MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_8_mask` writer - Mask register for GPIO8."]
pub type REG_GPIO_8_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO8MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_8_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO8MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO8MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_9_mask` reader - Mask register for GPIO9."]
pub type REG_GPIO_9_MASK_R = crate::BitReader<GPIO9MASK_A>;
#[doc = "Mask register for GPIO9.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO9MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO9MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO9MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_9_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9MASK_A {
        match self.bits {
            false => GPIO9MASK_A::UNMASKED,
            true => GPIO9MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO9MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO9MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_9_mask` writer - Mask register for GPIO9."]
pub type REG_GPIO_9_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO9MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_9_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO9MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO9MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_10_mask` reader - Mask register for GPIO10."]
pub type REG_GPIO_10_MASK_R = crate::BitReader<GPIO10MASK_A>;
#[doc = "Mask register for GPIO10.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO10MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO10MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO10MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_10_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10MASK_A {
        match self.bits {
            false => GPIO10MASK_A::UNMASKED,
            true => GPIO10MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO10MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO10MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_10_mask` writer - Mask register for GPIO10."]
pub type REG_GPIO_10_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO10MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_10_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO10MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO10MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_11_mask` reader - Mask register for GPIO11."]
pub type REG_GPIO_11_MASK_R = crate::BitReader<GPIO11MASK_A>;
#[doc = "Mask register for GPIO11.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO11MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO11MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO11MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_11_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11MASK_A {
        match self.bits {
            false => GPIO11MASK_A::UNMASKED,
            true => GPIO11MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO11MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO11MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_11_mask` writer - Mask register for GPIO11."]
pub type REG_GPIO_11_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO11MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_11_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO11MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO11MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_12_mask` reader - Mask register for GPIO12."]
pub type REG_GPIO_12_MASK_R = crate::BitReader<GPIO12MASK_A>;
#[doc = "Mask register for GPIO12.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO12MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO12MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_12_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12MASK_A {
        match self.bits {
            false => GPIO12MASK_A::UNMASKED,
            true => GPIO12MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO12MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO12MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_12_mask` writer - Mask register for GPIO12."]
pub type REG_GPIO_12_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO12MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_12_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO12MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO12MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_13_mask` reader - Mask register for GPIO13."]
pub type REG_GPIO_13_MASK_R = crate::BitReader<GPIO13MASK_A>;
#[doc = "Mask register for GPIO13.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO13MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO13MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_13_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13MASK_A {
        match self.bits {
            false => GPIO13MASK_A::UNMASKED,
            true => GPIO13MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO13MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO13MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_13_mask` writer - Mask register for GPIO13."]
pub type REG_GPIO_13_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO13MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_13_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO13MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO13MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_14_mask` reader - Mask register for GPIO14."]
pub type REG_GPIO_14_MASK_R = crate::BitReader<GPIO14MASK_A>;
#[doc = "Mask register for GPIO14.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO14MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO14MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_14_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14MASK_A {
        match self.bits {
            false => GPIO14MASK_A::UNMASKED,
            true => GPIO14MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO14MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO14MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_14_mask` writer - Mask register for GPIO14."]
pub type REG_GPIO_14_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO14MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_14_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO14MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO14MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_15_mask` reader - Mask register for GPIO15."]
pub type REG_GPIO_15_MASK_R = crate::BitReader<GPIO15MASK_A>;
#[doc = "Mask register for GPIO15.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO15MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO15MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_15_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15MASK_A {
        match self.bits {
            false => GPIO15MASK_A::UNMASKED,
            true => GPIO15MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO15MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO15MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_15_mask` writer - Mask register for GPIO15."]
pub type REG_GPIO_15_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO15MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_15_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO15MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO15MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_16_mask` reader - Mask register for GPIO16."]
pub type REG_GPIO_16_MASK_R = crate::BitReader<GPIO16MASK_A>;
#[doc = "Mask register for GPIO16.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO16MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO16MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO16MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_16_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16MASK_A {
        match self.bits {
            false => GPIO16MASK_A::UNMASKED,
            true => GPIO16MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO16MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO16MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_16_mask` writer - Mask register for GPIO16."]
pub type REG_GPIO_16_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO16MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_16_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO16MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO16MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_17_mask` reader - Mask register for GPIO17."]
pub type REG_GPIO_17_MASK_R = crate::BitReader<GPIO17MASK_A>;
#[doc = "Mask register for GPIO17.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO17MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO17MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO17MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_17_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17MASK_A {
        match self.bits {
            false => GPIO17MASK_A::UNMASKED,
            true => GPIO17MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO17MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO17MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_17_mask` writer - Mask register for GPIO17."]
pub type REG_GPIO_17_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO17MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_17_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO17MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO17MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_18_mask` reader - Mask register for GPIO18."]
pub type REG_GPIO_18_MASK_R = crate::BitReader<GPIO18MASK_A>;
#[doc = "Mask register for GPIO18.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO18MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO18MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_18_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18MASK_A {
        match self.bits {
            false => GPIO18MASK_A::UNMASKED,
            true => GPIO18MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO18MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO18MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_18_mask` writer - Mask register for GPIO18."]
pub type REG_GPIO_18_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO18MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_18_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO18MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO18MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_19_mask` reader - Mask register for GPIO19."]
pub type REG_GPIO_19_MASK_R = crate::BitReader<GPIO19MASK_A>;
#[doc = "Mask register for GPIO19.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO19MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO19MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_19_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19MASK_A {
        match self.bits {
            false => GPIO19MASK_A::UNMASKED,
            true => GPIO19MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO19MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO19MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_19_mask` writer - Mask register for GPIO19."]
pub type REG_GPIO_19_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO19MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_19_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO19MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO19MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_20_mask` reader - Mask register for GPIO20."]
pub type REG_GPIO_20_MASK_R = crate::BitReader<GPIO20MASK_A>;
#[doc = "Mask register for GPIO20.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO20MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO20MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_20_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20MASK_A {
        match self.bits {
            false => GPIO20MASK_A::UNMASKED,
            true => GPIO20MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO20MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO20MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_20_mask` writer - Mask register for GPIO20."]
pub type REG_GPIO_20_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO20MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_20_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO20MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO20MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_21_mask` reader - Mask register for GPIO21."]
pub type REG_GPIO_21_MASK_R = crate::BitReader<GPIO21MASK_A>;
#[doc = "Mask register for GPIO21.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO21MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO21MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_21_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21MASK_A {
        match self.bits {
            false => GPIO21MASK_A::UNMASKED,
            true => GPIO21MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO21MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO21MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_21_mask` writer - Mask register for GPIO21."]
pub type REG_GPIO_21_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO21MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_21_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO21MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO21MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_22_mask` reader - Mask register for GPIO22."]
pub type REG_GPIO_22_MASK_R = crate::BitReader<GPIO22MASK_A>;
#[doc = "Mask register for GPIO22.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO22MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO22MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_22_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22MASK_A {
        match self.bits {
            false => GPIO22MASK_A::UNMASKED,
            true => GPIO22MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO22MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO22MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_22_mask` writer - Mask register for GPIO22."]
pub type REG_GPIO_22_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO22MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_22_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO22MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO22MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_23_mask` reader - Mask register for GPIO23."]
pub type REG_GPIO_23_MASK_R = crate::BitReader<GPIO23MASK_A>;
#[doc = "Mask register for GPIO23.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO23MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO23MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_23_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23MASK_A {
        match self.bits {
            false => GPIO23MASK_A::UNMASKED,
            true => GPIO23MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO23MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO23MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_23_mask` writer - Mask register for GPIO23."]
pub type REG_GPIO_23_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO23MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_23_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO23MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO23MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_24_mask` reader - Mask register for GPIO24."]
pub type REG_GPIO_24_MASK_R = crate::BitReader<GPIO24MASK_A>;
#[doc = "Mask register for GPIO24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO24MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO24MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO24MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_24_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24MASK_A {
        match self.bits {
            false => GPIO24MASK_A::UNMASKED,
            true => GPIO24MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO24MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO24MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_24_mask` writer - Mask register for GPIO24."]
pub type REG_GPIO_24_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO24MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_24_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO24MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO24MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_25_mask` reader - Mask register for GPIO25."]
pub type REG_GPIO_25_MASK_R = crate::BitReader<GPIO25MASK_A>;
#[doc = "Mask register for GPIO25.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO25MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO25MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO25MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_25_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25MASK_A {
        match self.bits {
            false => GPIO25MASK_A::UNMASKED,
            true => GPIO25MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO25MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO25MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_25_mask` writer - Mask register for GPIO25."]
pub type REG_GPIO_25_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO25MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_25_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO25MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO25MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_26_mask` reader - Mask register for GPIO26."]
pub type REG_GPIO_26_MASK_R = crate::BitReader<GPIO26MASK_A>;
#[doc = "Mask register for GPIO26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO26MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO26MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO26MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_26_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26MASK_A {
        match self.bits {
            false => GPIO26MASK_A::UNMASKED,
            true => GPIO26MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO26MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO26MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_26_mask` writer - Mask register for GPIO26."]
pub type REG_GPIO_26_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO26MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_26_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO26MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO26MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_27_mask` reader - Mask register for GPIO27."]
pub type REG_GPIO_27_MASK_R = crate::BitReader<GPIO27MASK_A>;
#[doc = "Mask register for GPIO27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO27MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO27MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO27MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_27_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27MASK_A {
        match self.bits {
            false => GPIO27MASK_A::UNMASKED,
            true => GPIO27MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO27MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO27MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_27_mask` writer - Mask register for GPIO27."]
pub type REG_GPIO_27_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO27MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_27_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO27MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO27MASK_A::MASKED)
    }
}
#[doc = "Field `reg_gpio_28_mask` reader - Mask register for GPIO28."]
pub type REG_GPIO_28_MASK_R = crate::BitReader<GPIO28MASK_A>;
#[doc = "Mask register for GPIO28.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO28MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<GPIO28MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO28MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_28_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28MASK_A {
        match self.bits {
            false => GPIO28MASK_A::UNMASKED,
            true => GPIO28MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == GPIO28MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GPIO28MASK_A::MASKED
    }
}
#[doc = "Field `reg_gpio_28_mask` writer - Mask register for GPIO28."]
pub type REG_GPIO_28_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_INT_MASK1_SPEC, GPIO28MASK_A, O>;
impl<'a, const O: u8> REG_GPIO_28_MASK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(GPIO28MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GPIO28MASK_A::MASKED)
    }
}
impl R {
    #[doc = "Bit 0 - Mask register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_mask(&self) -> REG_GPIO_0_MASK_R {
        REG_GPIO_0_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_mask(&self) -> REG_GPIO_1_MASK_R {
        REG_GPIO_1_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_mask(&self) -> REG_GPIO_2_MASK_R {
        REG_GPIO_2_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_mask(&self) -> REG_GPIO_3_MASK_R {
        REG_GPIO_3_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_mask(&self) -> REG_GPIO_4_MASK_R {
        REG_GPIO_4_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_mask(&self) -> REG_GPIO_5_MASK_R {
        REG_GPIO_5_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_mask(&self) -> REG_GPIO_6_MASK_R {
        REG_GPIO_6_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_mask(&self) -> REG_GPIO_7_MASK_R {
        REG_GPIO_7_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_mask(&self) -> REG_GPIO_8_MASK_R {
        REG_GPIO_8_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_mask(&self) -> REG_GPIO_9_MASK_R {
        REG_GPIO_9_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_mask(&self) -> REG_GPIO_10_MASK_R {
        REG_GPIO_10_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_mask(&self) -> REG_GPIO_11_MASK_R {
        REG_GPIO_11_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_mask(&self) -> REG_GPIO_12_MASK_R {
        REG_GPIO_12_MASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_mask(&self) -> REG_GPIO_13_MASK_R {
        REG_GPIO_13_MASK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mask register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_mask(&self) -> REG_GPIO_14_MASK_R {
        REG_GPIO_14_MASK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_mask(&self) -> REG_GPIO_15_MASK_R {
        REG_GPIO_15_MASK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_mask(&self) -> REG_GPIO_16_MASK_R {
        REG_GPIO_16_MASK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_mask(&self) -> REG_GPIO_17_MASK_R {
        REG_GPIO_17_MASK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mask register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_mask(&self) -> REG_GPIO_18_MASK_R {
        REG_GPIO_18_MASK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mask register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_mask(&self) -> REG_GPIO_19_MASK_R {
        REG_GPIO_19_MASK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Mask register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_mask(&self) -> REG_GPIO_20_MASK_R {
        REG_GPIO_20_MASK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Mask register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_mask(&self) -> REG_GPIO_21_MASK_R {
        REG_GPIO_21_MASK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mask register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_mask(&self) -> REG_GPIO_22_MASK_R {
        REG_GPIO_22_MASK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mask register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_mask(&self) -> REG_GPIO_23_MASK_R {
        REG_GPIO_23_MASK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_mask(&self) -> REG_GPIO_24_MASK_R {
        REG_GPIO_24_MASK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mask register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_mask(&self) -> REG_GPIO_25_MASK_R {
        REG_GPIO_25_MASK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mask register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_mask(&self) -> REG_GPIO_26_MASK_R {
        REG_GPIO_26_MASK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Mask register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_mask(&self) -> REG_GPIO_27_MASK_R {
        REG_GPIO_27_MASK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Mask register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_mask(&self) -> REG_GPIO_28_MASK_R {
        REG_GPIO_28_MASK_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask register for GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_mask(&mut self) -> REG_GPIO_0_MASK_W<0> {
        REG_GPIO_0_MASK_W::new(self)
    }
    #[doc = "Bit 1 - Mask register for GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_mask(&mut self) -> REG_GPIO_1_MASK_W<1> {
        REG_GPIO_1_MASK_W::new(self)
    }
    #[doc = "Bit 2 - Mask register for GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_mask(&mut self) -> REG_GPIO_2_MASK_W<2> {
        REG_GPIO_2_MASK_W::new(self)
    }
    #[doc = "Bit 3 - Mask register for GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_mask(&mut self) -> REG_GPIO_3_MASK_W<3> {
        REG_GPIO_3_MASK_W::new(self)
    }
    #[doc = "Bit 4 - Mask register for GPIO4."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_mask(&mut self) -> REG_GPIO_4_MASK_W<4> {
        REG_GPIO_4_MASK_W::new(self)
    }
    #[doc = "Bit 5 - Mask register for GPIO5."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_mask(&mut self) -> REG_GPIO_5_MASK_W<5> {
        REG_GPIO_5_MASK_W::new(self)
    }
    #[doc = "Bit 6 - Mask register for GPIO6."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_mask(&mut self) -> REG_GPIO_6_MASK_W<6> {
        REG_GPIO_6_MASK_W::new(self)
    }
    #[doc = "Bit 7 - Mask register for GPIO7."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_mask(&mut self) -> REG_GPIO_7_MASK_W<7> {
        REG_GPIO_7_MASK_W::new(self)
    }
    #[doc = "Bit 8 - Mask register for GPIO8."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_8_mask(&mut self) -> REG_GPIO_8_MASK_W<8> {
        REG_GPIO_8_MASK_W::new(self)
    }
    #[doc = "Bit 9 - Mask register for GPIO9."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_9_mask(&mut self) -> REG_GPIO_9_MASK_W<9> {
        REG_GPIO_9_MASK_W::new(self)
    }
    #[doc = "Bit 10 - Mask register for GPIO10."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_10_mask(&mut self) -> REG_GPIO_10_MASK_W<10> {
        REG_GPIO_10_MASK_W::new(self)
    }
    #[doc = "Bit 11 - Mask register for GPIO11."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_11_mask(&mut self) -> REG_GPIO_11_MASK_W<11> {
        REG_GPIO_11_MASK_W::new(self)
    }
    #[doc = "Bit 12 - Mask register for GPIO12."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_mask(&mut self) -> REG_GPIO_12_MASK_W<12> {
        REG_GPIO_12_MASK_W::new(self)
    }
    #[doc = "Bit 13 - Mask register for GPIO13."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_mask(&mut self) -> REG_GPIO_13_MASK_W<13> {
        REG_GPIO_13_MASK_W::new(self)
    }
    #[doc = "Bit 14 - Mask register for GPIO14."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_mask(&mut self) -> REG_GPIO_14_MASK_W<14> {
        REG_GPIO_14_MASK_W::new(self)
    }
    #[doc = "Bit 15 - Mask register for GPIO15."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_mask(&mut self) -> REG_GPIO_15_MASK_W<15> {
        REG_GPIO_15_MASK_W::new(self)
    }
    #[doc = "Bit 16 - Mask register for GPIO16."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_16_mask(&mut self) -> REG_GPIO_16_MASK_W<16> {
        REG_GPIO_16_MASK_W::new(self)
    }
    #[doc = "Bit 17 - Mask register for GPIO17."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_17_mask(&mut self) -> REG_GPIO_17_MASK_W<17> {
        REG_GPIO_17_MASK_W::new(self)
    }
    #[doc = "Bit 18 - Mask register for GPIO18."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_mask(&mut self) -> REG_GPIO_18_MASK_W<18> {
        REG_GPIO_18_MASK_W::new(self)
    }
    #[doc = "Bit 19 - Mask register for GPIO19."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_mask(&mut self) -> REG_GPIO_19_MASK_W<19> {
        REG_GPIO_19_MASK_W::new(self)
    }
    #[doc = "Bit 20 - Mask register for GPIO20."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_mask(&mut self) -> REG_GPIO_20_MASK_W<20> {
        REG_GPIO_20_MASK_W::new(self)
    }
    #[doc = "Bit 21 - Mask register for GPIO21."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_mask(&mut self) -> REG_GPIO_21_MASK_W<21> {
        REG_GPIO_21_MASK_W::new(self)
    }
    #[doc = "Bit 22 - Mask register for GPIO22."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_mask(&mut self) -> REG_GPIO_22_MASK_W<22> {
        REG_GPIO_22_MASK_W::new(self)
    }
    #[doc = "Bit 23 - Mask register for GPIO23."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_mask(&mut self) -> REG_GPIO_23_MASK_W<23> {
        REG_GPIO_23_MASK_W::new(self)
    }
    #[doc = "Bit 24 - Mask register for GPIO24."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_mask(&mut self) -> REG_GPIO_24_MASK_W<24> {
        REG_GPIO_24_MASK_W::new(self)
    }
    #[doc = "Bit 25 - Mask register for GPIO25."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_mask(&mut self) -> REG_GPIO_25_MASK_W<25> {
        REG_GPIO_25_MASK_W::new(self)
    }
    #[doc = "Bit 26 - Mask register for GPIO26."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_mask(&mut self) -> REG_GPIO_26_MASK_W<26> {
        REG_GPIO_26_MASK_W::new(self)
    }
    #[doc = "Bit 27 - Mask register for GPIO27."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_mask(&mut self) -> REG_GPIO_27_MASK_W<27> {
        REG_GPIO_27_MASK_W::new(self)
    }
    #[doc = "Bit 28 - Mask register for GPIO28."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_mask(&mut self) -> REG_GPIO_28_MASK_W<28> {
        REG_GPIO_28_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt masking register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mask1](index.html) module"]
pub struct GPIO_INT_MASK1_SPEC;
impl crate::RegisterSpec for GPIO_INT_MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_mask1::R](R) reader structure"]
impl crate::Readable for GPIO_INT_MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_mask1::W](W) writer structure"]
impl crate::Writable for GPIO_INT_MASK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_INT_MASK1 to value 0xffff_ffff"]
impl crate::Resettable for GPIO_INT_MASK1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

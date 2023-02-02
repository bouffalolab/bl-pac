#[doc = "Register `GPIO_CFGCTL12` reader"]
pub struct R(crate::R<GPIO_CFGCTL12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL12` writer"]
pub struct W(crate::W<GPIO_CFGCTL12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL12_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_24_ie` reader - Input enable for GPIO24."]
pub type REG_GPIO_24_IE_R = crate::BitReader<GPIO24INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO24INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO24INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO24INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_24_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24INPUT_ENABLED_A {
        match self.bits {
            false => GPIO24INPUT_ENABLED_A::DISABLED,
            true => GPIO24INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO24INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO24INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_24_ie` writer - Input enable for GPIO24."]
pub type REG_GPIO_24_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL12_SPEC, GPIO24INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_24_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO24INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO24INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_24_smt` reader - Schmitt trigger enabled for GPIO24."]
pub type REG_GPIO_24_SMT_R = crate::BitReader<GPIO24SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO24SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO24SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO24SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_24_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24SCHMITT_A {
        match self.bits {
            false => GPIO24SCHMITT_A::DISABLED,
            true => GPIO24SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO24SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO24SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_24_smt` writer - Schmitt trigger enabled for GPIO24."]
pub type REG_GPIO_24_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL12_SPEC, GPIO24SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_24_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO24SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO24SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_24_drv` reader - Driving control enabled for GPIO24."]
pub type REG_GPIO_24_DRV_R = crate::FieldReader<u8, GPIO24DRIVING_A>;
#[doc = "Driving control enabled for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO24DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO24DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO24DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_24_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO24DRIVING_A> {
        match self.bits {
            0 => Some(GPIO24DRIVING_A::DISABLED),
            1 => Some(GPIO24DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO24DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO24DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_24_drv` writer - Driving control enabled for GPIO24."]
pub type REG_GPIO_24_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL12_SPEC, u8, GPIO24DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_24_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO24DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO24DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_24_pu` reader - Pull Up Resistor for GPIO24."]
pub type REG_GPIO_24_PU_R = crate::BitReader<GPIO24PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO24PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO24PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO24PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_24_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO24PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO24PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO24PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO24PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_24_pu` writer - Pull Up Resistor for GPIO24."]
pub type REG_GPIO_24_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL12_SPEC, GPIO24PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_24_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO24PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO24PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_24_pd` reader - Pull Down Resistor for GPIO24."]
pub type REG_GPIO_24_PD_R = crate::BitReader<GPIO24PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO24PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO24PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO24PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_24_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO24PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO24PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO24PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO24PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO24PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_24_pd` writer - Pull Down Resistor for GPIO24."]
pub type REG_GPIO_24_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL12_SPEC, GPIO24PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_24_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO24PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO24PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_25_ie` reader - Input enable for GPIO25."]
pub type REG_GPIO_25_IE_R = crate::BitReader<GPIO25INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO25.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO25INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO25INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO25INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_25_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25INPUT_ENABLED_A {
        match self.bits {
            false => GPIO25INPUT_ENABLED_A::DISABLED,
            true => GPIO25INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO25INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO25INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_25_ie` writer - Input enable for GPIO25."]
pub type REG_GPIO_25_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL12_SPEC, GPIO25INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_25_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO25INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO25INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_25_smt` reader - Schmitt trigger enabled for GPIO25."]
pub type REG_GPIO_25_SMT_R = crate::BitReader<GPIO25SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO25.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO25SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO25SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO25SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_25_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25SCHMITT_A {
        match self.bits {
            false => GPIO25SCHMITT_A::DISABLED,
            true => GPIO25SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO25SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO25SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_25_smt` writer - Schmitt trigger enabled for GPIO25."]
pub type REG_GPIO_25_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL12_SPEC, GPIO25SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_25_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO25SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO25SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_25_drv` reader - Driving control enabled for GPIO25."]
pub type REG_GPIO_25_DRV_R = crate::FieldReader<u8, GPIO25DRIVING_A>;
#[doc = "Driving control enabled for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO25DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO25DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO25DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_25_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO25DRIVING_A> {
        match self.bits {
            0 => Some(GPIO25DRIVING_A::DISABLED),
            1 => Some(GPIO25DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO25DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO25DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_25_drv` writer - Driving control enabled for GPIO25."]
pub type REG_GPIO_25_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL12_SPEC, u8, GPIO25DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_25_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO25DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO25DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_25_pu` reader - Pull Up Resistor for GPIO25."]
pub type REG_GPIO_25_PU_R = crate::BitReader<GPIO25PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO25PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO25PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO25PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_25_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO25PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO25PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO25PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO25PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_25_pu` writer - Pull Up Resistor for GPIO25."]
pub type REG_GPIO_25_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL12_SPEC, GPIO25PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_25_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO25PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO25PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_25_pd` reader - Pull Down Resistor for GPIO25."]
pub type REG_GPIO_25_PD_R = crate::BitReader<GPIO25PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO25PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO25PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO25PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_25_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO25PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO25PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO25PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO25PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO25PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_25_pd` writer - Pull Down Resistor for GPIO25."]
pub type REG_GPIO_25_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL12_SPEC, GPIO25PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_25_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO25PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO25PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_ie(&self) -> REG_GPIO_24_IE_R {
        REG_GPIO_24_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_smt(&self) -> REG_GPIO_24_SMT_R {
        REG_GPIO_24_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_drv(&self) -> REG_GPIO_24_DRV_R {
        REG_GPIO_24_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_pu(&self) -> REG_GPIO_24_PU_R {
        REG_GPIO_24_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_pd(&self) -> REG_GPIO_24_PD_R {
        REG_GPIO_24_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Input enable for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_ie(&self) -> REG_GPIO_25_IE_R {
        REG_GPIO_25_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_smt(&self) -> REG_GPIO_25_SMT_R {
        REG_GPIO_25_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_drv(&self) -> REG_GPIO_25_DRV_R {
        REG_GPIO_25_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_pu(&self) -> REG_GPIO_25_PU_R {
        REG_GPIO_25_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_pd(&self) -> REG_GPIO_25_PD_R {
        REG_GPIO_25_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO24."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_ie(&mut self) -> REG_GPIO_24_IE_W<0> {
        REG_GPIO_24_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO24."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_smt(&mut self) -> REG_GPIO_24_SMT_W<1> {
        REG_GPIO_24_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO24."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_drv(&mut self) -> REG_GPIO_24_DRV_W<2> {
        REG_GPIO_24_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO24."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_pu(&mut self) -> REG_GPIO_24_PU_W<4> {
        REG_GPIO_24_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO24."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_pd(&mut self) -> REG_GPIO_24_PD_W<5> {
        REG_GPIO_24_PD_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO25."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_ie(&mut self) -> REG_GPIO_25_IE_W<16> {
        REG_GPIO_25_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO25."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_smt(&mut self) -> REG_GPIO_25_SMT_W<17> {
        REG_GPIO_25_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO25."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_drv(&mut self) -> REG_GPIO_25_DRV_W<18> {
        REG_GPIO_25_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO25."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_pu(&mut self) -> REG_GPIO_25_PU_W<20> {
        REG_GPIO_25_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO25."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_pd(&mut self) -> REG_GPIO_25_PD_W<21> {
        REG_GPIO_25_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO24, GPIO25 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl12](index.html) module"]
pub struct GPIO_CFGCTL12_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl12::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl12::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL12 to value 0x0003_0023"]
impl crate::Resettable for GPIO_CFGCTL12_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0023;
}

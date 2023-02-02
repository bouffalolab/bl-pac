#[doc = "Register `GPIO_CFGCTL13` reader"]
pub struct R(crate::R<GPIO_CFGCTL13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL13` writer"]
pub struct W(crate::W<GPIO_CFGCTL13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL13_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_26_ie` reader - Input enable for GPIO26."]
pub type REG_GPIO_26_IE_R = crate::BitReader<GPIO26INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO26INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO26INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO26INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_26_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26INPUT_ENABLED_A {
        match self.bits {
            false => GPIO26INPUT_ENABLED_A::DISABLED,
            true => GPIO26INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO26INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO26INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_26_ie` writer - Input enable for GPIO26."]
pub type REG_GPIO_26_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL13_SPEC, GPIO26INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_26_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO26INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO26INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_26_smt` reader - Schmitt trigger enabled for GPIO26."]
pub type REG_GPIO_26_SMT_R = crate::BitReader<GPIO26SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO26SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO26SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO26SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_26_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26SCHMITT_A {
        match self.bits {
            false => GPIO26SCHMITT_A::DISABLED,
            true => GPIO26SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO26SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO26SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_26_smt` writer - Schmitt trigger enabled for GPIO26."]
pub type REG_GPIO_26_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL13_SPEC, GPIO26SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_26_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO26SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO26SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_26_drv` reader - Driving control enabled for GPIO26."]
pub type REG_GPIO_26_DRV_R = crate::FieldReader<u8, GPIO26DRIVING_A>;
#[doc = "Driving control enabled for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO26DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO26DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO26DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_26_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO26DRIVING_A> {
        match self.bits {
            0 => Some(GPIO26DRIVING_A::DISABLED),
            1 => Some(GPIO26DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO26DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO26DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_26_drv` writer - Driving control enabled for GPIO26."]
pub type REG_GPIO_26_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL13_SPEC, u8, GPIO26DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_26_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO26DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO26DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_26_pu` reader - Pull Up Resistor for GPIO26."]
pub type REG_GPIO_26_PU_R = crate::BitReader<GPIO26PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO26PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO26PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO26PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_26_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO26PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO26PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO26PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO26PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_26_pu` writer - Pull Up Resistor for GPIO26."]
pub type REG_GPIO_26_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL13_SPEC, GPIO26PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_26_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO26PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO26PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_26_pd` reader - Pull Down Resistor for GPIO26."]
pub type REG_GPIO_26_PD_R = crate::BitReader<GPIO26PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO26PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO26PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO26PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_26_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO26PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO26PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO26PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO26PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO26PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_26_pd` writer - Pull Down Resistor for GPIO26."]
pub type REG_GPIO_26_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL13_SPEC, GPIO26PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_26_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO26PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO26PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_27_ie` reader - Input enable for GPIO27."]
pub type REG_GPIO_27_IE_R = crate::BitReader<GPIO27INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO27INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO27INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO27INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_27_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27INPUT_ENABLED_A {
        match self.bits {
            false => GPIO27INPUT_ENABLED_A::DISABLED,
            true => GPIO27INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO27INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO27INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_27_ie` writer - Input enable for GPIO27."]
pub type REG_GPIO_27_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL13_SPEC, GPIO27INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_27_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO27INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO27INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_27_smt` reader - Schmitt trigger enabled for GPIO27."]
pub type REG_GPIO_27_SMT_R = crate::BitReader<GPIO27SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO27SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO27SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO27SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_27_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27SCHMITT_A {
        match self.bits {
            false => GPIO27SCHMITT_A::DISABLED,
            true => GPIO27SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO27SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO27SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_27_smt` writer - Schmitt trigger enabled for GPIO27."]
pub type REG_GPIO_27_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL13_SPEC, GPIO27SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_27_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO27SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO27SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_27_drv` reader - Driving control enabled for GPIO27."]
pub type REG_GPIO_27_DRV_R = crate::FieldReader<u8, GPIO27DRIVING_A>;
#[doc = "Driving control enabled for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO27DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO27DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO27DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_27_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO27DRIVING_A> {
        match self.bits {
            0 => Some(GPIO27DRIVING_A::DISABLED),
            1 => Some(GPIO27DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO27DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO27DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_27_drv` writer - Driving control enabled for GPIO27."]
pub type REG_GPIO_27_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL13_SPEC, u8, GPIO27DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_27_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO27DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO27DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_27_pu` reader - Pull Up Resistor for GPIO27."]
pub type REG_GPIO_27_PU_R = crate::BitReader<GPIO27PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO27PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO27PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO27PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_27_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO27PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO27PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO27PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO27PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_27_pu` writer - Pull Up Resistor for GPIO27."]
pub type REG_GPIO_27_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL13_SPEC, GPIO27PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_27_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO27PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO27PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_27_pd` reader - Pull Down Resistor for GPIO27."]
pub type REG_GPIO_27_PD_R = crate::BitReader<GPIO27PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO27PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO27PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO27PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_27_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO27PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO27PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO27PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO27PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO27PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_27_pd` writer - Pull Down Resistor for GPIO27."]
pub type REG_GPIO_27_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL13_SPEC, GPIO27PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_27_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO27PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO27PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_ie(&self) -> REG_GPIO_26_IE_R {
        REG_GPIO_26_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_smt(&self) -> REG_GPIO_26_SMT_R {
        REG_GPIO_26_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_drv(&self) -> REG_GPIO_26_DRV_R {
        REG_GPIO_26_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_pu(&self) -> REG_GPIO_26_PU_R {
        REG_GPIO_26_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_pd(&self) -> REG_GPIO_26_PD_R {
        REG_GPIO_26_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Input enable for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_ie(&self) -> REG_GPIO_27_IE_R {
        REG_GPIO_27_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_smt(&self) -> REG_GPIO_27_SMT_R {
        REG_GPIO_27_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_drv(&self) -> REG_GPIO_27_DRV_R {
        REG_GPIO_27_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_pu(&self) -> REG_GPIO_27_PU_R {
        REG_GPIO_27_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_pd(&self) -> REG_GPIO_27_PD_R {
        REG_GPIO_27_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO26."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_ie(&mut self) -> REG_GPIO_26_IE_W<0> {
        REG_GPIO_26_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO26."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_smt(&mut self) -> REG_GPIO_26_SMT_W<1> {
        REG_GPIO_26_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO26."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_drv(&mut self) -> REG_GPIO_26_DRV_W<2> {
        REG_GPIO_26_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO26."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_pu(&mut self) -> REG_GPIO_26_PU_W<4> {
        REG_GPIO_26_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO26."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_pd(&mut self) -> REG_GPIO_26_PD_W<5> {
        REG_GPIO_26_PD_W::new(self)
    }
    #[doc = "Bit 16 - Input enable for GPIO27."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_ie(&mut self) -> REG_GPIO_27_IE_W<16> {
        REG_GPIO_27_IE_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO27."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_smt(&mut self) -> REG_GPIO_27_SMT_W<17> {
        REG_GPIO_27_SMT_W::new(self)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO27."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_drv(&mut self) -> REG_GPIO_27_DRV_W<18> {
        REG_GPIO_27_DRV_W::new(self)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO27."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_pu(&mut self) -> REG_GPIO_27_PU_W<20> {
        REG_GPIO_27_PU_W::new(self)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO27."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_pd(&mut self) -> REG_GPIO_27_PD_W<21> {
        REG_GPIO_27_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO26, GPIO27 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl13](index.html) module"]
pub struct GPIO_CFGCTL13_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl13::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl13::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL13 to value 0x0003_0003"]
impl crate::Resettable for GPIO_CFGCTL13_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0003;
}

#[doc = "Register `GPIO_CFGCTL14` reader"]
pub struct R(crate::R<GPIO_CFGCTL14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL14` writer"]
pub struct W(crate::W<GPIO_CFGCTL14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL14_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_28_ie` reader - Input enable for GPIO28."]
pub type REG_GPIO_28_IE_R = crate::BitReader<GPIO28INPUT_ENABLED_A>;
#[doc = "Input enable for GPIO28.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO28INPUT_ENABLED_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO28INPUT_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO28INPUT_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_28_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28INPUT_ENABLED_A {
        match self.bits {
            false => GPIO28INPUT_ENABLED_A::DISABLED,
            true => GPIO28INPUT_ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO28INPUT_ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO28INPUT_ENABLED_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_28_ie` writer - Input enable for GPIO28."]
pub type REG_GPIO_28_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL14_SPEC, GPIO28INPUT_ENABLED_A, O>;
impl<'a, const O: u8> REG_GPIO_28_IE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO28INPUT_ENABLED_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO28INPUT_ENABLED_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_28_smt` reader - Schmitt trigger enabled for GPIO28."]
pub type REG_GPIO_28_SMT_R = crate::BitReader<GPIO28SCHMITT_A>;
#[doc = "Schmitt trigger enabled for GPIO28.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO28SCHMITT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO28SCHMITT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO28SCHMITT_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_28_SMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28SCHMITT_A {
        match self.bits {
            false => GPIO28SCHMITT_A::DISABLED,
            true => GPIO28SCHMITT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO28SCHMITT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO28SCHMITT_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_28_smt` writer - Schmitt trigger enabled for GPIO28."]
pub type REG_GPIO_28_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL14_SPEC, GPIO28SCHMITT_A, O>;
impl<'a, const O: u8> REG_GPIO_28_SMT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO28SCHMITT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO28SCHMITT_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_28_drv` reader - Driving control enabled for GPIO28."]
pub type REG_GPIO_28_DRV_R = crate::FieldReader<u8, GPIO28DRIVING_A>;
#[doc = "Driving control enabled for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO28DRIVING_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO28DRIVING_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO28DRIVING_A) -> Self {
        variant as _
    }
}
impl REG_GPIO_28_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO28DRIVING_A> {
        match self.bits {
            0 => Some(GPIO28DRIVING_A::DISABLED),
            1 => Some(GPIO28DRIVING_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO28DRIVING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO28DRIVING_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_28_drv` writer - Driving control enabled for GPIO28."]
pub type REG_GPIO_28_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL14_SPEC, u8, GPIO28DRIVING_A, 2, O>;
impl<'a, const O: u8> REG_GPIO_28_DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO28DRIVING_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO28DRIVING_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_28_pu` reader - Pull Up Resistor for GPIO28."]
pub type REG_GPIO_28_PU_R = crate::BitReader<GPIO28PULL_UP_RESISTOR_A>;
#[doc = "Pull Up Resistor for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO28PULL_UP_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO28PULL_UP_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO28PULL_UP_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_28_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28PULL_UP_RESISTOR_A {
        match self.bits {
            false => GPIO28PULL_UP_RESISTOR_A::DISABLED,
            true => GPIO28PULL_UP_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO28PULL_UP_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO28PULL_UP_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_28_pu` writer - Pull Up Resistor for GPIO28."]
pub type REG_GPIO_28_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL14_SPEC, GPIO28PULL_UP_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_28_PU_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO28PULL_UP_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO28PULL_UP_RESISTOR_A::ENABLED)
    }
}
#[doc = "Field `reg_gpio_28_pd` reader - Pull Down Resistor for GPIO28."]
pub type REG_GPIO_28_PD_R = crate::BitReader<GPIO28PULL_DOWN_RESISTOR_A>;
#[doc = "Pull Down Resistor for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO28PULL_DOWN_RESISTOR_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<GPIO28PULL_DOWN_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO28PULL_DOWN_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_GPIO_28_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO28PULL_DOWN_RESISTOR_A {
        match self.bits {
            false => GPIO28PULL_DOWN_RESISTOR_A::DISABLED,
            true => GPIO28PULL_DOWN_RESISTOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO28PULL_DOWN_RESISTOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO28PULL_DOWN_RESISTOR_A::ENABLED
    }
}
#[doc = "Field `reg_gpio_28_pd` writer - Pull Down Resistor for GPIO28."]
pub type REG_GPIO_28_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL14_SPEC, GPIO28PULL_DOWN_RESISTOR_A, O>;
impl<'a, const O: u8> REG_GPIO_28_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO28PULL_DOWN_RESISTOR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO28PULL_DOWN_RESISTOR_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_ie(&self) -> REG_GPIO_28_IE_R {
        REG_GPIO_28_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_smt(&self) -> REG_GPIO_28_SMT_R {
        REG_GPIO_28_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_drv(&self) -> REG_GPIO_28_DRV_R {
        REG_GPIO_28_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_pu(&self) -> REG_GPIO_28_PU_R {
        REG_GPIO_28_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_pd(&self) -> REG_GPIO_28_PD_R {
        REG_GPIO_28_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO28."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_ie(&mut self) -> REG_GPIO_28_IE_W<0> {
        REG_GPIO_28_IE_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO28."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_smt(&mut self) -> REG_GPIO_28_SMT_W<1> {
        REG_GPIO_28_SMT_W::new(self)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO28."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_drv(&mut self) -> REG_GPIO_28_DRV_W<2> {
        REG_GPIO_28_DRV_W::new(self)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO28."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_pu(&mut self) -> REG_GPIO_28_PU_W<4> {
        REG_GPIO_28_PU_W::new(self)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO28."]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_pd(&mut self) -> REG_GPIO_28_PD_W<5> {
        REG_GPIO_28_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO28 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl14](index.html) module"]
pub struct GPIO_CFGCTL14_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl14::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl14::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL14 to value 0x03"]
impl crate::Resettable for GPIO_CFGCTL14_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}

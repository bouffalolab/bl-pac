#[doc = "Register `flash_config` reader"]
pub struct R(crate::R<FLASH_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `flash_config` writer"]
pub struct W(crate::W<FLASH_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_CONFIG_SPEC>;
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
impl From<crate::W<FLASH_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clock_divide` reader - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clock_divide` writer - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `clock_enable` reader - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `clock_enable` writer - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CONFIG_SPEC, bool, O>;
#[doc = "Field `clock_source_0` reader - Peripheral clock source register 0"]
pub type CLOCK_SOURCE_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clock_source_0` writer - Peripheral clock source register 0"]
pub type CLOCK_SOURCE_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `clock_source_1` reader - Peripheral clock source register 1"]
pub type CLOCK_SOURCE_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clock_source_1` writer - Peripheral clock source register 1"]
pub type CLOCK_SOURCE_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_CONFIG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 8:10 - Peripheral clock divide factor"]
    #[inline(always)]
    pub fn clock_divide(&self) -> CLOCK_DIVIDE_R {
        CLOCK_DIVIDE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Peripheral level clock gate enable"]
    #[inline(always)]
    pub fn clock_enable(&self) -> CLOCK_ENABLE_R {
        CLOCK_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Peripheral clock source register 0"]
    #[inline(always)]
    pub fn clock_source_0(&self) -> CLOCK_SOURCE_0_R {
        CLOCK_SOURCE_0_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock source register 1"]
    #[inline(always)]
    pub fn clock_source_1(&self) -> CLOCK_SOURCE_1_R {
        CLOCK_SOURCE_1_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Peripheral clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clock_divide(&mut self) -> CLOCK_DIVIDE_W<8> {
        CLOCK_DIVIDE_W::new(self)
    }
    #[doc = "Bit 11 - Peripheral level clock gate enable"]
    #[inline(always)]
    #[must_use]
    pub fn clock_enable(&mut self) -> CLOCK_ENABLE_W<11> {
        CLOCK_ENABLE_W::new(self)
    }
    #[doc = "Bits 12:13 - Peripheral clock source register 0"]
    #[inline(always)]
    #[must_use]
    pub fn clock_source_0(&mut self) -> CLOCK_SOURCE_0_W<12> {
        CLOCK_SOURCE_0_W::new(self)
    }
    #[doc = "Bits 14:15 - Peripheral clock source register 1"]
    #[inline(always)]
    #[must_use]
    pub fn clock_source_1(&mut self) -> CLOCK_SOURCE_1_W<14> {
        CLOCK_SOURCE_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial flash configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_config](index.html) module"]
pub struct FLASH_CONFIG_SPEC;
impl crate::RegisterSpec for FLASH_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_config::R](R) reader structure"]
impl crate::Readable for FLASH_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_config::W](W) writer structure"]
impl crate::Writable for FLASH_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets flash_config to value 0"]
impl crate::Resettable for FLASH_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

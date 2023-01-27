#[doc = "Register `config` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `config` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write 1 to clear internal checksum states\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLEAR_AW {
    #[doc = "1: Write 1 to clear internal states"]
    CLEAR = 1,
}
impl From<CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clear` writer - Write 1 to clear internal checksum states"]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, CLEAR_AW, O>;
impl<'a, const O: u8> CLEAR_W<'a, O> {
    #[doc = "Write 1 to clear internal states"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLEAR_AW::CLEAR)
    }
}
#[doc = "Field `endian` reader - Sets endian of input data"]
pub type ENDIAN_R = crate::BitReader<ENDIAN_A>;
#[doc = "Sets endian of input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDIAN_A {
    #[doc = "0: Little endian"]
    LITTLE = 0,
    #[doc = "1: Big endian"]
    BIG = 1,
}
impl From<ENDIAN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIAN_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDIAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIAN_A {
        match self.bits {
            false => ENDIAN_A::LITTLE,
            true => ENDIAN_A::BIG,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE`"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == ENDIAN_A::LITTLE
    }
    #[doc = "Checks if the value of the field is `BIG`"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == ENDIAN_A::BIG
    }
}
#[doc = "Field `endian` writer - Sets endian of input data"]
pub type ENDIAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, ENDIAN_A, O>;
impl<'a, const O: u8> ENDIAN_W<'a, O> {
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn little(self) -> &'a mut W {
        self.variant(ENDIAN_A::LITTLE)
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn big(self) -> &'a mut W {
        self.variant(ENDIAN_A::BIG)
    }
}
impl R {
    #[doc = "Bit 1 - Sets endian of input data"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear internal checksum states"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<0> {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 1 - Sets endian of input data"]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> ENDIAN_W<1> {
        ENDIAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets config to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

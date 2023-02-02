#[doc = "Register `singen_ctrl0` reader"]
pub struct R(crate::R<SINGEN_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGEN_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGEN_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGEN_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `singen_ctrl0` writer"]
pub struct W(crate::W<SINGEN_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGEN_CTRL0_SPEC>;
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
impl From<crate::W<SINGEN_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGEN_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `singen_inc_step1` reader - "]
pub type SINGEN_INC_STEP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_inc_step1` writer - "]
pub type SINGEN_INC_STEP1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGEN_CTRL0_SPEC, u16, u16, 10, O>;
#[doc = "Field `singen_inc_step0` reader - "]
pub type SINGEN_INC_STEP0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_inc_step0` writer - "]
pub type SINGEN_INC_STEP0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGEN_CTRL0_SPEC, u16, u16, 10, O>;
#[doc = "Field `singen_unsign_en` reader - "]
pub type SINGEN_UNSIGN_EN_R = crate::BitReader<bool>;
#[doc = "Field `singen_unsign_en` writer - "]
pub type SINGEN_UNSIGN_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SINGEN_CTRL0_SPEC, bool, O>;
#[doc = "Field `singen_clkdiv_n` reader - "]
pub type SINGEN_CLKDIV_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `singen_clkdiv_n` writer - "]
pub type SINGEN_CLKDIV_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGEN_CTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `singen_en` reader - "]
pub type SINGEN_EN_R = crate::BitReader<bool>;
#[doc = "Field `singen_en` writer - "]
pub type SINGEN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINGEN_CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_inc_step1(&self) -> SINGEN_INC_STEP1_R {
        SINGEN_INC_STEP1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_inc_step0(&self) -> SINGEN_INC_STEP0_R {
        SINGEN_INC_STEP0_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_unsign_en(&self) -> SINGEN_UNSIGN_EN_R {
        SINGEN_UNSIGN_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn singen_clkdiv_n(&self) -> SINGEN_CLKDIV_N_R {
        SINGEN_CLKDIV_N_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn singen_en(&self) -> SINGEN_EN_R {
        SINGEN_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn singen_inc_step1(&mut self) -> SINGEN_INC_STEP1_W<0> {
        SINGEN_INC_STEP1_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn singen_inc_step0(&mut self) -> SINGEN_INC_STEP0_W<16> {
        SINGEN_INC_STEP0_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn singen_unsign_en(&mut self) -> SINGEN_UNSIGN_EN_W<28> {
        SINGEN_UNSIGN_EN_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn singen_clkdiv_n(&mut self) -> SINGEN_CLKDIV_N_W<29> {
        SINGEN_CLKDIV_N_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn singen_en(&mut self) -> SINGEN_EN_W<31> {
        SINGEN_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "singen_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl0](index.html) module"]
pub struct SINGEN_CTRL0_SPEC;
impl crate::RegisterSpec for SINGEN_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singen_ctrl0::R](R) reader structure"]
impl crate::Readable for SINGEN_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singen_ctrl0::W](W) writer structure"]
impl crate::Writable for SINGEN_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets singen_ctrl0 to value 0"]
impl crate::Resettable for SINGEN_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

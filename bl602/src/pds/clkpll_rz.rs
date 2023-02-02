#[doc = "Register `clkpll_rz` reader"]
pub struct R(crate::R<CLKPLL_RZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_RZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPLL_RZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPLL_RZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_rz` writer"]
pub struct W(crate::W<CLKPLL_RZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_RZ_SPEC>;
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
impl From<crate::W<CLKPLL_RZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPLL_RZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_c4_en` reader - "]
pub type CLKPLL_C4_EN_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_c4_en` writer - "]
pub type CLKPLL_C4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKPLL_RZ_SPEC, bool, O>;
#[doc = "Field `clkpll_r4` reader - "]
pub type CLKPLL_R4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_r4` writer - "]
pub type CLKPLL_R4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKPLL_RZ_SPEC, u8, u8, 2, O>;
#[doc = "Field `clkpll_r4_short` reader - "]
pub type CLKPLL_R4_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_r4_short` writer - "]
pub type CLKPLL_R4_SHORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKPLL_RZ_SPEC, bool, O>;
#[doc = "Field `clkpll_c3` reader - "]
pub type CLKPLL_C3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_c3` writer - "]
pub type CLKPLL_C3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKPLL_RZ_SPEC, u8, u8, 2, O>;
#[doc = "Field `clkpll_cz` reader - "]
pub type CLKPLL_CZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_cz` writer - "]
pub type CLKPLL_CZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKPLL_RZ_SPEC, u8, u8, 2, O>;
#[doc = "Field `clkpll_rz` reader - "]
pub type CLKPLL_RZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_rz` writer - "]
pub type CLKPLL_RZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKPLL_RZ_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_c4_en(&self) -> CLKPLL_C4_EN_R {
        CLKPLL_C4_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_r4(&self) -> CLKPLL_R4_R {
        CLKPLL_R4_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_r4_short(&self) -> CLKPLL_R4_SHORT_R {
        CLKPLL_R4_SHORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_c3(&self) -> CLKPLL_C3_R {
        CLKPLL_C3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn clkpll_cz(&self) -> CLKPLL_CZ_R {
        CLKPLL_CZ_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn clkpll_rz(&self) -> CLKPLL_RZ_R {
        CLKPLL_RZ_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_c4_en(&mut self) -> CLKPLL_C4_EN_W<0> {
        CLKPLL_C4_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_r4(&mut self) -> CLKPLL_R4_W<4> {
        CLKPLL_R4_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_r4_short(&mut self) -> CLKPLL_R4_SHORT_W<8> {
        CLKPLL_R4_SHORT_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_c3(&mut self) -> CLKPLL_C3_W<12> {
        CLKPLL_C3_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_cz(&mut self) -> CLKPLL_CZ_W<14> {
        CLKPLL_CZ_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_rz(&mut self) -> CLKPLL_RZ_W<16> {
        CLKPLL_RZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_rz.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_rz](index.html) module"]
pub struct CLKPLL_RZ_SPEC;
impl crate::RegisterSpec for CLKPLL_RZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_rz::R](R) reader structure"]
impl crate::Readable for CLKPLL_RZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_rz::W](W) writer structure"]
impl crate::Writable for CLKPLL_RZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clkpll_rz to value 0x0005_a020"]
impl crate::Resettable for CLKPLL_RZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0005_a020;
}

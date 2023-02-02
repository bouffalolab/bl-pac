#[doc = "Register `tzc_rom1_r0` reader"]
pub struct R(crate::R<TZC_ROM1_R0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_ROM1_R0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_ROM1_R0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_ROM1_R0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_rom1_r0` writer"]
pub struct W(crate::W<TZC_ROM1_R0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_ROM1_R0_SPEC>;
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
impl From<crate::W<TZC_ROM1_R0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_ROM1_R0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_rom1_r0_end` reader - "]
pub type TZC_ROM1_R0_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_rom1_r0_end` writer - "]
pub type TZC_ROM1_R0_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_ROM1_R0_SPEC, u16, u16, 16, O>;
#[doc = "Field `tzc_rom1_r0_start` reader - "]
pub type TZC_ROM1_R0_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_rom1_r0_start` writer - "]
pub type TZC_ROM1_R0_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_ROM1_R0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom1_r0_end(&self) -> TZC_ROM1_R0_END_R {
        TZC_ROM1_R0_END_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom1_r0_start(&self) -> TZC_ROM1_R0_START_R {
        TZC_ROM1_R0_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom1_r0_end(&mut self) -> TZC_ROM1_R0_END_W<0> {
        TZC_ROM1_R0_END_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom1_r0_start(&mut self) -> TZC_ROM1_R0_START_W<16> {
        TZC_ROM1_R0_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_rom1_r0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom1_r0](index.html) module"]
pub struct TZC_ROM1_R0_SPEC;
impl crate::RegisterSpec for TZC_ROM1_R0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_rom1_r0::R](R) reader structure"]
impl crate::Readable for TZC_ROM1_R0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_rom1_r0::W](W) writer structure"]
impl crate::Writable for TZC_ROM1_R0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_rom1_r0 to value 0xffff"]
impl crate::Resettable for TZC_ROM1_R0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}

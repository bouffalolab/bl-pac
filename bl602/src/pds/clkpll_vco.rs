#[doc = "Register `clkpll_vco` reader"]
pub struct R(crate::R<CLKPLL_VCO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_VCO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPLL_VCO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPLL_VCO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_vco` writer"]
pub struct W(crate::W<CLKPLL_VCO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_VCO_SPEC>;
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
impl From<crate::W<CLKPLL_VCO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPLL_VCO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_vco_speed` reader - "]
pub type CLKPLL_VCO_SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clkpll_vco_speed` writer - "]
pub type CLKPLL_VCO_SPEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKPLL_VCO_SPEC, u8, u8, 3, O>;
#[doc = "Field `clkpll_shrtr` reader - "]
pub type CLKPLL_SHRTR_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_shrtr` writer - "]
pub type CLKPLL_SHRTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKPLL_VCO_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clkpll_vco_speed(&self) -> CLKPLL_VCO_SPEED_R {
        CLKPLL_VCO_SPEED_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_shrtr(&self) -> CLKPLL_SHRTR_R {
        CLKPLL_SHRTR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_vco_speed(&mut self) -> CLKPLL_VCO_SPEED_W<0> {
        CLKPLL_VCO_SPEED_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_shrtr(&mut self) -> CLKPLL_SHRTR_W<3> {
        CLKPLL_SHRTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_vco.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_vco](index.html) module"]
pub struct CLKPLL_VCO_SPEC;
impl crate::RegisterSpec for CLKPLL_VCO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_vco::R](R) reader structure"]
impl crate::Readable for CLKPLL_VCO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_vco::W](W) writer structure"]
impl crate::Writable for CLKPLL_VCO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clkpll_vco to value 0x07"]
impl crate::Resettable for CLKPLL_VCO_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}

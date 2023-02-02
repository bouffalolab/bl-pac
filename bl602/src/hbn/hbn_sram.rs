#[doc = "Register `HBN_SRAM` reader"]
pub struct R(crate::R<HBN_SRAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_SRAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_SRAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_SRAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_SRAM` writer"]
pub struct W(crate::W<HBN_SRAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_SRAM_SPEC>;
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
impl From<crate::W<HBN_SRAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_SRAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `retram_ret` reader - "]
pub type RETRAM_RET_R = crate::BitReader<bool>;
#[doc = "Field `retram_ret` writer - "]
pub type RETRAM_RET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_SRAM_SPEC, bool, O>;
#[doc = "Field `retram_slp` reader - "]
pub type RETRAM_SLP_R = crate::BitReader<bool>;
#[doc = "Field `retram_slp` writer - "]
pub type RETRAM_SLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_SRAM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn retram_ret(&self) -> RETRAM_RET_R {
        RETRAM_RET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn retram_slp(&self) -> RETRAM_SLP_R {
        RETRAM_SLP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn retram_ret(&mut self) -> RETRAM_RET_W<6> {
        RETRAM_RET_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn retram_slp(&mut self) -> RETRAM_SLP_W<7> {
        RETRAM_SLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_SRAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_sram](index.html) module"]
pub struct HBN_SRAM_SPEC;
impl crate::RegisterSpec for HBN_SRAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_sram::R](R) reader structure"]
impl crate::Readable for HBN_SRAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_sram::W](W) writer structure"]
impl crate::Writable for HBN_SRAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_SRAM to value 0"]
impl crate::Resettable for HBN_SRAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

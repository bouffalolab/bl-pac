#[doc = "Register `gpadc_reg_isr` reader"]
pub struct R(crate::R<GPADC_REG_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_isr` writer"]
pub struct W(crate::W<GPADC_REG_ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_ISR_SPEC>;
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
impl From<crate::W<GPADC_REG_ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_neg_satur` reader - "]
pub type GPADC_NEG_SATUR_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_pos_satur` reader - "]
pub type GPADC_POS_SATUR_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_neg_satur_clr` reader - "]
pub type GPADC_NEG_SATUR_CLR_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_neg_satur_clr` writer - "]
pub type GPADC_NEG_SATUR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_ISR_SPEC, bool, O>;
#[doc = "Field `gpadc_pos_satur_clr` reader - "]
pub type GPADC_POS_SATUR_CLR_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_pos_satur_clr` writer - "]
pub type GPADC_POS_SATUR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_ISR_SPEC, bool, O>;
#[doc = "Field `gpadc_neg_satur_mask` reader - "]
pub type GPADC_NEG_SATUR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_neg_satur_mask` writer - "]
pub type GPADC_NEG_SATUR_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_ISR_SPEC, bool, O>;
#[doc = "Field `gpadc_pos_satur_mask` reader - "]
pub type GPADC_POS_SATUR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_pos_satur_mask` writer - "]
pub type GPADC_POS_SATUR_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_ISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_neg_satur(&self) -> GPADC_NEG_SATUR_R {
        GPADC_NEG_SATUR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_pos_satur(&self) -> GPADC_POS_SATUR_R {
        GPADC_POS_SATUR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_neg_satur_clr(&self) -> GPADC_NEG_SATUR_CLR_R {
        GPADC_NEG_SATUR_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_pos_satur_clr(&self) -> GPADC_POS_SATUR_CLR_R {
        GPADC_POS_SATUR_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_neg_satur_mask(&self) -> GPADC_NEG_SATUR_MASK_R {
        GPADC_NEG_SATUR_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_pos_satur_mask(&self) -> GPADC_POS_SATUR_MASK_R {
        GPADC_POS_SATUR_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_neg_satur_clr(&mut self) -> GPADC_NEG_SATUR_CLR_W<4> {
        GPADC_NEG_SATUR_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pos_satur_clr(&mut self) -> GPADC_POS_SATUR_CLR_W<5> {
        GPADC_POS_SATUR_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_neg_satur_mask(&mut self) -> GPADC_NEG_SATUR_MASK_W<8> {
        GPADC_NEG_SATUR_MASK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pos_satur_mask(&mut self) -> GPADC_POS_SATUR_MASK_W<9> {
        GPADC_POS_SATUR_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_isr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_isr](index.html) module"]
pub struct GPADC_REG_ISR_SPEC;
impl crate::RegisterSpec for GPADC_REG_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_isr::R](R) reader structure"]
impl crate::Readable for GPADC_REG_ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_isr::W](W) writer structure"]
impl crate::Writable for GPADC_REG_ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_isr to value 0"]
impl crate::Resettable for GPADC_REG_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

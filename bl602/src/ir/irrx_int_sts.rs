#[doc = "Register `irrx_int_sts` reader"]
pub struct R(crate::R<IRRX_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRRX_INT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRRX_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irrx_int_sts` writer"]
pub struct W(crate::W<IRRX_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRRX_INT_STS_SPEC>;
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
impl From<crate::W<IRRX_INT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRRX_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irrx_end_int` reader - "]
pub type IRRX_END_INT_R = crate::BitReader<bool>;
#[doc = "Field `cr_irrx_end_mask` reader - "]
pub type CR_IRRX_END_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_irrx_end_mask` writer - "]
pub type CR_IRRX_END_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IRRX_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_irrx_end_clr` reader - "]
pub type CR_IRRX_END_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_irrx_end_clr` writer - "]
pub type CR_IRRX_END_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRRX_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_irrx_end_en` reader - "]
pub type CR_IRRX_END_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_irrx_end_en` writer - "]
pub type CR_IRRX_END_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRRX_INT_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irrx_end_int(&self) -> IRRX_END_INT_R {
        IRRX_END_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irrx_end_mask(&self) -> CR_IRRX_END_MASK_R {
        CR_IRRX_END_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_irrx_end_clr(&self) -> CR_IRRX_END_CLR_R {
        CR_IRRX_END_CLR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_irrx_end_en(&self) -> CR_IRRX_END_EN_R {
        CR_IRRX_END_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irrx_end_mask(&mut self) -> CR_IRRX_END_MASK_W<8> {
        CR_IRRX_END_MASK_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irrx_end_clr(&mut self) -> CR_IRRX_END_CLR_W<16> {
        CR_IRRX_END_CLR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irrx_end_en(&mut self) -> CR_IRRX_END_EN_W<24> {
        CR_IRRX_END_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irrx_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_int_sts](index.html) module"]
pub struct IRRX_INT_STS_SPEC;
impl crate::RegisterSpec for IRRX_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_int_sts::R](R) reader structure"]
impl crate::Readable for IRRX_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irrx_int_sts::W](W) writer structure"]
impl crate::Writable for IRRX_INT_STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irrx_int_sts to value 0x0100_0100"]
impl crate::Resettable for IRRX_INT_STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0100;
}

#[doc = "Register `TCCR` reader"]
pub struct R(crate::R<TCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR` writer"]
pub struct W(crate::W<TCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR_SPEC>;
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
impl From<crate::W<TCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cs_1` reader - "]
pub type CS_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cs_1` writer - "]
pub type CS_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED_4` reader - "]
pub type RESERVED_4_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED_4` writer - "]
pub type RESERVED_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCCR_SPEC, bool, O>;
#[doc = "Field `cs_2` reader - "]
pub type CS_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cs_2` writer - "]
pub type CS_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED_7` writer - "]
pub type RESERVED_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCCR_SPEC, bool, O>;
#[doc = "Field `cs_wdt` reader - "]
pub type CS_WDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cs_wdt` writer - "]
pub type CS_WDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cs_1(&self) -> CS_1_R {
        CS_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reserved_4(&self) -> RESERVED_4_R {
        RESERVED_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cs_2(&self) -> CS_2_R {
        CS_2_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs_wdt(&self) -> CS_WDT_R {
        CS_WDT_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn cs_1(&mut self) -> CS_1_W<2> {
        CS_1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_4(&mut self) -> RESERVED_4_W<4> {
        RESERVED_4_W::new(self)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn cs_2(&mut self) -> CS_2_W<5> {
        CS_2_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_7(&mut self) -> RESERVED_7_W<7> {
        RESERVED_7_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn cs_wdt(&mut self) -> CS_WDT_W<8> {
        CS_WDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr](index.html) module"]
pub struct TCCR_SPEC;
impl crate::RegisterSpec for TCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tccr::R](R) reader structure"]
impl crate::Readable for TCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr::W](W) writer structure"]
impl crate::Writable for TCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR to value 0"]
impl crate::Resettable for TCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

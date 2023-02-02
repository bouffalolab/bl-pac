#[doc = "Register `clkpll_output_en` reader"]
pub struct R(crate::R<CLKPLL_OUTPUT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_OUTPUT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPLL_OUTPUT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPLL_OUTPUT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_output_en` writer"]
pub struct W(crate::W<CLKPLL_OUTPUT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_OUTPUT_EN_SPEC>;
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
impl From<crate::W<CLKPLL_OUTPUT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPLL_OUTPUT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_en_480m` reader - "]
pub type CLKPLL_EN_480M_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_en_480m` writer - "]
pub type CLKPLL_EN_480M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_OUTPUT_EN_SPEC, bool, O>;
#[doc = "Field `clkpll_en_240m` reader - "]
pub type CLKPLL_EN_240M_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_en_240m` writer - "]
pub type CLKPLL_EN_240M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_OUTPUT_EN_SPEC, bool, O>;
#[doc = "Field `clkpll_en_192m` reader - "]
pub type CLKPLL_EN_192M_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_en_192m` writer - "]
pub type CLKPLL_EN_192M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_OUTPUT_EN_SPEC, bool, O>;
#[doc = "Field `clkpll_en_160m` reader - "]
pub type CLKPLL_EN_160M_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_en_160m` writer - "]
pub type CLKPLL_EN_160M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_OUTPUT_EN_SPEC, bool, O>;
#[doc = "Field `clkpll_en_120m` reader - "]
pub type CLKPLL_EN_120M_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_en_120m` writer - "]
pub type CLKPLL_EN_120M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_OUTPUT_EN_SPEC, bool, O>;
#[doc = "Field `clkpll_en_96m` reader - "]
pub type CLKPLL_EN_96M_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_en_96m` writer - "]
pub type CLKPLL_EN_96M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_OUTPUT_EN_SPEC, bool, O>;
#[doc = "Field `clkpll_en_80m` reader - "]
pub type CLKPLL_EN_80M_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_en_80m` writer - "]
pub type CLKPLL_EN_80M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_OUTPUT_EN_SPEC, bool, O>;
#[doc = "Field `clkpll_en_48m` reader - "]
pub type CLKPLL_EN_48M_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_en_48m` writer - "]
pub type CLKPLL_EN_48M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_OUTPUT_EN_SPEC, bool, O>;
#[doc = "Field `clkpll_en_32m` reader - "]
pub type CLKPLL_EN_32M_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_en_32m` writer - "]
pub type CLKPLL_EN_32M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_OUTPUT_EN_SPEC, bool, O>;
#[doc = "Field `clkpll_en_div2_480m` reader - "]
pub type CLKPLL_EN_DIV2_480M_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_en_div2_480m` writer - "]
pub type CLKPLL_EN_DIV2_480M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_OUTPUT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_en_480m(&self) -> CLKPLL_EN_480M_R {
        CLKPLL_EN_480M_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_en_240m(&self) -> CLKPLL_EN_240M_R {
        CLKPLL_EN_240M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_en_192m(&self) -> CLKPLL_EN_192M_R {
        CLKPLL_EN_192M_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_en_160m(&self) -> CLKPLL_EN_160M_R {
        CLKPLL_EN_160M_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_en_120m(&self) -> CLKPLL_EN_120M_R {
        CLKPLL_EN_120M_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_en_96m(&self) -> CLKPLL_EN_96M_R {
        CLKPLL_EN_96M_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_en_80m(&self) -> CLKPLL_EN_80M_R {
        CLKPLL_EN_80M_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_en_48m(&self) -> CLKPLL_EN_48M_R {
        CLKPLL_EN_48M_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_en_32m(&self) -> CLKPLL_EN_32M_R {
        CLKPLL_EN_32M_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_en_div2_480m(&self) -> CLKPLL_EN_DIV2_480M_R {
        CLKPLL_EN_DIV2_480M_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_en_480m(&mut self) -> CLKPLL_EN_480M_W<0> {
        CLKPLL_EN_480M_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_en_240m(&mut self) -> CLKPLL_EN_240M_W<1> {
        CLKPLL_EN_240M_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_en_192m(&mut self) -> CLKPLL_EN_192M_W<2> {
        CLKPLL_EN_192M_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_en_160m(&mut self) -> CLKPLL_EN_160M_W<3> {
        CLKPLL_EN_160M_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_en_120m(&mut self) -> CLKPLL_EN_120M_W<4> {
        CLKPLL_EN_120M_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_en_96m(&mut self) -> CLKPLL_EN_96M_W<5> {
        CLKPLL_EN_96M_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_en_80m(&mut self) -> CLKPLL_EN_80M_W<6> {
        CLKPLL_EN_80M_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_en_48m(&mut self) -> CLKPLL_EN_48M_W<7> {
        CLKPLL_EN_48M_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_en_32m(&mut self) -> CLKPLL_EN_32M_W<8> {
        CLKPLL_EN_32M_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_en_div2_480m(&mut self) -> CLKPLL_EN_DIV2_480M_W<9> {
        CLKPLL_EN_DIV2_480M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_output_en.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_output_en](index.html) module"]
pub struct CLKPLL_OUTPUT_EN_SPEC;
impl crate::RegisterSpec for CLKPLL_OUTPUT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_output_en::R](R) reader structure"]
impl crate::Readable for CLKPLL_OUTPUT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_output_en::W](W) writer structure"]
impl crate::Writable for CLKPLL_OUTPUT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clkpll_output_en to value 0x0100"]
impl crate::Resettable for CLKPLL_OUTPUT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}

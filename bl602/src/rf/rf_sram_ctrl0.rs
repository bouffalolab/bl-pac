#[doc = "Register `rf_sram_ctrl0` reader"]
pub struct R(crate::R<RF_SRAM_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_SRAM_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_SRAM_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_SRAM_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_sram_ctrl0` writer"]
pub struct W(crate::W<RF_SRAM_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_SRAM_CTRL0_SPEC>;
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
impl From<crate::W<RF_SRAM_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_SRAM_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_sram_link_dly` reader - "]
pub type RF_SRAM_LINK_DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_sram_link_dly` writer - "]
pub type RF_SRAM_LINK_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SRAM_CTRL0_SPEC, u16, u16, 16, O>;
#[doc = "Field `rf_sram_link_mode` reader - "]
pub type RF_SRAM_LINK_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_sram_link_mode` writer - "]
pub type RF_SRAM_LINK_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SRAM_CTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `rf_sram_swap` reader - "]
pub type RF_SRAM_SWAP_R = crate::BitReader<bool>;
#[doc = "Field `rf_sram_swap` writer - "]
pub type RF_SRAM_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_SRAM_CTRL0_SPEC, bool, O>;
#[doc = "Field `rf_sram_ext_clr` reader - "]
pub type RF_SRAM_EXT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `rf_sram_ext_clr` writer - "]
pub type RF_SRAM_EXT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_SRAM_CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_link_dly(&self) -> RF_SRAM_LINK_DLY_R {
        RF_SRAM_LINK_DLY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rf_sram_link_mode(&self) -> RF_SRAM_LINK_MODE_R {
        RF_SRAM_LINK_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_sram_swap(&self) -> RF_SRAM_SWAP_R {
        RF_SRAM_SWAP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_sram_ext_clr(&self) -> RF_SRAM_EXT_CLR_R {
        RF_SRAM_EXT_CLR_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn rf_sram_link_dly(&mut self) -> RF_SRAM_LINK_DLY_W<0> {
        RF_SRAM_LINK_DLY_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn rf_sram_link_mode(&mut self) -> RF_SRAM_LINK_MODE_W<16> {
        RF_SRAM_LINK_MODE_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn rf_sram_swap(&mut self) -> RF_SRAM_SWAP_W<18> {
        RF_SRAM_SWAP_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn rf_sram_ext_clr(&mut self) -> RF_SRAM_EXT_CLR_W<19> {
        RF_SRAM_EXT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_sram_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl0](index.html) module"]
pub struct RF_SRAM_CTRL0_SPEC;
impl crate::RegisterSpec for RF_SRAM_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_sram_ctrl0::R](R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl0::W](W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_sram_ctrl0 to value 0"]
impl crate::Resettable for RF_SRAM_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

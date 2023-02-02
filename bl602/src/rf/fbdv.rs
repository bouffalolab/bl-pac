#[doc = "Register `fbdv` reader"]
pub struct R(crate::R<FBDV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBDV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBDV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBDV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fbdv` writer"]
pub struct W(crate::W<FBDV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBDV_SPEC>;
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
impl From<crate::W<FBDV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBDV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_fbdv_halfstep_en_hw` reader - "]
pub type LO_FBDV_HALFSTEP_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `lo_fbdv_halfstep_en_hw` writer - "]
pub type LO_FBDV_HALFSTEP_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
#[doc = "Field `lo_fbdv_halfstep_en` reader - "]
pub type LO_FBDV_HALFSTEP_EN_R = crate::BitReader<bool>;
#[doc = "Field `lo_fbdv_halfstep_en` writer - "]
pub type LO_FBDV_HALFSTEP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
#[doc = "Field `lo_fbdv_sel_sample_clk` reader - "]
pub type LO_FBDV_SEL_SAMPLE_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_fbdv_sel_sample_clk` writer - "]
pub type LO_FBDV_SEL_SAMPLE_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FBDV_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_fbdv_sel_fb_clk` reader - "]
pub type LO_FBDV_SEL_FB_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_fbdv_sel_fb_clk` writer - "]
pub type LO_FBDV_SEL_FB_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FBDV_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_fbdv_rst` reader - "]
pub type LO_FBDV_RST_R = crate::BitReader<bool>;
#[doc = "Field `lo_fbdv_rst` writer - "]
pub type LO_FBDV_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
#[doc = "Field `lo_fbdv_rst_hw` reader - "]
pub type LO_FBDV_RST_HW_R = crate::BitReader<bool>;
#[doc = "Field `lo_fbdv_rst_hw` writer - "]
pub type LO_FBDV_RST_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_hw(&self) -> LO_FBDV_HALFSTEP_EN_HW_R {
        LO_FBDV_HALFSTEP_EN_HW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en(&self) -> LO_FBDV_HALFSTEP_EN_R {
        LO_FBDV_HALFSTEP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_fbdv_sel_sample_clk(&self) -> LO_FBDV_SEL_SAMPLE_CLK_R {
        LO_FBDV_SEL_SAMPLE_CLK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_fbdv_sel_fb_clk(&self) -> LO_FBDV_SEL_FB_CLK_R {
        LO_FBDV_SEL_FB_CLK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_fbdv_rst(&self) -> LO_FBDV_RST_R {
        LO_FBDV_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_fbdv_rst_hw(&self) -> LO_FBDV_RST_HW_R {
        LO_FBDV_RST_HW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lo_fbdv_halfstep_en_hw(&mut self) -> LO_FBDV_HALFSTEP_EN_HW_W<0> {
        LO_FBDV_HALFSTEP_EN_HW_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn lo_fbdv_halfstep_en(&mut self) -> LO_FBDV_HALFSTEP_EN_W<4> {
        LO_FBDV_HALFSTEP_EN_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn lo_fbdv_sel_sample_clk(&mut self) -> LO_FBDV_SEL_SAMPLE_CLK_W<8> {
        LO_FBDV_SEL_SAMPLE_CLK_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn lo_fbdv_sel_fb_clk(&mut self) -> LO_FBDV_SEL_FB_CLK_W<12> {
        LO_FBDV_SEL_FB_CLK_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn lo_fbdv_rst(&mut self) -> LO_FBDV_RST_W<16> {
        LO_FBDV_RST_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn lo_fbdv_rst_hw(&mut self) -> LO_FBDV_RST_HW_W<20> {
        LO_FBDV_RST_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "fbdv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbdv](index.html) module"]
pub struct FBDV_SPEC;
impl crate::RegisterSpec for FBDV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbdv::R](R) reader structure"]
impl crate::Readable for FBDV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbdv::W](W) writer structure"]
impl crate::Writable for FBDV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fbdv to value 0"]
impl crate::Resettable for FBDV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

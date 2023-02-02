#[doc = "Register `acomp0_ctrl` reader"]
pub struct R(crate::R<ACOMP0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACOMP0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACOMP0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACOMP0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `acomp0_ctrl` writer"]
pub struct W(crate::W<ACOMP0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACOMP0_CTRL_SPEC>;
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
impl From<crate::W<ACOMP0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACOMP0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `acomp0_en` reader - "]
pub type ACOMP0_EN_R = crate::BitReader<bool>;
#[doc = "Field `acomp0_en` writer - "]
pub type ACOMP0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACOMP0_CTRL_SPEC, bool, O>;
#[doc = "Field `acomp0_hyst_seln` reader - "]
pub type ACOMP0_HYST_SELN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acomp0_hyst_seln` writer - "]
pub type ACOMP0_HYST_SELN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACOMP0_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `acomp0_hyst_selp` reader - "]
pub type ACOMP0_HYST_SELP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acomp0_hyst_selp` writer - "]
pub type ACOMP0_HYST_SELP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACOMP0_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `acomp0_bias_prog` reader - "]
pub type ACOMP0_BIAS_PROG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acomp0_bias_prog` writer - "]
pub type ACOMP0_BIAS_PROG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACOMP0_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `acomp0_level_sel` reader - "]
pub type ACOMP0_LEVEL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acomp0_level_sel` writer - "]
pub type ACOMP0_LEVEL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACOMP0_CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `acomp0_neg_sel` reader - "]
pub type ACOMP0_NEG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acomp0_neg_sel` writer - "]
pub type ACOMP0_NEG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACOMP0_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `acomp0_pos_sel` reader - "]
pub type ACOMP0_POS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acomp0_pos_sel` writer - "]
pub type ACOMP0_POS_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACOMP0_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `acomp0_muxen` reader - "]
pub type ACOMP0_MUXEN_R = crate::BitReader<bool>;
#[doc = "Field `acomp0_muxen` writer - "]
pub type ACOMP0_MUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACOMP0_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp0_en(&self) -> ACOMP0_EN_R {
        ACOMP0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn acomp0_hyst_seln(&self) -> ACOMP0_HYST_SELN_R {
        ACOMP0_HYST_SELN_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn acomp0_hyst_selp(&self) -> ACOMP0_HYST_SELP_R {
        ACOMP0_HYST_SELP_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp0_bias_prog(&self) -> ACOMP0_BIAS_PROG_R {
        ACOMP0_BIAS_PROG_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn acomp0_level_sel(&self) -> ACOMP0_LEVEL_SEL_R {
        ACOMP0_LEVEL_SEL_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn acomp0_neg_sel(&self) -> ACOMP0_NEG_SEL_R {
        ACOMP0_NEG_SEL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn acomp0_pos_sel(&self) -> ACOMP0_POS_SEL_R {
        ACOMP0_POS_SEL_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn acomp0_muxen(&self) -> ACOMP0_MUXEN_R {
        ACOMP0_MUXEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_en(&mut self) -> ACOMP0_EN_W<0> {
        ACOMP0_EN_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_hyst_seln(&mut self) -> ACOMP0_HYST_SELN_W<4> {
        ACOMP0_HYST_SELN_W::new(self)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_hyst_selp(&mut self) -> ACOMP0_HYST_SELP_W<7> {
        ACOMP0_HYST_SELP_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_bias_prog(&mut self) -> ACOMP0_BIAS_PROG_W<10> {
        ACOMP0_BIAS_PROG_W::new(self)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_level_sel(&mut self) -> ACOMP0_LEVEL_SEL_W<12> {
        ACOMP0_LEVEL_SEL_W::new(self)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_neg_sel(&mut self) -> ACOMP0_NEG_SEL_W<18> {
        ACOMP0_NEG_SEL_W::new(self)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_pos_sel(&mut self) -> ACOMP0_POS_SEL_W<22> {
        ACOMP0_POS_SEL_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_muxen(&mut self) -> ACOMP0_MUXEN_W<26> {
        ACOMP0_MUXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "acomp0_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acomp0_ctrl](index.html) module"]
pub struct ACOMP0_CTRL_SPEC;
impl crate::RegisterSpec for ACOMP0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acomp0_ctrl::R](R) reader structure"]
impl crate::Readable for ACOMP0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acomp0_ctrl::W](W) writer structure"]
impl crate::Writable for ACOMP0_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets acomp0_ctrl to value 0"]
impl crate::Resettable for ACOMP0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

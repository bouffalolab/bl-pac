#[doc = "Register `sf_ctrl_2` reader"]
pub struct R(crate::R<SF_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_2` writer"]
pub struct W(crate::W<SF_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_2_SPEC>;
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
impl From<crate::W<SF_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_if_pad_sel` reader - "]
pub type SF_IF_PAD_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if_pad_sel` writer - "]
pub type SF_IF_PAD_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_CTRL_2_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf_if_pad_sel_lock` reader - "]
pub type SF_IF_PAD_SEL_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_pad_sel_lock` writer - "]
pub type SF_IF_PAD_SEL_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_2_SPEC, bool, O>;
#[doc = "Field `sf_if_dtr_en` reader - "]
pub type SF_IF_DTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_dtr_en` writer - "]
pub type SF_IF_DTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_2_SPEC, bool, O>;
#[doc = "Field `sf_if_dqs_en` reader - "]
pub type SF_IF_DQS_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_dqs_en` writer - "]
pub type SF_IF_DQS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_if_pad_sel(&self) -> SF_IF_PAD_SEL_R {
        SF_IF_PAD_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_if_pad_sel_lock(&self) -> SF_IF_PAD_SEL_LOCK_R {
        SF_IF_PAD_SEL_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_if_dtr_en(&self) -> SF_IF_DTR_EN_R {
        SF_IF_DTR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_if_dqs_en(&self) -> SF_IF_DQS_EN_R {
        SF_IF_DQS_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_pad_sel(&mut self) -> SF_IF_PAD_SEL_W<0> {
        SF_IF_PAD_SEL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_pad_sel_lock(&mut self) -> SF_IF_PAD_SEL_LOCK_W<3> {
        SF_IF_PAD_SEL_LOCK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_dtr_en(&mut self) -> SF_IF_DTR_EN_W<4> {
        SF_IF_DTR_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_dqs_en(&mut self) -> SF_IF_DQS_EN_W<5> {
        SF_IF_DQS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_2](index.html) module"]
pub struct SF_CTRL_2_SPEC;
impl crate::RegisterSpec for SF_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_2::R](R) reader structure"]
impl crate::Readable for SF_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_2::W](W) writer structure"]
impl crate::Writable for SF_CTRL_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_ctrl_2 to value 0"]
impl crate::Resettable for SF_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `sf_ctrl_prot_en` reader"]
pub struct R(crate::R<SF_CTRL_PROT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_PROT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CTRL_PROT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CTRL_PROT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_prot_en` writer"]
pub struct W(crate::W<SF_CTRL_PROT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_PROT_EN_SPEC>;
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
impl From<crate::W<SF_CTRL_PROT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_CTRL_PROT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_ctrl_prot_en` reader - "]
pub type SF_CTRL_PROT_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_ctrl_prot_en` writer - "]
pub type SF_CTRL_PROT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_CTRL_PROT_EN_SPEC, bool, O>;
#[doc = "Field `sf_ctrl_id0_en` reader - "]
pub type SF_CTRL_ID0_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_ctrl_id0_en` writer - "]
pub type SF_CTRL_ID0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_CTRL_PROT_EN_SPEC, bool, O>;
#[doc = "Field `sf_ctrl_id1_en` reader - "]
pub type SF_CTRL_ID1_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_ctrl_id1_en` writer - "]
pub type SF_CTRL_ID1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_CTRL_PROT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en(&self) -> SF_CTRL_PROT_EN_R {
        SF_CTRL_PROT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en(&self) -> SF_CTRL_ID0_EN_R {
        SF_CTRL_ID0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en(&self) -> SF_CTRL_ID1_EN_R {
        SF_CTRL_ID1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sf_ctrl_prot_en(&mut self) -> SF_CTRL_PROT_EN_W<0> {
        SF_CTRL_PROT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sf_ctrl_id0_en(&mut self) -> SF_CTRL_ID0_EN_W<1> {
        SF_CTRL_ID0_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sf_ctrl_id1_en(&mut self) -> SF_CTRL_ID1_EN_W<2> {
        SF_CTRL_ID1_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_prot_en.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_prot_en](index.html) module"]
pub struct SF_CTRL_PROT_EN_SPEC;
impl crate::RegisterSpec for SF_CTRL_PROT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_prot_en::R](R) reader structure"]
impl crate::Readable for SF_CTRL_PROT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_prot_en::W](W) writer structure"]
impl crate::Writable for SF_CTRL_PROT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_ctrl_prot_en to value 0x07"]
impl crate::Resettable for SF_CTRL_PROT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}

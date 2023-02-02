#[doc = "Register `pud_ctrl_hw` reader"]
pub struct R(crate::R<PUD_CTRL_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUD_CTRL_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUD_CTRL_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUD_CTRL_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pud_ctrl_hw` writer"]
pub struct W(crate::W<PUD_CTRL_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUD_CTRL_HW_SPEC>;
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
impl From<crate::W<PUD_CTRL_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUD_CTRL_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pud_vco_hw` reader - "]
pub type PUD_VCO_HW_R = crate::BitReader<bool>;
#[doc = "Field `pud_vco_hw` writer - "]
pub type PUD_VCO_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUD_CTRL_HW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pud_vco_hw(&self) -> PUD_VCO_HW_R {
        PUD_VCO_HW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pud_vco_hw(&mut self) -> PUD_VCO_HW_W<20> {
        PUD_VCO_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pud_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pud_ctrl_hw](index.html) module"]
pub struct PUD_CTRL_HW_SPEC;
impl crate::RegisterSpec for PUD_CTRL_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pud_ctrl_hw::R](R) reader structure"]
impl crate::Readable for PUD_CTRL_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pud_ctrl_hw::W](W) writer structure"]
impl crate::Writable for PUD_CTRL_HW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pud_ctrl_hw to value 0"]
impl crate::Resettable for PUD_CTRL_HW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

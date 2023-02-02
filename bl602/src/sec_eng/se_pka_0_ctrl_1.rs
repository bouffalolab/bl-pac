#[doc = "Register `se_pka_0_ctrl_1` reader"]
pub struct R(crate::R<SE_PKA_0_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_PKA_0_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_PKA_0_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_PKA_0_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_pka_0_ctrl_1` writer"]
pub struct W(crate::W<SE_PKA_0_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_PKA_0_CTRL_1_SPEC>;
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
impl From<crate::W<SE_PKA_0_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_PKA_0_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_pka_0_hburst` reader - "]
pub type SE_PKA_0_HBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_pka_0_hburst` writer - "]
pub type SE_PKA_0_HBURST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_PKA_0_CTRL_1_SPEC, u8, u8, 3, O>;
#[doc = "Field `se_pka_0_hbypass` reader - "]
pub type SE_PKA_0_HBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_hbypass` writer - "]
pub type SE_PKA_0_HBYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_PKA_0_CTRL_1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn se_pka_0_hburst(&self) -> SE_PKA_0_HBURST_R {
        SE_PKA_0_HBURST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_hbypass(&self) -> SE_PKA_0_HBYPASS_R {
        SE_PKA_0_HBYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_hburst(&mut self) -> SE_PKA_0_HBURST_W<0> {
        SE_PKA_0_HBURST_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_hbypass(&mut self) -> SE_PKA_0_HBYPASS_W<3> {
        SE_PKA_0_HBYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_pka_0_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_ctrl_1](index.html) module"]
pub struct SE_PKA_0_CTRL_1_SPEC;
impl crate::RegisterSpec for SE_PKA_0_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_pka_0_ctrl_1::R](R) reader structure"]
impl crate::Readable for SE_PKA_0_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_pka_0_ctrl_1::W](W) writer structure"]
impl crate::Writable for SE_PKA_0_CTRL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_pka_0_ctrl_1 to value 0x05"]
impl crate::Resettable for SE_PKA_0_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}

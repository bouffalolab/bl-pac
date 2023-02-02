#[doc = "Register `tzc_glb_ctrl_3` reader"]
pub struct R(crate::R<TZC_GLB_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GLB_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_GLB_CTRL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_GLB_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_glb_ctrl_3` writer"]
pub struct W(crate::W<TZC_GLB_CTRL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_GLB_CTRL_3_SPEC>;
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
impl From<crate::W<TZC_GLB_CTRL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_GLB_CTRL_3_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reserved according to SDK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_3](index.html) module"]
pub struct TZC_GLB_CTRL_3_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_3::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_glb_ctrl_3::W](W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_3 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

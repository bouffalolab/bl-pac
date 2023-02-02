#[doc = "Register `vco3` reader"]
pub struct R(crate::R<VCO3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCO3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCO3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCO3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vco3` writer"]
pub struct W(crate::W<VCO3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCO3_SPEC>;
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
impl From<crate::W<VCO3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCO3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fcal_div` reader - "]
pub type FCAL_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `fcal_div` writer - "]
pub type FCAL_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCO3_SPEC, u16, u16, 16, O>;
#[doc = "Field `fcal_cnt_op` reader - "]
pub type FCAL_CNT_OP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `fcal_cnt_op` writer - "]
pub type FCAL_CNT_OP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCO3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn fcal_div(&self) -> FCAL_DIV_R {
        FCAL_DIV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn fcal_cnt_op(&self) -> FCAL_CNT_OP_R {
        FCAL_CNT_OP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_div(&mut self) -> FCAL_DIV_W<0> {
        FCAL_DIV_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_cnt_op(&mut self) -> FCAL_CNT_OP_W<16> {
        FCAL_CNT_OP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "vco3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco3](index.html) module"]
pub struct VCO3_SPEC;
impl crate::RegisterSpec for VCO3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vco3::R](R) reader structure"]
impl crate::Readable for VCO3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vco3::W](W) writer structure"]
impl crate::Writable for VCO3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets vco3 to value 0"]
impl crate::Resettable for VCO3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

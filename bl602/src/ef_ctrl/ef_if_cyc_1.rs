#[doc = "Register `ef_if_cyc_1` reader"]
pub struct R(crate::R<EF_IF_CYC_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_CYC_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_IF_CYC_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_IF_CYC_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_cyc_1` writer"]
pub struct W(crate::W<EF_IF_CYC_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_CYC_1_SPEC>;
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
impl From<crate::W<EF_IF_CYC_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_IF_CYC_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_if_cyc_pi` reader - "]
pub type EF_IF_CYC_PI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cyc_pi` writer - "]
pub type EF_IF_CYC_PI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CYC_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `ef_if_cyc_pp` reader - "]
pub type EF_IF_CYC_PP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cyc_pp` writer - "]
pub type EF_IF_CYC_PP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CYC_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `ef_if_cyc_wr_adr` reader - "]
pub type EF_IF_CYC_WR_ADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cyc_wr_adr` writer - "]
pub type EF_IF_CYC_WR_ADR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CYC_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `ef_if_cyc_ps_cs` reader - "]
pub type EF_IF_CYC_PS_CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cyc_ps_cs` writer - "]
pub type EF_IF_CYC_PS_CS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CYC_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `ef_if_cyc_pd_cs_h` reader - "]
pub type EF_IF_CYC_PD_CS_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cyc_pd_cs_h` writer - "]
pub type EF_IF_CYC_PD_CS_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CYC_1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_pi(&self) -> EF_IF_CYC_PI_R {
        EF_IF_CYC_PI_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn ef_if_cyc_pp(&self) -> EF_IF_CYC_PP_R {
        EF_IF_CYC_PP_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn ef_if_cyc_wr_adr(&self) -> EF_IF_CYC_WR_ADR_R {
        EF_IF_CYC_WR_ADR_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25"]
    #[inline(always)]
    pub fn ef_if_cyc_ps_cs(&self) -> EF_IF_CYC_PS_CS_R {
        EF_IF_CYC_PS_CS_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_h(&self) -> EF_IF_CYC_PD_CS_H_R {
        EF_IF_CYC_PD_CS_H_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_pi(&mut self) -> EF_IF_CYC_PI_W<0> {
        EF_IF_CYC_PI_W::new(self)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_pp(&mut self) -> EF_IF_CYC_PP_W<6> {
        EF_IF_CYC_PP_W::new(self)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_wr_adr(&mut self) -> EF_IF_CYC_WR_ADR_W<14> {
        EF_IF_CYC_WR_ADR_W::new(self)
    }
    #[doc = "Bits 20:25"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_ps_cs(&mut self) -> EF_IF_CYC_PS_CS_W<20> {
        EF_IF_CYC_PS_CS_W::new(self)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_pd_cs_h(&mut self) -> EF_IF_CYC_PD_CS_H_W<26> {
        EF_IF_CYC_PD_CS_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_cyc_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_cyc_1](index.html) module"]
pub struct EF_IF_CYC_1_SPEC;
impl crate::RegisterSpec for EF_IF_CYC_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_cyc_1::R](R) reader structure"]
impl crate::Readable for EF_IF_CYC_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_cyc_1::W](W) writer structure"]
impl crate::Writable for EF_IF_CYC_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_if_cyc_1 to value 0x0020_6609"]
impl crate::Resettable for EF_IF_CYC_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_6609;
}

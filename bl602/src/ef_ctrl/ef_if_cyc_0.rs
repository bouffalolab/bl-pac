#[doc = "Register `ef_if_cyc_0` reader"]
pub struct R(crate::R<EF_IF_CYC_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_CYC_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_IF_CYC_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_IF_CYC_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_cyc_0` writer"]
pub struct W(crate::W<EF_IF_CYC_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_CYC_0_SPEC>;
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
impl From<crate::W<EF_IF_CYC_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_IF_CYC_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_if_cyc_rd_dmy` reader - "]
pub type EF_IF_CYC_RD_DMY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cyc_rd_dmy` writer - "]
pub type EF_IF_CYC_RD_DMY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CYC_0_SPEC, u8, u8, 6, O>;
#[doc = "Field `ef_if_cyc_rd_dat` reader - "]
pub type EF_IF_CYC_RD_DAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cyc_rd_dat` writer - "]
pub type EF_IF_CYC_RD_DAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CYC_0_SPEC, u8, u8, 6, O>;
#[doc = "Field `ef_if_cyc_rd_adr` reader - "]
pub type EF_IF_CYC_RD_ADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cyc_rd_adr` writer - "]
pub type EF_IF_CYC_RD_ADR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CYC_0_SPEC, u8, u8, 6, O>;
#[doc = "Field `ef_if_cyc_cs` reader - "]
pub type EF_IF_CYC_CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cyc_cs` writer - "]
pub type EF_IF_CYC_CS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CYC_0_SPEC, u8, u8, 6, O>;
#[doc = "Field `ef_if_cyc_pd_cs_s` reader - "]
pub type EF_IF_CYC_PD_CS_S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cyc_pd_cs_s` writer - "]
pub type EF_IF_CYC_PD_CS_S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CYC_0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dmy(&self) -> EF_IF_CYC_RD_DMY_R {
        EF_IF_CYC_RD_DMY_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dat(&self) -> EF_IF_CYC_RD_DAT_R {
        EF_IF_CYC_RD_DAT_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_adr(&self) -> EF_IF_CYC_RD_ADR_R {
        EF_IF_CYC_RD_ADR_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn ef_if_cyc_cs(&self) -> EF_IF_CYC_CS_R {
        EF_IF_CYC_CS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_s(&self) -> EF_IF_CYC_PD_CS_S_R {
        EF_IF_CYC_PD_CS_S_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_rd_dmy(&mut self) -> EF_IF_CYC_RD_DMY_W<0> {
        EF_IF_CYC_RD_DMY_W::new(self)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_rd_dat(&mut self) -> EF_IF_CYC_RD_DAT_W<6> {
        EF_IF_CYC_RD_DAT_W::new(self)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_rd_adr(&mut self) -> EF_IF_CYC_RD_ADR_W<12> {
        EF_IF_CYC_RD_ADR_W::new(self)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_cs(&mut self) -> EF_IF_CYC_CS_W<18> {
        EF_IF_CYC_CS_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_pd_cs_s(&mut self) -> EF_IF_CYC_PD_CS_S_W<24> {
        EF_IF_CYC_PD_CS_S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_cyc_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_cyc_0](index.html) module"]
pub struct EF_IF_CYC_0_SPEC;
impl crate::RegisterSpec for EF_IF_CYC_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_cyc_0::R](R) reader structure"]
impl crate::Readable for EF_IF_CYC_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_cyc_0::W](W) writer structure"]
impl crate::Writable for EF_IF_CYC_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_if_cyc_0 to value 0x1600_0040"]
impl crate::Resettable for EF_IF_CYC_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1600_0040;
}

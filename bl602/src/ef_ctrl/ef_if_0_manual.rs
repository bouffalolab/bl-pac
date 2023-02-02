#[doc = "Register `ef_if_0_manual` reader"]
pub struct R(crate::R<EF_IF_0_MANUAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_0_MANUAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_IF_0_MANUAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_IF_0_MANUAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_0_manual` writer"]
pub struct W(crate::W<EF_IF_0_MANUAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_0_MANUAL_SPEC>;
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
impl From<crate::W<EF_IF_0_MANUAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_IF_0_MANUAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_if_a` reader - "]
pub type EF_IF_A_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ef_if_a` writer - "]
pub type EF_IF_A_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_0_MANUAL_SPEC, u16, u16, 10, O>;
#[doc = "Field `ef_if_pd` reader - "]
pub type EF_IF_PD_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_pd` writer - "]
pub type EF_IF_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_0_MANUAL_SPEC, bool, O>;
#[doc = "Field `ef_if_ps` reader - "]
pub type EF_IF_PS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_ps` writer - "]
pub type EF_IF_PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_0_MANUAL_SPEC, bool, O>;
#[doc = "Field `ef_if_strobe` reader - "]
pub type EF_IF_STROBE_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_strobe` writer - "]
pub type EF_IF_STROBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_0_MANUAL_SPEC, bool, O>;
#[doc = "Field `ef_if_pgenb` reader - "]
pub type EF_IF_PGENB_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_pgenb` writer - "]
pub type EF_IF_PGENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_0_MANUAL_SPEC, bool, O>;
#[doc = "Field `ef_if_load` reader - "]
pub type EF_IF_LOAD_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_load` writer - "]
pub type EF_IF_LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_0_MANUAL_SPEC, bool, O>;
#[doc = "Field `ef_if_csb` reader - "]
pub type EF_IF_CSB_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_csb` writer - "]
pub type EF_IF_CSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_0_MANUAL_SPEC, bool, O>;
#[doc = "Field `ef_if_0_q` reader - "]
pub type EF_IF_0_Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_prot_code_manual` reader - "]
pub type EF_IF_PROT_CODE_MANUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_prot_code_manual` writer - "]
pub type EF_IF_PROT_CODE_MANUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_0_MANUAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn ef_if_a(&self) -> EF_IF_A_R {
        EF_IF_A_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_if_pd(&self) -> EF_IF_PD_R {
        EF_IF_PD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_if_ps(&self) -> EF_IF_PS_R {
        EF_IF_PS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ef_if_strobe(&self) -> EF_IF_STROBE_R {
        EF_IF_STROBE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ef_if_pgenb(&self) -> EF_IF_PGENB_R {
        EF_IF_PGENB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_load(&self) -> EF_IF_LOAD_R {
        EF_IF_LOAD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_csb(&self) -> EF_IF_CSB_R {
        EF_IF_CSB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ef_if_0_q(&self) -> EF_IF_0_Q_R {
        EF_IF_0_Q_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_manual(&self) -> EF_IF_PROT_CODE_MANUAL_R {
        EF_IF_PROT_CODE_MANUAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_a(&mut self) -> EF_IF_A_W<0> {
        EF_IF_A_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_pd(&mut self) -> EF_IF_PD_W<10> {
        EF_IF_PD_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_ps(&mut self) -> EF_IF_PS_W<11> {
        EF_IF_PS_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_strobe(&mut self) -> EF_IF_STROBE_W<12> {
        EF_IF_STROBE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_pgenb(&mut self) -> EF_IF_PGENB_W<13> {
        EF_IF_PGENB_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_load(&mut self) -> EF_IF_LOAD_W<14> {
        EF_IF_LOAD_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_csb(&mut self) -> EF_IF_CSB_W<15> {
        EF_IF_CSB_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_prot_code_manual(&mut self) -> EF_IF_PROT_CODE_MANUAL_W<24> {
        EF_IF_PROT_CODE_MANUAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_0_manual.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_0_manual](index.html) module"]
pub struct EF_IF_0_MANUAL_SPEC;
impl crate::RegisterSpec for EF_IF_0_MANUAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_0_manual::R](R) reader structure"]
impl crate::Readable for EF_IF_0_MANUAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_0_manual::W](W) writer structure"]
impl crate::Writable for EF_IF_0_MANUAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_if_0_manual to value 0xe400"]
impl crate::Resettable for EF_IF_0_MANUAL_SPEC {
    const RESET_VALUE: Self::Ux = 0xe400;
}

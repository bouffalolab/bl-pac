#[doc = "Register `pds_ram1` reader"]
pub struct R(crate::R<PDS_RAM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_RAM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_RAM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_RAM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_ram1` writer"]
pub struct W(crate::W<PDS_RAM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_RAM1_SPEC>;
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
impl From<crate::W<PDS_RAM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_RAM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_np_sram_pwr` reader - "]
pub type CR_NP_SRAM_PWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_np_sram_pwr` writer - "]
pub type CR_NP_SRAM_PWR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_RAM1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_np_sram_pwr(&self) -> CR_NP_SRAM_PWR_R {
        CR_NP_SRAM_PWR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_np_sram_pwr(&mut self) -> CR_NP_SRAM_PWR_W<0> {
        CR_NP_SRAM_PWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_ram1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ram1](index.html) module"]
pub struct PDS_RAM1_SPEC;
impl crate::RegisterSpec for PDS_RAM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ram1::R](R) reader structure"]
impl crate::Readable for PDS_RAM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ram1::W](W) writer structure"]
impl crate::Writable for PDS_RAM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_ram1 to value 0"]
impl crate::Resettable for PDS_RAM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `irrx_pw_config` reader"]
pub struct R(crate::R<IRRX_PW_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_PW_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRRX_PW_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRRX_PW_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irrx_pw_config` writer"]
pub struct W(crate::W<IRRX_PW_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRRX_PW_CONFIG_SPEC>;
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
impl From<crate::W<IRRX_PW_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRRX_PW_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irrx_data_th` reader - "]
pub type CR_IRRX_DATA_TH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_irrx_data_th` writer - "]
pub type CR_IRRX_DATA_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRRX_PW_CONFIG_SPEC, u16, u16, 16, O>;
#[doc = "Field `cr_irrx_end_th` reader - "]
pub type CR_IRRX_END_TH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_irrx_end_th` writer - "]
pub type CR_IRRX_END_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRRX_PW_CONFIG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_irrx_data_th(&self) -> CR_IRRX_DATA_TH_R {
        CR_IRRX_DATA_TH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_irrx_end_th(&self) -> CR_IRRX_END_TH_R {
        CR_IRRX_END_TH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irrx_data_th(&mut self) -> CR_IRRX_DATA_TH_W<0> {
        CR_IRRX_DATA_TH_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irrx_end_th(&mut self) -> CR_IRRX_END_TH_W<16> {
        CR_IRRX_END_TH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irrx_pw_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_pw_config](index.html) module"]
pub struct IRRX_PW_CONFIG_SPEC;
impl crate::RegisterSpec for IRRX_PW_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_pw_config::R](R) reader structure"]
impl crate::Readable for IRRX_PW_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irrx_pw_config::W](W) writer structure"]
impl crate::Writable for IRRX_PW_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irrx_pw_config to value 0x2327_0d47"]
impl crate::Resettable for IRRX_PW_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x2327_0d47;
}

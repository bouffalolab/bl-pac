#[doc = "Register `irtx_pulse_width` reader"]
pub struct R(crate::R<IRTX_PULSE_WIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRTX_PULSE_WIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRTX_PULSE_WIDTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRTX_PULSE_WIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irtx_pulse_width` writer"]
pub struct W(crate::W<IRTX_PULSE_WIDTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRTX_PULSE_WIDTH_SPEC>;
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
impl From<crate::W<IRTX_PULSE_WIDTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRTX_PULSE_WIDTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irtx_pw_unit` reader - "]
pub type CR_IRTX_PW_UNIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_irtx_pw_unit` writer - "]
pub type CR_IRTX_PW_UNIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PULSE_WIDTH_SPEC, u16, u16, 12, O>;
#[doc = "Field `cr_irtx_mod_ph0_w` reader - "]
pub type CR_IRTX_MOD_PH0_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_mod_ph0_w` writer - "]
pub type CR_IRTX_MOD_PH0_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PULSE_WIDTH_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_irtx_mod_ph1_w` reader - "]
pub type CR_IRTX_MOD_PH1_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_mod_ph1_w` writer - "]
pub type CR_IRTX_MOD_PH1_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PULSE_WIDTH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_irtx_pw_unit(&self) -> CR_IRTX_PW_UNIT_R {
        CR_IRTX_PW_UNIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph0_w(&self) -> CR_IRTX_MOD_PH0_W_R {
        CR_IRTX_MOD_PH0_W_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph1_w(&self) -> CR_IRTX_MOD_PH1_W_R {
        CR_IRTX_MOD_PH1_W_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_pw_unit(&mut self) -> CR_IRTX_PW_UNIT_W<0> {
        CR_IRTX_PW_UNIT_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_mod_ph0_w(&mut self) -> CR_IRTX_MOD_PH0_W_W<16> {
        CR_IRTX_MOD_PH0_W_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_mod_ph1_w(&mut self) -> CR_IRTX_MOD_PH1_W_W<24> {
        CR_IRTX_MOD_PH1_W_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irtx_pulse_width.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_pulse_width](index.html) module"]
pub struct IRTX_PULSE_WIDTH_SPEC;
impl crate::RegisterSpec for IRTX_PULSE_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irtx_pulse_width::R](R) reader structure"]
impl crate::Readable for IRTX_PULSE_WIDTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irtx_pulse_width::W](W) writer structure"]
impl crate::Writable for IRTX_PULSE_WIDTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irtx_pulse_width to value 0x2211_0464"]
impl crate::Resettable for IRTX_PULSE_WIDTH_SPEC {
    const RESET_VALUE: Self::Ux = 0x2211_0464;
}

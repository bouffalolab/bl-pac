#[doc = "Register `singen_ctrl3` reader"]
pub struct R(crate::R<SINGEN_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGEN_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGEN_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGEN_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `singen_ctrl3` writer"]
pub struct W(crate::W<SINGEN_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGEN_CTRL3_SPEC>;
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
impl From<crate::W<SINGEN_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGEN_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `singen_gain_q` reader - "]
pub type SINGEN_GAIN_Q_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_gain_q` writer - "]
pub type SINGEN_GAIN_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGEN_CTRL3_SPEC, u16, u16, 11, O>;
#[doc = "Field `singen_start_addr1_q` reader - "]
pub type SINGEN_START_ADDR1_Q_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_start_addr1_q` writer - "]
pub type SINGEN_START_ADDR1_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGEN_CTRL3_SPEC, u16, u16, 10, O>;
#[doc = "Field `singen_start_addr0_q` reader - "]
pub type SINGEN_START_ADDR0_Q_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_start_addr0_q` writer - "]
pub type SINGEN_START_ADDR0_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGEN_CTRL3_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_q(&self) -> SINGEN_GAIN_Q_R {
        SINGEN_GAIN_Q_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_q(&self) -> SINGEN_START_ADDR1_Q_R {
        SINGEN_START_ADDR1_Q_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_q(&self) -> SINGEN_START_ADDR0_Q_R {
        SINGEN_START_ADDR0_Q_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn singen_gain_q(&mut self) -> SINGEN_GAIN_Q_W<0> {
        SINGEN_GAIN_Q_W::new(self)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    #[must_use]
    pub fn singen_start_addr1_q(&mut self) -> SINGEN_START_ADDR1_Q_W<12> {
        SINGEN_START_ADDR1_Q_W::new(self)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    #[must_use]
    pub fn singen_start_addr0_q(&mut self) -> SINGEN_START_ADDR0_Q_W<22> {
        SINGEN_START_ADDR0_Q_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "singen_ctrl3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl3](index.html) module"]
pub struct SINGEN_CTRL3_SPEC;
impl crate::RegisterSpec for SINGEN_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singen_ctrl3::R](R) reader structure"]
impl crate::Readable for SINGEN_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singen_ctrl3::W](W) writer structure"]
impl crate::Writable for SINGEN_CTRL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets singen_ctrl3 to value 0"]
impl crate::Resettable for SINGEN_CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

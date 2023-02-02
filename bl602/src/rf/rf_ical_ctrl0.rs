#[doc = "Register `rf_ical_ctrl0` reader"]
pub struct R(crate::R<RF_ICAL_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ICAL_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ICAL_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ICAL_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_ical_ctrl0` writer"]
pub struct W(crate::W<RF_ICAL_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ICAL_CTRL0_SPEC>;
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
impl From<crate::W<RF_ICAL_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ICAL_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_ical_r_cnt_n` reader - "]
pub type RF_ICAL_R_CNT_N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_ical_r_cnt_n` writer - "]
pub type RF_ICAL_R_CNT_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_ICAL_CTRL0_SPEC, u16, u16, 10, O>;
#[doc = "Field `rf_ical_a_cnt_n` reader - "]
pub type RF_ICAL_A_CNT_N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_ical_a_cnt_n` writer - "]
pub type RF_ICAL_A_CNT_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_ICAL_CTRL0_SPEC, u16, u16, 10, O>;
#[doc = "Field `rf_ical_f_cnt_n` reader - "]
pub type RF_ICAL_F_CNT_N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_ical_f_cnt_n` writer - "]
pub type RF_ICAL_F_CNT_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_ICAL_CTRL0_SPEC, u16, u16, 10, O>;
#[doc = "Field `rf_ical_a_ud_inv_en` reader - "]
pub type RF_ICAL_A_UD_INV_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_ical_a_ud_inv_en` writer - "]
pub type RF_ICAL_A_UD_INV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_ICAL_CTRL0_SPEC, bool, O>;
#[doc = "Field `rf_ical_f_ud_inv_en` reader - "]
pub type RF_ICAL_F_UD_INV_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_ical_f_ud_inv_en` writer - "]
pub type RF_ICAL_F_UD_INV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_ICAL_CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rf_ical_r_cnt_n(&self) -> RF_ICAL_R_CNT_N_R {
        RF_ICAL_R_CNT_N_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_a_cnt_n(&self) -> RF_ICAL_A_CNT_N_R {
        RF_ICAL_A_CNT_N_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_f_cnt_n(&self) -> RF_ICAL_F_CNT_N_R {
        RF_ICAL_F_CNT_N_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_ical_a_ud_inv_en(&self) -> RF_ICAL_A_UD_INV_EN_R {
        RF_ICAL_A_UD_INV_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_ical_f_ud_inv_en(&self) -> RF_ICAL_F_UD_INV_EN_R {
        RF_ICAL_F_UD_INV_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ical_r_cnt_n(&mut self) -> RF_ICAL_R_CNT_N_W<0> {
        RF_ICAL_R_CNT_N_W::new(self)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ical_a_cnt_n(&mut self) -> RF_ICAL_A_CNT_N_W<10> {
        RF_ICAL_A_CNT_N_W::new(self)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ical_f_cnt_n(&mut self) -> RF_ICAL_F_CNT_N_W<20> {
        RF_ICAL_F_CNT_N_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ical_a_ud_inv_en(&mut self) -> RF_ICAL_A_UD_INV_EN_W<30> {
        RF_ICAL_A_UD_INV_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ical_f_ud_inv_en(&mut self) -> RF_ICAL_F_UD_INV_EN_W<31> {
        RF_ICAL_F_UD_INV_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_ical_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ical_ctrl0](index.html) module"]
pub struct RF_ICAL_CTRL0_SPEC;
impl crate::RegisterSpec for RF_ICAL_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ical_ctrl0::R](R) reader structure"]
impl crate::Readable for RF_ICAL_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ical_ctrl0::W](W) writer structure"]
impl crate::Writable for RF_ICAL_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_ical_ctrl0 to value 0"]
impl crate::Resettable for RF_ICAL_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

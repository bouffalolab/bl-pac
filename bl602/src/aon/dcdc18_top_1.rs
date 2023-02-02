#[doc = "Register `dcdc18_top_1` reader"]
pub struct R(crate::R<DCDC18_TOP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC18_TOP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC18_TOP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC18_TOP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dcdc18_top_1` writer"]
pub struct W(crate::W<DCDC18_TOP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC18_TOP_1_SPEC>;
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
impl From<crate::W<DCDC18_TOP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC18_TOP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dcdc18_force_cs_zvs_aon` reader - "]
pub type DCDC18_FORCE_CS_ZVS_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_force_cs_zvs_aon` writer - "]
pub type DCDC18_FORCE_CS_ZVS_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC18_TOP_1_SPEC, bool, O>;
#[doc = "Field `dcdc18_cs_delay_aon` reader - "]
pub type DCDC18_CS_DELAY_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_cs_delay_aon` writer - "]
pub type DCDC18_CS_DELAY_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 3, O>;
#[doc = "Field `dcdc18_zvs_td_opt_aon` reader - "]
pub type DCDC18_ZVS_TD_OPT_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_zvs_td_opt_aon` writer - "]
pub type DCDC18_ZVS_TD_OPT_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 3, O>;
#[doc = "Field `dcdc18_nonoverlap_td_aon` reader - "]
pub type DCDC18_NONOVERLAP_TD_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_nonoverlap_td_aon` writer - "]
pub type DCDC18_NONOVERLAP_TD_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 5, O>;
#[doc = "Field `dcdc18_rc_sel_aon` reader - "]
pub type DCDC18_RC_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_rc_sel_aon` writer - "]
pub type DCDC18_RC_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `dcdc18_chf_sel_aon` reader - "]
pub type DCDC18_CHF_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_chf_sel_aon` writer - "]
pub type DCDC18_CHF_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `dcdc18_cfb_sel_aon` reader - "]
pub type DCDC18_CFB_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_cfb_sel_aon` writer - "]
pub type DCDC18_CFB_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `dcdc18_en_antiring_aon` reader - "]
pub type DCDC18_EN_ANTIRING_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_en_antiring_aon` writer - "]
pub type DCDC18_EN_ANTIRING_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC18_TOP_1_SPEC, bool, O>;
#[doc = "Field `dcdc18_pulldown_aon` reader - "]
pub type DCDC18_PULLDOWN_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_pulldown_aon` writer - "]
pub type DCDC18_PULLDOWN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC18_TOP_1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dcdc18_force_cs_zvs_aon(&self) -> DCDC18_FORCE_CS_ZVS_AON_R {
        DCDC18_FORCE_CS_ZVS_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn dcdc18_cs_delay_aon(&self) -> DCDC18_CS_DELAY_AON_R {
        DCDC18_CS_DELAY_AON_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc18_zvs_td_opt_aon(&self) -> DCDC18_ZVS_TD_OPT_AON_R {
        DCDC18_ZVS_TD_OPT_AON_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn dcdc18_nonoverlap_td_aon(&self) -> DCDC18_NONOVERLAP_TD_AON_R {
        DCDC18_NONOVERLAP_TD_AON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_rc_sel_aon(&self) -> DCDC18_RC_SEL_AON_R {
        DCDC18_RC_SEL_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_chf_sel_aon(&self) -> DCDC18_CHF_SEL_AON_R {
        DCDC18_CHF_SEL_AON_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dcdc18_cfb_sel_aon(&self) -> DCDC18_CFB_SEL_AON_R {
        DCDC18_CFB_SEL_AON_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dcdc18_en_antiring_aon(&self) -> DCDC18_EN_ANTIRING_AON_R {
        DCDC18_EN_ANTIRING_AON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dcdc18_pulldown_aon(&self) -> DCDC18_PULLDOWN_AON_R {
        DCDC18_PULLDOWN_AON_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_force_cs_zvs_aon(&mut self) -> DCDC18_FORCE_CS_ZVS_AON_W<0> {
        DCDC18_FORCE_CS_ZVS_AON_W::new(self)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_cs_delay_aon(&mut self) -> DCDC18_CS_DELAY_AON_W<1> {
        DCDC18_CS_DELAY_AON_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_zvs_td_opt_aon(&mut self) -> DCDC18_ZVS_TD_OPT_AON_W<4> {
        DCDC18_ZVS_TD_OPT_AON_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_nonoverlap_td_aon(&mut self) -> DCDC18_NONOVERLAP_TD_AON_W<8> {
        DCDC18_NONOVERLAP_TD_AON_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_rc_sel_aon(&mut self) -> DCDC18_RC_SEL_AON_W<16> {
        DCDC18_RC_SEL_AON_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_chf_sel_aon(&mut self) -> DCDC18_CHF_SEL_AON_W<20> {
        DCDC18_CHF_SEL_AON_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_cfb_sel_aon(&mut self) -> DCDC18_CFB_SEL_AON_W<24> {
        DCDC18_CFB_SEL_AON_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_en_antiring_aon(&mut self) -> DCDC18_EN_ANTIRING_AON_W<28> {
        DCDC18_EN_ANTIRING_AON_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_pulldown_aon(&mut self) -> DCDC18_PULLDOWN_AON_W<29> {
        DCDC18_PULLDOWN_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dcdc18_top_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc18_top_1](index.html) module"]
pub struct DCDC18_TOP_1_SPEC;
impl crate::RegisterSpec for DCDC18_TOP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc18_top_1::R](R) reader structure"]
impl crate::Readable for DCDC18_TOP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc18_top_1::W](W) writer structure"]
impl crate::Writable for DCDC18_TOP_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dcdc18_top_1 to value 0x1818_0048"]
impl crate::Resettable for DCDC18_TOP_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1818_0048;
}

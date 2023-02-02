#[doc = "Register `dfe_ctrl_4` reader"]
pub struct R(crate::R<DFE_CTRL_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_4` writer"]
pub struct W(crate::W<DFE_CTRL_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_4_SPEC>;
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
impl From<crate::W<DFE_CTRL_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_pf_th2` reader - "]
pub type RX_PF_TH2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_pf_th2` writer - "]
pub type RX_PF_TH2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_4_SPEC, u16, u16, 10, O>;
#[doc = "Field `rx_pf_th1` reader - "]
pub type RX_PF_TH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_pf_th1` writer - "]
pub type RX_PF_TH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_4_SPEC, u16, u16, 10, O>;
#[doc = "Field `rx_pf_q_en` reader - "]
pub type RX_PF_Q_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_pf_q_en` writer - "]
pub type RX_PF_Q_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_4_SPEC, bool, O>;
#[doc = "Field `rx_pf_i_en` reader - "]
pub type RX_PF_I_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_pf_i_en` writer - "]
pub type RX_PF_I_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_pf_th2(&self) -> RX_PF_TH2_R {
        RX_PF_TH2_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_pf_th1(&self) -> RX_PF_TH1_R {
        RX_PF_TH1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_pf_q_en(&self) -> RX_PF_Q_EN_R {
        RX_PF_Q_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_pf_i_en(&self) -> RX_PF_I_EN_R {
        RX_PF_I_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pf_th2(&mut self) -> RX_PF_TH2_W<0> {
        RX_PF_TH2_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pf_th1(&mut self) -> RX_PF_TH1_W<16> {
        RX_PF_TH1_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pf_q_en(&mut self) -> RX_PF_Q_EN_W<30> {
        RX_PF_Q_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pf_i_en(&mut self) -> RX_PF_I_EN_W<31> {
        RX_PF_I_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_4](index.html) module"]
pub struct DFE_CTRL_4_SPEC;
impl crate::RegisterSpec for DFE_CTRL_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_4::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_4::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_4 to value 0"]
impl crate::Resettable for DFE_CTRL_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

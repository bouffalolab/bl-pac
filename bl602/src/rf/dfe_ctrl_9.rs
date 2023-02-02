#[doc = "Register `dfe_ctrl_9` reader"]
pub struct R(crate::R<DFE_CTRL_9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_9` writer"]
pub struct W(crate::W<DFE_CTRL_9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_9_SPEC>;
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
impl From<crate::W<DFE_CTRL_9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_pm_iqacc_q` reader - "]
pub type RX_PM_IQACC_Q_R = crate::FieldReader<u32, u32>;
#[doc = "Field `rx_pm_iqacc_q` writer - "]
pub type RX_PM_IQACC_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_9_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn rx_pm_iqacc_q(&self) -> RX_PM_IQACC_Q_R {
        RX_PM_IQACC_Q_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pm_iqacc_q(&mut self) -> RX_PM_IQACC_Q_W<0> {
        RX_PM_IQACC_Q_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_9](index.html) module"]
pub struct DFE_CTRL_9_SPEC;
impl crate::RegisterSpec for DFE_CTRL_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_9::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_9::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_9 to value 0"]
impl crate::Resettable for DFE_CTRL_9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

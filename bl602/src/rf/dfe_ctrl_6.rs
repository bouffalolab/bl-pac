#[doc = "Register `dfe_ctrl_6` reader"]
pub struct R(crate::R<DFE_CTRL_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_6` writer"]
pub struct W(crate::W<DFE_CTRL_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_6_SPEC>;
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
impl From<crate::W<DFE_CTRL_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_pm_freqshift_cw` reader - "]
pub type RX_PM_FREQSHIFT_CW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `rx_pm_freqshift_cw` writer - "]
pub type RX_PM_FREQSHIFT_CW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_6_SPEC, u32, u32, 20, O>;
#[doc = "Field `rx_pm_freqshift_en` reader - "]
pub type RX_PM_FREQSHIFT_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_pm_freqshift_en` writer - "]
pub type RX_PM_FREQSHIFT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFE_CTRL_6_SPEC, bool, O>;
#[doc = "Field `rx_pm_done` reader - "]
pub type RX_PM_DONE_R = crate::BitReader<bool>;
#[doc = "Field `rx_pm_done` writer - "]
pub type RX_PM_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_6_SPEC, bool, O>;
#[doc = "Field `rx_pm_en` reader - "]
pub type RX_PM_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_pm_en` writer - "]
pub type RX_PM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFE_CTRL_6_SPEC, bool, O>;
#[doc = "Field `rx_pm_in_sel` reader - "]
pub type RX_PM_IN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_pm_in_sel` writer - "]
pub type RX_PM_IN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_6_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rx_pm_freqshift_cw(&self) -> RX_PM_FREQSHIFT_CW_R {
        RX_PM_FREQSHIFT_CW_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pm_freqshift_en(&self) -> RX_PM_FREQSHIFT_EN_R {
        RX_PM_FREQSHIFT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_pm_done(&self) -> RX_PM_DONE_R {
        RX_PM_DONE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_pm_en(&self) -> RX_PM_EN_R {
        RX_PM_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rx_pm_in_sel(&self) -> RX_PM_IN_SEL_R {
        RX_PM_IN_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pm_freqshift_cw(&mut self) -> RX_PM_FREQSHIFT_CW_W<0> {
        RX_PM_FREQSHIFT_CW_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pm_freqshift_en(&mut self) -> RX_PM_FREQSHIFT_EN_W<20> {
        RX_PM_FREQSHIFT_EN_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pm_done(&mut self) -> RX_PM_DONE_W<28> {
        RX_PM_DONE_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pm_en(&mut self) -> RX_PM_EN_W<29> {
        RX_PM_EN_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pm_in_sel(&mut self) -> RX_PM_IN_SEL_W<30> {
        RX_PM_IN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_6](index.html) module"]
pub struct DFE_CTRL_6_SPEC;
impl crate::RegisterSpec for DFE_CTRL_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_6::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_6::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_6 to value 0"]
impl crate::Resettable for DFE_CTRL_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `dfe_ctrl_7` reader"]
pub struct R(crate::R<DFE_CTRL_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_7` writer"]
pub struct W(crate::W<DFE_CTRL_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_7_SPEC>;
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
impl From<crate::W<DFE_CTRL_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_pm_start_ofs` reader - "]
pub type RX_PM_START_OFS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_pm_start_ofs` writer - "]
pub type RX_PM_START_OFS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_7_SPEC, u16, u16, 16, O>;
#[doc = "Field `rx_pm_acc_len` reader - "]
pub type RX_PM_ACC_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_pm_acc_len` writer - "]
pub type RX_PM_ACC_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_7_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_pm_start_ofs(&self) -> RX_PM_START_OFS_R {
        RX_PM_START_OFS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rx_pm_acc_len(&self) -> RX_PM_ACC_LEN_R {
        RX_PM_ACC_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pm_start_ofs(&mut self) -> RX_PM_START_OFS_W<0> {
        RX_PM_START_OFS_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pm_acc_len(&mut self) -> RX_PM_ACC_LEN_W<16> {
        RX_PM_ACC_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_7](index.html) module"]
pub struct DFE_CTRL_7_SPEC;
impl crate::RegisterSpec for DFE_CTRL_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_7::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_7::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_7 to value 0"]
impl crate::Resettable for DFE_CTRL_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

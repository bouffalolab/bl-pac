#[doc = "Register `pa_reg_wifi_ctrl_hw` reader"]
pub struct R(crate::R<PA_REG_WIFI_CTRL_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_REG_WIFI_CTRL_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA_REG_WIFI_CTRL_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA_REG_WIFI_CTRL_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pa_reg_wifi_ctrl_hw` writer"]
pub struct W(crate::W<PA_REG_WIFI_CTRL_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_REG_WIFI_CTRL_HW_SPEC>;
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
impl From<crate::W<PA_REG_WIFI_CTRL_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_REG_WIFI_CTRL_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_half_on_wifi` reader - "]
pub type PA_HALF_ON_WIFI_R = crate::BitReader<bool>;
#[doc = "Field `pa_half_on_wifi` writer - "]
pub type PA_HALF_ON_WIFI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PA_REG_WIFI_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `pa_etb_en_wifi` reader - "]
pub type PA_ETB_EN_WIFI_R = crate::BitReader<bool>;
#[doc = "Field `pa_etb_en_wifi` writer - "]
pub type PA_ETB_EN_WIFI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PA_REG_WIFI_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `pa_ib_fix_wifi` reader - "]
pub type PA_IB_FIX_WIFI_R = crate::BitReader<bool>;
#[doc = "Field `pa_ib_fix_wifi` writer - "]
pub type PA_IB_FIX_WIFI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PA_REG_WIFI_CTRL_HW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pa_half_on_wifi(&self) -> PA_HALF_ON_WIFI_R {
        PA_HALF_ON_WIFI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pa_etb_en_wifi(&self) -> PA_ETB_EN_WIFI_R {
        PA_ETB_EN_WIFI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix_wifi(&self) -> PA_IB_FIX_WIFI_R {
        PA_IB_FIX_WIFI_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pa_half_on_wifi(&mut self) -> PA_HALF_ON_WIFI_W<0> {
        PA_HALF_ON_WIFI_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pa_etb_en_wifi(&mut self) -> PA_ETB_EN_WIFI_W<8> {
        PA_ETB_EN_WIFI_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pa_ib_fix_wifi(&mut self) -> PA_IB_FIX_WIFI_W<16> {
        PA_IB_FIX_WIFI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pa_reg_wifi_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_reg_wifi_ctrl_hw](index.html) module"]
pub struct PA_REG_WIFI_CTRL_HW_SPEC;
impl crate::RegisterSpec for PA_REG_WIFI_CTRL_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_reg_wifi_ctrl_hw::R](R) reader structure"]
impl crate::Readable for PA_REG_WIFI_CTRL_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_reg_wifi_ctrl_hw::W](W) writer structure"]
impl crate::Writable for PA_REG_WIFI_CTRL_HW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pa_reg_wifi_ctrl_hw to value 0"]
impl crate::Resettable for PA_REG_WIFI_CTRL_HW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

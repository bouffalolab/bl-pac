#[doc = "Register `pfdcp` reader"]
pub struct R(crate::R<PFDCP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFDCP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFDCP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFDCP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pfdcp` writer"]
pub struct W(crate::W<PFDCP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFDCP_SPEC>;
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
impl From<crate::W<PFDCP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFDCP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_cp_sel` reader - "]
pub type LO_CP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `lo_cp_sel` writer - "]
pub type LO_CP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFDCP_SPEC, bool, O>;
#[doc = "Field `lo_cp_sel_hw` reader - "]
pub type LO_CP_SEL_HW_R = crate::BitReader<bool>;
#[doc = "Field `lo_cp_sel_hw` writer - "]
pub type LO_CP_SEL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFDCP_SPEC, bool, O>;
#[doc = "Field `lo_cp_startup_en` reader - "]
pub type LO_CP_STARTUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `lo_cp_startup_en` writer - "]
pub type LO_CP_STARTUP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFDCP_SPEC, bool, O>;
#[doc = "Field `lo_cp_ota_en` reader - "]
pub type LO_CP_OTA_EN_R = crate::BitReader<bool>;
#[doc = "Field `lo_cp_ota_en` writer - "]
pub type LO_CP_OTA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFDCP_SPEC, bool, O>;
#[doc = "Field `lo_cp_opamp_en` reader - "]
pub type LO_CP_OPAMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `lo_cp_opamp_en` writer - "]
pub type LO_CP_OPAMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFDCP_SPEC, bool, O>;
#[doc = "Field `lo_cp_hiz` reader - "]
pub type LO_CP_HIZ_R = crate::BitReader<bool>;
#[doc = "Field `lo_cp_hiz` writer - "]
pub type LO_CP_HIZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFDCP_SPEC, bool, O>;
#[doc = "Field `lo_pfd_rvdd_boost` reader - "]
pub type LO_PFD_RVDD_BOOST_R = crate::BitReader<bool>;
#[doc = "Field `lo_pfd_rvdd_boost` writer - "]
pub type LO_PFD_RVDD_BOOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFDCP_SPEC, bool, O>;
#[doc = "Field `lo_pfd_rst_csd` reader - "]
pub type LO_PFD_RST_CSD_R = crate::BitReader<bool>;
#[doc = "Field `lo_pfd_rst_csd` writer - "]
pub type LO_PFD_RST_CSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFDCP_SPEC, bool, O>;
#[doc = "Field `lo_pfd_rst_csd_hw` reader - "]
pub type LO_PFD_RST_CSD_HW_R = crate::BitReader<bool>;
#[doc = "Field `lo_pfd_rst_csd_hw` writer - "]
pub type LO_PFD_RST_CSD_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFDCP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_cp_sel(&self) -> LO_CP_SEL_R {
        LO_CP_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_cp_sel_hw(&self) -> LO_CP_SEL_HW_R {
        LO_CP_SEL_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_cp_startup_en(&self) -> LO_CP_STARTUP_EN_R {
        LO_CP_STARTUP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_cp_ota_en(&self) -> LO_CP_OTA_EN_R {
        LO_CP_OTA_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_cp_opamp_en(&self) -> LO_CP_OPAMP_EN_R {
        LO_CP_OPAMP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_cp_hiz(&self) -> LO_CP_HIZ_R {
        LO_CP_HIZ_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_pfd_rvdd_boost(&self) -> LO_PFD_RVDD_BOOST_R {
        LO_PFD_RVDD_BOOST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd(&self) -> LO_PFD_RST_CSD_R {
        LO_PFD_RST_CSD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd_hw(&self) -> LO_PFD_RST_CSD_HW_R {
        LO_PFD_RST_CSD_HW_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lo_cp_sel(&mut self) -> LO_CP_SEL_W<0> {
        LO_CP_SEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn lo_cp_sel_hw(&mut self) -> LO_CP_SEL_HW_W<4> {
        LO_CP_SEL_HW_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lo_cp_startup_en(&mut self) -> LO_CP_STARTUP_EN_W<8> {
        LO_CP_STARTUP_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn lo_cp_ota_en(&mut self) -> LO_CP_OTA_EN_W<12> {
        LO_CP_OTA_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn lo_cp_opamp_en(&mut self) -> LO_CP_OPAMP_EN_W<16> {
        LO_CP_OPAMP_EN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn lo_cp_hiz(&mut self) -> LO_CP_HIZ_W<20> {
        LO_CP_HIZ_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn lo_pfd_rvdd_boost(&mut self) -> LO_PFD_RVDD_BOOST_W<24> {
        LO_PFD_RVDD_BOOST_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn lo_pfd_rst_csd(&mut self) -> LO_PFD_RST_CSD_W<28> {
        LO_PFD_RST_CSD_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn lo_pfd_rst_csd_hw(&mut self) -> LO_PFD_RST_CSD_HW_W<29> {
        LO_PFD_RST_CSD_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pfdcp.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfdcp](index.html) module"]
pub struct PFDCP_SPEC;
impl crate::RegisterSpec for PFDCP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfdcp::R](R) reader structure"]
impl crate::Readable for PFDCP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfdcp::W](W) writer structure"]
impl crate::Writable for PFDCP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pfdcp to value 0"]
impl crate::Resettable for PFDCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

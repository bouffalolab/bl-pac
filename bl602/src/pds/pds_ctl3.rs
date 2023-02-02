#[doc = "Register `PDS_CTL3` reader"]
pub struct R(crate::R<PDS_CTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_CTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_CTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_CTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_CTL3` writer"]
pub struct W(crate::W<PDS_CTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_CTL3_SPEC>;
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
impl From<crate::W<PDS_CTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_CTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_pds_force_misc_pwr_off` reader - "]
pub type CR_PDS_FORCE_MISC_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_misc_pwr_off` writer - "]
pub type CR_PDS_FORCE_MISC_PWR_OFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL3_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_misc_iso_en` reader - "]
pub type CR_PDS_FORCE_MISC_ISO_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_misc_iso_en` writer - "]
pub type CR_PDS_FORCE_MISC_ISO_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL3_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_misc_pds_rst` reader - "]
pub type CR_PDS_FORCE_MISC_PDS_RST_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_misc_pds_rst` writer - "]
pub type CR_PDS_FORCE_MISC_PDS_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL3_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_misc_mem_stby` reader - "]
pub type CR_PDS_FORCE_MISC_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_misc_mem_stby` writer - "]
pub type CR_PDS_FORCE_MISC_MEM_STBY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL3_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_misc_gate_clk` reader - "]
pub type CR_PDS_FORCE_MISC_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_misc_gate_clk` writer - "]
pub type CR_PDS_FORCE_MISC_GATE_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL3_SPEC, bool, O>;
#[doc = "Field `cr_pds_np_iso_en` reader - "]
pub type CR_PDS_NP_ISO_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_np_iso_en` writer - "]
pub type CR_PDS_NP_ISO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL3_SPEC, bool, O>;
#[doc = "Field `cr_pds_wb_iso_en` reader - "]
pub type CR_PDS_WB_ISO_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_wb_iso_en` writer - "]
pub type CR_PDS_WB_ISO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL3_SPEC, bool, O>;
#[doc = "Field `cr_pds_misc_iso_en` reader - "]
pub type CR_PDS_MISC_ISO_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_misc_iso_en` writer - "]
pub type CR_PDS_MISC_ISO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pwr_off(&self) -> CR_PDS_FORCE_MISC_PWR_OFF_R {
        CR_PDS_FORCE_MISC_PWR_OFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_misc_iso_en(&self) -> CR_PDS_FORCE_MISC_ISO_EN_R {
        CR_PDS_FORCE_MISC_ISO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pds_rst(&self) -> CR_PDS_FORCE_MISC_PDS_RST_R {
        CR_PDS_FORCE_MISC_PDS_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_misc_mem_stby(&self) -> CR_PDS_FORCE_MISC_MEM_STBY_R {
        CR_PDS_FORCE_MISC_MEM_STBY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_force_misc_gate_clk(&self) -> CR_PDS_FORCE_MISC_GATE_CLK_R {
        CR_PDS_FORCE_MISC_GATE_CLK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_np_iso_en(&self) -> CR_PDS_NP_ISO_EN_R {
        CR_PDS_NP_ISO_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_wb_iso_en(&self) -> CR_PDS_WB_ISO_EN_R {
        CR_PDS_WB_ISO_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_misc_iso_en(&self) -> CR_PDS_MISC_ISO_EN_R {
        CR_PDS_MISC_ISO_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_misc_pwr_off(&mut self) -> CR_PDS_FORCE_MISC_PWR_OFF_W<1> {
        CR_PDS_FORCE_MISC_PWR_OFF_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_misc_iso_en(&mut self) -> CR_PDS_FORCE_MISC_ISO_EN_W<4> {
        CR_PDS_FORCE_MISC_ISO_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_misc_pds_rst(&mut self) -> CR_PDS_FORCE_MISC_PDS_RST_W<7> {
        CR_PDS_FORCE_MISC_PDS_RST_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_misc_mem_stby(&mut self) -> CR_PDS_FORCE_MISC_MEM_STBY_W<10> {
        CR_PDS_FORCE_MISC_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_misc_gate_clk(&mut self) -> CR_PDS_FORCE_MISC_GATE_CLK_W<13> {
        CR_PDS_FORCE_MISC_GATE_CLK_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_np_iso_en(&mut self) -> CR_PDS_NP_ISO_EN_W<24> {
        CR_PDS_NP_ISO_EN_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wb_iso_en(&mut self) -> CR_PDS_WB_ISO_EN_W<27> {
        CR_PDS_WB_ISO_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_misc_iso_en(&mut self) -> CR_PDS_MISC_ISO_EN_W<30> {
        CR_PDS_MISC_ISO_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_CTL3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl3](index.html) module"]
pub struct PDS_CTL3_SPEC;
impl crate::RegisterSpec for PDS_CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ctl3::R](R) reader structure"]
impl crate::Readable for PDS_CTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ctl3::W](W) writer structure"]
impl crate::Writable for PDS_CTL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_CTL3 to value 0x4900_0000"]
impl crate::Resettable for PDS_CTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x4900_0000;
}

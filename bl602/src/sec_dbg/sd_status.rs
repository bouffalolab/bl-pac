#[doc = "Register `sd_status` reader"]
pub struct R(crate::R<SD_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_status` writer"]
pub struct W(crate::W<SD_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_STATUS_SPEC>;
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
impl From<crate::W<SD_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sd_dbg_pwd_busy` reader - "]
pub type SD_DBG_PWD_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `sd_dbg_pwd_trig` reader - "]
pub type SD_DBG_PWD_TRIG_R = crate::BitReader<bool>;
#[doc = "Field `sd_dbg_pwd_trig` writer - "]
pub type SD_DBG_PWD_TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_STATUS_SPEC, bool, O>;
#[doc = "Field `sd_dbg_cci_read_en` reader - "]
pub type SD_DBG_CCI_READ_EN_R = crate::BitReader<bool>;
#[doc = "Field `sd_dbg_cci_read_en` writer - "]
pub type SD_DBG_CCI_READ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_STATUS_SPEC, bool, O>;
#[doc = "Field `sd_dbg_cci_clk_sel` reader - "]
pub type SD_DBG_CCI_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `sd_dbg_cci_clk_sel` writer - "]
pub type SD_DBG_CCI_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_STATUS_SPEC, bool, O>;
#[doc = "Field `sd_dbg_pwd_cnt` reader - "]
pub type SD_DBG_PWD_CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `sd_dbg_mode` reader - "]
pub type SD_DBG_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sd_dbg_ena` reader - "]
pub type SD_DBG_ENA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sd_dbg_pwd_busy(&self) -> SD_DBG_PWD_BUSY_R {
        SD_DBG_PWD_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sd_dbg_pwd_trig(&self) -> SD_DBG_PWD_TRIG_R {
        SD_DBG_PWD_TRIG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sd_dbg_cci_read_en(&self) -> SD_DBG_CCI_READ_EN_R {
        SD_DBG_CCI_READ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sd_dbg_cci_clk_sel(&self) -> SD_DBG_CCI_CLK_SEL_R {
        SD_DBG_CCI_CLK_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:23"]
    #[inline(always)]
    pub fn sd_dbg_pwd_cnt(&self) -> SD_DBG_PWD_CNT_R {
        SD_DBG_PWD_CNT_R::new((self.bits >> 4) & 0x000f_ffff)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sd_dbg_mode(&self) -> SD_DBG_MODE_R {
        SD_DBG_MODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sd_dbg_ena(&self) -> SD_DBG_ENA_R {
        SD_DBG_ENA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sd_dbg_pwd_trig(&mut self) -> SD_DBG_PWD_TRIG_W<1> {
        SD_DBG_PWD_TRIG_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sd_dbg_cci_read_en(&mut self) -> SD_DBG_CCI_READ_EN_W<2> {
        SD_DBG_CCI_READ_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sd_dbg_cci_clk_sel(&mut self) -> SD_DBG_CCI_CLK_SEL_W<3> {
        SD_DBG_CCI_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sd_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_status](index.html) module"]
pub struct SD_STATUS_SPEC;
impl crate::RegisterSpec for SD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_status::R](R) reader structure"]
impl crate::Readable for SD_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_status::W](W) writer structure"]
impl crate::Writable for SD_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_status to value 0"]
impl crate::Resettable for SD_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

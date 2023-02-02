#[doc = "Register `cci_cfg` reader"]
pub struct R(crate::R<CCI_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCI_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCI_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCI_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cci_cfg` writer"]
pub struct W(crate::W<CCI_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCI_CFG_SPEC>;
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
impl From<crate::W<CCI_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCI_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cci_en` reader - "]
pub type CCI_EN_R = crate::BitReader<bool>;
#[doc = "Field `cci_en` writer - "]
pub type CCI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCI_CFG_SPEC, bool, O>;
#[doc = "Field `cci_slv_sel_cci2` reader - "]
pub type CCI_SLV_SEL_CCI2_R = crate::BitReader<bool>;
#[doc = "Field `cci_slv_sel_cci2` writer - "]
pub type CCI_SLV_SEL_CCI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCI_CFG_SPEC, bool, O>;
#[doc = "Field `cci_mas_sel_cci2` reader - "]
pub type CCI_MAS_SEL_CCI2_R = crate::BitReader<bool>;
#[doc = "Field `cci_mas_sel_cci2` writer - "]
pub type CCI_MAS_SEL_CCI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCI_CFG_SPEC, bool, O>;
#[doc = "Field `cci_mas_hw_mode` reader - "]
pub type CCI_MAS_HW_MODE_R = crate::BitReader<bool>;
#[doc = "Field `cci_mas_hw_mode` writer - "]
pub type CCI_MAS_HW_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCI_CFG_SPEC, bool, O>;
#[doc = "Field `reg_m_cci_sclk_en` reader - "]
pub type REG_M_CCI_SCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_m_cci_sclk_en` writer - "]
pub type REG_M_CCI_SCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCI_CFG_SPEC, bool, O>;
#[doc = "Field `reg_div_m_cci_sclk` reader - "]
pub type REG_DIV_M_CCI_SCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_div_m_cci_sclk` writer - "]
pub type REG_DIV_M_CCI_SCLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCI_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cfg_cci1_pre_read` reader - "]
pub type CFG_CCI1_PRE_READ_R = crate::BitReader<bool>;
#[doc = "Field `cfg_cci1_pre_read` writer - "]
pub type CFG_CCI1_PRE_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCI_CFG_SPEC, bool, O>;
#[doc = "Field `reg_scci_clk_inv` reader - "]
pub type REG_SCCI_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `reg_scci_clk_inv` writer - "]
pub type REG_SCCI_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCI_CFG_SPEC, bool, O>;
#[doc = "Field `reg_mcci_clk_inv` reader - "]
pub type REG_MCCI_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcci_clk_inv` writer - "]
pub type REG_MCCI_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCI_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_en(&self) -> CCI_EN_R {
        CCI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_slv_sel_cci2(&self) -> CCI_SLV_SEL_CCI2_R {
        CCI_SLV_SEL_CCI2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cci_mas_sel_cci2(&self) -> CCI_MAS_SEL_CCI2_R {
        CCI_MAS_SEL_CCI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cci_mas_hw_mode(&self) -> CCI_MAS_HW_MODE_R {
        CCI_MAS_HW_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_m_cci_sclk_en(&self) -> REG_M_CCI_SCLK_EN_R {
        REG_M_CCI_SCLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn reg_div_m_cci_sclk(&self) -> REG_DIV_M_CCI_SCLK_R {
        REG_DIV_M_CCI_SCLK_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_cci1_pre_read(&self) -> CFG_CCI1_PRE_READ_R {
        CFG_CCI1_PRE_READ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_scci_clk_inv(&self) -> REG_SCCI_CLK_INV_R {
        REG_SCCI_CLK_INV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mcci_clk_inv(&self) -> REG_MCCI_CLK_INV_R {
        REG_MCCI_CLK_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cci_en(&mut self) -> CCI_EN_W<0> {
        CCI_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cci_slv_sel_cci2(&mut self) -> CCI_SLV_SEL_CCI2_W<1> {
        CCI_SLV_SEL_CCI2_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cci_mas_sel_cci2(&mut self) -> CCI_MAS_SEL_CCI2_W<2> {
        CCI_MAS_SEL_CCI2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cci_mas_hw_mode(&mut self) -> CCI_MAS_HW_MODE_W<3> {
        CCI_MAS_HW_MODE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_m_cci_sclk_en(&mut self) -> REG_M_CCI_SCLK_EN_W<4> {
        REG_M_CCI_SCLK_EN_W::new(self)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_div_m_cci_sclk(&mut self) -> REG_DIV_M_CCI_SCLK_W<5> {
        REG_DIV_M_CCI_SCLK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_cci1_pre_read(&mut self) -> CFG_CCI1_PRE_READ_W<7> {
        CFG_CCI1_PRE_READ_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_scci_clk_inv(&mut self) -> REG_SCCI_CLK_INV_W<8> {
        REG_SCCI_CLK_INV_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcci_clk_inv(&mut self) -> REG_MCCI_CLK_INV_W<9> {
        REG_MCCI_CLK_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cci_cfg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_cfg](index.html) module"]
pub struct CCI_CFG_SPEC;
impl crate::RegisterSpec for CCI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cci_cfg::R](R) reader structure"]
impl crate::Readable for CCI_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cci_cfg::W](W) writer structure"]
impl crate::Writable for CCI_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cci_cfg to value 0x0221"]
impl crate::Resettable for CCI_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0221;
}

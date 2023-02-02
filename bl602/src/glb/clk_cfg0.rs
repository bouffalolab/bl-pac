#[doc = "Register `clk_cfg0` reader"]
pub struct R(crate::R<CLK_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg0` writer"]
pub struct W(crate::W<CLK_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG0_SPEC>;
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
impl From<crate::W<CLK_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_pll_en` reader - PLL enable"]
pub type REG_PLL_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_pll_en` writer - PLL enable"]
pub type REG_PLL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_fclk_en` reader - "]
pub type REG_FCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_fclk_en` writer - "]
pub type REG_FCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_hclk_en` reader - MCU clock enable"]
pub type REG_HCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_hclk_en` writer - MCU clock enable"]
pub type REG_HCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_bclk_en` reader - Base clock enable"]
pub type REG_BCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_bclk_en` writer - Base clock enable"]
pub type REG_BCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_pll_sel` reader - PLL clock selection (0: 48MHz, 1: 120MHz, 2: 160MHz and 3: 192MHz)"]
pub type REG_PLL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pll_sel` writer - PLL clock selection (0: 48MHz, 1: 120MHz, 2: 160MHz and 3: 192MHz)"]
pub type REG_PLL_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `hbn_root_clk_sel` reader - "]
pub type HBN_ROOT_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_hclk_div` reader - MCU clock divider"]
pub type REG_HCLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_hclk_div` writer - MCU clock divider"]
pub type REG_HCLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG0_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_bclk_div` reader - Base clock divider"]
pub type REG_BCLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_bclk_div` writer - Base clock divider"]
pub type REG_BCLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG0_SPEC, u8, u8, 8, O>;
#[doc = "Field `fclk_sw_state` reader - "]
pub type FCLK_SW_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `chip_rdy` reader - "]
pub type CHIP_RDY_R = crate::BitReader<bool>;
#[doc = "Field `glb_id` reader - "]
pub type GLB_ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - PLL enable"]
    #[inline(always)]
    pub fn reg_pll_en(&self) -> REG_PLL_EN_R {
        REG_PLL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_fclk_en(&self) -> REG_FCLK_EN_R {
        REG_FCLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCU clock enable"]
    #[inline(always)]
    pub fn reg_hclk_en(&self) -> REG_HCLK_EN_R {
        REG_HCLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Base clock enable"]
    #[inline(always)]
    pub fn reg_bclk_en(&self) -> REG_BCLK_EN_R {
        REG_BCLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PLL clock selection (0: 48MHz, 1: 120MHz, 2: 160MHz and 3: 192MHz)"]
    #[inline(always)]
    pub fn reg_pll_sel(&self) -> REG_PLL_SEL_R {
        REG_PLL_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&self) -> HBN_ROOT_CLK_SEL_R {
        HBN_ROOT_CLK_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - MCU clock divider"]
    #[inline(always)]
    pub fn reg_hclk_div(&self) -> REG_HCLK_DIV_R {
        REG_HCLK_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Base clock divider"]
    #[inline(always)]
    pub fn reg_bclk_div(&self) -> REG_BCLK_DIV_R {
        REG_BCLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn fclk_sw_state(&self) -> FCLK_SW_STATE_R {
        FCLK_SW_STATE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn chip_rdy(&self) -> CHIP_RDY_R {
        CHIP_RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn glb_id(&self) -> GLB_ID_R {
        GLB_ID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pll_en(&mut self) -> REG_PLL_EN_W<0> {
        REG_PLL_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_fclk_en(&mut self) -> REG_FCLK_EN_W<1> {
        REG_FCLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - MCU clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn reg_hclk_en(&mut self) -> REG_HCLK_EN_W<2> {
        REG_HCLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - Base clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bclk_en(&mut self) -> REG_BCLK_EN_W<3> {
        REG_BCLK_EN_W::new(self)
    }
    #[doc = "Bits 4:5 - PLL clock selection (0: 48MHz, 1: 120MHz, 2: 160MHz and 3: 192MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pll_sel(&mut self) -> REG_PLL_SEL_W<4> {
        REG_PLL_SEL_W::new(self)
    }
    #[doc = "Bits 8:15 - MCU clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn reg_hclk_div(&mut self) -> REG_HCLK_DIV_W<8> {
        REG_HCLK_DIV_W::new(self)
    }
    #[doc = "Bits 16:23 - Base clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bclk_div(&mut self) -> REG_BCLK_DIV_W<16> {
        REG_BCLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration for processor and bus\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg0](index.html) module"]
pub struct CLK_CFG0_SPEC;
impl crate::RegisterSpec for CLK_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg0::R](R) reader structure"]
impl crate::Readable for CLK_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg0::W](W) writer structure"]
impl crate::Writable for CLK_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk_cfg0 to value 0x6000_000f"]
impl crate::Resettable for CLK_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_000f;
}

#[doc = "Register `swrst_cfg2` reader"]
pub struct R(crate::R<SWRST_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRST_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRST_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `swrst_cfg2` writer"]
pub struct W(crate::W<SWRST_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRST_CFG2_SPEC>;
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
impl From<crate::W<SWRST_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRST_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_ctrl_pwron_rst` reader - "]
pub type REG_CTRL_PWRON_RST_R = crate::BitReader<bool>;
#[doc = "Field `reg_ctrl_pwron_rst` writer - "]
pub type REG_CTRL_PWRON_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reg_ctrl_cpu_reset` reader - "]
pub type REG_CTRL_CPU_RESET_R = crate::BitReader<bool>;
#[doc = "Field `reg_ctrl_cpu_reset` writer - "]
pub type REG_CTRL_CPU_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reg_ctrl_sys_reset` reader - "]
pub type REG_CTRL_SYS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `reg_ctrl_sys_reset` writer - "]
pub type REG_CTRL_SYS_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reg_ctrl_reset_dummy` reader - "]
pub type REG_CTRL_RESET_DUMMY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ctrl_reset_dummy` writer - "]
pub type REG_CTRL_RESET_DUMMY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWRST_CFG2_SPEC, u8, u8, 4, O>;
#[doc = "Field `pka_clk_sel` reader - "]
pub type PKA_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `pka_clk_sel` writer - "]
pub type PKA_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&self) -> REG_CTRL_PWRON_RST_R {
        REG_CTRL_PWRON_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ctrl_cpu_reset(&self) -> REG_CTRL_CPU_RESET_R {
        REG_CTRL_CPU_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&self) -> REG_CTRL_SYS_RESET_R {
        REG_CTRL_SYS_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_ctrl_reset_dummy(&self) -> REG_CTRL_RESET_DUMMY_R {
        REG_CTRL_RESET_DUMMY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pka_clk_sel(&self) -> PKA_CLK_SEL_R {
        PKA_CLK_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_pwron_rst(&mut self) -> REG_CTRL_PWRON_RST_W<0> {
        REG_CTRL_PWRON_RST_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_cpu_reset(&mut self) -> REG_CTRL_CPU_RESET_W<1> {
        REG_CTRL_CPU_RESET_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_sys_reset(&mut self) -> REG_CTRL_SYS_RESET_W<2> {
        REG_CTRL_SYS_RESET_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_reset_dummy(&mut self) -> REG_CTRL_RESET_DUMMY_W<4> {
        REG_CTRL_RESET_DUMMY_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pka_clk_sel(&mut self) -> PKA_CLK_SEL_W<24> {
        PKA_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "swrst_cfg2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg2](index.html) module"]
pub struct SWRST_CFG2_SPEC;
impl crate::RegisterSpec for SWRST_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst_cfg2::R](R) reader structure"]
impl crate::Readable for SWRST_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrst_cfg2::W](W) writer structure"]
impl crate::Writable for SWRST_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets swrst_cfg2 to value 0"]
impl crate::Resettable for SWRST_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

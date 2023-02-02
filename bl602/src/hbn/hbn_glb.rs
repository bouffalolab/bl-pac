#[doc = "Register `HBN_GLB` reader"]
pub struct R(crate::R<HBN_GLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_GLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_GLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_GLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_GLB` writer"]
pub struct W(crate::W<HBN_GLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_GLB_SPEC>;
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
impl From<crate::W<HBN_GLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_GLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hbn_root_clk_sel` reader - "]
pub type HBN_ROOT_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_root_clk_sel` writer - "]
pub type HBN_ROOT_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_GLB_SPEC, u8, u8, 2, O>;
#[doc = "Field `hbn_uart_clk_sel` reader - "]
pub type HBN_UART_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `hbn_uart_clk_sel` writer - "]
pub type HBN_UART_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_GLB_SPEC, bool, O>;
#[doc = "Field `hbn_f32k_sel` reader - "]
pub type HBN_F32K_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_f32k_sel` writer - "]
pub type HBN_F32K_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HBN_GLB_SPEC, u8, u8, 2, O>;
#[doc = "Field `hbn_pu_rc32k` reader - "]
pub type HBN_PU_RC32K_R = crate::BitReader<bool>;
#[doc = "Field `hbn_pu_rc32k` writer - "]
pub type HBN_PU_RC32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_GLB_SPEC, bool, O>;
#[doc = "Field `sw_ldo11soc_vout_sel_aon` reader - "]
pub type SW_LDO11SOC_VOUT_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sw_ldo11soc_vout_sel_aon` writer - "]
pub type SW_LDO11SOC_VOUT_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_GLB_SPEC, u8, u8, 4, O>;
#[doc = "Field `sw_ldo11_rt_vout_sel` reader - "]
pub type SW_LDO11_RT_VOUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sw_ldo11_rt_vout_sel` writer - "]
pub type SW_LDO11_RT_VOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_GLB_SPEC, u8, u8, 4, O>;
#[doc = "Field `sw_ldo11_aon_vout_sel` reader - "]
pub type SW_LDO11_AON_VOUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sw_ldo11_aon_vout_sel` writer - "]
pub type SW_LDO11_AON_VOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_GLB_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&self) -> HBN_ROOT_CLK_SEL_R {
        HBN_ROOT_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HBN_UART_CLK_SEL_R {
        HBN_UART_CLK_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_f32k_sel(&self) -> HBN_F32K_SEL_R {
        HBN_F32K_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hbn_pu_rc32k(&self) -> HBN_PU_RC32K_R {
        HBN_PU_RC32K_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sw_ldo11soc_vout_sel_aon(&self) -> SW_LDO11SOC_VOUT_SEL_AON_R {
        SW_LDO11SOC_VOUT_SEL_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sw_ldo11_rt_vout_sel(&self) -> SW_LDO11_RT_VOUT_SEL_R {
        SW_LDO11_RT_VOUT_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sw_ldo11_aon_vout_sel(&self) -> SW_LDO11_AON_VOUT_SEL_R {
        SW_LDO11_AON_VOUT_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_root_clk_sel(&mut self) -> HBN_ROOT_CLK_SEL_W<0> {
        HBN_ROOT_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_uart_clk_sel(&mut self) -> HBN_UART_CLK_SEL_W<2> {
        HBN_UART_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_f32k_sel(&mut self) -> HBN_F32K_SEL_W<3> {
        HBN_F32K_SEL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_pu_rc32k(&mut self) -> HBN_PU_RC32K_W<5> {
        HBN_PU_RC32K_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ldo11soc_vout_sel_aon(&mut self) -> SW_LDO11SOC_VOUT_SEL_AON_W<16> {
        SW_LDO11SOC_VOUT_SEL_AON_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ldo11_rt_vout_sel(&mut self) -> SW_LDO11_RT_VOUT_SEL_W<24> {
        SW_LDO11_RT_VOUT_SEL_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ldo11_aon_vout_sel(&mut self) -> SW_LDO11_AON_VOUT_SEL_W<28> {
        SW_LDO11_AON_VOUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_GLB.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_glb](index.html) module"]
pub struct HBN_GLB_SPEC;
impl crate::RegisterSpec for HBN_GLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_glb::R](R) reader structure"]
impl crate::Readable for HBN_GLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_glb::W](W) writer structure"]
impl crate::Writable for HBN_GLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_GLB to value 0xaa0a_0020"]
impl crate::Resettable for HBN_GLB_SPEC {
    const RESET_VALUE: Self::Ux = 0xaa0a_0020;
}

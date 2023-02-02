#[doc = "Register `clk_cfg2` reader"]
pub struct R(crate::R<CLK_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg2` writer"]
pub struct W(crate::W<CLK_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG2_SPEC>;
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
impl From<crate::W<CLK_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_clk_div` reader - "]
pub type UART_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_clk_div` writer - "]
pub type UART_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `uart_clk_en` reader - "]
pub type UART_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `uart_clk_en` writer - "]
pub type UART_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG2_SPEC, bool, O>;
#[doc = "Field `hbn_uart_clk_sel` reader - "]
pub type HBN_UART_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `sf_clk_div` reader - "]
pub type SF_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_clk_div` writer - "]
pub type SF_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `sf_clk_en` reader - "]
pub type SF_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_clk_en` writer - "]
pub type SF_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG2_SPEC, bool, O>;
#[doc = "Field `sf_clk_sel` reader - "]
pub type SF_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_clk_sel` writer - "]
pub type SF_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf_clk_sel2` reader - "]
pub type SF_CLK_SEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_clk_sel2` writer - "]
pub type SF_CLK_SEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ir_clk_div` reader - "]
pub type IR_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ir_clk_div` writer - "]
pub type IR_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG2_SPEC, u8, u8, 6, O>;
#[doc = "Field `ir_clk_en` reader - "]
pub type IR_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `ir_clk_en` writer - "]
pub type IR_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG2_SPEC, bool, O>;
#[doc = "Field `dma_clk_en` reader - "]
pub type DMA_CLK_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dma_clk_en` writer - "]
pub type DMA_CLK_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uart_clk_div(&self) -> UART_CLK_DIV_R {
        UART_CLK_DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HBN_UART_CLK_SEL_R {
        HBN_UART_CLK_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_clk_div(&self) -> SF_CLK_DIV_R {
        SF_CLK_DIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_clk_en(&self) -> SF_CLK_EN_R {
        SF_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sf_clk_sel(&self) -> SF_CLK_SEL_R {
        SF_CLK_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sf_clk_sel2(&self) -> SF_CLK_SEL2_R {
        SF_CLK_SEL2_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ir_clk_div(&self) -> IR_CLK_DIV_R {
        IR_CLK_DIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ir_clk_en(&self) -> IR_CLK_EN_R {
        IR_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn uart_clk_div(&mut self) -> UART_CLK_DIV_W<0> {
        UART_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W<4> {
        UART_CLK_EN_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_div(&mut self) -> SF_CLK_DIV_W<8> {
        SF_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_en(&mut self) -> SF_CLK_EN_W<11> {
        SF_CLK_EN_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_sel(&mut self) -> SF_CLK_SEL_W<12> {
        SF_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_sel2(&mut self) -> SF_CLK_SEL2_W<14> {
        SF_CLK_SEL2_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn ir_clk_div(&mut self) -> IR_CLK_DIV_W<16> {
        IR_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ir_clk_en(&mut self) -> IR_CLK_EN_W<23> {
        IR_CLK_EN_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W<24> {
        DMA_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration for UART and Flash\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg2](index.html) module"]
pub struct CLK_CFG2_SPEC;
impl crate::RegisterSpec for CLK_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg2::R](R) reader structure"]
impl crate::Readable for CLK_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg2::W](W) writer structure"]
impl crate::Writable for CLK_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk_cfg2 to value 0xff8f_2b17"]
impl crate::Resettable for CLK_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0xff8f_2b17;
}

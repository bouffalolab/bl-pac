#[doc = "Register `ppu_ctrl_hw` reader"]
pub struct R(crate::R<PPU_CTRL_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPU_CTRL_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPU_CTRL_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPU_CTRL_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ppu_ctrl_hw` writer"]
pub struct W(crate::W<PPU_CTRL_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPU_CTRL_HW_SPEC>;
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
impl From<crate::W<PPU_CTRL_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPU_CTRL_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ppu_lna_hw` reader - "]
pub type PPU_LNA_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_lna_hw` writer - "]
pub type PPU_LNA_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPU_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `ppu_rmxgm_hw` reader - "]
pub type PPU_RMXGM_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_rmxgm_hw` writer - "]
pub type PPU_RMXGM_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPU_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `ppu_rbb_hw` reader - "]
pub type PPU_RBB_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_rbb_hw` writer - "]
pub type PPU_RBB_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPU_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `ppu_vco_hw` reader - "]
pub type PPU_VCO_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_vco_hw` writer - "]
pub type PPU_VCO_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPU_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `ppu_fbdv_hw` reader - "]
pub type PPU_FBDV_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_fbdv_hw` writer - "]
pub type PPU_FBDV_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPU_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `ppu_pfd_hw` reader - "]
pub type PPU_PFD_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_pfd_hw` writer - "]
pub type PPU_PFD_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPU_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `ppu_osmx_hw` reader - "]
pub type PPU_OSMX_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_osmx_hw` writer - "]
pub type PPU_OSMX_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPU_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `ppu_rxbuf_hw` reader - "]
pub type PPU_RXBUF_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_rxbuf_hw` writer - "]
pub type PPU_RXBUF_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPU_CTRL_HW_SPEC, bool, O>;
#[doc = "Field `ppu_txbuf_hw` reader - "]
pub type PPU_TXBUF_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_txbuf_hw` writer - "]
pub type PPU_TXBUF_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPU_CTRL_HW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ppu_lna_hw(&self) -> PPU_LNA_HW_R {
        PPU_LNA_HW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ppu_rmxgm_hw(&self) -> PPU_RMXGM_HW_R {
        PPU_RMXGM_HW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ppu_rbb_hw(&self) -> PPU_RBB_HW_R {
        PPU_RBB_HW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ppu_vco_hw(&self) -> PPU_VCO_HW_R {
        PPU_VCO_HW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ppu_fbdv_hw(&self) -> PPU_FBDV_HW_R {
        PPU_FBDV_HW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ppu_pfd_hw(&self) -> PPU_PFD_HW_R {
        PPU_PFD_HW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ppu_osmx_hw(&self) -> PPU_OSMX_HW_R {
        PPU_OSMX_HW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ppu_rxbuf_hw(&self) -> PPU_RXBUF_HW_R {
        PPU_RXBUF_HW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ppu_txbuf_hw(&self) -> PPU_TXBUF_HW_R {
        PPU_TXBUF_HW_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_lna_hw(&mut self) -> PPU_LNA_HW_W<8> {
        PPU_LNA_HW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_rmxgm_hw(&mut self) -> PPU_RMXGM_HW_W<9> {
        PPU_RMXGM_HW_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_rbb_hw(&mut self) -> PPU_RBB_HW_W<11> {
        PPU_RBB_HW_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_vco_hw(&mut self) -> PPU_VCO_HW_W<20> {
        PPU_VCO_HW_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_fbdv_hw(&mut self) -> PPU_FBDV_HW_W<21> {
        PPU_FBDV_HW_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_pfd_hw(&mut self) -> PPU_PFD_HW_W<22> {
        PPU_PFD_HW_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_osmx_hw(&mut self) -> PPU_OSMX_HW_W<23> {
        PPU_OSMX_HW_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_rxbuf_hw(&mut self) -> PPU_RXBUF_HW_W<24> {
        PPU_RXBUF_HW_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_txbuf_hw(&mut self) -> PPU_TXBUF_HW_W<25> {
        PPU_TXBUF_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ppu_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppu_ctrl_hw](index.html) module"]
pub struct PPU_CTRL_HW_SPEC;
impl crate::RegisterSpec for PPU_CTRL_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppu_ctrl_hw::R](R) reader structure"]
impl crate::Readable for PPU_CTRL_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppu_ctrl_hw::W](W) writer structure"]
impl crate::Writable for PPU_CTRL_HW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ppu_ctrl_hw to value 0"]
impl crate::Resettable for PPU_CTRL_HW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

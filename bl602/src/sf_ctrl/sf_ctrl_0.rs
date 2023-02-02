#[doc = "Register `sf_ctrl_0` reader"]
pub struct R(crate::R<SF_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_0` writer"]
pub struct W(crate::W<SF_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_0_SPEC>;
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
impl From<crate::W<SF_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_clk_sf_rx_inv_sel` reader - "]
pub type SF_CLK_SF_RX_INV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `sf_clk_sf_rx_inv_sel` writer - "]
pub type SF_CLK_SF_RX_INV_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_clk_out_gate_en` reader - "]
pub type SF_CLK_OUT_GATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_clk_out_gate_en` writer - "]
pub type SF_CLK_OUT_GATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_clk_out_inv_sel` reader - "]
pub type SF_CLK_OUT_INV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `sf_clk_out_inv_sel` writer - "]
pub type SF_CLK_OUT_INV_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_clk_sahb_sram_sel` reader - "]
pub type SF_CLK_SAHB_SRAM_SEL_R = crate::BitReader<bool>;
#[doc = "Field `sf_clk_sahb_sram_sel` writer - "]
pub type SF_CLK_SAHB_SRAM_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_if_read_dly_n` reader - "]
pub type SF_IF_READ_DLY_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if_read_dly_n` writer - "]
pub type SF_IF_READ_DLY_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_CTRL_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `sf_if_read_dly_en` reader - "]
pub type SF_IF_READ_DLY_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_read_dly_en` writer - "]
pub type SF_IF_READ_DLY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_if_int` reader - "]
pub type SF_IF_INT_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_int_clr` reader - "]
pub type SF_IF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_int_clr` writer - "]
pub type SF_IF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_if_int_set` reader - "]
pub type SF_IF_INT_SET_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_int_set` writer - "]
pub type SF_IF_INT_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_aes_dly_mode` reader - "]
pub type SF_AES_DLY_MODE_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_dly_mode` writer - "]
pub type SF_AES_DLY_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_aes_dout_endian` reader - "]
pub type SF_AES_DOUT_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_dout_endian` writer - "]
pub type SF_AES_DOUT_ENDIAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_aes_ctr_plus_en` reader - "]
pub type SF_AES_CTR_PLUS_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_ctr_plus_en` writer - "]
pub type SF_AES_CTR_PLUS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_aes_key_endian` reader - "]
pub type SF_AES_KEY_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_key_endian` writer - "]
pub type SF_AES_KEY_ENDIAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_aes_iv_endian` reader - "]
pub type SF_AES_IV_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_iv_endian` writer - "]
pub type SF_AES_IV_ENDIAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `sf_id` reader - "]
pub type SF_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_id` writer - "]
pub type SF_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SF_CTRL_0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_clk_sf_rx_inv_sel(&self) -> SF_CLK_SF_RX_INV_SEL_R {
        SF_CLK_SF_RX_INV_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_clk_out_gate_en(&self) -> SF_CLK_OUT_GATE_EN_R {
        SF_CLK_OUT_GATE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_clk_out_inv_sel(&self) -> SF_CLK_OUT_INV_SEL_R {
        SF_CLK_OUT_INV_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_clk_sahb_sram_sel(&self) -> SF_CLK_SAHB_SRAM_SEL_R {
        SF_CLK_SAHB_SRAM_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_if_read_dly_n(&self) -> SF_IF_READ_DLY_N_R {
        SF_IF_READ_DLY_N_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_if_read_dly_en(&self) -> SF_IF_READ_DLY_EN_R {
        SF_IF_READ_DLY_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if_int(&self) -> SF_IF_INT_R {
        SF_IF_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_int_clr(&self) -> SF_IF_INT_CLR_R {
        SF_IF_INT_CLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_int_set(&self) -> SF_IF_INT_SET_R {
        SF_IF_INT_SET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sf_aes_dly_mode(&self) -> SF_AES_DLY_MODE_R {
        SF_AES_DLY_MODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sf_aes_dout_endian(&self) -> SF_AES_DOUT_ENDIAN_R {
        SF_AES_DOUT_ENDIAN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sf_aes_ctr_plus_en(&self) -> SF_AES_CTR_PLUS_EN_R {
        SF_AES_CTR_PLUS_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sf_aes_key_endian(&self) -> SF_AES_KEY_ENDIAN_R {
        SF_AES_KEY_ENDIAN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_aes_iv_endian(&self) -> SF_AES_IV_ENDIAN_R {
        SF_AES_IV_ENDIAN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sf_id(&self) -> SF_ID_R {
        SF_ID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_sf_rx_inv_sel(&mut self) -> SF_CLK_SF_RX_INV_SEL_W<2> {
        SF_CLK_SF_RX_INV_SEL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_out_gate_en(&mut self) -> SF_CLK_OUT_GATE_EN_W<3> {
        SF_CLK_OUT_GATE_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_out_inv_sel(&mut self) -> SF_CLK_OUT_INV_SEL_W<4> {
        SF_CLK_OUT_INV_SEL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sf_clk_sahb_sram_sel(&mut self) -> SF_CLK_SAHB_SRAM_SEL_W<5> {
        SF_CLK_SAHB_SRAM_SEL_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_read_dly_n(&mut self) -> SF_IF_READ_DLY_N_W<8> {
        SF_IF_READ_DLY_N_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_read_dly_en(&mut self) -> SF_IF_READ_DLY_EN_W<11> {
        SF_IF_READ_DLY_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_int_clr(&mut self) -> SF_IF_INT_CLR_W<17> {
        SF_IF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_int_set(&mut self) -> SF_IF_INT_SET_W<18> {
        SF_IF_INT_SET_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_dly_mode(&mut self) -> SF_AES_DLY_MODE_W<19> {
        SF_AES_DLY_MODE_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_dout_endian(&mut self) -> SF_AES_DOUT_ENDIAN_W<20> {
        SF_AES_DOUT_ENDIAN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_ctr_plus_en(&mut self) -> SF_AES_CTR_PLUS_EN_W<21> {
        SF_AES_CTR_PLUS_EN_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_key_endian(&mut self) -> SF_AES_KEY_ENDIAN_W<22> {
        SF_AES_KEY_ENDIAN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_iv_endian(&mut self) -> SF_AES_IV_ENDIAN_W<23> {
        SF_AES_IV_ENDIAN_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn sf_id(&mut self) -> SF_ID_W<24> {
        SF_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_0](index.html) module"]
pub struct SF_CTRL_0_SPEC;
impl crate::RegisterSpec for SF_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_0::R](R) reader structure"]
impl crate::Readable for SF_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_0::W](W) writer structure"]
impl crate::Writable for SF_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_ctrl_0 to value 0x1ad2_001c"]
impl crate::Resettable for SF_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1ad2_001c;
}

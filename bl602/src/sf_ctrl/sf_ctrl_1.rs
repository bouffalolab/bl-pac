#[doc = "Register `sf_ctrl_1` reader"]
pub struct R(crate::R<SF_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_1` writer"]
pub struct W(crate::W<SF_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_1_SPEC>;
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
impl From<crate::W<SF_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_if_sr_pat_mask` reader - "]
pub type SF_IF_SR_PAT_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if_sr_pat_mask` writer - "]
pub type SF_IF_SR_PAT_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_CTRL_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `sf_if_sr_pat` reader - "]
pub type SF_IF_SR_PAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if_sr_pat` writer - "]
pub type SF_IF_SR_PAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_CTRL_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `sf_if_sr_int` reader - "]
pub type SF_IF_SR_INT_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_sr_int_en` reader - "]
pub type SF_IF_SR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_sr_int_en` writer - "]
pub type SF_IF_SR_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `sf_if_sr_int_set` reader - "]
pub type SF_IF_SR_INT_SET_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_sr_int_set` writer - "]
pub type SF_IF_SR_INT_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `sf_if_0_ack_lat` reader - "]
pub type SF_IF_0_ACK_LAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if_0_ack_lat` writer - "]
pub type SF_IF_0_ACK_LAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_CTRL_1_SPEC, u8, u8, 3, O>;
#[doc = "Field `sf_if_reg_hold` reader - "]
pub type SF_IF_REG_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_reg_hold` writer - "]
pub type SF_IF_REG_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `sf_if_reg_wp` reader - "]
pub type SF_IF_REG_WP_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_reg_wp` writer - "]
pub type SF_IF_REG_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `sf_ahb2sif_stopped` reader - "]
pub type SF_AHB2SIF_STOPPED_R = crate::BitReader<bool>;
#[doc = "Field `sf_ahb2sif_stop` reader - "]
pub type SF_AHB2SIF_STOP_R = crate::BitReader<bool>;
#[doc = "Field `sf_ahb2sif_stop` writer - "]
pub type SF_AHB2SIF_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `sf_if_fn_sel` reader - "]
pub type SF_IF_FN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_fn_sel` writer - "]
pub type SF_IF_FN_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `sf_if_en` reader - "]
pub type SF_IF_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_en` writer - "]
pub type SF_IF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `sf_ahb2sif_en` reader - "]
pub type SF_AHB2SIF_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_ahb2sif_en` writer - "]
pub type SF_AHB2SIF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `sf_ahb2sram_en` reader - "]
pub type SF_AHB2SRAM_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_ahb2sram_en` writer - "]
pub type SF_AHB2SRAM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sf_if_sr_pat_mask(&self) -> SF_IF_SR_PAT_MASK_R {
        SF_IF_SR_PAT_MASK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sf_if_sr_pat(&self) -> SF_IF_SR_PAT_R {
        SF_IF_SR_PAT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if_sr_int(&self) -> SF_IF_SR_INT_R {
        SF_IF_SR_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_sr_int_en(&self) -> SF_IF_SR_INT_EN_R {
        SF_IF_SR_INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_sr_int_set(&self) -> SF_IF_SR_INT_SET_R {
        SF_IF_SR_INT_SET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_ack_lat(&self) -> SF_IF_0_ACK_LAT_R {
        SF_IF_0_ACK_LAT_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_reg_hold(&self) -> SF_IF_REG_HOLD_R {
        SF_IF_REG_HOLD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_reg_wp(&self) -> SF_IF_REG_WP_R {
        SF_IF_REG_WP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_ahb2sif_stopped(&self) -> SF_AHB2SIF_STOPPED_R {
        SF_AHB2SIF_STOPPED_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_ahb2sif_stop(&self) -> SF_AHB2SIF_STOP_R {
        SF_AHB2SIF_STOP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if_fn_sel(&self) -> SF_IF_FN_SEL_R {
        SF_IF_FN_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if_en(&self) -> SF_IF_EN_R {
        SF_IF_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_ahb2sif_en(&self) -> SF_AHB2SIF_EN_R {
        SF_AHB2SIF_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_ahb2sram_en(&self) -> SF_AHB2SRAM_EN_R {
        SF_AHB2SRAM_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_sr_pat_mask(&mut self) -> SF_IF_SR_PAT_MASK_W<0> {
        SF_IF_SR_PAT_MASK_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_sr_pat(&mut self) -> SF_IF_SR_PAT_W<8> {
        SF_IF_SR_PAT_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_sr_int_en(&mut self) -> SF_IF_SR_INT_EN_W<17> {
        SF_IF_SR_INT_EN_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_sr_int_set(&mut self) -> SF_IF_SR_INT_SET_W<18> {
        SF_IF_SR_INT_SET_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_0_ack_lat(&mut self) -> SF_IF_0_ACK_LAT_W<20> {
        SF_IF_0_ACK_LAT_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_reg_hold(&mut self) -> SF_IF_REG_HOLD_W<24> {
        SF_IF_REG_HOLD_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_reg_wp(&mut self) -> SF_IF_REG_WP_W<25> {
        SF_IF_REG_WP_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn sf_ahb2sif_stop(&mut self) -> SF_AHB2SIF_STOP_W<27> {
        SF_AHB2SIF_STOP_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_fn_sel(&mut self) -> SF_IF_FN_SEL_W<28> {
        SF_IF_FN_SEL_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_en(&mut self) -> SF_IF_EN_W<29> {
        SF_IF_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn sf_ahb2sif_en(&mut self) -> SF_AHB2SIF_EN_W<30> {
        SF_AHB2SIF_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn sf_ahb2sram_en(&mut self) -> SF_AHB2SRAM_EN_W<31> {
        SF_AHB2SRAM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_1](index.html) module"]
pub struct SF_CTRL_1_SPEC;
impl crate::RegisterSpec for SF_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_1::R](R) reader structure"]
impl crate::Readable for SF_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_1::W](W) writer structure"]
impl crate::Writable for SF_CTRL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_ctrl_1 to value 0xf360_0000"]
impl crate::Resettable for SF_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xf360_0000;
}

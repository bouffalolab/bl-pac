#[doc = "Register `lo_reg_ctrl_hw1` reader"]
pub struct R(crate::R<LO_REG_CTRL_HW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_REG_CTRL_HW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_REG_CTRL_HW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_REG_CTRL_HW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_reg_ctrl_hw1` writer"]
pub struct W(crate::W<LO_REG_CTRL_HW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_REG_CTRL_HW1_SPEC>;
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
impl From<crate::W<LO_REG_CTRL_HW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_REG_CTRL_HW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_fbdv_halfstep_en_rx` reader - "]
pub type LO_FBDV_HALFSTEP_EN_RX_R = crate::BitReader<bool>;
#[doc = "Field `lo_fbdv_halfstep_en_rx` writer - "]
pub type LO_FBDV_HALFSTEP_EN_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LO_REG_CTRL_HW1_SPEC, bool, O>;
#[doc = "Field `lo_fbdv_halfstep_en_tx` reader - "]
pub type LO_FBDV_HALFSTEP_EN_TX_R = crate::BitReader<bool>;
#[doc = "Field `lo_fbdv_halfstep_en_tx` writer - "]
pub type LO_FBDV_HALFSTEP_EN_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LO_REG_CTRL_HW1_SPEC, bool, O>;
#[doc = "Field `lo_cp_sel_rx` reader - "]
pub type LO_CP_SEL_RX_R = crate::BitReader<bool>;
#[doc = "Field `lo_cp_sel_rx` writer - "]
pub type LO_CP_SEL_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, LO_REG_CTRL_HW1_SPEC, bool, O>;
#[doc = "Field `lo_cp_sel_tx` reader - "]
pub type LO_CP_SEL_TX_R = crate::BitReader<bool>;
#[doc = "Field `lo_cp_sel_tx` writer - "]
pub type LO_CP_SEL_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, LO_REG_CTRL_HW1_SPEC, bool, O>;
#[doc = "Field `lo_lf_cz_rx` reader - "]
pub type LO_LF_CZ_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_cz_rx` writer - "]
pub type LO_LF_CZ_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_REG_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_cz_tx` reader - "]
pub type LO_LF_CZ_TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_cz_tx` writer - "]
pub type LO_LF_CZ_TX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_REG_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_rz_rx` reader - "]
pub type LO_LF_RZ_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_rz_rx` writer - "]
pub type LO_LF_RZ_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_REG_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_rz_tx` reader - "]
pub type LO_LF_RZ_TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_rz_tx` writer - "]
pub type LO_LF_RZ_TX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_REG_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_r4_rx` reader - "]
pub type LO_LF_R4_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_r4_rx` writer - "]
pub type LO_LF_R4_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_REG_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_r4_tx` reader - "]
pub type LO_LF_R4_TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_r4_tx` writer - "]
pub type LO_LF_R4_TX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_REG_CTRL_HW1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_rx(&self) -> LO_FBDV_HALFSTEP_EN_RX_R {
        LO_FBDV_HALFSTEP_EN_RX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_tx(&self) -> LO_FBDV_HALFSTEP_EN_TX_R {
        LO_FBDV_HALFSTEP_EN_TX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lo_cp_sel_rx(&self) -> LO_CP_SEL_RX_R {
        LO_CP_SEL_RX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lo_cp_sel_tx(&self) -> LO_CP_SEL_TX_R {
        LO_CP_SEL_TX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_cz_rx(&self) -> LO_LF_CZ_RX_R {
        LO_LF_CZ_RX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_tx(&self) -> LO_LF_CZ_TX_R {
        LO_LF_CZ_TX_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz_rx(&self) -> LO_LF_RZ_RX_R {
        LO_LF_RZ_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_rz_tx(&self) -> LO_LF_RZ_TX_R {
        LO_LF_RZ_TX_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_lf_r4_rx(&self) -> LO_LF_R4_RX_R {
        LO_LF_R4_RX_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_lf_r4_tx(&self) -> LO_LF_R4_TX_R {
        LO_LF_R4_TX_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lo_fbdv_halfstep_en_rx(&mut self) -> LO_FBDV_HALFSTEP_EN_RX_W<0> {
        LO_FBDV_HALFSTEP_EN_RX_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn lo_fbdv_halfstep_en_tx(&mut self) -> LO_FBDV_HALFSTEP_EN_TX_W<1> {
        LO_FBDV_HALFSTEP_EN_TX_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn lo_cp_sel_rx(&mut self) -> LO_CP_SEL_RX_W<2> {
        LO_CP_SEL_RX_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn lo_cp_sel_tx(&mut self) -> LO_CP_SEL_TX_W<3> {
        LO_CP_SEL_TX_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_cz_rx(&mut self) -> LO_LF_CZ_RX_W<4> {
        LO_LF_CZ_RX_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_cz_tx(&mut self) -> LO_LF_CZ_TX_W<8> {
        LO_LF_CZ_TX_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_rz_rx(&mut self) -> LO_LF_RZ_RX_W<12> {
        LO_LF_RZ_RX_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_rz_tx(&mut self) -> LO_LF_RZ_TX_W<16> {
        LO_LF_RZ_TX_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_r4_rx(&mut self) -> LO_LF_R4_RX_W<20> {
        LO_LF_R4_RX_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_r4_tx(&mut self) -> LO_LF_R4_TX_W<24> {
        LO_LF_R4_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_reg_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_reg_ctrl_hw1](index.html) module"]
pub struct LO_REG_CTRL_HW1_SPEC;
impl crate::RegisterSpec for LO_REG_CTRL_HW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_reg_ctrl_hw1::R](R) reader structure"]
impl crate::Readable for LO_REG_CTRL_HW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_reg_ctrl_hw1::W](W) writer structure"]
impl crate::Writable for LO_REG_CTRL_HW1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo_reg_ctrl_hw1 to value 0"]
impl crate::Resettable for LO_REG_CTRL_HW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `rbb3` reader"]
pub struct R(crate::R<RBB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb3` writer"]
pub struct W(crate::W<RBB3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB3_SPEC>;
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
impl From<crate::W<RBB3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rbb_bt_mode_hw` reader - "]
pub type RBB_BT_MODE_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bt_mode_hw` writer - "]
pub type RBB_BT_MODE_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB3_SPEC, bool, O>;
#[doc = "Field `rbb_bt_mode` reader - "]
pub type RBB_BT_MODE_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bt_mode` writer - "]
pub type RBB_BT_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB3_SPEC, bool, O>;
#[doc = "Field `rbb_bt_fif_tune` reader - "]
pub type RBB_BT_FIF_TUNE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_bt_fif_tune` writer - "]
pub type RBB_BT_FIF_TUNE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB3_SPEC, u8, u8, 2, O>;
#[doc = "Field `rbb_deq` reader - "]
pub type RBB_DEQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_deq` writer - "]
pub type RBB_DEQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB3_SPEC, u8, u8, 2, O>;
#[doc = "Field `rbb_bm_op` reader - "]
pub type RBB_BM_OP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_bm_op` writer - "]
pub type RBB_BM_OP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB3_SPEC, u8, u8, 3, O>;
#[doc = "Field `rbb_vcm` reader - "]
pub type RBB_VCM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_vcm` writer - "]
pub type RBB_VCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB3_SPEC, u8, u8, 2, O>;
#[doc = "Field `rbb_bq_iqbias_short` reader - "]
pub type RBB_BQ_IQBIAS_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bq_iqbias_short` writer - "]
pub type RBB_BQ_IQBIAS_SHORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB3_SPEC, bool, O>;
#[doc = "Field `rbb_tia_iqbias_short` reader - "]
pub type RBB_TIA_IQBIAS_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `rbb_tia_iqbias_short` writer - "]
pub type RBB_TIA_IQBIAS_SHORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB3_SPEC, bool, O>;
#[doc = "Field `rbb_bw` reader - "]
pub type RBB_BW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_bw` writer - "]
pub type RBB_BW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB3_SPEC, u8, u8, 2, O>;
#[doc = "Field `rxiqcal_en` reader - "]
pub type RXIQCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `rxiqcal_en` writer - "]
pub type RXIQCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB3_SPEC, bool, O>;
#[doc = "Field `pwr_det_en` reader - "]
pub type PWR_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `pwr_det_en` writer - "]
pub type PWR_DET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_hw(&self) -> RBB_BT_MODE_HW_R {
        RBB_BT_MODE_HW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bt_mode(&self) -> RBB_BT_MODE_R {
        RBB_BT_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rbb_bt_fif_tune(&self) -> RBB_BT_FIF_TUNE_R {
        RBB_BT_FIF_TUNE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rbb_deq(&self) -> RBB_DEQ_R {
        RBB_DEQ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rbb_bm_op(&self) -> RBB_BM_OP_R {
        RBB_BM_OP_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rbb_vcm(&self) -> RBB_VCM_R {
        RBB_VCM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rbb_bq_iqbias_short(&self) -> RBB_BQ_IQBIAS_SHORT_R {
        RBB_BQ_IQBIAS_SHORT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rbb_tia_iqbias_short(&self) -> RBB_TIA_IQBIAS_SHORT_R {
        RBB_TIA_IQBIAS_SHORT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rbb_bw(&self) -> RBB_BW_R {
        RBB_BW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rxiqcal_en(&self) -> RXIQCAL_EN_R {
        RXIQCAL_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pwr_det_en(&self) -> PWR_DET_EN_R {
        PWR_DET_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bt_mode_hw(&mut self) -> RBB_BT_MODE_HW_W<0> {
        RBB_BT_MODE_HW_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bt_mode(&mut self) -> RBB_BT_MODE_W<4> {
        RBB_BT_MODE_W::new(self)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bt_fif_tune(&mut self) -> RBB_BT_FIF_TUNE_W<5> {
        RBB_BT_FIF_TUNE_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_deq(&mut self) -> RBB_DEQ_W<8> {
        RBB_DEQ_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bm_op(&mut self) -> RBB_BM_OP_W<12> {
        RBB_BM_OP_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_vcm(&mut self) -> RBB_VCM_W<16> {
        RBB_VCM_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bq_iqbias_short(&mut self) -> RBB_BQ_IQBIAS_SHORT_W<20> {
        RBB_BQ_IQBIAS_SHORT_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_tia_iqbias_short(&mut self) -> RBB_TIA_IQBIAS_SHORT_W<21> {
        RBB_TIA_IQBIAS_SHORT_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bw(&mut self) -> RBB_BW_W<24> {
        RBB_BW_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn rxiqcal_en(&mut self) -> RXIQCAL_EN_W<28> {
        RXIQCAL_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_det_en(&mut self) -> PWR_DET_EN_W<31> {
        PWR_DET_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb3](index.html) module"]
pub struct RBB3_SPEC;
impl crate::RegisterSpec for RBB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb3::R](R) reader structure"]
impl crate::Readable for RBB3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb3::W](W) writer structure"]
impl crate::Writable for RBB3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rbb3 to value 0"]
impl crate::Resettable for RBB3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

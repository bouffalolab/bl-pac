#[doc = "Register `rfctrl_hw_en` reader"]
pub struct R(crate::R<RFCTRL_HW_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCTRL_HW_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCTRL_HW_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCTRL_HW_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfctrl_hw_en` writer"]
pub struct W(crate::W<RFCTRL_HW_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCTRL_HW_EN_SPEC>;
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
impl From<crate::W<RFCTRL_HW_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCTRL_HW_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_ctrl_hw` reader - "]
pub type PU_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_ctrl_hw` writer - "]
pub type PU_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `rx_gain_ctrl_hw` reader - "]
pub type RX_GAIN_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rx_gain_ctrl_hw` writer - "]
pub type RX_GAIN_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `tx_gain_ctrl_hw` reader - "]
pub type TX_GAIN_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `tx_gain_ctrl_hw` writer - "]
pub type TX_GAIN_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `lna_ctrl_hw` reader - "]
pub type LNA_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `lna_ctrl_hw` writer - "]
pub type LNA_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `rbb_bw_ctrl_hw` reader - "]
pub type RBB_BW_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bw_ctrl_hw` writer - "]
pub type RBB_BW_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `trxcal_ctrl_hw` reader - "]
pub type TRXCAL_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `trxcal_ctrl_hw` writer - "]
pub type TRXCAL_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `lo_ctrl_hw` reader - "]
pub type LO_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `lo_ctrl_hw` writer - "]
pub type LO_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `inc_acal_ctrl_en_hw` reader - "]
pub type INC_ACAL_CTRL_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `inc_acal_ctrl_en_hw` writer - "]
pub type INC_ACAL_CTRL_EN_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `inc_fcal_ctrl_en_hw` reader - "]
pub type INC_FCAL_CTRL_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `inc_fcal_ctrl_en_hw` writer - "]
pub type INC_FCAL_CTRL_EN_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `sdm_ctrl_hw` reader - "]
pub type SDM_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `sdm_ctrl_hw` writer - "]
pub type SDM_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_en_ctrl_hw` reader - "]
pub type RBB_PKDET_EN_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_en_ctrl_hw` writer - "]
pub type RBB_PKDET_EN_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_out_rstn_ctrl_hw` reader - "]
pub type RBB_PKDET_OUT_RSTN_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_out_rstn_ctrl_hw` writer - "]
pub type RBB_PKDET_OUT_RSTN_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
#[doc = "Field `adda_ctrl_hw` reader - "]
pub type ADDA_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `adda_ctrl_hw` writer - "]
pub type ADDA_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCTRL_HW_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ctrl_hw(&self) -> PU_CTRL_HW_R {
        PU_CTRL_HW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_gain_ctrl_hw(&self) -> RX_GAIN_CTRL_HW_R {
        RX_GAIN_CTRL_HW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_gain_ctrl_hw(&self) -> TX_GAIN_CTRL_HW_R {
        TX_GAIN_CTRL_HW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lna_ctrl_hw(&self) -> LNA_CTRL_HW_R {
        LNA_CTRL_HW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bw_ctrl_hw(&self) -> RBB_BW_CTRL_HW_R {
        RBB_BW_CTRL_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trxcal_ctrl_hw(&self) -> TRXCAL_CTRL_HW_R {
        TRXCAL_CTRL_HW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_ctrl_hw(&self) -> LO_CTRL_HW_R {
        LO_CTRL_HW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inc_acal_ctrl_en_hw(&self) -> INC_ACAL_CTRL_EN_HW_R {
        INC_ACAL_CTRL_EN_HW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inc_fcal_ctrl_en_hw(&self) -> INC_FCAL_CTRL_EN_HW_R {
        INC_FCAL_CTRL_EN_HW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdm_ctrl_hw(&self) -> SDM_CTRL_HW_R {
        SDM_CTRL_HW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rbb_pkdet_en_ctrl_hw(&self) -> RBB_PKDET_EN_CTRL_HW_R {
        RBB_PKDET_EN_CTRL_HW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&self) -> RBB_PKDET_OUT_RSTN_CTRL_HW_R {
        RBB_PKDET_OUT_RSTN_CTRL_HW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adda_ctrl_hw(&self) -> ADDA_CTRL_HW_R {
        ADDA_CTRL_HW_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ctrl_hw(&mut self) -> PU_CTRL_HW_W<0> {
        PU_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_gain_ctrl_hw(&mut self) -> RX_GAIN_CTRL_HW_W<1> {
        RX_GAIN_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_gain_ctrl_hw(&mut self) -> TX_GAIN_CTRL_HW_W<2> {
        TX_GAIN_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn lna_ctrl_hw(&mut self) -> LNA_CTRL_HW_W<3> {
        LNA_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bw_ctrl_hw(&mut self) -> RBB_BW_CTRL_HW_W<4> {
        RBB_BW_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn trxcal_ctrl_hw(&mut self) -> TRXCAL_CTRL_HW_W<5> {
        TRXCAL_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn lo_ctrl_hw(&mut self) -> LO_CTRL_HW_W<6> {
        LO_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn inc_acal_ctrl_en_hw(&mut self) -> INC_ACAL_CTRL_EN_HW_W<7> {
        INC_ACAL_CTRL_EN_HW_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn inc_fcal_ctrl_en_hw(&mut self) -> INC_FCAL_CTRL_EN_HW_W<8> {
        INC_FCAL_CTRL_EN_HW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sdm_ctrl_hw(&mut self) -> SDM_CTRL_HW_W<9> {
        SDM_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_en_ctrl_hw(&mut self) -> RBB_PKDET_EN_CTRL_HW_W<10> {
        RBB_PKDET_EN_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&mut self) -> RBB_PKDET_OUT_RSTN_CTRL_HW_W<11> {
        RBB_PKDET_OUT_RSTN_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn adda_ctrl_hw(&mut self) -> ADDA_CTRL_HW_W<12> {
        ADDA_CTRL_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control logic switch\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfctrl_hw_en](index.html) module"]
pub struct RFCTRL_HW_EN_SPEC;
impl crate::RegisterSpec for RFCTRL_HW_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfctrl_hw_en::R](R) reader structure"]
impl crate::Readable for RFCTRL_HW_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfctrl_hw_en::W](W) writer structure"]
impl crate::Writable for RFCTRL_HW_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rfctrl_hw_en to value 0"]
impl crate::Resettable for RFCTRL_HW_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `ef_if_ctrl_0` reader"]
pub struct R(crate::R<EF_IF_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_IF_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_IF_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_ctrl_0` writer"]
pub struct W(crate::W<EF_IF_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_CTRL_0_SPEC>;
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
impl From<crate::W<EF_IF_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_IF_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_if_0_autoload_p1_done` reader - "]
pub type EF_IF_0_AUTOLOAD_P1_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_autoload_done` reader - "]
pub type EF_IF_0_AUTOLOAD_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_busy` reader - "]
pub type EF_IF_0_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_rw` reader - "]
pub type EF_IF_0_RW_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_rw` writer - "]
pub type EF_IF_0_RW_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_if_0_trig` reader - "]
pub type EF_IF_0_TRIG_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_trig` writer - "]
pub type EF_IF_0_TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_if_0_manual_en` reader - "]
pub type EF_IF_0_MANUAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_manual_en` writer - "]
pub type EF_IF_0_MANUAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_if_0_cyc_modify` reader - "]
pub type EF_IF_0_CYC_MODIFY_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_cyc_modify` writer - "]
pub type EF_IF_0_CYC_MODIFY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_clk_sahb_data_sel` reader - "]
pub type EF_CLK_SAHB_DATA_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ef_clk_sahb_data_sel` writer - "]
pub type EF_CLK_SAHB_DATA_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_if_prot_code_ctrl` reader - "]
pub type EF_IF_PROT_CODE_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_prot_code_ctrl` writer - "]
pub type EF_IF_PROT_CODE_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CTRL_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `ef_if_por_dig` reader - "]
pub type EF_IF_POR_DIG_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_por_dig` writer - "]
pub type EF_IF_POR_DIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_clk_sahb_data_gate` reader - "]
pub type EF_CLK_SAHB_DATA_GATE_R = crate::BitReader<bool>;
#[doc = "Field `ef_clk_sahb_data_gate` writer - "]
pub type EF_CLK_SAHB_DATA_GATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_if_auto_rd_en` reader - "]
pub type EF_IF_AUTO_RD_EN_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_auto_rd_en` writer - "]
pub type EF_IF_AUTO_RD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_if_cyc_modify_lock` reader - "]
pub type EF_IF_CYC_MODIFY_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_cyc_modify_lock` writer - "]
pub type EF_IF_CYC_MODIFY_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_if_0_int` reader - "]
pub type EF_IF_0_INT_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_int_clr` reader - "]
pub type EF_IF_0_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_int_clr` writer - "]
pub type EF_IF_0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_if_0_int_set` reader - "]
pub type EF_IF_0_INT_SET_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_int_set` writer - "]
pub type EF_IF_0_INT_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_if_prot_code_cyc` reader - "]
pub type EF_IF_PROT_CODE_CYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_prot_code_cyc` writer - "]
pub type EF_IF_PROT_CODE_CYC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_IF_CTRL_0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ef_if_0_autoload_p1_done(&self) -> EF_IF_0_AUTOLOAD_P1_DONE_R {
        EF_IF_0_AUTOLOAD_P1_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_if_0_autoload_done(&self) -> EF_IF_0_AUTOLOAD_DONE_R {
        EF_IF_0_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_if_0_busy(&self) -> EF_IF_0_BUSY_R {
        EF_IF_0_BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_if_0_rw(&self) -> EF_IF_0_RW_R {
        EF_IF_0_RW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_if_0_trig(&self) -> EF_IF_0_TRIG_R {
        EF_IF_0_TRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_if_0_manual_en(&self) -> EF_IF_0_MANUAL_EN_R {
        EF_IF_0_MANUAL_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_0_cyc_modify(&self) -> EF_IF_0_CYC_MODIFY_R {
        EF_IF_0_CYC_MODIFY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_sel(&self) -> EF_CLK_SAHB_DATA_SEL_R {
        EF_CLK_SAHB_DATA_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ef_if_prot_code_ctrl(&self) -> EF_IF_PROT_CODE_CTRL_R {
        EF_IF_PROT_CODE_CTRL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_por_dig(&self) -> EF_IF_POR_DIG_R {
        EF_IF_POR_DIG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_gate(&self) -> EF_CLK_SAHB_DATA_GATE_R {
        EF_CLK_SAHB_DATA_GATE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_auto_rd_en(&self) -> EF_IF_AUTO_RD_EN_R {
        EF_IF_AUTO_RD_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_cyc_modify_lock(&self) -> EF_IF_CYC_MODIFY_LOCK_R {
        EF_IF_CYC_MODIFY_LOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_0_int(&self) -> EF_IF_0_INT_R {
        EF_IF_0_INT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_0_int_clr(&self) -> EF_IF_0_INT_CLR_R {
        EF_IF_0_INT_CLR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_0_int_set(&self) -> EF_IF_0_INT_SET_R {
        EF_IF_0_INT_SET_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_cyc(&self) -> EF_IF_PROT_CODE_CYC_R {
        EF_IF_PROT_CODE_CYC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_0_rw(&mut self) -> EF_IF_0_RW_W<3> {
        EF_IF_0_RW_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_0_trig(&mut self) -> EF_IF_0_TRIG_W<4> {
        EF_IF_0_TRIG_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_0_manual_en(&mut self) -> EF_IF_0_MANUAL_EN_W<5> {
        EF_IF_0_MANUAL_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_0_cyc_modify(&mut self) -> EF_IF_0_CYC_MODIFY_W<6> {
        EF_IF_0_CYC_MODIFY_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ef_clk_sahb_data_sel(&mut self) -> EF_CLK_SAHB_DATA_SEL_W<7> {
        EF_CLK_SAHB_DATA_SEL_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_prot_code_ctrl(&mut self) -> EF_IF_PROT_CODE_CTRL_W<8> {
        EF_IF_PROT_CODE_CTRL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_por_dig(&mut self) -> EF_IF_POR_DIG_W<16> {
        EF_IF_POR_DIG_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ef_clk_sahb_data_gate(&mut self) -> EF_CLK_SAHB_DATA_GATE_W<17> {
        EF_CLK_SAHB_DATA_GATE_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_auto_rd_en(&mut self) -> EF_IF_AUTO_RD_EN_W<18> {
        EF_IF_AUTO_RD_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_cyc_modify_lock(&mut self) -> EF_IF_CYC_MODIFY_LOCK_W<19> {
        EF_IF_CYC_MODIFY_LOCK_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_0_int_clr(&mut self) -> EF_IF_0_INT_CLR_W<21> {
        EF_IF_0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_0_int_set(&mut self) -> EF_IF_0_INT_SET_W<22> {
        EF_IF_0_INT_SET_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_prot_code_cyc(&mut self) -> EF_IF_PROT_CODE_CYC_W<24> {
        EF_IF_PROT_CODE_CYC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_ctrl_0](index.html) module"]
pub struct EF_IF_CTRL_0_SPEC;
impl crate::RegisterSpec for EF_IF_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_ctrl_0::R](R) reader structure"]
impl crate::Readable for EF_IF_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_ctrl_0::W](W) writer structure"]
impl crate::Writable for EF_IF_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_if_ctrl_0 to value 0x0024_0003"]
impl crate::Resettable for EF_IF_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0024_0003;
}

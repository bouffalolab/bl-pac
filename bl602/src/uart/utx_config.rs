#[doc = "Register `utx_config` reader"]
pub struct R(crate::R<UTX_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UTX_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UTX_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UTX_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `utx_config` writer"]
pub struct W(crate::W<UTX_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UTX_CONFIG_SPEC>;
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
impl From<crate::W<UTX_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UTX_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_utx_en` reader - "]
pub type CR_UTX_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_en` writer - "]
pub type CR_UTX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_utx_cts_en` reader - "]
pub type CR_UTX_CTS_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_cts_en` writer - "]
pub type CR_UTX_CTS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_utx_frm_en` reader - "]
pub type CR_UTX_FRM_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_frm_en` writer - "]
pub type CR_UTX_FRM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_utx_prt_en` reader - "]
pub type CR_UTX_PRT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_prt_en` writer - "]
pub type CR_UTX_PRT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_utx_prt_sel` reader - "]
pub type CR_UTX_PRT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_prt_sel` writer - "]
pub type CR_UTX_PRT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, UTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_utx_ir_en` reader - "]
pub type CR_UTX_IR_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_ir_en` writer - "]
pub type CR_UTX_IR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_utx_ir_inv` reader - "]
pub type CR_UTX_IR_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_ir_inv` writer - "]
pub type CR_UTX_IR_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, UTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_utx_bit_cnt_d` reader - "]
pub type CR_UTX_BIT_CNT_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_utx_bit_cnt_d` writer - "]
pub type CR_UTX_BIT_CNT_D_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UTX_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `cr_utx_bit_cnt_p` reader - "]
pub type CR_UTX_BIT_CNT_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_utx_bit_cnt_p` writer - "]
pub type CR_UTX_BIT_CNT_P_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UTX_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_utx_len` reader - "]
pub type CR_UTX_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_utx_len` writer - "]
pub type CR_UTX_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UTX_CONFIG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_en(&self) -> CR_UTX_EN_R {
        CR_UTX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_cts_en(&self) -> CR_UTX_CTS_EN_R {
        CR_UTX_CTS_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frm_en(&self) -> CR_UTX_FRM_EN_R {
        CR_UTX_FRM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_utx_prt_en(&self) -> CR_UTX_PRT_EN_R {
        CR_UTX_PRT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_utx_prt_sel(&self) -> CR_UTX_PRT_SEL_R {
        CR_UTX_PRT_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_ir_en(&self) -> CR_UTX_IR_EN_R {
        CR_UTX_IR_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_utx_ir_inv(&self) -> CR_UTX_IR_INV_R {
        CR_UTX_IR_INV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_d(&self) -> CR_UTX_BIT_CNT_D_R {
        CR_UTX_BIT_CNT_D_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_p(&self) -> CR_UTX_BIT_CNT_P_R {
        CR_UTX_BIT_CNT_P_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_len(&self) -> CR_UTX_LEN_R {
        CR_UTX_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_en(&mut self) -> CR_UTX_EN_W<0> {
        CR_UTX_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_cts_en(&mut self) -> CR_UTX_CTS_EN_W<1> {
        CR_UTX_CTS_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_frm_en(&mut self) -> CR_UTX_FRM_EN_W<2> {
        CR_UTX_FRM_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_prt_en(&mut self) -> CR_UTX_PRT_EN_W<4> {
        CR_UTX_PRT_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_prt_sel(&mut self) -> CR_UTX_PRT_SEL_W<5> {
        CR_UTX_PRT_SEL_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_ir_en(&mut self) -> CR_UTX_IR_EN_W<6> {
        CR_UTX_IR_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_ir_inv(&mut self) -> CR_UTX_IR_INV_W<7> {
        CR_UTX_IR_INV_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_bit_cnt_d(&mut self) -> CR_UTX_BIT_CNT_D_W<8> {
        CR_UTX_BIT_CNT_D_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_bit_cnt_p(&mut self) -> CR_UTX_BIT_CNT_P_W<12> {
        CR_UTX_BIT_CNT_P_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_len(&mut self) -> CR_UTX_LEN_W<16> {
        CR_UTX_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "utx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utx_config](index.html) module"]
pub struct UTX_CONFIG_SPEC;
impl crate::RegisterSpec for UTX_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [utx_config::R](R) reader structure"]
impl crate::Readable for UTX_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [utx_config::W](W) writer structure"]
impl crate::Writable for UTX_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets utx_config to value 0x1700"]
impl crate::Resettable for UTX_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x1700;
}

#[doc = "Register `urx_config` reader"]
pub struct R(crate::R<URX_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<URX_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<URX_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<URX_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `urx_config` writer"]
pub struct W(crate::W<URX_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<URX_CONFIG_SPEC>;
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
impl From<crate::W<URX_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<URX_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_urx_en` reader - "]
pub type CR_URX_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_en` writer - "]
pub type CR_URX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, URX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_urx_rts_sw_mode` reader - "]
pub type CR_URX_RTS_SW_MODE_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_rts_sw_mode` writer - "]
pub type CR_URX_RTS_SW_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, URX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_urx_rts_sw_val` reader - "]
pub type CR_URX_RTS_SW_VAL_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_rts_sw_val` writer - "]
pub type CR_URX_RTS_SW_VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, URX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_urx_abr_en` reader - "]
pub type CR_URX_ABR_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_abr_en` writer - "]
pub type CR_URX_ABR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, URX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_urx_prt_en` reader - "]
pub type CR_URX_PRT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_prt_en` writer - "]
pub type CR_URX_PRT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, URX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_urx_prt_sel` reader - "]
pub type CR_URX_PRT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_prt_sel` writer - "]
pub type CR_URX_PRT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, URX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_urx_ir_en` reader - "]
pub type CR_URX_IR_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_ir_en` writer - "]
pub type CR_URX_IR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, URX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_urx_ir_inv` reader - "]
pub type CR_URX_IR_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_ir_inv` writer - "]
pub type CR_URX_IR_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, URX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_urx_bit_cnt_d` reader - "]
pub type CR_URX_BIT_CNT_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_urx_bit_cnt_d` writer - "]
pub type CR_URX_BIT_CNT_D_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, URX_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `cr_urx_deg_en` reader - "]
pub type CR_URX_DEG_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_deg_en` writer - "]
pub type CR_URX_DEG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, URX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_urx_deg_cnt` reader - "]
pub type CR_URX_DEG_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_urx_deg_cnt` writer - "]
pub type CR_URX_DEG_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, URX_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_urx_len` reader - "]
pub type CR_URX_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_urx_len` writer - "]
pub type CR_URX_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, URX_CONFIG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_urx_en(&self) -> CR_URX_EN_R {
        CR_URX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_mode(&self) -> CR_URX_RTS_SW_MODE_R {
        CR_URX_RTS_SW_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_val(&self) -> CR_URX_RTS_SW_VAL_R {
        CR_URX_RTS_SW_VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_abr_en(&self) -> CR_URX_ABR_EN_R {
        CR_URX_ABR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_prt_en(&self) -> CR_URX_PRT_EN_R {
        CR_URX_PRT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_prt_sel(&self) -> CR_URX_PRT_SEL_R {
        CR_URX_PRT_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_urx_ir_en(&self) -> CR_URX_IR_EN_R {
        CR_URX_IR_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_ir_inv(&self) -> CR_URX_IR_INV_R {
        CR_URX_IR_INV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_urx_bit_cnt_d(&self) -> CR_URX_BIT_CNT_D_R {
        CR_URX_BIT_CNT_D_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_deg_en(&self) -> CR_URX_DEG_EN_R {
        CR_URX_DEG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_urx_deg_cnt(&self) -> CR_URX_DEG_CNT_R {
        CR_URX_DEG_CNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_urx_len(&self) -> CR_URX_LEN_R {
        CR_URX_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_en(&mut self) -> CR_URX_EN_W<0> {
        CR_URX_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_rts_sw_mode(&mut self) -> CR_URX_RTS_SW_MODE_W<1> {
        CR_URX_RTS_SW_MODE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_rts_sw_val(&mut self) -> CR_URX_RTS_SW_VAL_W<2> {
        CR_URX_RTS_SW_VAL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_abr_en(&mut self) -> CR_URX_ABR_EN_W<3> {
        CR_URX_ABR_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_prt_en(&mut self) -> CR_URX_PRT_EN_W<4> {
        CR_URX_PRT_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_prt_sel(&mut self) -> CR_URX_PRT_SEL_W<5> {
        CR_URX_PRT_SEL_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_ir_en(&mut self) -> CR_URX_IR_EN_W<6> {
        CR_URX_IR_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_ir_inv(&mut self) -> CR_URX_IR_INV_W<7> {
        CR_URX_IR_INV_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_bit_cnt_d(&mut self) -> CR_URX_BIT_CNT_D_W<8> {
        CR_URX_BIT_CNT_D_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_deg_en(&mut self) -> CR_URX_DEG_EN_W<11> {
        CR_URX_DEG_EN_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_deg_cnt(&mut self) -> CR_URX_DEG_CNT_W<12> {
        CR_URX_DEG_CNT_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_len(&mut self) -> CR_URX_LEN_W<16> {
        CR_URX_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "urx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [urx_config](index.html) module"]
pub struct URX_CONFIG_SPEC;
impl crate::RegisterSpec for URX_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [urx_config::R](R) reader structure"]
impl crate::Readable for URX_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [urx_config::W](W) writer structure"]
impl crate::Writable for URX_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets urx_config to value 0x0700"]
impl crate::Resettable for URX_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700;
}

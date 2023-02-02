#[doc = "Register `irtx_config` reader"]
pub struct R(crate::R<IRTX_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRTX_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRTX_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRTX_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irtx_config` writer"]
pub struct W(crate::W<IRTX_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRTX_CONFIG_SPEC>;
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
impl From<crate::W<IRTX_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRTX_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irtx_en` reader - "]
pub type CR_IRTX_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_en` writer - "]
pub type CR_IRTX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_out_inv` reader - "]
pub type CR_IRTX_OUT_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_out_inv` writer - "]
pub type CR_IRTX_OUT_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_mod_en` reader - "]
pub type CR_IRTX_MOD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_mod_en` writer - "]
pub type CR_IRTX_MOD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_swm_en` reader - "]
pub type CR_IRTX_SWM_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_swm_en` writer - "]
pub type CR_IRTX_SWM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_data_en` reader - "]
pub type CR_IRTX_DATA_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_data_en` writer - "]
pub type CR_IRTX_DATA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_logic0_hl_inv` reader - "]
pub type CR_IRTX_LOGIC0_HL_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_logic0_hl_inv` writer - "]
pub type CR_IRTX_LOGIC0_HL_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_logic1_hl_inv` reader - "]
pub type CR_IRTX_LOGIC1_HL_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_logic1_hl_inv` writer - "]
pub type CR_IRTX_LOGIC1_HL_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_head_en` reader - "]
pub type CR_IRTX_HEAD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_head_en` writer - "]
pub type CR_IRTX_HEAD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_head_hl_inv` reader - "]
pub type CR_IRTX_HEAD_HL_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_head_hl_inv` writer - "]
pub type CR_IRTX_HEAD_HL_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_tail_en` reader - "]
pub type CR_IRTX_TAIL_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_tail_en` writer - "]
pub type CR_IRTX_TAIL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_tail_hl_inv` reader - "]
pub type CR_IRTX_TAIL_HL_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_irtx_tail_hl_inv` writer - "]
pub type CR_IRTX_TAIL_HL_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IRTX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irtx_data_num` reader - "]
pub type CR_IRTX_DATA_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_data_num` writer - "]
pub type CR_IRTX_DATA_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_CONFIG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irtx_en(&self) -> CR_IRTX_EN_R {
        CR_IRTX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irtx_out_inv(&self) -> CR_IRTX_OUT_INV_R {
        CR_IRTX_OUT_INV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_irtx_mod_en(&self) -> CR_IRTX_MOD_EN_R {
        CR_IRTX_MOD_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_irtx_swm_en(&self) -> CR_IRTX_SWM_EN_R {
        CR_IRTX_SWM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irtx_data_en(&self) -> CR_IRTX_DATA_EN_R {
        CR_IRTX_DATA_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_irtx_logic0_hl_inv(&self) -> CR_IRTX_LOGIC0_HL_INV_R {
        CR_IRTX_LOGIC0_HL_INV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_irtx_logic1_hl_inv(&self) -> CR_IRTX_LOGIC1_HL_INV_R {
        CR_IRTX_LOGIC1_HL_INV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_head_en(&self) -> CR_IRTX_HEAD_EN_R {
        CR_IRTX_HEAD_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_irtx_head_hl_inv(&self) -> CR_IRTX_HEAD_HL_INV_R {
        CR_IRTX_HEAD_HL_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_irtx_tail_en(&self) -> CR_IRTX_TAIL_EN_R {
        CR_IRTX_TAIL_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_irtx_tail_hl_inv(&self) -> CR_IRTX_TAIL_HL_INV_R {
        CR_IRTX_TAIL_HL_INV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn cr_irtx_data_num(&self) -> CR_IRTX_DATA_NUM_R {
        CR_IRTX_DATA_NUM_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_en(&mut self) -> CR_IRTX_EN_W<0> {
        CR_IRTX_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_out_inv(&mut self) -> CR_IRTX_OUT_INV_W<1> {
        CR_IRTX_OUT_INV_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_mod_en(&mut self) -> CR_IRTX_MOD_EN_W<2> {
        CR_IRTX_MOD_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_swm_en(&mut self) -> CR_IRTX_SWM_EN_W<3> {
        CR_IRTX_SWM_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_data_en(&mut self) -> CR_IRTX_DATA_EN_W<4> {
        CR_IRTX_DATA_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_logic0_hl_inv(&mut self) -> CR_IRTX_LOGIC0_HL_INV_W<5> {
        CR_IRTX_LOGIC0_HL_INV_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_logic1_hl_inv(&mut self) -> CR_IRTX_LOGIC1_HL_INV_W<6> {
        CR_IRTX_LOGIC1_HL_INV_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_head_en(&mut self) -> CR_IRTX_HEAD_EN_W<8> {
        CR_IRTX_HEAD_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_head_hl_inv(&mut self) -> CR_IRTX_HEAD_HL_INV_W<9> {
        CR_IRTX_HEAD_HL_INV_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_tail_en(&mut self) -> CR_IRTX_TAIL_EN_W<10> {
        CR_IRTX_TAIL_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_tail_hl_inv(&mut self) -> CR_IRTX_TAIL_HL_INV_W<11> {
        CR_IRTX_TAIL_HL_INV_W::new(self)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_data_num(&mut self) -> CR_IRTX_DATA_NUM_W<12> {
        CR_IRTX_DATA_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irtx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_config](index.html) module"]
pub struct IRTX_CONFIG_SPEC;
impl crate::RegisterSpec for IRTX_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irtx_config::R](R) reader structure"]
impl crate::Readable for IRTX_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irtx_config::W](W) writer structure"]
impl crate::Writable for IRTX_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irtx_config to value 0x0001_f510"]
impl crate::Resettable for IRTX_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_f510;
}

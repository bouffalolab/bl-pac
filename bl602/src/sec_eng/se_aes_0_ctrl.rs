#[doc = "Register `se_aes_0_ctrl` reader"]
pub struct R(crate::R<SE_AES_0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_AES_0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_AES_0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_AES_0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_aes_0_ctrl` writer"]
pub struct W(crate::W<SE_AES_0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_AES_0_CTRL_SPEC>;
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
impl From<crate::W<SE_AES_0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_AES_0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_aes_0_busy` reader - "]
pub type SE_AES_0_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_trig_1t` reader - "]
pub type SE_AES_0_TRIG_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_trig_1t` writer - "]
pub type SE_AES_0_TRIG_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_SPEC, bool, O>;
#[doc = "Field `se_aes_0_en` reader - "]
pub type SE_AES_0_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_en` writer - "]
pub type SE_AES_0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SE_AES_0_CTRL_SPEC, bool, O>;
#[doc = "Field `se_aes_0_mode` reader - "]
pub type SE_AES_0_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_aes_0_mode` writer - "]
pub type SE_AES_0_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_AES_0_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `se_aes_0_dec_en` reader - "]
pub type SE_AES_0_DEC_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_dec_en` writer - "]
pub type SE_AES_0_DEC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_SPEC, bool, O>;
#[doc = "Field `se_aes_0_dec_key_sel` reader - "]
pub type SE_AES_0_DEC_KEY_SEL_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_dec_key_sel` writer - "]
pub type SE_AES_0_DEC_KEY_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_SPEC, bool, O>;
#[doc = "Field `se_aes_0_hw_key_en` reader - "]
pub type SE_AES_0_HW_KEY_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_hw_key_en` writer - "]
pub type SE_AES_0_HW_KEY_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_SPEC, bool, O>;
#[doc = "Field `se_aes_0_int` reader - "]
pub type SE_AES_0_INT_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_int_clr_1t` reader - "]
pub type SE_AES_0_INT_CLR_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_int_clr_1t` writer - "]
pub type SE_AES_0_INT_CLR_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_SPEC, bool, O>;
#[doc = "Field `se_aes_0_int_set_1t` reader - "]
pub type SE_AES_0_INT_SET_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_int_set_1t` writer - "]
pub type SE_AES_0_INT_SET_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_SPEC, bool, O>;
#[doc = "Field `se_aes_0_int_mask` reader - "]
pub type SE_AES_0_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_int_mask` writer - "]
pub type SE_AES_0_INT_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_SPEC, bool, O>;
#[doc = "Field `se_aes_0_block_mode` reader - "]
pub type SE_AES_0_BLOCK_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_aes_0_block_mode` writer - "]
pub type SE_AES_0_BLOCK_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_AES_0_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `se_aes_0_iv_sel` reader - "]
pub type SE_AES_0_IV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_iv_sel` writer - "]
pub type SE_AES_0_IV_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_SPEC, bool, O>;
#[doc = "Field `se_aes_0_link_mode` reader - "]
pub type SE_AES_0_LINK_MODE_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_link_mode` writer - "]
pub type SE_AES_0_LINK_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_SPEC, bool, O>;
#[doc = "Field `se_aes_0_msg_len` reader - "]
pub type SE_AES_0_MSG_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `se_aes_0_msg_len` writer - "]
pub type SE_AES_0_MSG_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_AES_0_CTRL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_busy(&self) -> SE_AES_0_BUSY_R {
        SE_AES_0_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_trig_1t(&self) -> SE_AES_0_TRIG_1T_R {
        SE_AES_0_TRIG_1T_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_en(&self) -> SE_AES_0_EN_R {
        SE_AES_0_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn se_aes_0_mode(&self) -> SE_AES_0_MODE_R {
        SE_AES_0_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_aes_0_dec_en(&self) -> SE_AES_0_DEC_EN_R {
        SE_AES_0_DEC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_0_dec_key_sel(&self) -> SE_AES_0_DEC_KEY_SEL_R {
        SE_AES_0_DEC_KEY_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn se_aes_0_hw_key_en(&self) -> SE_AES_0_HW_KEY_EN_R {
        SE_AES_0_HW_KEY_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_aes_0_int(&self) -> SE_AES_0_INT_R {
        SE_AES_0_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_aes_0_int_clr_1t(&self) -> SE_AES_0_INT_CLR_1T_R {
        SE_AES_0_INT_CLR_1T_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_aes_0_int_set_1t(&self) -> SE_AES_0_INT_SET_1T_R {
        SE_AES_0_INT_SET_1T_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_aes_0_int_mask(&self) -> SE_AES_0_INT_MASK_R {
        SE_AES_0_INT_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn se_aes_0_block_mode(&self) -> SE_AES_0_BLOCK_MODE_R {
        SE_AES_0_BLOCK_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_aes_0_iv_sel(&self) -> SE_AES_0_IV_SEL_R {
        SE_AES_0_IV_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_aes_0_link_mode(&self) -> SE_AES_0_LINK_MODE_R {
        SE_AES_0_LINK_MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_aes_0_msg_len(&self) -> SE_AES_0_MSG_LEN_R {
        SE_AES_0_MSG_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_trig_1t(&mut self) -> SE_AES_0_TRIG_1T_W<1> {
        SE_AES_0_TRIG_1T_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_en(&mut self) -> SE_AES_0_EN_W<2> {
        SE_AES_0_EN_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_mode(&mut self) -> SE_AES_0_MODE_W<3> {
        SE_AES_0_MODE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_dec_en(&mut self) -> SE_AES_0_DEC_EN_W<5> {
        SE_AES_0_DEC_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_dec_key_sel(&mut self) -> SE_AES_0_DEC_KEY_SEL_W<6> {
        SE_AES_0_DEC_KEY_SEL_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_hw_key_en(&mut self) -> SE_AES_0_HW_KEY_EN_W<7> {
        SE_AES_0_HW_KEY_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_int_clr_1t(&mut self) -> SE_AES_0_INT_CLR_1T_W<9> {
        SE_AES_0_INT_CLR_1T_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_int_set_1t(&mut self) -> SE_AES_0_INT_SET_1T_W<10> {
        SE_AES_0_INT_SET_1T_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_int_mask(&mut self) -> SE_AES_0_INT_MASK_W<11> {
        SE_AES_0_INT_MASK_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_block_mode(&mut self) -> SE_AES_0_BLOCK_MODE_W<12> {
        SE_AES_0_BLOCK_MODE_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_iv_sel(&mut self) -> SE_AES_0_IV_SEL_W<14> {
        SE_AES_0_IV_SEL_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_link_mode(&mut self) -> SE_AES_0_LINK_MODE_W<15> {
        SE_AES_0_LINK_MODE_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_msg_len(&mut self) -> SE_AES_0_MSG_LEN_W<16> {
        SE_AES_0_MSG_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_aes_0_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_ctrl](index.html) module"]
pub struct SE_AES_0_CTRL_SPEC;
impl crate::RegisterSpec for SE_AES_0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_aes_0_ctrl::R](R) reader structure"]
impl crate::Readable for SE_AES_0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_aes_0_ctrl::W](W) writer structure"]
impl crate::Writable for SE_AES_0_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_aes_0_ctrl to value 0"]
impl crate::Resettable for SE_AES_0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

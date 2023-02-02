#[doc = "Register `se_aes_0_endian` reader"]
pub struct R(crate::R<SE_AES_0_ENDIAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_AES_0_ENDIAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_AES_0_ENDIAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_AES_0_ENDIAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_aes_0_endian` writer"]
pub struct W(crate::W<SE_AES_0_ENDIAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_AES_0_ENDIAN_SPEC>;
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
impl From<crate::W<SE_AES_0_ENDIAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_AES_0_ENDIAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_aes_0_dout_endian` reader - "]
pub type SE_AES_0_DOUT_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_dout_endian` writer - "]
pub type SE_AES_0_DOUT_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_ENDIAN_SPEC, bool, O>;
#[doc = "Field `se_aes_0_din_endian` reader - "]
pub type SE_AES_0_DIN_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_din_endian` writer - "]
pub type SE_AES_0_DIN_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_ENDIAN_SPEC, bool, O>;
#[doc = "Field `se_aes_0_key_endian` reader - "]
pub type SE_AES_0_KEY_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_key_endian` writer - "]
pub type SE_AES_0_KEY_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_ENDIAN_SPEC, bool, O>;
#[doc = "Field `se_aes_0_iv_endian` reader - "]
pub type SE_AES_0_IV_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_iv_endian` writer - "]
pub type SE_AES_0_IV_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_ENDIAN_SPEC, bool, O>;
#[doc = "Field `se_aes_0_ctr_len` reader - "]
pub type SE_AES_0_CTR_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_aes_0_ctr_len` writer - "]
pub type SE_AES_0_CTR_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_AES_0_ENDIAN_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_dout_endian(&self) -> SE_AES_0_DOUT_ENDIAN_R {
        SE_AES_0_DOUT_ENDIAN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_din_endian(&self) -> SE_AES_0_DIN_ENDIAN_R {
        SE_AES_0_DIN_ENDIAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_key_endian(&self) -> SE_AES_0_KEY_ENDIAN_R {
        SE_AES_0_KEY_ENDIAN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_aes_0_iv_endian(&self) -> SE_AES_0_IV_ENDIAN_R {
        SE_AES_0_IV_ENDIAN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn se_aes_0_ctr_len(&self) -> SE_AES_0_CTR_LEN_R {
        SE_AES_0_CTR_LEN_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_dout_endian(&mut self) -> SE_AES_0_DOUT_ENDIAN_W<0> {
        SE_AES_0_DOUT_ENDIAN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_din_endian(&mut self) -> SE_AES_0_DIN_ENDIAN_W<1> {
        SE_AES_0_DIN_ENDIAN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_key_endian(&mut self) -> SE_AES_0_KEY_ENDIAN_W<2> {
        SE_AES_0_KEY_ENDIAN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_iv_endian(&mut self) -> SE_AES_0_IV_ENDIAN_W<3> {
        SE_AES_0_IV_ENDIAN_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_ctr_len(&mut self) -> SE_AES_0_CTR_LEN_W<30> {
        SE_AES_0_CTR_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_aes_0_endian.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_endian](index.html) module"]
pub struct SE_AES_0_ENDIAN_SPEC;
impl crate::RegisterSpec for SE_AES_0_ENDIAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_aes_0_endian::R](R) reader structure"]
impl crate::Readable for SE_AES_0_ENDIAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_aes_0_endian::W](W) writer structure"]
impl crate::Writable for SE_AES_0_ENDIAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_aes_0_endian to value 0x0f"]
impl crate::Resettable for SE_AES_0_ENDIAN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}

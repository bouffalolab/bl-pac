#[doc = "Register `sf_aes_r1` reader"]
pub struct R(crate::R<SF_AES_R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_AES_R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_AES_R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_AES_R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_aes_r1` writer"]
pub struct W(crate::W<SF_AES_R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_AES_R1_SPEC>;
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
impl From<crate::W<SF_AES_R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_AES_R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_aes_r1_end` reader - "]
pub type SF_AES_R1_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sf_aes_r1_end` writer - "]
pub type SF_AES_R1_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_AES_R1_SPEC, u16, u16, 14, O>;
#[doc = "Field `sf_aes_r1_start` reader - "]
pub type SF_AES_R1_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sf_aes_r1_start` writer - "]
pub type SF_AES_R1_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_AES_R1_SPEC, u16, u16, 14, O>;
#[doc = "Field `sf_aes_r1_hw_key_en` reader - "]
pub type SF_AES_R1_HW_KEY_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_r1_hw_key_en` writer - "]
pub type SF_AES_R1_HW_KEY_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_AES_R1_SPEC, bool, O>;
#[doc = "Field `sf_aes_r1_en` reader - "]
pub type SF_AES_R1_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_r1_en` writer - "]
pub type SF_AES_R1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_AES_R1_SPEC, bool, O>;
#[doc = "Field `sf_aes_r1_lock` reader - "]
pub type SF_AES_R1_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_r1_lock` writer - "]
pub type SF_AES_R1_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_AES_R1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_r1_end(&self) -> SF_AES_R1_END_R {
        SF_AES_R1_END_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_r1_start(&self) -> SF_AES_R1_START_R {
        SF_AES_R1_START_R::new(((self.bits >> 14) & 0x3fff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_r1_hw_key_en(&self) -> SF_AES_R1_HW_KEY_EN_R {
        SF_AES_R1_HW_KEY_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_r1_en(&self) -> SF_AES_R1_EN_R {
        SF_AES_R1_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_r1_lock(&self) -> SF_AES_R1_LOCK_R {
        SF_AES_R1_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_r1_end(&mut self) -> SF_AES_R1_END_W<0> {
        SF_AES_R1_END_W::new(self)
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_r1_start(&mut self) -> SF_AES_R1_START_W<14> {
        SF_AES_R1_START_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_r1_hw_key_en(&mut self) -> SF_AES_R1_HW_KEY_EN_W<29> {
        SF_AES_R1_HW_KEY_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_r1_en(&mut self) -> SF_AES_R1_EN_W<30> {
        SF_AES_R1_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_r1_lock(&mut self) -> SF_AES_R1_LOCK_W<31> {
        SF_AES_R1_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_aes_r1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_r1](index.html) module"]
pub struct SF_AES_R1_SPEC;
impl crate::RegisterSpec for SF_AES_R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_aes_r1::R](R) reader structure"]
impl crate::Readable for SF_AES_R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_aes_r1::W](W) writer structure"]
impl crate::Writable for SF_AES_R1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_aes_r1 to value 0x3fff"]
impl crate::Resettable for SF_AES_R1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}

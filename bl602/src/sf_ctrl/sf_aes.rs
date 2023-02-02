#[doc = "Register `sf_aes` reader"]
pub struct R(crate::R<SF_AES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_AES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_AES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_AES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_aes` writer"]
pub struct W(crate::W<SF_AES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_AES_SPEC>;
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
impl From<crate::W<SF_AES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_AES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_aes_en` reader - "]
pub type SF_AES_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_en` writer - "]
pub type SF_AES_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_AES_SPEC, bool, O>;
#[doc = "Field `sf_aes_mode` reader - "]
pub type SF_AES_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_aes_mode` writer - "]
pub type SF_AES_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SF_AES_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf_aes_pref_trig` reader - "]
pub type SF_AES_PREF_TRIG_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_pref_trig` writer - "]
pub type SF_AES_PREF_TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_AES_SPEC, bool, O>;
#[doc = "Field `sf_aes_pref_busy` reader - "]
pub type SF_AES_PREF_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `sf_aes_status` reader - "]
pub type SF_AES_STATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_aes_en(&self) -> SF_AES_EN_R {
        SF_AES_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sf_aes_mode(&self) -> SF_AES_MODE_R {
        SF_AES_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_aes_pref_trig(&self) -> SF_AES_PREF_TRIG_R {
        SF_AES_PREF_TRIG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_aes_pref_busy(&self) -> SF_AES_PREF_BUSY_R {
        SF_AES_PREF_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn sf_aes_status(&self) -> SF_AES_STATUS_R {
        SF_AES_STATUS_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_en(&mut self) -> SF_AES_EN_W<0> {
        SF_AES_EN_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_mode(&mut self) -> SF_AES_MODE_W<1> {
        SF_AES_MODE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sf_aes_pref_trig(&mut self) -> SF_AES_PREF_TRIG_W<3> {
        SF_AES_PREF_TRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_aes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes](index.html) module"]
pub struct SF_AES_SPEC;
impl crate::RegisterSpec for SF_AES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_aes::R](R) reader structure"]
impl crate::Readable for SF_AES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_aes::W](W) writer structure"]
impl crate::Writable for SF_AES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_aes to value 0x40"]
impl crate::Resettable for SF_AES_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}

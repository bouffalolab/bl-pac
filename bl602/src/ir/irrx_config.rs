#[doc = "Register `irrx_config` reader"]
pub struct R(crate::R<IRRX_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRRX_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRRX_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irrx_config` writer"]
pub struct W(crate::W<IRRX_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRRX_CONFIG_SPEC>;
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
impl From<crate::W<IRRX_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRRX_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irrx_en` reader - "]
pub type CR_IRRX_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_irrx_en` writer - "]
pub type CR_IRRX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRRX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irrx_in_inv` reader - "]
pub type CR_IRRX_IN_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_irrx_in_inv` writer - "]
pub type CR_IRRX_IN_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRRX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irrx_mode` reader - "]
pub type CR_IRRX_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irrx_mode` writer - "]
pub type CR_IRRX_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRRX_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_irrx_deg_en` reader - "]
pub type CR_IRRX_DEG_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_irrx_deg_en` writer - "]
pub type CR_IRRX_DEG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRRX_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_irrx_deg_cnt` reader - "]
pub type CR_IRRX_DEG_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irrx_deg_cnt` writer - "]
pub type CR_IRRX_DEG_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRRX_CONFIG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irrx_en(&self) -> CR_IRRX_EN_R {
        CR_IRRX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irrx_in_inv(&self) -> CR_IRRX_IN_INV_R {
        CR_IRRX_IN_INV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_irrx_mode(&self) -> CR_IRRX_MODE_R {
        CR_IRRX_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irrx_deg_en(&self) -> CR_IRRX_DEG_EN_R {
        CR_IRRX_DEG_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irrx_deg_cnt(&self) -> CR_IRRX_DEG_CNT_R {
        CR_IRRX_DEG_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irrx_en(&mut self) -> CR_IRRX_EN_W<0> {
        CR_IRRX_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irrx_in_inv(&mut self) -> CR_IRRX_IN_INV_W<1> {
        CR_IRRX_IN_INV_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irrx_mode(&mut self) -> CR_IRRX_MODE_W<2> {
        CR_IRRX_MODE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irrx_deg_en(&mut self) -> CR_IRRX_DEG_EN_W<4> {
        CR_IRRX_DEG_EN_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irrx_deg_cnt(&mut self) -> CR_IRRX_DEG_CNT_W<8> {
        CR_IRRX_DEG_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irrx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_config](index.html) module"]
pub struct IRRX_CONFIG_SPEC;
impl crate::RegisterSpec for IRRX_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_config::R](R) reader structure"]
impl crate::Readable for IRRX_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irrx_config::W](W) writer structure"]
impl crate::Writable for IRRX_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irrx_config to value 0x02"]
impl crate::Resettable for IRRX_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}

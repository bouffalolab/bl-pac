#[doc = "Register `se_trng_0_test` reader"]
pub struct R(crate::R<SE_TRNG_0_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_TRNG_0_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_TRNG_0_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_TRNG_0_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_trng_0_test` writer"]
pub struct W(crate::W<SE_TRNG_0_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_TRNG_0_TEST_SPEC>;
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
impl From<crate::W<SE_TRNG_0_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_TRNG_0_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_trng_0_test_en` reader - "]
pub type SE_TRNG_0_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_test_en` writer - "]
pub type SE_TRNG_0_TEST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_TEST_SPEC, bool, O>;
#[doc = "Field `se_trng_0_cp_test_en` reader - "]
pub type SE_TRNG_0_CP_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_cp_test_en` writer - "]
pub type SE_TRNG_0_CP_TEST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_TEST_SPEC, bool, O>;
#[doc = "Field `se_trng_0_cp_bypass` reader - "]
pub type SE_TRNG_0_CP_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_cp_bypass` writer - "]
pub type SE_TRNG_0_CP_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_TEST_SPEC, bool, O>;
#[doc = "Field `se_trng_0_ht_dis` reader - "]
pub type SE_TRNG_0_HT_DIS_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_ht_dis` writer - "]
pub type SE_TRNG_0_HT_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_TEST_SPEC, bool, O>;
#[doc = "Field `se_trng_0_ht_alarm_n` reader - "]
pub type SE_TRNG_0_HT_ALARM_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_trng_0_ht_alarm_n` writer - "]
pub type SE_TRNG_0_HT_ALARM_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_TRNG_0_TEST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_test_en(&self) -> SE_TRNG_0_TEST_EN_R {
        SE_TRNG_0_TEST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_cp_test_en(&self) -> SE_TRNG_0_CP_TEST_EN_R {
        SE_TRNG_0_CP_TEST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_cp_bypass(&self) -> SE_TRNG_0_CP_BYPASS_R {
        SE_TRNG_0_CP_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_ht_dis(&self) -> SE_TRNG_0_HT_DIS_R {
        SE_TRNG_0_HT_DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn se_trng_0_ht_alarm_n(&self) -> SE_TRNG_0_HT_ALARM_N_R {
        SE_TRNG_0_HT_ALARM_N_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_test_en(&mut self) -> SE_TRNG_0_TEST_EN_W<0> {
        SE_TRNG_0_TEST_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_cp_test_en(&mut self) -> SE_TRNG_0_CP_TEST_EN_W<1> {
        SE_TRNG_0_CP_TEST_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_cp_bypass(&mut self) -> SE_TRNG_0_CP_BYPASS_W<2> {
        SE_TRNG_0_CP_BYPASS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_ht_dis(&mut self) -> SE_TRNG_0_HT_DIS_W<3> {
        SE_TRNG_0_HT_DIS_W::new(self)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_ht_alarm_n(&mut self) -> SE_TRNG_0_HT_ALARM_N_W<4> {
        SE_TRNG_0_HT_ALARM_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_trng_0_test.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_test](index.html) module"]
pub struct SE_TRNG_0_TEST_SPEC;
impl crate::RegisterSpec for SE_TRNG_0_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_trng_0_test::R](R) reader structure"]
impl crate::Readable for SE_TRNG_0_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_trng_0_test::W](W) writer structure"]
impl crate::Writable for SE_TRNG_0_TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_trng_0_test to value 0"]
impl crate::Resettable for SE_TRNG_0_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

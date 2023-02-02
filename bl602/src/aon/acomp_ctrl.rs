#[doc = "Register `acomp_ctrl` reader"]
pub struct R(crate::R<ACOMP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACOMP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACOMP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACOMP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `acomp_ctrl` writer"]
pub struct W(crate::W<ACOMP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACOMP_CTRL_SPEC>;
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
impl From<crate::W<ACOMP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACOMP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `acomp1_rstn_ana` reader - "]
pub type ACOMP1_RSTN_ANA_R = crate::BitReader<bool>;
#[doc = "Field `acomp1_rstn_ana` writer - "]
pub type ACOMP1_RSTN_ANA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACOMP_CTRL_SPEC, bool, O>;
#[doc = "Field `acomp0_rstn_ana` reader - "]
pub type ACOMP0_RSTN_ANA_R = crate::BitReader<bool>;
#[doc = "Field `acomp0_rstn_ana` writer - "]
pub type ACOMP0_RSTN_ANA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACOMP_CTRL_SPEC, bool, O>;
#[doc = "Field `acomp1_test_en` reader - "]
pub type ACOMP1_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `acomp1_test_en` writer - "]
pub type ACOMP1_TEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACOMP_CTRL_SPEC, bool, O>;
#[doc = "Field `acomp0_test_en` reader - "]
pub type ACOMP0_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `acomp0_test_en` writer - "]
pub type ACOMP0_TEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACOMP_CTRL_SPEC, bool, O>;
#[doc = "Field `acomp1_test_sel` reader - "]
pub type ACOMP1_TEST_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acomp1_test_sel` writer - "]
pub type ACOMP1_TEST_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACOMP_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `acomp0_test_sel` reader - "]
pub type ACOMP0_TEST_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acomp0_test_sel` writer - "]
pub type ACOMP0_TEST_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACOMP_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `acomp1_out_raw` reader - "]
pub type ACOMP1_OUT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `acomp0_out_raw` reader - "]
pub type ACOMP0_OUT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `acomp_reserved` reader - "]
pub type ACOMP_RESERVED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acomp_reserved` writer - "]
pub type ACOMP_RESERVED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACOMP_CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_rstn_ana(&self) -> ACOMP1_RSTN_ANA_R {
        ACOMP1_RSTN_ANA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acomp0_rstn_ana(&self) -> ACOMP0_RSTN_ANA_R {
        ACOMP0_RSTN_ANA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn acomp1_test_en(&self) -> ACOMP1_TEST_EN_R {
        ACOMP1_TEST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn acomp0_test_en(&self) -> ACOMP0_TEST_EN_R {
        ACOMP0_TEST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_test_sel(&self) -> ACOMP1_TEST_SEL_R {
        ACOMP1_TEST_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn acomp0_test_sel(&self) -> ACOMP0_TEST_SEL_R {
        ACOMP0_TEST_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn acomp1_out_raw(&self) -> ACOMP1_OUT_RAW_R {
        ACOMP1_OUT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn acomp0_out_raw(&self) -> ACOMP0_OUT_RAW_R {
        ACOMP0_OUT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn acomp_reserved(&self) -> ACOMP_RESERVED_R {
        ACOMP_RESERVED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_rstn_ana(&mut self) -> ACOMP1_RSTN_ANA_W<0> {
        ACOMP1_RSTN_ANA_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_rstn_ana(&mut self) -> ACOMP0_RSTN_ANA_W<1> {
        ACOMP0_RSTN_ANA_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_test_en(&mut self) -> ACOMP1_TEST_EN_W<8> {
        ACOMP1_TEST_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_test_en(&mut self) -> ACOMP0_TEST_EN_W<9> {
        ACOMP0_TEST_EN_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_test_sel(&mut self) -> ACOMP1_TEST_SEL_W<10> {
        ACOMP1_TEST_SEL_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_test_sel(&mut self) -> ACOMP0_TEST_SEL_W<12> {
        ACOMP0_TEST_SEL_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn acomp_reserved(&mut self) -> ACOMP_RESERVED_W<24> {
        ACOMP_RESERVED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "acomp_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acomp_ctrl](index.html) module"]
pub struct ACOMP_CTRL_SPEC;
impl crate::RegisterSpec for ACOMP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acomp_ctrl::R](R) reader structure"]
impl crate::Readable for ACOMP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acomp_ctrl::W](W) writer structure"]
impl crate::Writable for ACOMP_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets acomp_ctrl to value 0x03"]
impl crate::Resettable for ACOMP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}

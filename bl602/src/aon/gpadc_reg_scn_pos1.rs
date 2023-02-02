#[doc = "Register `gpadc_reg_scn_pos1` reader"]
pub struct R(crate::R<GPADC_REG_SCN_POS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_SCN_POS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_SCN_POS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_SCN_POS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_scn_pos1` writer"]
pub struct W(crate::W<GPADC_REG_SCN_POS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_SCN_POS1_SPEC>;
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
impl From<crate::W<GPADC_REG_SCN_POS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_SCN_POS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_scan_pos_0` reader - "]
pub type GPADC_SCAN_POS_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_scan_pos_0` writer - "]
pub type GPADC_SCAN_POS_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_SCN_POS1_SPEC, u8, u8, 5, O>;
#[doc = "Field `gpadc_scan_pos_1` reader - "]
pub type GPADC_SCAN_POS_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_scan_pos_1` writer - "]
pub type GPADC_SCAN_POS_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_SCN_POS1_SPEC, u8, u8, 5, O>;
#[doc = "Field `gpadc_scan_pos_2` reader - "]
pub type GPADC_SCAN_POS_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_scan_pos_2` writer - "]
pub type GPADC_SCAN_POS_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_SCN_POS1_SPEC, u8, u8, 5, O>;
#[doc = "Field `gpadc_scan_pos_3` reader - "]
pub type GPADC_SCAN_POS_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_scan_pos_3` writer - "]
pub type GPADC_SCAN_POS_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_SCN_POS1_SPEC, u8, u8, 5, O>;
#[doc = "Field `gpadc_scan_pos_4` reader - "]
pub type GPADC_SCAN_POS_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_scan_pos_4` writer - "]
pub type GPADC_SCAN_POS_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_SCN_POS1_SPEC, u8, u8, 5, O>;
#[doc = "Field `gpadc_scan_pos_5` reader - "]
pub type GPADC_SCAN_POS_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_scan_pos_5` writer - "]
pub type GPADC_SCAN_POS_5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_SCN_POS1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_0(&self) -> GPADC_SCAN_POS_0_R {
        GPADC_SCAN_POS_0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_1(&self) -> GPADC_SCAN_POS_1_R {
        GPADC_SCAN_POS_1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_2(&self) -> GPADC_SCAN_POS_2_R {
        GPADC_SCAN_POS_2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_3(&self) -> GPADC_SCAN_POS_3_R {
        GPADC_SCAN_POS_3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_4(&self) -> GPADC_SCAN_POS_4_R {
        GPADC_SCAN_POS_4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_5(&self) -> GPADC_SCAN_POS_5_R {
        GPADC_SCAN_POS_5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_0(&mut self) -> GPADC_SCAN_POS_0_W<0> {
        GPADC_SCAN_POS_0_W::new(self)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_1(&mut self) -> GPADC_SCAN_POS_1_W<5> {
        GPADC_SCAN_POS_1_W::new(self)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_2(&mut self) -> GPADC_SCAN_POS_2_W<10> {
        GPADC_SCAN_POS_2_W::new(self)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_3(&mut self) -> GPADC_SCAN_POS_3_W<15> {
        GPADC_SCAN_POS_3_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_4(&mut self) -> GPADC_SCAN_POS_4_W<20> {
        GPADC_SCAN_POS_4_W::new(self)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_5(&mut self) -> GPADC_SCAN_POS_5_W<25> {
        GPADC_SCAN_POS_5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adc converation sequence 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_scn_pos1](index.html) module"]
pub struct GPADC_REG_SCN_POS1_SPEC;
impl crate::RegisterSpec for GPADC_REG_SCN_POS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_scn_pos1::R](R) reader structure"]
impl crate::Readable for GPADC_REG_SCN_POS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_scn_pos1::W](W) writer structure"]
impl crate::Writable for GPADC_REG_SCN_POS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_scn_pos1 to value 0x1ef7_bdef"]
impl crate::Resettable for GPADC_REG_SCN_POS1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1ef7_bdef;
}

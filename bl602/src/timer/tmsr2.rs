#[doc = "Register `TMSR2` reader"]
pub struct R(crate::R<TMSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tmsr_0` reader - "]
pub type TMSR_0_R = crate::BitReader<bool>;
#[doc = "Field `tmsr_1` reader - "]
pub type TMSR_1_R = crate::BitReader<bool>;
#[doc = "Field `tmsr_2` reader - "]
pub type TMSR_2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmsr_0(&self) -> TMSR_0_R {
        TMSR_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmsr_1(&self) -> TMSR_1_R {
        TMSR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tmsr_2(&self) -> TMSR_2_R {
        TMSR_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "TMSR2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmsr2](index.html) module"]
pub struct TMSR2_SPEC;
impl crate::RegisterSpec for TMSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmsr2::R](R) reader structure"]
impl crate::Readable for TMSR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TMSR2 to value 0"]
impl crate::Resettable for TMSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

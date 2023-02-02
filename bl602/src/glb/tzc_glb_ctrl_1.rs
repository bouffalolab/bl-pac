#[doc = "Register `tzc_glb_ctrl_1` reader"]
pub struct R(crate::R<TZC_GLB_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GLB_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_GLB_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_GLB_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tzc_glb_swrst_s20_lock` reader - "]
pub type TZC_GLB_SWRST_S20_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s21_lock` reader - "]
pub type TZC_GLB_SWRST_S21_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s22_lock` reader - "]
pub type TZC_GLB_SWRST_S22_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s23_lock` reader - "]
pub type TZC_GLB_SWRST_S23_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s24_lock` reader - "]
pub type TZC_GLB_SWRST_S24_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s25_lock` reader - "]
pub type TZC_GLB_SWRST_S25_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s26_lock` reader - "]
pub type TZC_GLB_SWRST_S26_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s27_lock` reader - "]
pub type TZC_GLB_SWRST_S27_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s28_lock` reader - "]
pub type TZC_GLB_SWRST_S28_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s29_lock` reader - "]
pub type TZC_GLB_SWRST_S29_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s2a_lock` reader - "]
pub type TZC_GLB_SWRST_S2A_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s2b_lock` reader - "]
pub type TZC_GLB_SWRST_S2B_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s2c_lock` reader - "]
pub type TZC_GLB_SWRST_S2C_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s2d_lock` reader - "]
pub type TZC_GLB_SWRST_S2D_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s2e_lock` reader - "]
pub type TZC_GLB_SWRST_S2E_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s2f_lock` reader - "]
pub type TZC_GLB_SWRST_S2F_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s10_lock` reader - "]
pub type TZC_GLB_SWRST_S10_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s11_lock` reader - "]
pub type TZC_GLB_SWRST_S11_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s12_lock` reader - "]
pub type TZC_GLB_SWRST_S12_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s13_lock` reader - "]
pub type TZC_GLB_SWRST_S13_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s14_lock` reader - "]
pub type TZC_GLB_SWRST_S14_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s15_lock` reader - "]
pub type TZC_GLB_SWRST_S15_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s16_lock` reader - "]
pub type TZC_GLB_SWRST_S16_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s17_lock` reader - "]
pub type TZC_GLB_SWRST_S17_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s18_lock` reader - "]
pub type TZC_GLB_SWRST_S18_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s19_lock` reader - "]
pub type TZC_GLB_SWRST_S19_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s1a_lock` reader - "]
pub type TZC_GLB_SWRST_S1A_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s1b_lock` reader - "]
pub type TZC_GLB_SWRST_S1B_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s1c_lock` reader - "]
pub type TZC_GLB_SWRST_S1C_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s1d_lock` reader - "]
pub type TZC_GLB_SWRST_S1D_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s1e_lock` reader - "]
pub type TZC_GLB_SWRST_S1E_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_swrst_s1f_lock` reader - "]
pub type TZC_GLB_SWRST_S1F_LOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s20_lock(&self) -> TZC_GLB_SWRST_S20_LOCK_R {
        TZC_GLB_SWRST_S20_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s21_lock(&self) -> TZC_GLB_SWRST_S21_LOCK_R {
        TZC_GLB_SWRST_S21_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s22_lock(&self) -> TZC_GLB_SWRST_S22_LOCK_R {
        TZC_GLB_SWRST_S22_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s23_lock(&self) -> TZC_GLB_SWRST_S23_LOCK_R {
        TZC_GLB_SWRST_S23_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s24_lock(&self) -> TZC_GLB_SWRST_S24_LOCK_R {
        TZC_GLB_SWRST_S24_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s25_lock(&self) -> TZC_GLB_SWRST_S25_LOCK_R {
        TZC_GLB_SWRST_S25_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s26_lock(&self) -> TZC_GLB_SWRST_S26_LOCK_R {
        TZC_GLB_SWRST_S26_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s27_lock(&self) -> TZC_GLB_SWRST_S27_LOCK_R {
        TZC_GLB_SWRST_S27_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s28_lock(&self) -> TZC_GLB_SWRST_S28_LOCK_R {
        TZC_GLB_SWRST_S28_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s29_lock(&self) -> TZC_GLB_SWRST_S29_LOCK_R {
        TZC_GLB_SWRST_S29_LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2a_lock(&self) -> TZC_GLB_SWRST_S2A_LOCK_R {
        TZC_GLB_SWRST_S2A_LOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2b_lock(&self) -> TZC_GLB_SWRST_S2B_LOCK_R {
        TZC_GLB_SWRST_S2B_LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2c_lock(&self) -> TZC_GLB_SWRST_S2C_LOCK_R {
        TZC_GLB_SWRST_S2C_LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2d_lock(&self) -> TZC_GLB_SWRST_S2D_LOCK_R {
        TZC_GLB_SWRST_S2D_LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2e_lock(&self) -> TZC_GLB_SWRST_S2E_LOCK_R {
        TZC_GLB_SWRST_S2E_LOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2f_lock(&self) -> TZC_GLB_SWRST_S2F_LOCK_R {
        TZC_GLB_SWRST_S2F_LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s10_lock(&self) -> TZC_GLB_SWRST_S10_LOCK_R {
        TZC_GLB_SWRST_S10_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s11_lock(&self) -> TZC_GLB_SWRST_S11_LOCK_R {
        TZC_GLB_SWRST_S11_LOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s12_lock(&self) -> TZC_GLB_SWRST_S12_LOCK_R {
        TZC_GLB_SWRST_S12_LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s13_lock(&self) -> TZC_GLB_SWRST_S13_LOCK_R {
        TZC_GLB_SWRST_S13_LOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s14_lock(&self) -> TZC_GLB_SWRST_S14_LOCK_R {
        TZC_GLB_SWRST_S14_LOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s15_lock(&self) -> TZC_GLB_SWRST_S15_LOCK_R {
        TZC_GLB_SWRST_S15_LOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s16_lock(&self) -> TZC_GLB_SWRST_S16_LOCK_R {
        TZC_GLB_SWRST_S16_LOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s17_lock(&self) -> TZC_GLB_SWRST_S17_LOCK_R {
        TZC_GLB_SWRST_S17_LOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s18_lock(&self) -> TZC_GLB_SWRST_S18_LOCK_R {
        TZC_GLB_SWRST_S18_LOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s19_lock(&self) -> TZC_GLB_SWRST_S19_LOCK_R {
        TZC_GLB_SWRST_S19_LOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1a_lock(&self) -> TZC_GLB_SWRST_S1A_LOCK_R {
        TZC_GLB_SWRST_S1A_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1b_lock(&self) -> TZC_GLB_SWRST_S1B_LOCK_R {
        TZC_GLB_SWRST_S1B_LOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1c_lock(&self) -> TZC_GLB_SWRST_S1C_LOCK_R {
        TZC_GLB_SWRST_S1C_LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1d_lock(&self) -> TZC_GLB_SWRST_S1D_LOCK_R {
        TZC_GLB_SWRST_S1D_LOCK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1e_lock(&self) -> TZC_GLB_SWRST_S1E_LOCK_R {
        TZC_GLB_SWRST_S1E_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1f_lock(&self) -> TZC_GLB_SWRST_S1F_LOCK_R {
        TZC_GLB_SWRST_S1F_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "tzc_glb_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_1](index.html) module"]
pub struct TZC_GLB_CTRL_1_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_1::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_1 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `tzc_glb_ctrl_2` reader"]
pub struct R(crate::R<TZC_GLB_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GLB_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_GLB_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_GLB_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tzc_glb_gpio_0_lock` reader - "]
pub type TZC_GLB_GPIO_0_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_1_lock` reader - "]
pub type TZC_GLB_GPIO_1_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_2_lock` reader - "]
pub type TZC_GLB_GPIO_2_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_3_lock` reader - "]
pub type TZC_GLB_GPIO_3_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_4_lock` reader - "]
pub type TZC_GLB_GPIO_4_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_5_lock` reader - "]
pub type TZC_GLB_GPIO_5_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_6_lock` reader - "]
pub type TZC_GLB_GPIO_6_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_7_lock` reader - "]
pub type TZC_GLB_GPIO_7_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_8_lock` reader - "]
pub type TZC_GLB_GPIO_8_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_9_lock` reader - "]
pub type TZC_GLB_GPIO_9_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_10_lock` reader - "]
pub type TZC_GLB_GPIO_10_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_11_lock` reader - "]
pub type TZC_GLB_GPIO_11_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_12_lock` reader - "]
pub type TZC_GLB_GPIO_12_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_13_lock` reader - "]
pub type TZC_GLB_GPIO_13_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_14_lock` reader - "]
pub type TZC_GLB_GPIO_14_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_15_lock` reader - "]
pub type TZC_GLB_GPIO_15_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_16_lock` reader - "]
pub type TZC_GLB_GPIO_16_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_17_lock` reader - "]
pub type TZC_GLB_GPIO_17_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_18_lock` reader - "]
pub type TZC_GLB_GPIO_18_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_19_lock` reader - "]
pub type TZC_GLB_GPIO_19_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_20_lock` reader - "]
pub type TZC_GLB_GPIO_20_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_21_lock` reader - "]
pub type TZC_GLB_GPIO_21_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_22_lock` reader - "]
pub type TZC_GLB_GPIO_22_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_23_lock` reader - "]
pub type TZC_GLB_GPIO_23_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_24_lock` reader - "]
pub type TZC_GLB_GPIO_24_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_25_lock` reader - "]
pub type TZC_GLB_GPIO_25_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_26_lock` reader - "]
pub type TZC_GLB_GPIO_26_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_27_lock` reader - "]
pub type TZC_GLB_GPIO_27_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_28_lock` reader - "]
pub type TZC_GLB_GPIO_28_LOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_gpio_0_lock(&self) -> TZC_GLB_GPIO_0_LOCK_R {
        TZC_GLB_GPIO_0_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_gpio_1_lock(&self) -> TZC_GLB_GPIO_1_LOCK_R {
        TZC_GLB_GPIO_1_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_gpio_2_lock(&self) -> TZC_GLB_GPIO_2_LOCK_R {
        TZC_GLB_GPIO_2_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_gpio_3_lock(&self) -> TZC_GLB_GPIO_3_LOCK_R {
        TZC_GLB_GPIO_3_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_gpio_4_lock(&self) -> TZC_GLB_GPIO_4_LOCK_R {
        TZC_GLB_GPIO_4_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_gpio_5_lock(&self) -> TZC_GLB_GPIO_5_LOCK_R {
        TZC_GLB_GPIO_5_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_glb_gpio_6_lock(&self) -> TZC_GLB_GPIO_6_LOCK_R {
        TZC_GLB_GPIO_6_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_glb_gpio_7_lock(&self) -> TZC_GLB_GPIO_7_LOCK_R {
        TZC_GLB_GPIO_7_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_gpio_8_lock(&self) -> TZC_GLB_GPIO_8_LOCK_R {
        TZC_GLB_GPIO_8_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_glb_gpio_9_lock(&self) -> TZC_GLB_GPIO_9_LOCK_R {
        TZC_GLB_GPIO_9_LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_glb_gpio_10_lock(&self) -> TZC_GLB_GPIO_10_LOCK_R {
        TZC_GLB_GPIO_10_LOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_glb_gpio_11_lock(&self) -> TZC_GLB_GPIO_11_LOCK_R {
        TZC_GLB_GPIO_11_LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_gpio_12_lock(&self) -> TZC_GLB_GPIO_12_LOCK_R {
        TZC_GLB_GPIO_12_LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_gpio_13_lock(&self) -> TZC_GLB_GPIO_13_LOCK_R {
        TZC_GLB_GPIO_13_LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_gpio_14_lock(&self) -> TZC_GLB_GPIO_14_LOCK_R {
        TZC_GLB_GPIO_14_LOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_gpio_15_lock(&self) -> TZC_GLB_GPIO_15_LOCK_R {
        TZC_GLB_GPIO_15_LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_glb_gpio_16_lock(&self) -> TZC_GLB_GPIO_16_LOCK_R {
        TZC_GLB_GPIO_16_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_glb_gpio_17_lock(&self) -> TZC_GLB_GPIO_17_LOCK_R {
        TZC_GLB_GPIO_17_LOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_glb_gpio_18_lock(&self) -> TZC_GLB_GPIO_18_LOCK_R {
        TZC_GLB_GPIO_18_LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_glb_gpio_19_lock(&self) -> TZC_GLB_GPIO_19_LOCK_R {
        TZC_GLB_GPIO_19_LOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_glb_gpio_20_lock(&self) -> TZC_GLB_GPIO_20_LOCK_R {
        TZC_GLB_GPIO_20_LOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_gpio_21_lock(&self) -> TZC_GLB_GPIO_21_LOCK_R {
        TZC_GLB_GPIO_21_LOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_gpio_22_lock(&self) -> TZC_GLB_GPIO_22_LOCK_R {
        TZC_GLB_GPIO_22_LOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_glb_gpio_23_lock(&self) -> TZC_GLB_GPIO_23_LOCK_R {
        TZC_GLB_GPIO_23_LOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_gpio_24_lock(&self) -> TZC_GLB_GPIO_24_LOCK_R {
        TZC_GLB_GPIO_24_LOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_gpio_25_lock(&self) -> TZC_GLB_GPIO_25_LOCK_R {
        TZC_GLB_GPIO_25_LOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_gpio_26_lock(&self) -> TZC_GLB_GPIO_26_LOCK_R {
        TZC_GLB_GPIO_26_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_gpio_27_lock(&self) -> TZC_GLB_GPIO_27_LOCK_R {
        TZC_GLB_GPIO_27_LOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_gpio_28_lock(&self) -> TZC_GLB_GPIO_28_LOCK_R {
        TZC_GLB_GPIO_28_LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "tzc_glb_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_2](index.html) module"]
pub struct TZC_GLB_CTRL_2_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_2::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_2 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `lo_sdm_ctrl_hw1` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_SDM_CTRL_HW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_SDM_CTRL_HW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw1` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW1_SPEC>;
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
impl From<crate::W<LO_SDM_CTRL_HW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_SDM_CTRL_HW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdm_dither_sel_wlan_2412` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2412_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2412` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2412_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2417` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2417_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2417` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2417_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2422` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2422_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2422` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2422_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2427` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2427_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2427` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2427_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2432` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2432_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2432` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2432_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2437` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2437_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2437` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2437_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2442` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2442_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2442` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2442_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2447` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2447_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2447` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2447_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2452` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2452_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2452` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2452_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2457` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2457_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2457` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2457_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2462` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2462_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2462` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2462_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2467` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2467_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2467` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2467_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2472` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2472_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2472` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2472_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2484` reader - "]
pub type LO_SDM_DITHER_SEL_WLAN_2484_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2484` writer - "]
pub type LO_SDM_DITHER_SEL_WLAN_2484_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2412(&self) -> LO_SDM_DITHER_SEL_WLAN_2412_R {
        LO_SDM_DITHER_SEL_WLAN_2412_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2417(&self) -> LO_SDM_DITHER_SEL_WLAN_2417_R {
        LO_SDM_DITHER_SEL_WLAN_2417_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2422(&self) -> LO_SDM_DITHER_SEL_WLAN_2422_R {
        LO_SDM_DITHER_SEL_WLAN_2422_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2427(&self) -> LO_SDM_DITHER_SEL_WLAN_2427_R {
        LO_SDM_DITHER_SEL_WLAN_2427_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2432(&self) -> LO_SDM_DITHER_SEL_WLAN_2432_R {
        LO_SDM_DITHER_SEL_WLAN_2432_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2437(&self) -> LO_SDM_DITHER_SEL_WLAN_2437_R {
        LO_SDM_DITHER_SEL_WLAN_2437_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2442(&self) -> LO_SDM_DITHER_SEL_WLAN_2442_R {
        LO_SDM_DITHER_SEL_WLAN_2442_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2447(&self) -> LO_SDM_DITHER_SEL_WLAN_2447_R {
        LO_SDM_DITHER_SEL_WLAN_2447_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2452(&self) -> LO_SDM_DITHER_SEL_WLAN_2452_R {
        LO_SDM_DITHER_SEL_WLAN_2452_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2457(&self) -> LO_SDM_DITHER_SEL_WLAN_2457_R {
        LO_SDM_DITHER_SEL_WLAN_2457_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2462(&self) -> LO_SDM_DITHER_SEL_WLAN_2462_R {
        LO_SDM_DITHER_SEL_WLAN_2462_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2467(&self) -> LO_SDM_DITHER_SEL_WLAN_2467_R {
        LO_SDM_DITHER_SEL_WLAN_2467_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2472(&self) -> LO_SDM_DITHER_SEL_WLAN_2472_R {
        LO_SDM_DITHER_SEL_WLAN_2472_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2484(&self) -> LO_SDM_DITHER_SEL_WLAN_2484_R {
        LO_SDM_DITHER_SEL_WLAN_2484_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2412(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2412_W<0> {
        LO_SDM_DITHER_SEL_WLAN_2412_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2417(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2417_W<2> {
        LO_SDM_DITHER_SEL_WLAN_2417_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2422(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2422_W<4> {
        LO_SDM_DITHER_SEL_WLAN_2422_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2427(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2427_W<6> {
        LO_SDM_DITHER_SEL_WLAN_2427_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2432(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2432_W<8> {
        LO_SDM_DITHER_SEL_WLAN_2432_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2437(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2437_W<10> {
        LO_SDM_DITHER_SEL_WLAN_2437_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2442(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2442_W<12> {
        LO_SDM_DITHER_SEL_WLAN_2442_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2447(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2447_W<14> {
        LO_SDM_DITHER_SEL_WLAN_2447_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2452(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2452_W<16> {
        LO_SDM_DITHER_SEL_WLAN_2452_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2457(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2457_W<18> {
        LO_SDM_DITHER_SEL_WLAN_2457_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2462(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2462_W<20> {
        LO_SDM_DITHER_SEL_WLAN_2462_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2467(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2467_W<22> {
        LO_SDM_DITHER_SEL_WLAN_2467_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2472(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2472_W<24> {
        LO_SDM_DITHER_SEL_WLAN_2472_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_wlan_2484(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2484_W<26> {
        LO_SDM_DITHER_SEL_WLAN_2484_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw1](index.html) module"]
pub struct LO_SDM_CTRL_HW1_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw1::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw1::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw1 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

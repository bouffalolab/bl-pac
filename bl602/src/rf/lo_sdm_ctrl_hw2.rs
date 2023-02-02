#[doc = "Register `lo_sdm_ctrl_hw2` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_SDM_CTRL_HW2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_SDM_CTRL_HW2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw2` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW2_SPEC>;
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
impl From<crate::W<LO_SDM_CTRL_HW2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_SDM_CTRL_HW2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2402` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2402_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2402` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2402_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2404` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2404_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2404` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2404_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2406` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2406_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2406` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2406_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2408` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2408_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2408` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2408_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2410` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2410_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2410` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2410_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2412` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2412_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2412` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2412_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2414` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2414_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2414` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2414_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2416` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2416_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2416` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2416_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2418` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2418_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2418` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2418_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2420` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2420_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2420` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2420_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2422` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2422_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2422` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2422_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2424` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2424_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2424` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2424_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2426` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2426_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2426` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2426_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2428` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2428_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2428` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2428_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2430` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2430_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2430` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2430_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2432` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2432_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2432` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2432_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2402(&self) -> LO_SDM_DITHER_SEL_BLE_2402_R {
        LO_SDM_DITHER_SEL_BLE_2402_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2404(&self) -> LO_SDM_DITHER_SEL_BLE_2404_R {
        LO_SDM_DITHER_SEL_BLE_2404_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2406(&self) -> LO_SDM_DITHER_SEL_BLE_2406_R {
        LO_SDM_DITHER_SEL_BLE_2406_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2408(&self) -> LO_SDM_DITHER_SEL_BLE_2408_R {
        LO_SDM_DITHER_SEL_BLE_2408_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2410(&self) -> LO_SDM_DITHER_SEL_BLE_2410_R {
        LO_SDM_DITHER_SEL_BLE_2410_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2412(&self) -> LO_SDM_DITHER_SEL_BLE_2412_R {
        LO_SDM_DITHER_SEL_BLE_2412_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2414(&self) -> LO_SDM_DITHER_SEL_BLE_2414_R {
        LO_SDM_DITHER_SEL_BLE_2414_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2416(&self) -> LO_SDM_DITHER_SEL_BLE_2416_R {
        LO_SDM_DITHER_SEL_BLE_2416_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2418(&self) -> LO_SDM_DITHER_SEL_BLE_2418_R {
        LO_SDM_DITHER_SEL_BLE_2418_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2420(&self) -> LO_SDM_DITHER_SEL_BLE_2420_R {
        LO_SDM_DITHER_SEL_BLE_2420_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2422(&self) -> LO_SDM_DITHER_SEL_BLE_2422_R {
        LO_SDM_DITHER_SEL_BLE_2422_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2424(&self) -> LO_SDM_DITHER_SEL_BLE_2424_R {
        LO_SDM_DITHER_SEL_BLE_2424_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2426(&self) -> LO_SDM_DITHER_SEL_BLE_2426_R {
        LO_SDM_DITHER_SEL_BLE_2426_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2428(&self) -> LO_SDM_DITHER_SEL_BLE_2428_R {
        LO_SDM_DITHER_SEL_BLE_2428_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2430(&self) -> LO_SDM_DITHER_SEL_BLE_2430_R {
        LO_SDM_DITHER_SEL_BLE_2430_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2432(&self) -> LO_SDM_DITHER_SEL_BLE_2432_R {
        LO_SDM_DITHER_SEL_BLE_2432_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2402(&mut self) -> LO_SDM_DITHER_SEL_BLE_2402_W<0> {
        LO_SDM_DITHER_SEL_BLE_2402_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2404(&mut self) -> LO_SDM_DITHER_SEL_BLE_2404_W<2> {
        LO_SDM_DITHER_SEL_BLE_2404_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2406(&mut self) -> LO_SDM_DITHER_SEL_BLE_2406_W<4> {
        LO_SDM_DITHER_SEL_BLE_2406_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2408(&mut self) -> LO_SDM_DITHER_SEL_BLE_2408_W<6> {
        LO_SDM_DITHER_SEL_BLE_2408_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2410(&mut self) -> LO_SDM_DITHER_SEL_BLE_2410_W<8> {
        LO_SDM_DITHER_SEL_BLE_2410_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2412(&mut self) -> LO_SDM_DITHER_SEL_BLE_2412_W<10> {
        LO_SDM_DITHER_SEL_BLE_2412_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2414(&mut self) -> LO_SDM_DITHER_SEL_BLE_2414_W<12> {
        LO_SDM_DITHER_SEL_BLE_2414_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2416(&mut self) -> LO_SDM_DITHER_SEL_BLE_2416_W<14> {
        LO_SDM_DITHER_SEL_BLE_2416_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2418(&mut self) -> LO_SDM_DITHER_SEL_BLE_2418_W<16> {
        LO_SDM_DITHER_SEL_BLE_2418_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2420(&mut self) -> LO_SDM_DITHER_SEL_BLE_2420_W<18> {
        LO_SDM_DITHER_SEL_BLE_2420_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2422(&mut self) -> LO_SDM_DITHER_SEL_BLE_2422_W<20> {
        LO_SDM_DITHER_SEL_BLE_2422_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2424(&mut self) -> LO_SDM_DITHER_SEL_BLE_2424_W<22> {
        LO_SDM_DITHER_SEL_BLE_2424_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2426(&mut self) -> LO_SDM_DITHER_SEL_BLE_2426_W<24> {
        LO_SDM_DITHER_SEL_BLE_2426_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2428(&mut self) -> LO_SDM_DITHER_SEL_BLE_2428_W<26> {
        LO_SDM_DITHER_SEL_BLE_2428_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2430(&mut self) -> LO_SDM_DITHER_SEL_BLE_2430_W<28> {
        LO_SDM_DITHER_SEL_BLE_2430_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2432(&mut self) -> LO_SDM_DITHER_SEL_BLE_2432_W<30> {
        LO_SDM_DITHER_SEL_BLE_2432_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw2](index.html) module"]
pub struct LO_SDM_CTRL_HW2_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw2::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw2::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw2 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

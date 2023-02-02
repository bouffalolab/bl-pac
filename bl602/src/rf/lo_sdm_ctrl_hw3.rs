#[doc = "Register `lo_sdm_ctrl_hw3` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_SDM_CTRL_HW3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_SDM_CTRL_HW3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw3` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW3_SPEC>;
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
impl From<crate::W<LO_SDM_CTRL_HW3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_SDM_CTRL_HW3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2434` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2434_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2434` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2434_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2436` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2436_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2436` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2436_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2438` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2438_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2438` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2438_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2440` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2440_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2440` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2440_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2442` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2442_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2442` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2442_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2444` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2444_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2444` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2444_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2446` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2446_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2446` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2446_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2448` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2448_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2448` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2448_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2450` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2450_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2450` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2450_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2452` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2452_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2452` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2452_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2454` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2454_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2454` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2454_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2456` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2456_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2456` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2456_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2458` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2458_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2458` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2458_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2460` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2460_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2460` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2460_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2462` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2462_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2462` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2462_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2464` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2464_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2464` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2464_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2434(&self) -> LO_SDM_DITHER_SEL_BLE_2434_R {
        LO_SDM_DITHER_SEL_BLE_2434_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2436(&self) -> LO_SDM_DITHER_SEL_BLE_2436_R {
        LO_SDM_DITHER_SEL_BLE_2436_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2438(&self) -> LO_SDM_DITHER_SEL_BLE_2438_R {
        LO_SDM_DITHER_SEL_BLE_2438_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2440(&self) -> LO_SDM_DITHER_SEL_BLE_2440_R {
        LO_SDM_DITHER_SEL_BLE_2440_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2442(&self) -> LO_SDM_DITHER_SEL_BLE_2442_R {
        LO_SDM_DITHER_SEL_BLE_2442_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2444(&self) -> LO_SDM_DITHER_SEL_BLE_2444_R {
        LO_SDM_DITHER_SEL_BLE_2444_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2446(&self) -> LO_SDM_DITHER_SEL_BLE_2446_R {
        LO_SDM_DITHER_SEL_BLE_2446_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2448(&self) -> LO_SDM_DITHER_SEL_BLE_2448_R {
        LO_SDM_DITHER_SEL_BLE_2448_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2450(&self) -> LO_SDM_DITHER_SEL_BLE_2450_R {
        LO_SDM_DITHER_SEL_BLE_2450_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2452(&self) -> LO_SDM_DITHER_SEL_BLE_2452_R {
        LO_SDM_DITHER_SEL_BLE_2452_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2454(&self) -> LO_SDM_DITHER_SEL_BLE_2454_R {
        LO_SDM_DITHER_SEL_BLE_2454_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2456(&self) -> LO_SDM_DITHER_SEL_BLE_2456_R {
        LO_SDM_DITHER_SEL_BLE_2456_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2458(&self) -> LO_SDM_DITHER_SEL_BLE_2458_R {
        LO_SDM_DITHER_SEL_BLE_2458_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2460(&self) -> LO_SDM_DITHER_SEL_BLE_2460_R {
        LO_SDM_DITHER_SEL_BLE_2460_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2462(&self) -> LO_SDM_DITHER_SEL_BLE_2462_R {
        LO_SDM_DITHER_SEL_BLE_2462_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2464(&self) -> LO_SDM_DITHER_SEL_BLE_2464_R {
        LO_SDM_DITHER_SEL_BLE_2464_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2434(&mut self) -> LO_SDM_DITHER_SEL_BLE_2434_W<0> {
        LO_SDM_DITHER_SEL_BLE_2434_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2436(&mut self) -> LO_SDM_DITHER_SEL_BLE_2436_W<2> {
        LO_SDM_DITHER_SEL_BLE_2436_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2438(&mut self) -> LO_SDM_DITHER_SEL_BLE_2438_W<4> {
        LO_SDM_DITHER_SEL_BLE_2438_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2440(&mut self) -> LO_SDM_DITHER_SEL_BLE_2440_W<6> {
        LO_SDM_DITHER_SEL_BLE_2440_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2442(&mut self) -> LO_SDM_DITHER_SEL_BLE_2442_W<8> {
        LO_SDM_DITHER_SEL_BLE_2442_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2444(&mut self) -> LO_SDM_DITHER_SEL_BLE_2444_W<10> {
        LO_SDM_DITHER_SEL_BLE_2444_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2446(&mut self) -> LO_SDM_DITHER_SEL_BLE_2446_W<12> {
        LO_SDM_DITHER_SEL_BLE_2446_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2448(&mut self) -> LO_SDM_DITHER_SEL_BLE_2448_W<14> {
        LO_SDM_DITHER_SEL_BLE_2448_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2450(&mut self) -> LO_SDM_DITHER_SEL_BLE_2450_W<16> {
        LO_SDM_DITHER_SEL_BLE_2450_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2452(&mut self) -> LO_SDM_DITHER_SEL_BLE_2452_W<18> {
        LO_SDM_DITHER_SEL_BLE_2452_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2454(&mut self) -> LO_SDM_DITHER_SEL_BLE_2454_W<20> {
        LO_SDM_DITHER_SEL_BLE_2454_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2456(&mut self) -> LO_SDM_DITHER_SEL_BLE_2456_W<22> {
        LO_SDM_DITHER_SEL_BLE_2456_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2458(&mut self) -> LO_SDM_DITHER_SEL_BLE_2458_W<24> {
        LO_SDM_DITHER_SEL_BLE_2458_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2460(&mut self) -> LO_SDM_DITHER_SEL_BLE_2460_W<26> {
        LO_SDM_DITHER_SEL_BLE_2460_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2462(&mut self) -> LO_SDM_DITHER_SEL_BLE_2462_W<28> {
        LO_SDM_DITHER_SEL_BLE_2462_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2464(&mut self) -> LO_SDM_DITHER_SEL_BLE_2464_W<30> {
        LO_SDM_DITHER_SEL_BLE_2464_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw3](index.html) module"]
pub struct LO_SDM_CTRL_HW3_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw3::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw3::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw3 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

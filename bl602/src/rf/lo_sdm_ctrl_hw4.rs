#[doc = "Register `lo_sdm_ctrl_hw4` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_SDM_CTRL_HW4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_SDM_CTRL_HW4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw4` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW4_SPEC>;
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
impl From<crate::W<LO_SDM_CTRL_HW4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_SDM_CTRL_HW4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2466` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2466_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2466` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2466_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW4_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2468` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2468_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2468` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2468_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW4_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2470` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2470_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2470` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2470_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW4_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2472` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2472_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2472` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2472_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW4_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2474` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2474_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2474` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2474_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW4_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2476` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2476_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2476` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2476_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW4_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2478` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2478_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2478` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2478_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW4_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_2480` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_2480_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_2480` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_2480_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW4_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_sdm_dither_sel_ble_tx` reader - "]
pub type LO_SDM_DITHER_SEL_BLE_TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_sdm_dither_sel_ble_tx` writer - "]
pub type LO_SDM_DITHER_SEL_BLE_TX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_SDM_CTRL_HW4_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2466(&self) -> LO_SDM_DITHER_SEL_BLE_2466_R {
        LO_SDM_DITHER_SEL_BLE_2466_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2468(&self) -> LO_SDM_DITHER_SEL_BLE_2468_R {
        LO_SDM_DITHER_SEL_BLE_2468_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2470(&self) -> LO_SDM_DITHER_SEL_BLE_2470_R {
        LO_SDM_DITHER_SEL_BLE_2470_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2472(&self) -> LO_SDM_DITHER_SEL_BLE_2472_R {
        LO_SDM_DITHER_SEL_BLE_2472_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2474(&self) -> LO_SDM_DITHER_SEL_BLE_2474_R {
        LO_SDM_DITHER_SEL_BLE_2474_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2476(&self) -> LO_SDM_DITHER_SEL_BLE_2476_R {
        LO_SDM_DITHER_SEL_BLE_2476_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2478(&self) -> LO_SDM_DITHER_SEL_BLE_2478_R {
        LO_SDM_DITHER_SEL_BLE_2478_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2480(&self) -> LO_SDM_DITHER_SEL_BLE_2480_R {
        LO_SDM_DITHER_SEL_BLE_2480_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_tx(&self) -> LO_SDM_DITHER_SEL_BLE_TX_R {
        LO_SDM_DITHER_SEL_BLE_TX_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2466(&mut self) -> LO_SDM_DITHER_SEL_BLE_2466_W<0> {
        LO_SDM_DITHER_SEL_BLE_2466_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2468(&mut self) -> LO_SDM_DITHER_SEL_BLE_2468_W<2> {
        LO_SDM_DITHER_SEL_BLE_2468_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2470(&mut self) -> LO_SDM_DITHER_SEL_BLE_2470_W<4> {
        LO_SDM_DITHER_SEL_BLE_2470_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2472(&mut self) -> LO_SDM_DITHER_SEL_BLE_2472_W<6> {
        LO_SDM_DITHER_SEL_BLE_2472_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2474(&mut self) -> LO_SDM_DITHER_SEL_BLE_2474_W<8> {
        LO_SDM_DITHER_SEL_BLE_2474_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2476(&mut self) -> LO_SDM_DITHER_SEL_BLE_2476_W<10> {
        LO_SDM_DITHER_SEL_BLE_2476_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2478(&mut self) -> LO_SDM_DITHER_SEL_BLE_2478_W<12> {
        LO_SDM_DITHER_SEL_BLE_2478_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_2480(&mut self) -> LO_SDM_DITHER_SEL_BLE_2480_W<14> {
        LO_SDM_DITHER_SEL_BLE_2480_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn lo_sdm_dither_sel_ble_tx(&mut self) -> LO_SDM_DITHER_SEL_BLE_TX_W<16> {
        LO_SDM_DITHER_SEL_BLE_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw4](index.html) module"]
pub struct LO_SDM_CTRL_HW4_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw4::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw4::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw4 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `clk_cfg1` reader"]
pub struct R(crate::R<CLK_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg1` writer"]
pub struct W(crate::W<CLK_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG1_SPEC>;
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
impl From<crate::W<CLK_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifi_mac_core_div` reader - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
pub type WIFI_MAC_CORE_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifi_mac_core_div` writer - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
pub type WIFI_MAC_CORE_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `wifi_mac_wt_div` reader - WiFi encryption clock divider"]
pub type WIFI_MAC_WT_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifi_mac_wt_div` writer - WiFi encryption clock divider"]
pub type WIFI_MAC_WT_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `ble_clk_sel` reader - "]
pub type BLE_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ble_clk_sel` writer - "]
pub type BLE_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG1_SPEC, u8, u8, 6, O>;
#[doc = "Field `ble_en` reader - Bluetooth clock enable"]
pub type BLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `ble_en` writer - Bluetooth clock enable"]
pub type BLE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
    #[inline(always)]
    pub fn wifi_mac_core_div(&self) -> WIFI_MAC_CORE_DIV_R {
        WIFI_MAC_CORE_DIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - WiFi encryption clock divider"]
    #[inline(always)]
    pub fn wifi_mac_wt_div(&self) -> WIFI_MAC_WT_DIV_R {
        WIFI_MAC_WT_DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ble_clk_sel(&self) -> BLE_CLK_SEL_R {
        BLE_CLK_SEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Bluetooth clock enable"]
    #[inline(always)]
    pub fn ble_en(&self) -> BLE_EN_R {
        BLE_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_mac_core_div(&mut self) -> WIFI_MAC_CORE_DIV_W<0> {
        WIFI_MAC_CORE_DIV_W::new(self)
    }
    #[doc = "Bits 4:7 - WiFi encryption clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_mac_wt_div(&mut self) -> WIFI_MAC_WT_DIV_W<4> {
        WIFI_MAC_WT_DIV_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn ble_clk_sel(&mut self) -> BLE_CLK_SEL_W<16> {
        BLE_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 24 - Bluetooth clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ble_en(&mut self) -> BLE_EN_W<24> {
        BLE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clk_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg1](index.html) module"]
pub struct CLK_CFG1_SPEC;
impl crate::RegisterSpec for CLK_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg1::R](R) reader structure"]
impl crate::Readable for CLK_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg1::W](W) writer structure"]
impl crate::Writable for CLK_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk_cfg1 to value 0x0110_0001"]
impl crate::Resettable for CLK_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0110_0001;
}

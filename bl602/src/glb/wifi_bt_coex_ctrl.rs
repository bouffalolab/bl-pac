#[doc = "Register `WIFI_BT_COEX_CTRL` reader"]
pub struct R(crate::R<WIFI_BT_COEX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_BT_COEX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_BT_COEX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_BT_COEX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIFI_BT_COEX_CTRL` writer"]
pub struct W(crate::W<WIFI_BT_COEX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_BT_COEX_CTRL_SPEC>;
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
impl From<crate::W<WIFI_BT_COEX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_BT_COEX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `coex_bt_channel` reader - "]
pub type COEX_BT_CHANNEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `coex_bt_channel` writer - "]
pub type COEX_BT_CHANNEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_BT_COEX_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `coex_bt_pti` reader - "]
pub type COEX_BT_PTI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `coex_bt_pti` writer - "]
pub type COEX_BT_PTI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_BT_COEX_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `coex_bt_bw` reader - "]
pub type COEX_BT_BW_R = crate::BitReader<bool>;
#[doc = "Field `coex_bt_bw` writer - "]
pub type COEX_BT_BW_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFI_BT_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `en_gpio_bt_coex` reader - "]
pub type EN_GPIO_BT_COEX_R = crate::BitReader<bool>;
#[doc = "Field `en_gpio_bt_coex` writer - "]
pub type EN_GPIO_BT_COEX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_BT_COEX_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn coex_bt_channel(&self) -> COEX_BT_CHANNEL_R {
        COEX_BT_CHANNEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn coex_bt_pti(&self) -> COEX_BT_PTI_R {
        COEX_BT_PTI_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn coex_bt_bw(&self) -> COEX_BT_BW_R {
        COEX_BT_BW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn en_gpio_bt_coex(&self) -> EN_GPIO_BT_COEX_R {
        EN_GPIO_BT_COEX_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn coex_bt_channel(&mut self) -> COEX_BT_CHANNEL_W<0> {
        COEX_BT_CHANNEL_W::new(self)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    #[must_use]
    pub fn coex_bt_pti(&mut self) -> COEX_BT_PTI_W<7> {
        COEX_BT_PTI_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn coex_bt_bw(&mut self) -> COEX_BT_BW_W<11> {
        COEX_BT_BW_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn en_gpio_bt_coex(&mut self) -> EN_GPIO_BT_COEX_W<12> {
        EN_GPIO_BT_COEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WIFI_BT_COEX_CTRL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_bt_coex_ctrl](index.html) module"]
pub struct WIFI_BT_COEX_CTRL_SPEC;
impl crate::RegisterSpec for WIFI_BT_COEX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_bt_coex_ctrl::R](R) reader structure"]
impl crate::Readable for WIFI_BT_COEX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_bt_coex_ctrl::W](W) writer structure"]
impl crate::Writable for WIFI_BT_COEX_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WIFI_BT_COEX_CTRL to value 0"]
impl crate::Resettable for WIFI_BT_COEX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

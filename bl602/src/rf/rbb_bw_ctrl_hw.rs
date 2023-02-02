#[doc = "Register `rbb_bw_ctrl_hw` reader"]
pub struct R(crate::R<RBB_BW_CTRL_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_BW_CTRL_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB_BW_CTRL_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB_BW_CTRL_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb_bw_ctrl_hw` writer"]
pub struct W(crate::W<RBB_BW_CTRL_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_BW_CTRL_HW_SPEC>;
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
impl From<crate::W<RBB_BW_CTRL_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB_BW_CTRL_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rbb_bt_mode_ble` reader - "]
pub type RBB_BT_MODE_BLE_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bt_mode_ble` writer - "]
pub type RBB_BT_MODE_BLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RBB_BW_CTRL_HW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_ble(&self) -> RBB_BT_MODE_BLE_R {
        RBB_BT_MODE_BLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bt_mode_ble(&mut self) -> RBB_BT_MODE_BLE_W<0> {
        RBB_BT_MODE_BLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb_bw_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_bw_ctrl_hw](index.html) module"]
pub struct RBB_BW_CTRL_HW_SPEC;
impl crate::RegisterSpec for RBB_BW_CTRL_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb_bw_ctrl_hw::R](R) reader structure"]
impl crate::Readable for RBB_BW_CTRL_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb_bw_ctrl_hw::W](W) writer structure"]
impl crate::Writable for RBB_BW_CTRL_HW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rbb_bw_ctrl_hw to value 0"]
impl crate::Resettable for RBB_BW_CTRL_HW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `ef_data_0_lock` reader"]
pub struct R(crate::R<EF_DATA_0_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_DATA_0_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_DATA_0_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_DATA_0_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_data_0_lock` writer"]
pub struct W(crate::W<EF_DATA_0_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_DATA_0_LOCK_SPEC>;
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
impl From<crate::W<EF_DATA_0_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_DATA_0_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_ana_trim_1` reader - "]
pub type EF_ANA_TRIM_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ef_ana_trim_1` writer - "]
pub type EF_ANA_TRIM_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_DATA_0_LOCK_SPEC, u16, u16, 13, O>;
#[doc = "Field `wr_lock_key_slot_4_l` reader - "]
pub type WR_LOCK_KEY_SLOT_4_L_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_4_l` writer - "]
pub type WR_LOCK_KEY_SLOT_4_L_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_5_l` reader - "]
pub type WR_LOCK_KEY_SLOT_5_L_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_5_l` writer - "]
pub type WR_LOCK_KEY_SLOT_5_L_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_boot_mode` reader - "]
pub type WR_LOCK_BOOT_MODE_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_boot_mode` writer - "]
pub type WR_LOCK_BOOT_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_dbg_pwd` reader - "]
pub type WR_LOCK_DBG_PWD_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_dbg_pwd` writer - "]
pub type WR_LOCK_DBG_PWD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_sw_usage_0` reader - "]
pub type WR_LOCK_SW_USAGE_0_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_sw_usage_0` writer - "]
pub type WR_LOCK_SW_USAGE_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_wifi_mac` reader - "]
pub type WR_LOCK_WIFI_MAC_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_wifi_mac` writer - "]
pub type WR_LOCK_WIFI_MAC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_0` reader - "]
pub type WR_LOCK_KEY_SLOT_0_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_0` writer - "]
pub type WR_LOCK_KEY_SLOT_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_1` reader - "]
pub type WR_LOCK_KEY_SLOT_1_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_1` writer - "]
pub type WR_LOCK_KEY_SLOT_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_2` reader - "]
pub type WR_LOCK_KEY_SLOT_2_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_2` writer - "]
pub type WR_LOCK_KEY_SLOT_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_3` reader - "]
pub type WR_LOCK_KEY_SLOT_3_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_3` writer - "]
pub type WR_LOCK_KEY_SLOT_3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_4_h` reader - "]
pub type WR_LOCK_KEY_SLOT_4_H_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_4_h` writer - "]
pub type WR_LOCK_KEY_SLOT_4_H_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_5_h` reader - "]
pub type WR_LOCK_KEY_SLOT_5_H_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_5_h` writer - "]
pub type WR_LOCK_KEY_SLOT_5_H_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_dbg_pwd` reader - "]
pub type RD_LOCK_DBG_PWD_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_dbg_pwd` writer - "]
pub type RD_LOCK_DBG_PWD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_0` reader - "]
pub type RD_LOCK_KEY_SLOT_0_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_0` writer - "]
pub type RD_LOCK_KEY_SLOT_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_1` reader - "]
pub type RD_LOCK_KEY_SLOT_1_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_1` writer - "]
pub type RD_LOCK_KEY_SLOT_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_2` reader - "]
pub type RD_LOCK_KEY_SLOT_2_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_2` writer - "]
pub type RD_LOCK_KEY_SLOT_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_3` reader - "]
pub type RD_LOCK_KEY_SLOT_3_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_3` writer - "]
pub type RD_LOCK_KEY_SLOT_3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_4` reader - "]
pub type RD_LOCK_KEY_SLOT_4_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_4` writer - "]
pub type RD_LOCK_KEY_SLOT_4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_5` reader - "]
pub type RD_LOCK_KEY_SLOT_5_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_5` writer - "]
pub type RD_LOCK_KEY_SLOT_5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_0_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn ef_ana_trim_1(&self) -> EF_ANA_TRIM_1_R {
        EF_ANA_TRIM_1_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_l(&self) -> WR_LOCK_KEY_SLOT_4_L_R {
        WR_LOCK_KEY_SLOT_4_L_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_l(&self) -> WR_LOCK_KEY_SLOT_5_L_R {
        WR_LOCK_KEY_SLOT_5_L_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_lock_boot_mode(&self) -> WR_LOCK_BOOT_MODE_R {
        WR_LOCK_BOOT_MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wr_lock_dbg_pwd(&self) -> WR_LOCK_DBG_PWD_R {
        WR_LOCK_DBG_PWD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_0(&self) -> WR_LOCK_SW_USAGE_0_R {
        WR_LOCK_SW_USAGE_0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wr_lock_wifi_mac(&self) -> WR_LOCK_WIFI_MAC_R {
        WR_LOCK_WIFI_MAC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wr_lock_key_slot_0(&self) -> WR_LOCK_KEY_SLOT_0_R {
        WR_LOCK_KEY_SLOT_0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wr_lock_key_slot_1(&self) -> WR_LOCK_KEY_SLOT_1_R {
        WR_LOCK_KEY_SLOT_1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wr_lock_key_slot_2(&self) -> WR_LOCK_KEY_SLOT_2_R {
        WR_LOCK_KEY_SLOT_2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wr_lock_key_slot_3(&self) -> WR_LOCK_KEY_SLOT_3_R {
        WR_LOCK_KEY_SLOT_3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_h(&self) -> WR_LOCK_KEY_SLOT_4_H_R {
        WR_LOCK_KEY_SLOT_4_H_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_h(&self) -> WR_LOCK_KEY_SLOT_5_H_R {
        WR_LOCK_KEY_SLOT_5_H_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_lock_dbg_pwd(&self) -> RD_LOCK_DBG_PWD_R {
        RD_LOCK_DBG_PWD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_0(&self) -> RD_LOCK_KEY_SLOT_0_R {
        RD_LOCK_KEY_SLOT_0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_1(&self) -> RD_LOCK_KEY_SLOT_1_R {
        RD_LOCK_KEY_SLOT_1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_2(&self) -> RD_LOCK_KEY_SLOT_2_R {
        RD_LOCK_KEY_SLOT_2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_3(&self) -> RD_LOCK_KEY_SLOT_3_R {
        RD_LOCK_KEY_SLOT_3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rd_lock_key_slot_4(&self) -> RD_LOCK_KEY_SLOT_4_R {
        RD_LOCK_KEY_SLOT_4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_lock_key_slot_5(&self) -> RD_LOCK_KEY_SLOT_5_R {
        RD_LOCK_KEY_SLOT_5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    #[must_use]
    pub fn ef_ana_trim_1(&mut self) -> EF_ANA_TRIM_1_W<0> {
        EF_ANA_TRIM_1_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_4_l(&mut self) -> WR_LOCK_KEY_SLOT_4_L_W<13> {
        WR_LOCK_KEY_SLOT_4_L_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_5_l(&mut self) -> WR_LOCK_KEY_SLOT_5_L_W<14> {
        WR_LOCK_KEY_SLOT_5_L_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_boot_mode(&mut self) -> WR_LOCK_BOOT_MODE_W<15> {
        WR_LOCK_BOOT_MODE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_dbg_pwd(&mut self) -> WR_LOCK_DBG_PWD_W<16> {
        WR_LOCK_DBG_PWD_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_sw_usage_0(&mut self) -> WR_LOCK_SW_USAGE_0_W<17> {
        WR_LOCK_SW_USAGE_0_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_wifi_mac(&mut self) -> WR_LOCK_WIFI_MAC_W<18> {
        WR_LOCK_WIFI_MAC_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_0(&mut self) -> WR_LOCK_KEY_SLOT_0_W<19> {
        WR_LOCK_KEY_SLOT_0_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_1(&mut self) -> WR_LOCK_KEY_SLOT_1_W<20> {
        WR_LOCK_KEY_SLOT_1_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_2(&mut self) -> WR_LOCK_KEY_SLOT_2_W<21> {
        WR_LOCK_KEY_SLOT_2_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_3(&mut self) -> WR_LOCK_KEY_SLOT_3_W<22> {
        WR_LOCK_KEY_SLOT_3_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_4_h(&mut self) -> WR_LOCK_KEY_SLOT_4_H_W<23> {
        WR_LOCK_KEY_SLOT_4_H_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_5_h(&mut self) -> WR_LOCK_KEY_SLOT_5_H_W<24> {
        WR_LOCK_KEY_SLOT_5_H_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_dbg_pwd(&mut self) -> RD_LOCK_DBG_PWD_W<25> {
        RD_LOCK_DBG_PWD_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_0(&mut self) -> RD_LOCK_KEY_SLOT_0_W<26> {
        RD_LOCK_KEY_SLOT_0_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_1(&mut self) -> RD_LOCK_KEY_SLOT_1_W<27> {
        RD_LOCK_KEY_SLOT_1_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_2(&mut self) -> RD_LOCK_KEY_SLOT_2_W<28> {
        RD_LOCK_KEY_SLOT_2_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_3(&mut self) -> RD_LOCK_KEY_SLOT_3_W<29> {
        RD_LOCK_KEY_SLOT_3_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_4(&mut self) -> RD_LOCK_KEY_SLOT_4_W<30> {
        RD_LOCK_KEY_SLOT_4_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_5(&mut self) -> RD_LOCK_KEY_SLOT_5_W<31> {
        RD_LOCK_KEY_SLOT_5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_data_0_lock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_data_0_lock](index.html) module"]
pub struct EF_DATA_0_LOCK_SPEC;
impl crate::RegisterSpec for EF_DATA_0_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_data_0_lock::R](R) reader structure"]
impl crate::Readable for EF_DATA_0_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_data_0_lock::W](W) writer structure"]
impl crate::Writable for EF_DATA_0_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_data_0_lock to value 0"]
impl crate::Resettable for EF_DATA_0_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

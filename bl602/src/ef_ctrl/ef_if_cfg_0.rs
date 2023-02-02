#[doc = "Register `ef_if_cfg_0` reader"]
pub struct R(crate::R<EF_IF_CFG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_CFG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_IF_CFG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_IF_CFG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ef_if_sf_aes_mode` reader - "]
pub type EF_IF_SF_AES_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_sboot_sign_mode` reader - "]
pub type EF_IF_SBOOT_SIGN_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_sboot_en` reader - "]
pub type EF_IF_SBOOT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_cpu1_enc_en` reader - "]
pub type EF_IF_CPU1_ENC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_cpu0_enc_en` reader - "]
pub type EF_IF_CPU0_ENC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_boot_sel` reader - "]
pub type EF_IF_BOOT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_sw_usage_1` reader - "]
pub type EF_IF_SW_USAGE_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_sdu_dis` reader - "]
pub type EF_IF_SDU_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_ble_dis` reader - "]
pub type EF_IF_BLE_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_wifi_dis` reader - "]
pub type EF_IF_WIFI_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_key_enc_en` reader - "]
pub type EF_IF_0_KEY_ENC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_cam_dis` reader - "]
pub type EF_IF_CAM_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_sf_dis` reader - "]
pub type EF_IF_SF_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_cpu1_dis` reader - "]
pub type EF_IF_CPU1_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_cpu_rst_dbg_dis` reader - "]
pub type EF_IF_CPU_RST_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_se_dbg_dis` reader - "]
pub type EF_IF_SE_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_efuse_dbg_dis` reader - "]
pub type EF_IF_EFUSE_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_dbg_jtag_1_dis` reader - "]
pub type EF_IF_DBG_JTAG_1_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_dbg_jtag_0_dis` reader - "]
pub type EF_IF_DBG_JTAG_0_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_dbg_mode` reader - "]
pub type EF_IF_DBG_MODE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_if_sf_aes_mode(&self) -> EF_IF_SF_AES_MODE_R {
        EF_IF_SF_AES_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_if_sboot_sign_mode(&self) -> EF_IF_SBOOT_SIGN_MODE_R {
        EF_IF_SBOOT_SIGN_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_if_sboot_en(&self) -> EF_IF_SBOOT_EN_R {
        EF_IF_SBOOT_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_cpu1_enc_en(&self) -> EF_IF_CPU1_ENC_EN_R {
        EF_IF_CPU1_ENC_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_if_cpu0_enc_en(&self) -> EF_IF_CPU0_ENC_EN_R {
        EF_IF_CPU0_ENC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ef_if_boot_sel(&self) -> EF_IF_BOOT_SEL_R {
        EF_IF_BOOT_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_if_sw_usage_1(&self) -> EF_IF_SW_USAGE_1_R {
        EF_IF_SW_USAGE_1_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_sdu_dis(&self) -> EF_IF_SDU_DIS_R {
        EF_IF_SDU_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_ble_dis(&self) -> EF_IF_BLE_DIS_R {
        EF_IF_BLE_DIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_wifi_dis(&self) -> EF_IF_WIFI_DIS_R {
        EF_IF_WIFI_DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_if_0_key_enc_en(&self) -> EF_IF_0_KEY_ENC_EN_R {
        EF_IF_0_KEY_ENC_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_cam_dis(&self) -> EF_IF_CAM_DIS_R {
        EF_IF_CAM_DIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_sf_dis(&self) -> EF_IF_SF_DIS_R {
        EF_IF_SF_DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_cpu1_dis(&self) -> EF_IF_CPU1_DIS_R {
        EF_IF_CPU1_DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_cpu_rst_dbg_dis(&self) -> EF_IF_CPU_RST_DBG_DIS_R {
        EF_IF_CPU_RST_DBG_DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_se_dbg_dis(&self) -> EF_IF_SE_DBG_DIS_R {
        EF_IF_SE_DBG_DIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_if_efuse_dbg_dis(&self) -> EF_IF_EFUSE_DBG_DIS_R {
        EF_IF_EFUSE_DBG_DIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_1_dis(&self) -> EF_IF_DBG_JTAG_1_DIS_R {
        EF_IF_DBG_JTAG_1_DIS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_0_dis(&self) -> EF_IF_DBG_JTAG_0_DIS_R {
        EF_IF_DBG_JTAG_0_DIS_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_if_dbg_mode(&self) -> EF_IF_DBG_MODE_R {
        EF_IF_DBG_MODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "ef_if_cfg_0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_cfg_0](index.html) module"]
pub struct EF_IF_CFG_0_SPEC;
impl crate::RegisterSpec for EF_IF_CFG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_cfg_0::R](R) reader structure"]
impl crate::Readable for EF_IF_CFG_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ef_if_cfg_0 to value 0"]
impl crate::Resettable for EF_IF_CFG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

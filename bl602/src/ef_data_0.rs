#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ef_cfg_0."]
    pub ef_cfg_0: EF_CFG_0,
    #[doc = "0x04 - ef_dbg_pwd_low."]
    pub ef_dbg_pwd_low: EF_DBG_PWD_LOW,
    #[doc = "0x08 - ef_dbg_pwd_high."]
    pub ef_dbg_pwd_high: EF_DBG_PWD_HIGH,
    #[doc = "0x0c - ef_ana_trim_0."]
    pub ef_ana_trim_0: EF_ANA_TRIM_0,
    #[doc = "0x10 - ef_sw_usage_0."]
    pub ef_sw_usage_0: EF_SW_USAGE_0,
    #[doc = "0x14 - ef_wifi_mac_low."]
    pub ef_wifi_mac_low: EF_WIFI_MAC_LOW,
    #[doc = "0x18 - ef_wifi_mac_high."]
    pub ef_wifi_mac_high: EF_WIFI_MAC_HIGH,
    #[doc = "0x1c - ef_key_slot_0_w0."]
    pub ef_key_slot_0_w0: EF_KEY_SLOT_0_W0,
    #[doc = "0x20 - ef_key_slot_0_w1."]
    pub ef_key_slot_0_w1: EF_KEY_SLOT_0_W1,
    #[doc = "0x24 - ef_key_slot_0_w2."]
    pub ef_key_slot_0_w2: EF_KEY_SLOT_0_W2,
    #[doc = "0x28 - ef_key_slot_0_w3."]
    pub ef_key_slot_0_w3: EF_KEY_SLOT_0_W3,
    #[doc = "0x2c - ef_key_slot_1_w0."]
    pub ef_key_slot_1_w0: EF_KEY_SLOT_1_W0,
    #[doc = "0x30 - ef_key_slot_1_w1."]
    pub ef_key_slot_1_w1: EF_KEY_SLOT_1_W1,
    #[doc = "0x34 - ef_key_slot_1_w2."]
    pub ef_key_slot_1_w2: EF_KEY_SLOT_1_W2,
    #[doc = "0x38 - ef_key_slot_1_w3."]
    pub ef_key_slot_1_w3: EF_KEY_SLOT_1_W3,
    #[doc = "0x3c - ef_key_slot_2_w0."]
    pub ef_key_slot_2_w0: EF_KEY_SLOT_2_W0,
    #[doc = "0x40 - ef_key_slot_2_w1."]
    pub ef_key_slot_2_w1: EF_KEY_SLOT_2_W1,
    #[doc = "0x44 - ef_key_slot_2_w2."]
    pub ef_key_slot_2_w2: EF_KEY_SLOT_2_W2,
    #[doc = "0x48 - ef_key_slot_2_w3."]
    pub ef_key_slot_2_w3: EF_KEY_SLOT_2_W3,
    #[doc = "0x4c - ef_key_slot_3_w0."]
    pub ef_key_slot_3_w0: EF_KEY_SLOT_3_W0,
    #[doc = "0x50 - ef_key_slot_3_w1."]
    pub ef_key_slot_3_w1: EF_KEY_SLOT_3_W1,
    #[doc = "0x54 - ef_key_slot_3_w2."]
    pub ef_key_slot_3_w2: EF_KEY_SLOT_3_W2,
    #[doc = "0x58 - ef_key_slot_3_w3."]
    pub ef_key_slot_3_w3: EF_KEY_SLOT_3_W3,
    #[doc = "0x5c - ef_key_slot_4_w0."]
    pub ef_key_slot_4_w0: EF_KEY_SLOT_4_W0,
    #[doc = "0x60 - ef_key_slot_4_w1."]
    pub ef_key_slot_4_w1: EF_KEY_SLOT_4_W1,
    #[doc = "0x64 - ef_key_slot_4_w2."]
    pub ef_key_slot_4_w2: EF_KEY_SLOT_4_W2,
    #[doc = "0x68 - ef_key_slot_4_w3."]
    pub ef_key_slot_4_w3: EF_KEY_SLOT_4_W3,
    #[doc = "0x6c - ef_key_slot_5_w0."]
    pub ef_key_slot_5_w0: EF_KEY_SLOT_5_W0,
    #[doc = "0x70 - ef_key_slot_5_w1."]
    pub ef_key_slot_5_w1: EF_KEY_SLOT_5_W1,
    #[doc = "0x74 - ef_key_slot_5_w2."]
    pub ef_key_slot_5_w2: EF_KEY_SLOT_5_W2,
    #[doc = "0x78 - ef_key_slot_5_w3."]
    pub ef_key_slot_5_w3: EF_KEY_SLOT_5_W3,
    #[doc = "0x7c - ef_data_0_lock."]
    pub ef_data_0_lock: EF_DATA_0_LOCK,
}
#[doc = "ef_cfg_0 (rw) register accessor: an alias for `Reg<EF_CFG_0_SPEC>`"]
pub type EF_CFG_0 = crate::Reg<ef_cfg_0::EF_CFG_0_SPEC>;
#[doc = "ef_cfg_0."]
pub mod ef_cfg_0;
#[doc = "ef_dbg_pwd_low (rw) register accessor: an alias for `Reg<EF_DBG_PWD_LOW_SPEC>`"]
pub type EF_DBG_PWD_LOW = crate::Reg<ef_dbg_pwd_low::EF_DBG_PWD_LOW_SPEC>;
#[doc = "ef_dbg_pwd_low."]
pub mod ef_dbg_pwd_low;
#[doc = "ef_dbg_pwd_high (rw) register accessor: an alias for `Reg<EF_DBG_PWD_HIGH_SPEC>`"]
pub type EF_DBG_PWD_HIGH = crate::Reg<ef_dbg_pwd_high::EF_DBG_PWD_HIGH_SPEC>;
#[doc = "ef_dbg_pwd_high."]
pub mod ef_dbg_pwd_high;
#[doc = "ef_ana_trim_0 (rw) register accessor: an alias for `Reg<EF_ANA_TRIM_0_SPEC>`"]
pub type EF_ANA_TRIM_0 = crate::Reg<ef_ana_trim_0::EF_ANA_TRIM_0_SPEC>;
#[doc = "ef_ana_trim_0."]
pub mod ef_ana_trim_0;
#[doc = "ef_sw_usage_0 (rw) register accessor: an alias for `Reg<EF_SW_USAGE_0_SPEC>`"]
pub type EF_SW_USAGE_0 = crate::Reg<ef_sw_usage_0::EF_SW_USAGE_0_SPEC>;
#[doc = "ef_sw_usage_0."]
pub mod ef_sw_usage_0;
#[doc = "ef_wifi_mac_low (rw) register accessor: an alias for `Reg<EF_WIFI_MAC_LOW_SPEC>`"]
pub type EF_WIFI_MAC_LOW = crate::Reg<ef_wifi_mac_low::EF_WIFI_MAC_LOW_SPEC>;
#[doc = "ef_wifi_mac_low."]
pub mod ef_wifi_mac_low;
#[doc = "ef_wifi_mac_high (rw) register accessor: an alias for `Reg<EF_WIFI_MAC_HIGH_SPEC>`"]
pub type EF_WIFI_MAC_HIGH = crate::Reg<ef_wifi_mac_high::EF_WIFI_MAC_HIGH_SPEC>;
#[doc = "ef_wifi_mac_high."]
pub mod ef_wifi_mac_high;
#[doc = "ef_key_slot_0_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_0_W0_SPEC>`"]
pub type EF_KEY_SLOT_0_W0 = crate::Reg<ef_key_slot_0_w0::EF_KEY_SLOT_0_W0_SPEC>;
#[doc = "ef_key_slot_0_w0."]
pub mod ef_key_slot_0_w0;
#[doc = "ef_key_slot_0_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_0_W1_SPEC>`"]
pub type EF_KEY_SLOT_0_W1 = crate::Reg<ef_key_slot_0_w1::EF_KEY_SLOT_0_W1_SPEC>;
#[doc = "ef_key_slot_0_w1."]
pub mod ef_key_slot_0_w1;
#[doc = "ef_key_slot_0_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_0_W2_SPEC>`"]
pub type EF_KEY_SLOT_0_W2 = crate::Reg<ef_key_slot_0_w2::EF_KEY_SLOT_0_W2_SPEC>;
#[doc = "ef_key_slot_0_w2."]
pub mod ef_key_slot_0_w2;
#[doc = "ef_key_slot_0_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_0_W3_SPEC>`"]
pub type EF_KEY_SLOT_0_W3 = crate::Reg<ef_key_slot_0_w3::EF_KEY_SLOT_0_W3_SPEC>;
#[doc = "ef_key_slot_0_w3."]
pub mod ef_key_slot_0_w3;
#[doc = "ef_key_slot_1_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_1_W0_SPEC>`"]
pub type EF_KEY_SLOT_1_W0 = crate::Reg<ef_key_slot_1_w0::EF_KEY_SLOT_1_W0_SPEC>;
#[doc = "ef_key_slot_1_w0."]
pub mod ef_key_slot_1_w0;
#[doc = "ef_key_slot_1_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_1_W1_SPEC>`"]
pub type EF_KEY_SLOT_1_W1 = crate::Reg<ef_key_slot_1_w1::EF_KEY_SLOT_1_W1_SPEC>;
#[doc = "ef_key_slot_1_w1."]
pub mod ef_key_slot_1_w1;
#[doc = "ef_key_slot_1_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_1_W2_SPEC>`"]
pub type EF_KEY_SLOT_1_W2 = crate::Reg<ef_key_slot_1_w2::EF_KEY_SLOT_1_W2_SPEC>;
#[doc = "ef_key_slot_1_w2."]
pub mod ef_key_slot_1_w2;
#[doc = "ef_key_slot_1_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_1_W3_SPEC>`"]
pub type EF_KEY_SLOT_1_W3 = crate::Reg<ef_key_slot_1_w3::EF_KEY_SLOT_1_W3_SPEC>;
#[doc = "ef_key_slot_1_w3."]
pub mod ef_key_slot_1_w3;
#[doc = "ef_key_slot_2_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_2_W0_SPEC>`"]
pub type EF_KEY_SLOT_2_W0 = crate::Reg<ef_key_slot_2_w0::EF_KEY_SLOT_2_W0_SPEC>;
#[doc = "ef_key_slot_2_w0."]
pub mod ef_key_slot_2_w0;
#[doc = "ef_key_slot_2_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_2_W1_SPEC>`"]
pub type EF_KEY_SLOT_2_W1 = crate::Reg<ef_key_slot_2_w1::EF_KEY_SLOT_2_W1_SPEC>;
#[doc = "ef_key_slot_2_w1."]
pub mod ef_key_slot_2_w1;
#[doc = "ef_key_slot_2_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_2_W2_SPEC>`"]
pub type EF_KEY_SLOT_2_W2 = crate::Reg<ef_key_slot_2_w2::EF_KEY_SLOT_2_W2_SPEC>;
#[doc = "ef_key_slot_2_w2."]
pub mod ef_key_slot_2_w2;
#[doc = "ef_key_slot_2_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_2_W3_SPEC>`"]
pub type EF_KEY_SLOT_2_W3 = crate::Reg<ef_key_slot_2_w3::EF_KEY_SLOT_2_W3_SPEC>;
#[doc = "ef_key_slot_2_w3."]
pub mod ef_key_slot_2_w3;
#[doc = "ef_key_slot_3_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_3_W0_SPEC>`"]
pub type EF_KEY_SLOT_3_W0 = crate::Reg<ef_key_slot_3_w0::EF_KEY_SLOT_3_W0_SPEC>;
#[doc = "ef_key_slot_3_w0."]
pub mod ef_key_slot_3_w0;
#[doc = "ef_key_slot_3_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_3_W1_SPEC>`"]
pub type EF_KEY_SLOT_3_W1 = crate::Reg<ef_key_slot_3_w1::EF_KEY_SLOT_3_W1_SPEC>;
#[doc = "ef_key_slot_3_w1."]
pub mod ef_key_slot_3_w1;
#[doc = "ef_key_slot_3_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_3_W2_SPEC>`"]
pub type EF_KEY_SLOT_3_W2 = crate::Reg<ef_key_slot_3_w2::EF_KEY_SLOT_3_W2_SPEC>;
#[doc = "ef_key_slot_3_w2."]
pub mod ef_key_slot_3_w2;
#[doc = "ef_key_slot_3_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_3_W3_SPEC>`"]
pub type EF_KEY_SLOT_3_W3 = crate::Reg<ef_key_slot_3_w3::EF_KEY_SLOT_3_W3_SPEC>;
#[doc = "ef_key_slot_3_w3."]
pub mod ef_key_slot_3_w3;
#[doc = "ef_key_slot_4_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_4_W0_SPEC>`"]
pub type EF_KEY_SLOT_4_W0 = crate::Reg<ef_key_slot_4_w0::EF_KEY_SLOT_4_W0_SPEC>;
#[doc = "ef_key_slot_4_w0."]
pub mod ef_key_slot_4_w0;
#[doc = "ef_key_slot_4_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_4_W1_SPEC>`"]
pub type EF_KEY_SLOT_4_W1 = crate::Reg<ef_key_slot_4_w1::EF_KEY_SLOT_4_W1_SPEC>;
#[doc = "ef_key_slot_4_w1."]
pub mod ef_key_slot_4_w1;
#[doc = "ef_key_slot_4_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_4_W2_SPEC>`"]
pub type EF_KEY_SLOT_4_W2 = crate::Reg<ef_key_slot_4_w2::EF_KEY_SLOT_4_W2_SPEC>;
#[doc = "ef_key_slot_4_w2."]
pub mod ef_key_slot_4_w2;
#[doc = "ef_key_slot_4_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_4_W3_SPEC>`"]
pub type EF_KEY_SLOT_4_W3 = crate::Reg<ef_key_slot_4_w3::EF_KEY_SLOT_4_W3_SPEC>;
#[doc = "ef_key_slot_4_w3."]
pub mod ef_key_slot_4_w3;
#[doc = "ef_key_slot_5_w0 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_5_W0_SPEC>`"]
pub type EF_KEY_SLOT_5_W0 = crate::Reg<ef_key_slot_5_w0::EF_KEY_SLOT_5_W0_SPEC>;
#[doc = "ef_key_slot_5_w0."]
pub mod ef_key_slot_5_w0;
#[doc = "ef_key_slot_5_w1 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_5_W1_SPEC>`"]
pub type EF_KEY_SLOT_5_W1 = crate::Reg<ef_key_slot_5_w1::EF_KEY_SLOT_5_W1_SPEC>;
#[doc = "ef_key_slot_5_w1."]
pub mod ef_key_slot_5_w1;
#[doc = "ef_key_slot_5_w2 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_5_W2_SPEC>`"]
pub type EF_KEY_SLOT_5_W2 = crate::Reg<ef_key_slot_5_w2::EF_KEY_SLOT_5_W2_SPEC>;
#[doc = "ef_key_slot_5_w2."]
pub mod ef_key_slot_5_w2;
#[doc = "ef_key_slot_5_w3 (rw) register accessor: an alias for `Reg<EF_KEY_SLOT_5_W3_SPEC>`"]
pub type EF_KEY_SLOT_5_W3 = crate::Reg<ef_key_slot_5_w3::EF_KEY_SLOT_5_W3_SPEC>;
#[doc = "ef_key_slot_5_w3."]
pub mod ef_key_slot_5_w3;
#[doc = "ef_data_0_lock (rw) register accessor: an alias for `Reg<EF_DATA_0_LOCK_SPEC>`"]
pub type EF_DATA_0_LOCK = crate::Reg<ef_data_0_lock::EF_DATA_0_LOCK_SPEC>;
#[doc = "ef_data_0_lock."]
pub mod ef_data_0_lock;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - irtx_config."]
    pub irtx_config: IRTX_CONFIG,
    #[doc = "0x04 - irtx_int_sts."]
    pub irtx_int_sts: IRTX_INT_STS,
    #[doc = "0x08 - irtx_data_word0."]
    pub irtx_data_word0: IRTX_DATA_WORD0,
    #[doc = "0x0c - irtx_data_word1."]
    pub irtx_data_word1: IRTX_DATA_WORD1,
    #[doc = "0x10 - irtx_pulse_width."]
    pub irtx_pulse_width: IRTX_PULSE_WIDTH,
    #[doc = "0x14 - irtx_pw."]
    pub irtx_pw: IRTX_PW,
    _reserved6: [u8; 0x28],
    #[doc = "0x40 - irtx_swm_pw_0."]
    pub irtx_swm_pw_0: IRTX_SWM_PW_0,
    #[doc = "0x44 - irtx_swm_pw_1."]
    pub irtx_swm_pw_1: IRTX_SWM_PW_1,
    #[doc = "0x48 - irtx_swm_pw_2."]
    pub irtx_swm_pw_2: IRTX_SWM_PW_2,
    #[doc = "0x4c - irtx_swm_pw_3."]
    pub irtx_swm_pw_3: IRTX_SWM_PW_3,
    #[doc = "0x50 - irtx_swm_pw_4."]
    pub irtx_swm_pw_4: IRTX_SWM_PW_4,
    #[doc = "0x54 - irtx_swm_pw_5."]
    pub irtx_swm_pw_5: IRTX_SWM_PW_5,
    #[doc = "0x58 - irtx_swm_pw_6."]
    pub irtx_swm_pw_6: IRTX_SWM_PW_6,
    #[doc = "0x5c - irtx_swm_pw_7."]
    pub irtx_swm_pw_7: IRTX_SWM_PW_7,
    _reserved14: [u8; 0x20],
    #[doc = "0x80 - irrx_config."]
    pub irrx_config: IRRX_CONFIG,
    #[doc = "0x84 - irrx_int_sts."]
    pub irrx_int_sts: IRRX_INT_STS,
    #[doc = "0x88 - irrx_pw_config."]
    pub irrx_pw_config: IRRX_PW_CONFIG,
    _reserved17: [u8; 0x04],
    #[doc = "0x90 - irrx_data_count."]
    pub irrx_data_count: IRRX_DATA_COUNT,
    #[doc = "0x94 - irrx_data_word0."]
    pub irrx_data_word0: IRRX_DATA_WORD0,
    #[doc = "0x98 - irrx_data_word1."]
    pub irrx_data_word1: IRRX_DATA_WORD1,
    _reserved20: [u8; 0x24],
    #[doc = "0xc0 - irrx_swm_fifo_config_0."]
    pub irrx_swm_fifo_config_0: IRRX_SWM_FIFO_CONFIG_0,
    #[doc = "0xc4 - irrx_swm_fifo_rdata."]
    pub irrx_swm_fifo_rdata: IRRX_SWM_FIFO_RDATA,
}
#[doc = "irtx_config (rw) register accessor: an alias for `Reg<IRTX_CONFIG_SPEC>`"]
pub type IRTX_CONFIG = crate::Reg<irtx_config::IRTX_CONFIG_SPEC>;
#[doc = "irtx_config."]
pub mod irtx_config;
#[doc = "irtx_int_sts (rw) register accessor: an alias for `Reg<IRTX_INT_STS_SPEC>`"]
pub type IRTX_INT_STS = crate::Reg<irtx_int_sts::IRTX_INT_STS_SPEC>;
#[doc = "irtx_int_sts."]
pub mod irtx_int_sts;
#[doc = "irtx_data_word0 (rw) register accessor: an alias for `Reg<IRTX_DATA_WORD0_SPEC>`"]
pub type IRTX_DATA_WORD0 = crate::Reg<irtx_data_word0::IRTX_DATA_WORD0_SPEC>;
#[doc = "irtx_data_word0."]
pub mod irtx_data_word0;
#[doc = "irtx_data_word1 (rw) register accessor: an alias for `Reg<IRTX_DATA_WORD1_SPEC>`"]
pub type IRTX_DATA_WORD1 = crate::Reg<irtx_data_word1::IRTX_DATA_WORD1_SPEC>;
#[doc = "irtx_data_word1."]
pub mod irtx_data_word1;
#[doc = "irtx_pulse_width (rw) register accessor: an alias for `Reg<IRTX_PULSE_WIDTH_SPEC>`"]
pub type IRTX_PULSE_WIDTH = crate::Reg<irtx_pulse_width::IRTX_PULSE_WIDTH_SPEC>;
#[doc = "irtx_pulse_width."]
pub mod irtx_pulse_width;
#[doc = "irtx_pw (rw) register accessor: an alias for `Reg<IRTX_PW_SPEC>`"]
pub type IRTX_PW = crate::Reg<irtx_pw::IRTX_PW_SPEC>;
#[doc = "irtx_pw."]
pub mod irtx_pw;
#[doc = "irtx_swm_pw_0 (rw) register accessor: an alias for `Reg<IRTX_SWM_PW_0_SPEC>`"]
pub type IRTX_SWM_PW_0 = crate::Reg<irtx_swm_pw_0::IRTX_SWM_PW_0_SPEC>;
#[doc = "irtx_swm_pw_0."]
pub mod irtx_swm_pw_0;
#[doc = "irtx_swm_pw_1 (rw) register accessor: an alias for `Reg<IRTX_SWM_PW_1_SPEC>`"]
pub type IRTX_SWM_PW_1 = crate::Reg<irtx_swm_pw_1::IRTX_SWM_PW_1_SPEC>;
#[doc = "irtx_swm_pw_1."]
pub mod irtx_swm_pw_1;
#[doc = "irtx_swm_pw_2 (rw) register accessor: an alias for `Reg<IRTX_SWM_PW_2_SPEC>`"]
pub type IRTX_SWM_PW_2 = crate::Reg<irtx_swm_pw_2::IRTX_SWM_PW_2_SPEC>;
#[doc = "irtx_swm_pw_2."]
pub mod irtx_swm_pw_2;
#[doc = "irtx_swm_pw_3 (rw) register accessor: an alias for `Reg<IRTX_SWM_PW_3_SPEC>`"]
pub type IRTX_SWM_PW_3 = crate::Reg<irtx_swm_pw_3::IRTX_SWM_PW_3_SPEC>;
#[doc = "irtx_swm_pw_3."]
pub mod irtx_swm_pw_3;
#[doc = "irtx_swm_pw_4 (rw) register accessor: an alias for `Reg<IRTX_SWM_PW_4_SPEC>`"]
pub type IRTX_SWM_PW_4 = crate::Reg<irtx_swm_pw_4::IRTX_SWM_PW_4_SPEC>;
#[doc = "irtx_swm_pw_4."]
pub mod irtx_swm_pw_4;
#[doc = "irtx_swm_pw_5 (rw) register accessor: an alias for `Reg<IRTX_SWM_PW_5_SPEC>`"]
pub type IRTX_SWM_PW_5 = crate::Reg<irtx_swm_pw_5::IRTX_SWM_PW_5_SPEC>;
#[doc = "irtx_swm_pw_5."]
pub mod irtx_swm_pw_5;
#[doc = "irtx_swm_pw_6 (rw) register accessor: an alias for `Reg<IRTX_SWM_PW_6_SPEC>`"]
pub type IRTX_SWM_PW_6 = crate::Reg<irtx_swm_pw_6::IRTX_SWM_PW_6_SPEC>;
#[doc = "irtx_swm_pw_6."]
pub mod irtx_swm_pw_6;
#[doc = "irtx_swm_pw_7 (rw) register accessor: an alias for `Reg<IRTX_SWM_PW_7_SPEC>`"]
pub type IRTX_SWM_PW_7 = crate::Reg<irtx_swm_pw_7::IRTX_SWM_PW_7_SPEC>;
#[doc = "irtx_swm_pw_7."]
pub mod irtx_swm_pw_7;
#[doc = "irrx_config (rw) register accessor: an alias for `Reg<IRRX_CONFIG_SPEC>`"]
pub type IRRX_CONFIG = crate::Reg<irrx_config::IRRX_CONFIG_SPEC>;
#[doc = "irrx_config."]
pub mod irrx_config;
#[doc = "irrx_int_sts (rw) register accessor: an alias for `Reg<IRRX_INT_STS_SPEC>`"]
pub type IRRX_INT_STS = crate::Reg<irrx_int_sts::IRRX_INT_STS_SPEC>;
#[doc = "irrx_int_sts."]
pub mod irrx_int_sts;
#[doc = "irrx_pw_config (rw) register accessor: an alias for `Reg<IRRX_PW_CONFIG_SPEC>`"]
pub type IRRX_PW_CONFIG = crate::Reg<irrx_pw_config::IRRX_PW_CONFIG_SPEC>;
#[doc = "irrx_pw_config."]
pub mod irrx_pw_config;
#[doc = "irrx_data_count (rw) register accessor: an alias for `Reg<IRRX_DATA_COUNT_SPEC>`"]
pub type IRRX_DATA_COUNT = crate::Reg<irrx_data_count::IRRX_DATA_COUNT_SPEC>;
#[doc = "irrx_data_count."]
pub mod irrx_data_count;
#[doc = "irrx_data_word0 (rw) register accessor: an alias for `Reg<IRRX_DATA_WORD0_SPEC>`"]
pub type IRRX_DATA_WORD0 = crate::Reg<irrx_data_word0::IRRX_DATA_WORD0_SPEC>;
#[doc = "irrx_data_word0."]
pub mod irrx_data_word0;
#[doc = "irrx_data_word1 (rw) register accessor: an alias for `Reg<IRRX_DATA_WORD1_SPEC>`"]
pub type IRRX_DATA_WORD1 = crate::Reg<irrx_data_word1::IRRX_DATA_WORD1_SPEC>;
#[doc = "irrx_data_word1."]
pub mod irrx_data_word1;
#[doc = "irrx_swm_fifo_config_0 (rw) register accessor: an alias for `Reg<IRRX_SWM_FIFO_CONFIG_0_SPEC>`"]
pub type IRRX_SWM_FIFO_CONFIG_0 = crate::Reg<irrx_swm_fifo_config_0::IRRX_SWM_FIFO_CONFIG_0_SPEC>;
#[doc = "irrx_swm_fifo_config_0."]
pub mod irrx_swm_fifo_config_0;
#[doc = "irrx_swm_fifo_rdata (r) register accessor: an alias for `Reg<IRRX_SWM_FIFO_RDATA_SPEC>`"]
pub type IRRX_SWM_FIFO_RDATA = crate::Reg<irrx_swm_fifo_rdata::IRRX_SWM_FIFO_RDATA_SPEC>;
#[doc = "irrx_swm_fifo_rdata."]
pub mod irrx_swm_fifo_rdata;

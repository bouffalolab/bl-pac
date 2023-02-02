#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - l1c_config."]
    pub l1c_config: L1C_CONFIG,
    #[doc = "0x04 - hit_cnt_lsb."]
    pub hit_cnt_lsb: HIT_CNT_LSB,
    #[doc = "0x08 - hit_cnt_msb."]
    pub hit_cnt_msb: HIT_CNT_MSB,
    #[doc = "0x0c - miss_cnt."]
    pub miss_cnt: MISS_CNT,
    #[doc = "0x10 - l1c_range."]
    pub l1c_range: L1C_RANGE,
    _reserved5: [u8; 0x01ec],
    #[doc = "0x200 - l1c_bmx_err_addr_en."]
    pub l1c_bmx_err_addr_en: L1C_BMX_ERR_ADDR_EN,
    #[doc = "0x204 - l1c_bmx_err_addr."]
    pub l1c_bmx_err_addr: L1C_BMX_ERR_ADDR,
    #[doc = "0x208 - irom1_misr_dataout_0."]
    pub irom1_misr_dataout_0: IROM1_MISR_DATAOUT_0,
    #[doc = "0x20c - irom1_misr_dataout_1."]
    pub irom1_misr_dataout_1: IROM1_MISR_DATAOUT_1,
    #[doc = "0x210 - cpu_clk_gate."]
    pub cpu_clk_gate: CPU_CLK_GATE,
}
#[doc = "l1c_config (rw) register accessor: an alias for `Reg<L1C_CONFIG_SPEC>`"]
pub type L1C_CONFIG = crate::Reg<l1c_config::L1C_CONFIG_SPEC>;
#[doc = "l1c_config."]
pub mod l1c_config;
#[doc = "hit_cnt_lsb (r) register accessor: an alias for `Reg<HIT_CNT_LSB_SPEC>`"]
pub type HIT_CNT_LSB = crate::Reg<hit_cnt_lsb::HIT_CNT_LSB_SPEC>;
#[doc = "hit_cnt_lsb."]
pub mod hit_cnt_lsb;
#[doc = "hit_cnt_msb (r) register accessor: an alias for `Reg<HIT_CNT_MSB_SPEC>`"]
pub type HIT_CNT_MSB = crate::Reg<hit_cnt_msb::HIT_CNT_MSB_SPEC>;
#[doc = "hit_cnt_msb."]
pub mod hit_cnt_msb;
#[doc = "miss_cnt (r) register accessor: an alias for `Reg<MISS_CNT_SPEC>`"]
pub type MISS_CNT = crate::Reg<miss_cnt::MISS_CNT_SPEC>;
#[doc = "miss_cnt."]
pub mod miss_cnt;
#[doc = "l1c_range (rw) register accessor: an alias for `Reg<L1C_RANGE_SPEC>`"]
pub type L1C_RANGE = crate::Reg<l1c_range::L1C_RANGE_SPEC>;
#[doc = "l1c_range."]
pub mod l1c_range;
#[doc = "l1c_bmx_err_addr_en (rw) register accessor: an alias for `Reg<L1C_BMX_ERR_ADDR_EN_SPEC>`"]
pub type L1C_BMX_ERR_ADDR_EN = crate::Reg<l1c_bmx_err_addr_en::L1C_BMX_ERR_ADDR_EN_SPEC>;
#[doc = "l1c_bmx_err_addr_en."]
pub mod l1c_bmx_err_addr_en;
#[doc = "l1c_bmx_err_addr (r) register accessor: an alias for `Reg<L1C_BMX_ERR_ADDR_SPEC>`"]
pub type L1C_BMX_ERR_ADDR = crate::Reg<l1c_bmx_err_addr::L1C_BMX_ERR_ADDR_SPEC>;
#[doc = "l1c_bmx_err_addr."]
pub mod l1c_bmx_err_addr;
#[doc = "irom1_misr_dataout_0 (r) register accessor: an alias for `Reg<IROM1_MISR_DATAOUT_0_SPEC>`"]
pub type IROM1_MISR_DATAOUT_0 = crate::Reg<irom1_misr_dataout_0::IROM1_MISR_DATAOUT_0_SPEC>;
#[doc = "irom1_misr_dataout_0."]
pub mod irom1_misr_dataout_0;
#[doc = "irom1_misr_dataout_1 (rw) register accessor: an alias for `Reg<IROM1_MISR_DATAOUT_1_SPEC>`"]
pub type IROM1_MISR_DATAOUT_1 = crate::Reg<irom1_misr_dataout_1::IROM1_MISR_DATAOUT_1_SPEC>;
#[doc = "irom1_misr_dataout_1."]
pub mod irom1_misr_dataout_1;
#[doc = "cpu_clk_gate (rw) register accessor: an alias for `Reg<CPU_CLK_GATE_SPEC>`"]
pub type CPU_CLK_GATE = crate::Reg<cpu_clk_gate::CPU_CLK_GATE_SPEC>;
#[doc = "cpu_clk_gate."]
pub mod cpu_clk_gate;

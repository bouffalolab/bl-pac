#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Silicon revision"]
    pub rf_rev: RF_REV,
    #[doc = "0x04 - Digital Control"]
    pub rf_fsm_ctrl_hw: RF_FSM_CTRL_HW,
    #[doc = "0x08 - rfsm status reg"]
    pub rf_fsm_ctrl_sw: RF_FSM_CTRL_SW,
    #[doc = "0x0c - Control logic switch"]
    pub rfctrl_hw_en: RFCTRL_HW_EN,
    #[doc = "0x10 - temp_comp."]
    pub temp_comp: TEMP_COMP,
    #[doc = "0x14 - rfcal_status."]
    pub rfcal_status: RFCAL_STATUS,
    #[doc = "0x18 - rfcal_status2."]
    pub rfcal_status2: RFCAL_STATUS2,
    #[doc = "0x1c - Calibration mode register"]
    pub rfcal_ctrlen: RFCAL_CTRLEN,
    #[doc = "0x20 - rf calibration state enabl in full cal list"]
    pub rfcal_stateen: RFCAL_STATEEN,
    #[doc = "0x24 - SARADC Control Registers"]
    pub saradc_resv: SARADC_RESV,
    #[doc = "0x28 - ZRF Control register 0"]
    pub rf_base_ctrl1: RF_BASE_CTRL1,
    #[doc = "0x2c - ZRF Control register 0"]
    pub rf_base_ctrl2: RF_BASE_CTRL2,
    #[doc = "0x30 - pucr1."]
    pub pucr1: PUCR1,
    #[doc = "0x34 - read only from hardware logic"]
    pub pucr1_hw: PUCR1_HW,
    #[doc = "0x38 - pucr2."]
    pub pucr2: PUCR2,
    #[doc = "0x3c - pucr2_hw."]
    pub pucr2_hw: PUCR2_HW,
    #[doc = "0x40 - ppu_ctrl_hw."]
    pub ppu_ctrl_hw: PPU_CTRL_HW,
    #[doc = "0x44 - pud_ctrl_hw."]
    pub pud_ctrl_hw: PUD_CTRL_HW,
    #[doc = "0x48 - gain control1"]
    pub trx_gain1: TRX_GAIN1,
    #[doc = "0x4c - trx gain hardware readback"]
    pub trx_gain_hw: TRX_GAIN_HW,
    #[doc = "0x50 - dc test register"]
    pub ten_dc: TEN_DC,
    #[doc = "0x54 - digital test register"]
    pub ten_dig: TEN_DIG,
    #[doc = "0x58 - ac test register"]
    pub ten_ac: TEN_AC,
    #[doc = "0x5c - pmip_mv2aon."]
    pub pmip_mv2aon: PMIP_MV2AON,
    #[doc = "0x60 - RX normal bias mode registers"]
    pub cip: CIP,
    #[doc = "0x64 - pa1."]
    pub pa1: PA1,
    #[doc = "0x68 - RX normal bias mode registers"]
    pub pa2: PA2,
    #[doc = "0x6c - tmx."]
    pub tmx: TMX,
    #[doc = "0x70 - tbb."]
    pub tbb: TBB,
    #[doc = "0x74 - lna."]
    pub lna: LNA,
    #[doc = "0x78 - rmxgm."]
    pub rmxgm: RMXGM,
    #[doc = "0x7c - rbb1."]
    pub rbb1: RBB1,
    #[doc = "0x80 - rbb2."]
    pub rbb2: RBB2,
    #[doc = "0x84 - rbb3."]
    pub rbb3: RBB3,
    #[doc = "0x88 - rbb4."]
    pub rbb4: RBB4,
    #[doc = "0x8c - adda1."]
    pub adda1: ADDA1,
    #[doc = "0x90 - adda2."]
    pub adda2: ADDA2,
    _reserved37: [u8; 0x0c],
    #[doc = "0xa0 - vco1."]
    pub vco1: VCO1,
    #[doc = "0xa4 - vco2."]
    pub vco2: VCO2,
    #[doc = "0xa8 - vco3."]
    pub vco3: VCO3,
    #[doc = "0xac - vco4."]
    pub vco4: VCO4,
    #[doc = "0xb0 - pfdcp."]
    pub pfdcp: PFDCP,
    #[doc = "0xb4 - lo."]
    pub lo: LO,
    #[doc = "0xb8 - fbdv."]
    pub fbdv: FBDV,
    #[doc = "0xbc - lodist."]
    pub lodist: LODIST,
    #[doc = "0xc0 - sdm1."]
    pub sdm1: SDM1,
    #[doc = "0xc4 - sdm2."]
    pub sdm2: SDM2,
    #[doc = "0xc8 - sdm3."]
    pub sdm3: SDM3,
    _reserved48: [u8; 0x20],
    #[doc = "0xec - rf_resv_reg_0."]
    pub rf_resv_reg_0: RF_RESV_REG_0,
    #[doc = "0xf0 - rf_resv_reg_1."]
    pub rf_resv_reg_1: RF_RESV_REG_1,
    #[doc = "0xf4 - rf_resv_reg_2."]
    pub rf_resv_reg_2: RF_RESV_REG_2,
    #[doc = "0xf8 - rrf_gain_index1."]
    pub rrf_gain_index1: RRF_GAIN_INDEX1,
    #[doc = "0xfc - rrf_gain_index2."]
    pub rrf_gain_index2: RRF_GAIN_INDEX2,
    #[doc = "0x100 - lna_ctrl_hw_mux."]
    pub lna_ctrl_hw_mux: LNA_CTRL_HW_MUX,
    #[doc = "0x104 - rbb_gain_index1."]
    pub rbb_gain_index1: RBB_GAIN_INDEX1,
    #[doc = "0x108 - rbb_gain_index2."]
    pub rbb_gain_index2: RBB_GAIN_INDEX2,
    #[doc = "0x10c - rbb_gain_index3."]
    pub rbb_gain_index3: RBB_GAIN_INDEX3,
    #[doc = "0x110 - rbb_gain_index4."]
    pub rbb_gain_index4: RBB_GAIN_INDEX4,
    #[doc = "0x114 - rbb_gain_index5."]
    pub rbb_gain_index5: RBB_GAIN_INDEX5,
    #[doc = "0x118 - tbb_gain_index1."]
    pub tbb_gain_index1: TBB_GAIN_INDEX1,
    #[doc = "0x11c - tbb_gain_index2."]
    pub tbb_gain_index2: TBB_GAIN_INDEX2,
    #[doc = "0x120 - tbb_gain_index3."]
    pub tbb_gain_index3: TBB_GAIN_INDEX3,
    #[doc = "0x124 - tbb_gain_index4."]
    pub tbb_gain_index4: TBB_GAIN_INDEX4,
    #[doc = "0x128 - pa_reg_ctrl_hw1."]
    pub pa_reg_ctrl_hw1: PA_REG_CTRL_HW1,
    #[doc = "0x12c - pa_reg_ctrl_hw2."]
    pub pa_reg_ctrl_hw2: PA_REG_CTRL_HW2,
    #[doc = "0x130 - pa_reg_wifi_ctrl_hw."]
    pub pa_reg_wifi_ctrl_hw: PA_REG_WIFI_CTRL_HW,
    #[doc = "0x134 - adda_reg_ctrl_hw."]
    pub adda_reg_ctrl_hw: ADDA_REG_CTRL_HW,
    #[doc = "0x138 - lo_reg_ctrl_hw1."]
    pub lo_reg_ctrl_hw1: LO_REG_CTRL_HW1,
    #[doc = "0x13c - lo_cal_ctrl_hw1."]
    pub lo_cal_ctrl_hw1: LO_CAL_CTRL_HW1,
    #[doc = "0x140 - lo_cal_ctrl_hw2."]
    pub lo_cal_ctrl_hw2: LO_CAL_CTRL_HW2,
    #[doc = "0x144 - lo_cal_ctrl_hw3."]
    pub lo_cal_ctrl_hw3: LO_CAL_CTRL_HW3,
    #[doc = "0x148 - lo_cal_ctrl_hw4."]
    pub lo_cal_ctrl_hw4: LO_CAL_CTRL_HW4,
    #[doc = "0x14c - lo_cal_ctrl_hw5."]
    pub lo_cal_ctrl_hw5: LO_CAL_CTRL_HW5,
    #[doc = "0x150 - lo_cal_ctrl_hw6."]
    pub lo_cal_ctrl_hw6: LO_CAL_CTRL_HW6,
    #[doc = "0x154 - lo_cal_ctrl_hw7."]
    pub lo_cal_ctrl_hw7: LO_CAL_CTRL_HW7,
    #[doc = "0x158 - lo_cal_ctrl_hw8."]
    pub lo_cal_ctrl_hw8: LO_CAL_CTRL_HW8,
    #[doc = "0x15c - lo_cal_ctrl_hw9."]
    pub lo_cal_ctrl_hw9: LO_CAL_CTRL_HW9,
    #[doc = "0x160 - lo_cal_ctrl_hw10."]
    pub lo_cal_ctrl_hw10: LO_CAL_CTRL_HW10,
    #[doc = "0x164 - lo_cal_ctrl_hw11."]
    pub lo_cal_ctrl_hw11: LO_CAL_CTRL_HW11,
    #[doc = "0x168 - rosdac_ctrl_hw1."]
    pub rosdac_ctrl_hw1: ROSDAC_CTRL_HW1,
    #[doc = "0x16c - rosdac_ctrl_hw2."]
    pub rosdac_ctrl_hw2: ROSDAC_CTRL_HW2,
    #[doc = "0x170 - rxiq_ctrl_hw1."]
    pub rxiq_ctrl_hw1: RXIQ_CTRL_HW1,
    #[doc = "0x174 - rxiq_ctrl_hw2."]
    pub rxiq_ctrl_hw2: RXIQ_CTRL_HW2,
    #[doc = "0x178 - rxiq_ctrl_hw3."]
    pub rxiq_ctrl_hw3: RXIQ_CTRL_HW3,
    #[doc = "0x17c - rxiq_ctrl_hw4."]
    pub rxiq_ctrl_hw4: RXIQ_CTRL_HW4,
    #[doc = "0x180 - tosdac_ctrl_hw1."]
    pub tosdac_ctrl_hw1: TOSDAC_CTRL_HW1,
    #[doc = "0x184 - tosdac_ctrl_hw2."]
    pub tosdac_ctrl_hw2: TOSDAC_CTRL_HW2,
    #[doc = "0x188 - tosdac_ctrl_hw3."]
    pub tosdac_ctrl_hw3: TOSDAC_CTRL_HW3,
    #[doc = "0x18c - tosdac_ctrl_hw4."]
    pub tosdac_ctrl_hw4: TOSDAC_CTRL_HW4,
    #[doc = "0x190 - tx_iq_gain_hw0."]
    pub tx_iq_gain_hw0: TX_IQ_GAIN_HW0,
    #[doc = "0x194 - tx_iq_gain_hw1."]
    pub tx_iq_gain_hw1: TX_IQ_GAIN_HW1,
    #[doc = "0x198 - tx_iq_gain_hw2."]
    pub tx_iq_gain_hw2: TX_IQ_GAIN_HW2,
    #[doc = "0x19c - tx_iq_gain_hw3."]
    pub tx_iq_gain_hw3: TX_IQ_GAIN_HW3,
    #[doc = "0x1a0 - tx_iq_gain_hw4."]
    pub tx_iq_gain_hw4: TX_IQ_GAIN_HW4,
    #[doc = "0x1a4 - tx_iq_gain_hw5."]
    pub tx_iq_gain_hw5: TX_IQ_GAIN_HW5,
    #[doc = "0x1a8 - tx_iq_gain_hw6."]
    pub tx_iq_gain_hw6: TX_IQ_GAIN_HW6,
    #[doc = "0x1ac - tx_iq_gain_hw7."]
    pub tx_iq_gain_hw7: TX_IQ_GAIN_HW7,
    #[doc = "0x1b0 - lo_sdm_ctrl_hw1."]
    pub lo_sdm_ctrl_hw1: LO_SDM_CTRL_HW1,
    #[doc = "0x1b4 - lo_sdm_ctrl_hw2."]
    pub lo_sdm_ctrl_hw2: LO_SDM_CTRL_HW2,
    #[doc = "0x1b8 - lo_sdm_ctrl_hw3."]
    pub lo_sdm_ctrl_hw3: LO_SDM_CTRL_HW3,
    #[doc = "0x1bc - lo_sdm_ctrl_hw4."]
    pub lo_sdm_ctrl_hw4: LO_SDM_CTRL_HW4,
    #[doc = "0x1c0 - lo_sdm_ctrl_hw5."]
    pub lo_sdm_ctrl_hw5: LO_SDM_CTRL_HW5,
    #[doc = "0x1c4 - lo_sdm_ctrl_hw6."]
    pub lo_sdm_ctrl_hw6: LO_SDM_CTRL_HW6,
    #[doc = "0x1c8 - lo_sdm_ctrl_hw7."]
    pub lo_sdm_ctrl_hw7: LO_SDM_CTRL_HW7,
    #[doc = "0x1cc - lo_sdm_ctrl_hw8."]
    pub lo_sdm_ctrl_hw8: LO_SDM_CTRL_HW8,
    #[doc = "0x1d0 - rbb_bw_ctrl_hw."]
    pub rbb_bw_ctrl_hw: RBB_BW_CTRL_HW,
    _reserved106: [u8; 0x38],
    #[doc = "0x20c - singen_ctrl0."]
    pub singen_ctrl0: SINGEN_CTRL0,
    #[doc = "0x210 - singen_ctrl1."]
    pub singen_ctrl1: SINGEN_CTRL1,
    #[doc = "0x214 - singen_ctrl2."]
    pub singen_ctrl2: SINGEN_CTRL2,
    #[doc = "0x218 - singen_ctrl3."]
    pub singen_ctrl3: SINGEN_CTRL3,
    #[doc = "0x21c - singen_ctrl4."]
    pub singen_ctrl4: SINGEN_CTRL4,
    #[doc = "0x220 - rfif_dfe_ctrl0."]
    pub rfif_dfe_ctrl0: RFIF_DFE_CTRL0,
    #[doc = "0x224 - rfif_test_read."]
    pub rfif_test_read: RFIF_TEST_READ,
    #[doc = "0x228 - rfif_dig_ctrl."]
    pub rfif_dig_ctrl: RFIF_DIG_CTRL,
    #[doc = "0x22c - rf_data_temp_0."]
    pub rf_data_temp_0: RF_DATA_TEMP_0,
    #[doc = "0x230 - rf_data_temp_1."]
    pub rf_data_temp_1: RF_DATA_TEMP_1,
    #[doc = "0x234 - rf_data_temp_2."]
    pub rf_data_temp_2: RF_DATA_TEMP_2,
    #[doc = "0x238 - rf_data_temp_3."]
    pub rf_data_temp_3: RF_DATA_TEMP_3,
    #[doc = "0x23c - rf_sram_ctrl0."]
    pub rf_sram_ctrl0: RF_SRAM_CTRL0,
    #[doc = "0x240 - rf_sram_ctrl1."]
    pub rf_sram_ctrl1: RF_SRAM_CTRL1,
    #[doc = "0x244 - rf_sram_ctrl2."]
    pub rf_sram_ctrl2: RF_SRAM_CTRL2,
    #[doc = "0x248 - rf_sram_ctrl3."]
    pub rf_sram_ctrl3: RF_SRAM_CTRL3,
    #[doc = "0x24c - rf_sram_ctrl4."]
    pub rf_sram_ctrl4: RF_SRAM_CTRL4,
    #[doc = "0x250 - rf_sram_ctrl5."]
    pub rf_sram_ctrl5: RF_SRAM_CTRL5,
    #[doc = "0x254 - rf_sram_ctrl6."]
    pub rf_sram_ctrl6: RF_SRAM_CTRL6,
    #[doc = "0x258 - rf_ical_ctrl0."]
    pub rf_ical_ctrl0: RF_ICAL_CTRL0,
    #[doc = "0x25c - rf_ical_ctrl1."]
    pub rf_ical_ctrl1: RF_ICAL_CTRL1,
    #[doc = "0x260 - rf_ical_ctrl2."]
    pub rf_ical_ctrl2: RF_ICAL_CTRL2,
    #[doc = "0x264 - rf_fsm_ctrl0."]
    pub rf_fsm_ctrl0: RF_FSM_CTRL0,
    #[doc = "0x268 - rf_fsm_ctrl1."]
    pub rf_fsm_ctrl1: RF_FSM_CTRL1,
    #[doc = "0x26c - rf_fsm_ctrl2."]
    pub rf_fsm_ctrl2: RF_FSM_CTRL2,
    #[doc = "0x270 - rf_pkdet_ctrl0."]
    pub rf_pkdet_ctrl0: RF_PKDET_CTRL0,
    _reserved132: [u8; 0x038c],
    #[doc = "0x600 - dfe_ctrl_0."]
    pub dfe_ctrl_0: DFE_CTRL_0,
    #[doc = "0x604 - dfe_ctrl_1."]
    pub dfe_ctrl_1: DFE_CTRL_1,
    #[doc = "0x608 - dfe_ctrl_2."]
    pub dfe_ctrl_2: DFE_CTRL_2,
    #[doc = "0x60c - dfe_ctrl_3."]
    pub dfe_ctrl_3: DFE_CTRL_3,
    #[doc = "0x610 - dfe_ctrl_4."]
    pub dfe_ctrl_4: DFE_CTRL_4,
    #[doc = "0x614 - dfe_ctrl_5."]
    pub dfe_ctrl_5: DFE_CTRL_5,
    #[doc = "0x618 - dfe_ctrl_6."]
    pub dfe_ctrl_6: DFE_CTRL_6,
    #[doc = "0x61c - dfe_ctrl_7."]
    pub dfe_ctrl_7: DFE_CTRL_7,
    #[doc = "0x620 - dfe_ctrl_8."]
    pub dfe_ctrl_8: DFE_CTRL_8,
    #[doc = "0x624 - dfe_ctrl_9."]
    pub dfe_ctrl_9: DFE_CTRL_9,
    #[doc = "0x628 - dfe_ctrl_10."]
    pub dfe_ctrl_10: DFE_CTRL_10,
    #[doc = "0x62c - dfe_ctrl_11."]
    pub dfe_ctrl_11: DFE_CTRL_11,
    #[doc = "0x630 - dfe_ctrl_12."]
    pub dfe_ctrl_12: DFE_CTRL_12,
    #[doc = "0x634 - dfe_ctrl_13."]
    pub dfe_ctrl_13: DFE_CTRL_13,
    #[doc = "0x638 - dfe_ctrl_14."]
    pub dfe_ctrl_14: DFE_CTRL_14,
    #[doc = "0x63c - dfe_ctrl_15."]
    pub dfe_ctrl_15: DFE_CTRL_15,
    #[doc = "0x640 - dfe_ctrl_16."]
    pub dfe_ctrl_16: DFE_CTRL_16,
    #[doc = "0x644 - dfe_ctrl_17."]
    pub dfe_ctrl_17: DFE_CTRL_17,
    #[doc = "0x648 - dfe_ctrl_18."]
    pub dfe_ctrl_18: DFE_CTRL_18,
}
#[doc = "rf_rev (rw) register accessor: an alias for `Reg<RF_REV_SPEC>`"]
pub type RF_REV = crate::Reg<rf_rev::RF_REV_SPEC>;
#[doc = "Silicon revision"]
pub mod rf_rev;
#[doc = "rf_fsm_ctrl_hw (rw) register accessor: an alias for `Reg<RF_FSM_CTRL_HW_SPEC>`"]
pub type RF_FSM_CTRL_HW = crate::Reg<rf_fsm_ctrl_hw::RF_FSM_CTRL_HW_SPEC>;
#[doc = "Digital Control"]
pub mod rf_fsm_ctrl_hw;
#[doc = "rf_fsm_ctrl_sw (rw) register accessor: an alias for `Reg<RF_FSM_CTRL_SW_SPEC>`"]
pub type RF_FSM_CTRL_SW = crate::Reg<rf_fsm_ctrl_sw::RF_FSM_CTRL_SW_SPEC>;
#[doc = "rfsm status reg"]
pub mod rf_fsm_ctrl_sw;
#[doc = "rfctrl_hw_en (rw) register accessor: an alias for `Reg<RFCTRL_HW_EN_SPEC>`"]
pub type RFCTRL_HW_EN = crate::Reg<rfctrl_hw_en::RFCTRL_HW_EN_SPEC>;
#[doc = "Control logic switch"]
pub mod rfctrl_hw_en;
#[doc = "temp_comp (rw) register accessor: an alias for `Reg<TEMP_COMP_SPEC>`"]
pub type TEMP_COMP = crate::Reg<temp_comp::TEMP_COMP_SPEC>;
#[doc = "temp_comp."]
pub mod temp_comp;
#[doc = "rfcal_status (rw) register accessor: an alias for `Reg<RFCAL_STATUS_SPEC>`"]
pub type RFCAL_STATUS = crate::Reg<rfcal_status::RFCAL_STATUS_SPEC>;
#[doc = "rfcal_status."]
pub mod rfcal_status;
#[doc = "rfcal_status2 (rw) register accessor: an alias for `Reg<RFCAL_STATUS2_SPEC>`"]
pub type RFCAL_STATUS2 = crate::Reg<rfcal_status2::RFCAL_STATUS2_SPEC>;
#[doc = "rfcal_status2."]
pub mod rfcal_status2;
#[doc = "rfcal_ctrlen (rw) register accessor: an alias for `Reg<RFCAL_CTRLEN_SPEC>`"]
pub type RFCAL_CTRLEN = crate::Reg<rfcal_ctrlen::RFCAL_CTRLEN_SPEC>;
#[doc = "Calibration mode register"]
pub mod rfcal_ctrlen;
#[doc = "rfcal_stateen (rw) register accessor: an alias for `Reg<RFCAL_STATEEN_SPEC>`"]
pub type RFCAL_STATEEN = crate::Reg<rfcal_stateen::RFCAL_STATEEN_SPEC>;
#[doc = "rf calibration state enabl in full cal list"]
pub mod rfcal_stateen;
#[doc = "saradc_resv (rw) register accessor: an alias for `Reg<SARADC_RESV_SPEC>`"]
pub type SARADC_RESV = crate::Reg<saradc_resv::SARADC_RESV_SPEC>;
#[doc = "SARADC Control Registers"]
pub mod saradc_resv;
#[doc = "rf_base_ctrl1 (rw) register accessor: an alias for `Reg<RF_BASE_CTRL1_SPEC>`"]
pub type RF_BASE_CTRL1 = crate::Reg<rf_base_ctrl1::RF_BASE_CTRL1_SPEC>;
#[doc = "ZRF Control register 0"]
pub mod rf_base_ctrl1;
#[doc = "rf_base_ctrl2 (rw) register accessor: an alias for `Reg<RF_BASE_CTRL2_SPEC>`"]
pub type RF_BASE_CTRL2 = crate::Reg<rf_base_ctrl2::RF_BASE_CTRL2_SPEC>;
#[doc = "ZRF Control register 0"]
pub mod rf_base_ctrl2;
#[doc = "pucr1 (rw) register accessor: an alias for `Reg<PUCR1_SPEC>`"]
pub type PUCR1 = crate::Reg<pucr1::PUCR1_SPEC>;
#[doc = "pucr1."]
pub mod pucr1;
#[doc = "pucr1_hw (rw) register accessor: an alias for `Reg<PUCR1_HW_SPEC>`"]
pub type PUCR1_HW = crate::Reg<pucr1_hw::PUCR1_HW_SPEC>;
#[doc = "read only from hardware logic"]
pub mod pucr1_hw;
#[doc = "pucr2 (rw) register accessor: an alias for `Reg<PUCR2_SPEC>`"]
pub type PUCR2 = crate::Reg<pucr2::PUCR2_SPEC>;
#[doc = "pucr2."]
pub mod pucr2;
#[doc = "pucr2_hw (rw) register accessor: an alias for `Reg<PUCR2_HW_SPEC>`"]
pub type PUCR2_HW = crate::Reg<pucr2_hw::PUCR2_HW_SPEC>;
#[doc = "pucr2_hw."]
pub mod pucr2_hw;
#[doc = "ppu_ctrl_hw (rw) register accessor: an alias for `Reg<PPU_CTRL_HW_SPEC>`"]
pub type PPU_CTRL_HW = crate::Reg<ppu_ctrl_hw::PPU_CTRL_HW_SPEC>;
#[doc = "ppu_ctrl_hw."]
pub mod ppu_ctrl_hw;
#[doc = "pud_ctrl_hw (rw) register accessor: an alias for `Reg<PUD_CTRL_HW_SPEC>`"]
pub type PUD_CTRL_HW = crate::Reg<pud_ctrl_hw::PUD_CTRL_HW_SPEC>;
#[doc = "pud_ctrl_hw."]
pub mod pud_ctrl_hw;
#[doc = "trx_gain1 (rw) register accessor: an alias for `Reg<TRX_GAIN1_SPEC>`"]
pub type TRX_GAIN1 = crate::Reg<trx_gain1::TRX_GAIN1_SPEC>;
#[doc = "gain control1"]
pub mod trx_gain1;
#[doc = "trx_gain_hw (rw) register accessor: an alias for `Reg<TRX_GAIN_HW_SPEC>`"]
pub type TRX_GAIN_HW = crate::Reg<trx_gain_hw::TRX_GAIN_HW_SPEC>;
#[doc = "trx gain hardware readback"]
pub mod trx_gain_hw;
#[doc = "ten_dc (rw) register accessor: an alias for `Reg<TEN_DC_SPEC>`"]
pub type TEN_DC = crate::Reg<ten_dc::TEN_DC_SPEC>;
#[doc = "dc test register"]
pub mod ten_dc;
#[doc = "ten_dig (rw) register accessor: an alias for `Reg<TEN_DIG_SPEC>`"]
pub type TEN_DIG = crate::Reg<ten_dig::TEN_DIG_SPEC>;
#[doc = "digital test register"]
pub mod ten_dig;
#[doc = "ten_ac (rw) register accessor: an alias for `Reg<TEN_AC_SPEC>`"]
pub type TEN_AC = crate::Reg<ten_ac::TEN_AC_SPEC>;
#[doc = "ac test register"]
pub mod ten_ac;
#[doc = "pmip_mv2aon (rw) register accessor: an alias for `Reg<PMIP_MV2AON_SPEC>`"]
pub type PMIP_MV2AON = crate::Reg<pmip_mv2aon::PMIP_MV2AON_SPEC>;
#[doc = "pmip_mv2aon."]
pub mod pmip_mv2aon;
#[doc = "cip (rw) register accessor: an alias for `Reg<CIP_SPEC>`"]
pub type CIP = crate::Reg<cip::CIP_SPEC>;
#[doc = "RX normal bias mode registers"]
pub mod cip;
#[doc = "pa1 (rw) register accessor: an alias for `Reg<PA1_SPEC>`"]
pub type PA1 = crate::Reg<pa1::PA1_SPEC>;
#[doc = "pa1."]
pub mod pa1;
#[doc = "pa2 (rw) register accessor: an alias for `Reg<PA2_SPEC>`"]
pub type PA2 = crate::Reg<pa2::PA2_SPEC>;
#[doc = "RX normal bias mode registers"]
pub mod pa2;
#[doc = "tmx (rw) register accessor: an alias for `Reg<TMX_SPEC>`"]
pub type TMX = crate::Reg<tmx::TMX_SPEC>;
#[doc = "tmx."]
pub mod tmx;
#[doc = "tbb (rw) register accessor: an alias for `Reg<TBB_SPEC>`"]
pub type TBB = crate::Reg<tbb::TBB_SPEC>;
#[doc = "tbb."]
pub mod tbb;
#[doc = "lna (rw) register accessor: an alias for `Reg<LNA_SPEC>`"]
pub type LNA = crate::Reg<lna::LNA_SPEC>;
#[doc = "lna."]
pub mod lna;
#[doc = "rmxgm (rw) register accessor: an alias for `Reg<RMXGM_SPEC>`"]
pub type RMXGM = crate::Reg<rmxgm::RMXGM_SPEC>;
#[doc = "rmxgm."]
pub mod rmxgm;
#[doc = "rbb1 (rw) register accessor: an alias for `Reg<RBB1_SPEC>`"]
pub type RBB1 = crate::Reg<rbb1::RBB1_SPEC>;
#[doc = "rbb1."]
pub mod rbb1;
#[doc = "rbb2 (rw) register accessor: an alias for `Reg<RBB2_SPEC>`"]
pub type RBB2 = crate::Reg<rbb2::RBB2_SPEC>;
#[doc = "rbb2."]
pub mod rbb2;
#[doc = "rbb3 (rw) register accessor: an alias for `Reg<RBB3_SPEC>`"]
pub type RBB3 = crate::Reg<rbb3::RBB3_SPEC>;
#[doc = "rbb3."]
pub mod rbb3;
#[doc = "rbb4 (rw) register accessor: an alias for `Reg<RBB4_SPEC>`"]
pub type RBB4 = crate::Reg<rbb4::RBB4_SPEC>;
#[doc = "rbb4."]
pub mod rbb4;
#[doc = "adda1 (rw) register accessor: an alias for `Reg<ADDA1_SPEC>`"]
pub type ADDA1 = crate::Reg<adda1::ADDA1_SPEC>;
#[doc = "adda1."]
pub mod adda1;
#[doc = "adda2 (rw) register accessor: an alias for `Reg<ADDA2_SPEC>`"]
pub type ADDA2 = crate::Reg<adda2::ADDA2_SPEC>;
#[doc = "adda2."]
pub mod adda2;
#[doc = "vco1 (rw) register accessor: an alias for `Reg<VCO1_SPEC>`"]
pub type VCO1 = crate::Reg<vco1::VCO1_SPEC>;
#[doc = "vco1."]
pub mod vco1;
#[doc = "vco2 (rw) register accessor: an alias for `Reg<VCO2_SPEC>`"]
pub type VCO2 = crate::Reg<vco2::VCO2_SPEC>;
#[doc = "vco2."]
pub mod vco2;
#[doc = "vco3 (rw) register accessor: an alias for `Reg<VCO3_SPEC>`"]
pub type VCO3 = crate::Reg<vco3::VCO3_SPEC>;
#[doc = "vco3."]
pub mod vco3;
#[doc = "vco4 (rw) register accessor: an alias for `Reg<VCO4_SPEC>`"]
pub type VCO4 = crate::Reg<vco4::VCO4_SPEC>;
#[doc = "vco4."]
pub mod vco4;
#[doc = "pfdcp (rw) register accessor: an alias for `Reg<PFDCP_SPEC>`"]
pub type PFDCP = crate::Reg<pfdcp::PFDCP_SPEC>;
#[doc = "pfdcp."]
pub mod pfdcp;
#[doc = "lo (rw) register accessor: an alias for `Reg<LO_SPEC>`"]
pub type LO = crate::Reg<lo::LO_SPEC>;
#[doc = "lo."]
pub mod lo;
#[doc = "fbdv (rw) register accessor: an alias for `Reg<FBDV_SPEC>`"]
pub type FBDV = crate::Reg<fbdv::FBDV_SPEC>;
#[doc = "fbdv."]
pub mod fbdv;
#[doc = "lodist (rw) register accessor: an alias for `Reg<LODIST_SPEC>`"]
pub type LODIST = crate::Reg<lodist::LODIST_SPEC>;
#[doc = "lodist."]
pub mod lodist;
#[doc = "sdm1 (rw) register accessor: an alias for `Reg<SDM1_SPEC>`"]
pub type SDM1 = crate::Reg<sdm1::SDM1_SPEC>;
#[doc = "sdm1."]
pub mod sdm1;
#[doc = "sdm2 (rw) register accessor: an alias for `Reg<SDM2_SPEC>`"]
pub type SDM2 = crate::Reg<sdm2::SDM2_SPEC>;
#[doc = "sdm2."]
pub mod sdm2;
#[doc = "sdm3 (rw) register accessor: an alias for `Reg<SDM3_SPEC>`"]
pub type SDM3 = crate::Reg<sdm3::SDM3_SPEC>;
#[doc = "sdm3."]
pub mod sdm3;
#[doc = "rf_resv_reg_0 (rw) register accessor: an alias for `Reg<RF_RESV_REG_0_SPEC>`"]
pub type RF_RESV_REG_0 = crate::Reg<rf_resv_reg_0::RF_RESV_REG_0_SPEC>;
#[doc = "rf_resv_reg_0."]
pub mod rf_resv_reg_0;
#[doc = "rf_resv_reg_1 (rw) register accessor: an alias for `Reg<RF_RESV_REG_1_SPEC>`"]
pub type RF_RESV_REG_1 = crate::Reg<rf_resv_reg_1::RF_RESV_REG_1_SPEC>;
#[doc = "rf_resv_reg_1."]
pub mod rf_resv_reg_1;
#[doc = "rf_resv_reg_2 (rw) register accessor: an alias for `Reg<RF_RESV_REG_2_SPEC>`"]
pub type RF_RESV_REG_2 = crate::Reg<rf_resv_reg_2::RF_RESV_REG_2_SPEC>;
#[doc = "rf_resv_reg_2."]
pub mod rf_resv_reg_2;
#[doc = "rrf_gain_index1 (rw) register accessor: an alias for `Reg<RRF_GAIN_INDEX1_SPEC>`"]
pub type RRF_GAIN_INDEX1 = crate::Reg<rrf_gain_index1::RRF_GAIN_INDEX1_SPEC>;
#[doc = "rrf_gain_index1."]
pub mod rrf_gain_index1;
#[doc = "rrf_gain_index2 (rw) register accessor: an alias for `Reg<RRF_GAIN_INDEX2_SPEC>`"]
pub type RRF_GAIN_INDEX2 = crate::Reg<rrf_gain_index2::RRF_GAIN_INDEX2_SPEC>;
#[doc = "rrf_gain_index2."]
pub mod rrf_gain_index2;
#[doc = "lna_ctrl_hw_mux (rw) register accessor: an alias for `Reg<LNA_CTRL_HW_MUX_SPEC>`"]
pub type LNA_CTRL_HW_MUX = crate::Reg<lna_ctrl_hw_mux::LNA_CTRL_HW_MUX_SPEC>;
#[doc = "lna_ctrl_hw_mux."]
pub mod lna_ctrl_hw_mux;
#[doc = "rbb_gain_index1 (rw) register accessor: an alias for `Reg<RBB_GAIN_INDEX1_SPEC>`"]
pub type RBB_GAIN_INDEX1 = crate::Reg<rbb_gain_index1::RBB_GAIN_INDEX1_SPEC>;
#[doc = "rbb_gain_index1."]
pub mod rbb_gain_index1;
#[doc = "rbb_gain_index2 (rw) register accessor: an alias for `Reg<RBB_GAIN_INDEX2_SPEC>`"]
pub type RBB_GAIN_INDEX2 = crate::Reg<rbb_gain_index2::RBB_GAIN_INDEX2_SPEC>;
#[doc = "rbb_gain_index2."]
pub mod rbb_gain_index2;
#[doc = "rbb_gain_index3 (rw) register accessor: an alias for `Reg<RBB_GAIN_INDEX3_SPEC>`"]
pub type RBB_GAIN_INDEX3 = crate::Reg<rbb_gain_index3::RBB_GAIN_INDEX3_SPEC>;
#[doc = "rbb_gain_index3."]
pub mod rbb_gain_index3;
#[doc = "rbb_gain_index4 (rw) register accessor: an alias for `Reg<RBB_GAIN_INDEX4_SPEC>`"]
pub type RBB_GAIN_INDEX4 = crate::Reg<rbb_gain_index4::RBB_GAIN_INDEX4_SPEC>;
#[doc = "rbb_gain_index4."]
pub mod rbb_gain_index4;
#[doc = "rbb_gain_index5 (rw) register accessor: an alias for `Reg<RBB_GAIN_INDEX5_SPEC>`"]
pub type RBB_GAIN_INDEX5 = crate::Reg<rbb_gain_index5::RBB_GAIN_INDEX5_SPEC>;
#[doc = "rbb_gain_index5."]
pub mod rbb_gain_index5;
#[doc = "tbb_gain_index1 (rw) register accessor: an alias for `Reg<TBB_GAIN_INDEX1_SPEC>`"]
pub type TBB_GAIN_INDEX1 = crate::Reg<tbb_gain_index1::TBB_GAIN_INDEX1_SPEC>;
#[doc = "tbb_gain_index1."]
pub mod tbb_gain_index1;
#[doc = "tbb_gain_index2 (rw) register accessor: an alias for `Reg<TBB_GAIN_INDEX2_SPEC>`"]
pub type TBB_GAIN_INDEX2 = crate::Reg<tbb_gain_index2::TBB_GAIN_INDEX2_SPEC>;
#[doc = "tbb_gain_index2."]
pub mod tbb_gain_index2;
#[doc = "tbb_gain_index3 (rw) register accessor: an alias for `Reg<TBB_GAIN_INDEX3_SPEC>`"]
pub type TBB_GAIN_INDEX3 = crate::Reg<tbb_gain_index3::TBB_GAIN_INDEX3_SPEC>;
#[doc = "tbb_gain_index3."]
pub mod tbb_gain_index3;
#[doc = "tbb_gain_index4 (rw) register accessor: an alias for `Reg<TBB_GAIN_INDEX4_SPEC>`"]
pub type TBB_GAIN_INDEX4 = crate::Reg<tbb_gain_index4::TBB_GAIN_INDEX4_SPEC>;
#[doc = "tbb_gain_index4."]
pub mod tbb_gain_index4;
#[doc = "pa_reg_ctrl_hw1 (rw) register accessor: an alias for `Reg<PA_REG_CTRL_HW1_SPEC>`"]
pub type PA_REG_CTRL_HW1 = crate::Reg<pa_reg_ctrl_hw1::PA_REG_CTRL_HW1_SPEC>;
#[doc = "pa_reg_ctrl_hw1."]
pub mod pa_reg_ctrl_hw1;
#[doc = "pa_reg_ctrl_hw2 (rw) register accessor: an alias for `Reg<PA_REG_CTRL_HW2_SPEC>`"]
pub type PA_REG_CTRL_HW2 = crate::Reg<pa_reg_ctrl_hw2::PA_REG_CTRL_HW2_SPEC>;
#[doc = "pa_reg_ctrl_hw2."]
pub mod pa_reg_ctrl_hw2;
#[doc = "pa_reg_wifi_ctrl_hw (rw) register accessor: an alias for `Reg<PA_REG_WIFI_CTRL_HW_SPEC>`"]
pub type PA_REG_WIFI_CTRL_HW = crate::Reg<pa_reg_wifi_ctrl_hw::PA_REG_WIFI_CTRL_HW_SPEC>;
#[doc = "pa_reg_wifi_ctrl_hw."]
pub mod pa_reg_wifi_ctrl_hw;
#[doc = "adda_reg_ctrl_hw (rw) register accessor: an alias for `Reg<ADDA_REG_CTRL_HW_SPEC>`"]
pub type ADDA_REG_CTRL_HW = crate::Reg<adda_reg_ctrl_hw::ADDA_REG_CTRL_HW_SPEC>;
#[doc = "adda_reg_ctrl_hw."]
pub mod adda_reg_ctrl_hw;
#[doc = "lo_reg_ctrl_hw1 (rw) register accessor: an alias for `Reg<LO_REG_CTRL_HW1_SPEC>`"]
pub type LO_REG_CTRL_HW1 = crate::Reg<lo_reg_ctrl_hw1::LO_REG_CTRL_HW1_SPEC>;
#[doc = "lo_reg_ctrl_hw1."]
pub mod lo_reg_ctrl_hw1;
#[doc = "lo_cal_ctrl_hw1 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW1_SPEC>`"]
pub type LO_CAL_CTRL_HW1 = crate::Reg<lo_cal_ctrl_hw1::LO_CAL_CTRL_HW1_SPEC>;
#[doc = "lo_cal_ctrl_hw1."]
pub mod lo_cal_ctrl_hw1;
#[doc = "lo_cal_ctrl_hw2 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW2_SPEC>`"]
pub type LO_CAL_CTRL_HW2 = crate::Reg<lo_cal_ctrl_hw2::LO_CAL_CTRL_HW2_SPEC>;
#[doc = "lo_cal_ctrl_hw2."]
pub mod lo_cal_ctrl_hw2;
#[doc = "lo_cal_ctrl_hw3 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW3_SPEC>`"]
pub type LO_CAL_CTRL_HW3 = crate::Reg<lo_cal_ctrl_hw3::LO_CAL_CTRL_HW3_SPEC>;
#[doc = "lo_cal_ctrl_hw3."]
pub mod lo_cal_ctrl_hw3;
#[doc = "lo_cal_ctrl_hw4 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW4_SPEC>`"]
pub type LO_CAL_CTRL_HW4 = crate::Reg<lo_cal_ctrl_hw4::LO_CAL_CTRL_HW4_SPEC>;
#[doc = "lo_cal_ctrl_hw4."]
pub mod lo_cal_ctrl_hw4;
#[doc = "lo_cal_ctrl_hw5 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW5_SPEC>`"]
pub type LO_CAL_CTRL_HW5 = crate::Reg<lo_cal_ctrl_hw5::LO_CAL_CTRL_HW5_SPEC>;
#[doc = "lo_cal_ctrl_hw5."]
pub mod lo_cal_ctrl_hw5;
#[doc = "lo_cal_ctrl_hw6 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW6_SPEC>`"]
pub type LO_CAL_CTRL_HW6 = crate::Reg<lo_cal_ctrl_hw6::LO_CAL_CTRL_HW6_SPEC>;
#[doc = "lo_cal_ctrl_hw6."]
pub mod lo_cal_ctrl_hw6;
#[doc = "lo_cal_ctrl_hw7 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW7_SPEC>`"]
pub type LO_CAL_CTRL_HW7 = crate::Reg<lo_cal_ctrl_hw7::LO_CAL_CTRL_HW7_SPEC>;
#[doc = "lo_cal_ctrl_hw7."]
pub mod lo_cal_ctrl_hw7;
#[doc = "lo_cal_ctrl_hw8 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW8_SPEC>`"]
pub type LO_CAL_CTRL_HW8 = crate::Reg<lo_cal_ctrl_hw8::LO_CAL_CTRL_HW8_SPEC>;
#[doc = "lo_cal_ctrl_hw8."]
pub mod lo_cal_ctrl_hw8;
#[doc = "lo_cal_ctrl_hw9 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW9_SPEC>`"]
pub type LO_CAL_CTRL_HW9 = crate::Reg<lo_cal_ctrl_hw9::LO_CAL_CTRL_HW9_SPEC>;
#[doc = "lo_cal_ctrl_hw9."]
pub mod lo_cal_ctrl_hw9;
#[doc = "lo_cal_ctrl_hw10 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW10_SPEC>`"]
pub type LO_CAL_CTRL_HW10 = crate::Reg<lo_cal_ctrl_hw10::LO_CAL_CTRL_HW10_SPEC>;
#[doc = "lo_cal_ctrl_hw10."]
pub mod lo_cal_ctrl_hw10;
#[doc = "lo_cal_ctrl_hw11 (rw) register accessor: an alias for `Reg<LO_CAL_CTRL_HW11_SPEC>`"]
pub type LO_CAL_CTRL_HW11 = crate::Reg<lo_cal_ctrl_hw11::LO_CAL_CTRL_HW11_SPEC>;
#[doc = "lo_cal_ctrl_hw11."]
pub mod lo_cal_ctrl_hw11;
#[doc = "rosdac_ctrl_hw1 (rw) register accessor: an alias for `Reg<ROSDAC_CTRL_HW1_SPEC>`"]
pub type ROSDAC_CTRL_HW1 = crate::Reg<rosdac_ctrl_hw1::ROSDAC_CTRL_HW1_SPEC>;
#[doc = "rosdac_ctrl_hw1."]
pub mod rosdac_ctrl_hw1;
#[doc = "rosdac_ctrl_hw2 (rw) register accessor: an alias for `Reg<ROSDAC_CTRL_HW2_SPEC>`"]
pub type ROSDAC_CTRL_HW2 = crate::Reg<rosdac_ctrl_hw2::ROSDAC_CTRL_HW2_SPEC>;
#[doc = "rosdac_ctrl_hw2."]
pub mod rosdac_ctrl_hw2;
#[doc = "rxiq_ctrl_hw1 (rw) register accessor: an alias for `Reg<RXIQ_CTRL_HW1_SPEC>`"]
pub type RXIQ_CTRL_HW1 = crate::Reg<rxiq_ctrl_hw1::RXIQ_CTRL_HW1_SPEC>;
#[doc = "rxiq_ctrl_hw1."]
pub mod rxiq_ctrl_hw1;
#[doc = "rxiq_ctrl_hw2 (rw) register accessor: an alias for `Reg<RXIQ_CTRL_HW2_SPEC>`"]
pub type RXIQ_CTRL_HW2 = crate::Reg<rxiq_ctrl_hw2::RXIQ_CTRL_HW2_SPEC>;
#[doc = "rxiq_ctrl_hw2."]
pub mod rxiq_ctrl_hw2;
#[doc = "rxiq_ctrl_hw3 (rw) register accessor: an alias for `Reg<RXIQ_CTRL_HW3_SPEC>`"]
pub type RXIQ_CTRL_HW3 = crate::Reg<rxiq_ctrl_hw3::RXIQ_CTRL_HW3_SPEC>;
#[doc = "rxiq_ctrl_hw3."]
pub mod rxiq_ctrl_hw3;
#[doc = "rxiq_ctrl_hw4 (rw) register accessor: an alias for `Reg<RXIQ_CTRL_HW4_SPEC>`"]
pub type RXIQ_CTRL_HW4 = crate::Reg<rxiq_ctrl_hw4::RXIQ_CTRL_HW4_SPEC>;
#[doc = "rxiq_ctrl_hw4."]
pub mod rxiq_ctrl_hw4;
#[doc = "tosdac_ctrl_hw1 (rw) register accessor: an alias for `Reg<TOSDAC_CTRL_HW1_SPEC>`"]
pub type TOSDAC_CTRL_HW1 = crate::Reg<tosdac_ctrl_hw1::TOSDAC_CTRL_HW1_SPEC>;
#[doc = "tosdac_ctrl_hw1."]
pub mod tosdac_ctrl_hw1;
#[doc = "tosdac_ctrl_hw2 (rw) register accessor: an alias for `Reg<TOSDAC_CTRL_HW2_SPEC>`"]
pub type TOSDAC_CTRL_HW2 = crate::Reg<tosdac_ctrl_hw2::TOSDAC_CTRL_HW2_SPEC>;
#[doc = "tosdac_ctrl_hw2."]
pub mod tosdac_ctrl_hw2;
#[doc = "tosdac_ctrl_hw3 (rw) register accessor: an alias for `Reg<TOSDAC_CTRL_HW3_SPEC>`"]
pub type TOSDAC_CTRL_HW3 = crate::Reg<tosdac_ctrl_hw3::TOSDAC_CTRL_HW3_SPEC>;
#[doc = "tosdac_ctrl_hw3."]
pub mod tosdac_ctrl_hw3;
#[doc = "tosdac_ctrl_hw4 (rw) register accessor: an alias for `Reg<TOSDAC_CTRL_HW4_SPEC>`"]
pub type TOSDAC_CTRL_HW4 = crate::Reg<tosdac_ctrl_hw4::TOSDAC_CTRL_HW4_SPEC>;
#[doc = "tosdac_ctrl_hw4."]
pub mod tosdac_ctrl_hw4;
#[doc = "tx_iq_gain_hw0 (rw) register accessor: an alias for `Reg<TX_IQ_GAIN_HW0_SPEC>`"]
pub type TX_IQ_GAIN_HW0 = crate::Reg<tx_iq_gain_hw0::TX_IQ_GAIN_HW0_SPEC>;
#[doc = "tx_iq_gain_hw0."]
pub mod tx_iq_gain_hw0;
#[doc = "tx_iq_gain_hw1 (rw) register accessor: an alias for `Reg<TX_IQ_GAIN_HW1_SPEC>`"]
pub type TX_IQ_GAIN_HW1 = crate::Reg<tx_iq_gain_hw1::TX_IQ_GAIN_HW1_SPEC>;
#[doc = "tx_iq_gain_hw1."]
pub mod tx_iq_gain_hw1;
#[doc = "tx_iq_gain_hw2 (rw) register accessor: an alias for `Reg<TX_IQ_GAIN_HW2_SPEC>`"]
pub type TX_IQ_GAIN_HW2 = crate::Reg<tx_iq_gain_hw2::TX_IQ_GAIN_HW2_SPEC>;
#[doc = "tx_iq_gain_hw2."]
pub mod tx_iq_gain_hw2;
#[doc = "tx_iq_gain_hw3 (rw) register accessor: an alias for `Reg<TX_IQ_GAIN_HW3_SPEC>`"]
pub type TX_IQ_GAIN_HW3 = crate::Reg<tx_iq_gain_hw3::TX_IQ_GAIN_HW3_SPEC>;
#[doc = "tx_iq_gain_hw3."]
pub mod tx_iq_gain_hw3;
#[doc = "tx_iq_gain_hw4 (rw) register accessor: an alias for `Reg<TX_IQ_GAIN_HW4_SPEC>`"]
pub type TX_IQ_GAIN_HW4 = crate::Reg<tx_iq_gain_hw4::TX_IQ_GAIN_HW4_SPEC>;
#[doc = "tx_iq_gain_hw4."]
pub mod tx_iq_gain_hw4;
#[doc = "tx_iq_gain_hw5 (rw) register accessor: an alias for `Reg<TX_IQ_GAIN_HW5_SPEC>`"]
pub type TX_IQ_GAIN_HW5 = crate::Reg<tx_iq_gain_hw5::TX_IQ_GAIN_HW5_SPEC>;
#[doc = "tx_iq_gain_hw5."]
pub mod tx_iq_gain_hw5;
#[doc = "tx_iq_gain_hw6 (rw) register accessor: an alias for `Reg<TX_IQ_GAIN_HW6_SPEC>`"]
pub type TX_IQ_GAIN_HW6 = crate::Reg<tx_iq_gain_hw6::TX_IQ_GAIN_HW6_SPEC>;
#[doc = "tx_iq_gain_hw6."]
pub mod tx_iq_gain_hw6;
#[doc = "tx_iq_gain_hw7 (rw) register accessor: an alias for `Reg<TX_IQ_GAIN_HW7_SPEC>`"]
pub type TX_IQ_GAIN_HW7 = crate::Reg<tx_iq_gain_hw7::TX_IQ_GAIN_HW7_SPEC>;
#[doc = "tx_iq_gain_hw7."]
pub mod tx_iq_gain_hw7;
#[doc = "lo_sdm_ctrl_hw1 (rw) register accessor: an alias for `Reg<LO_SDM_CTRL_HW1_SPEC>`"]
pub type LO_SDM_CTRL_HW1 = crate::Reg<lo_sdm_ctrl_hw1::LO_SDM_CTRL_HW1_SPEC>;
#[doc = "lo_sdm_ctrl_hw1."]
pub mod lo_sdm_ctrl_hw1;
#[doc = "lo_sdm_ctrl_hw2 (rw) register accessor: an alias for `Reg<LO_SDM_CTRL_HW2_SPEC>`"]
pub type LO_SDM_CTRL_HW2 = crate::Reg<lo_sdm_ctrl_hw2::LO_SDM_CTRL_HW2_SPEC>;
#[doc = "lo_sdm_ctrl_hw2."]
pub mod lo_sdm_ctrl_hw2;
#[doc = "lo_sdm_ctrl_hw3 (rw) register accessor: an alias for `Reg<LO_SDM_CTRL_HW3_SPEC>`"]
pub type LO_SDM_CTRL_HW3 = crate::Reg<lo_sdm_ctrl_hw3::LO_SDM_CTRL_HW3_SPEC>;
#[doc = "lo_sdm_ctrl_hw3."]
pub mod lo_sdm_ctrl_hw3;
#[doc = "lo_sdm_ctrl_hw4 (rw) register accessor: an alias for `Reg<LO_SDM_CTRL_HW4_SPEC>`"]
pub type LO_SDM_CTRL_HW4 = crate::Reg<lo_sdm_ctrl_hw4::LO_SDM_CTRL_HW4_SPEC>;
#[doc = "lo_sdm_ctrl_hw4."]
pub mod lo_sdm_ctrl_hw4;
#[doc = "lo_sdm_ctrl_hw5 (rw) register accessor: an alias for `Reg<LO_SDM_CTRL_HW5_SPEC>`"]
pub type LO_SDM_CTRL_HW5 = crate::Reg<lo_sdm_ctrl_hw5::LO_SDM_CTRL_HW5_SPEC>;
#[doc = "lo_sdm_ctrl_hw5."]
pub mod lo_sdm_ctrl_hw5;
#[doc = "lo_sdm_ctrl_hw6 (rw) register accessor: an alias for `Reg<LO_SDM_CTRL_HW6_SPEC>`"]
pub type LO_SDM_CTRL_HW6 = crate::Reg<lo_sdm_ctrl_hw6::LO_SDM_CTRL_HW6_SPEC>;
#[doc = "lo_sdm_ctrl_hw6."]
pub mod lo_sdm_ctrl_hw6;
#[doc = "lo_sdm_ctrl_hw7 (rw) register accessor: an alias for `Reg<LO_SDM_CTRL_HW7_SPEC>`"]
pub type LO_SDM_CTRL_HW7 = crate::Reg<lo_sdm_ctrl_hw7::LO_SDM_CTRL_HW7_SPEC>;
#[doc = "lo_sdm_ctrl_hw7."]
pub mod lo_sdm_ctrl_hw7;
#[doc = "lo_sdm_ctrl_hw8 (rw) register accessor: an alias for `Reg<LO_SDM_CTRL_HW8_SPEC>`"]
pub type LO_SDM_CTRL_HW8 = crate::Reg<lo_sdm_ctrl_hw8::LO_SDM_CTRL_HW8_SPEC>;
#[doc = "lo_sdm_ctrl_hw8."]
pub mod lo_sdm_ctrl_hw8;
#[doc = "rbb_bw_ctrl_hw (rw) register accessor: an alias for `Reg<RBB_BW_CTRL_HW_SPEC>`"]
pub type RBB_BW_CTRL_HW = crate::Reg<rbb_bw_ctrl_hw::RBB_BW_CTRL_HW_SPEC>;
#[doc = "rbb_bw_ctrl_hw."]
pub mod rbb_bw_ctrl_hw;
#[doc = "singen_ctrl0 (rw) register accessor: an alias for `Reg<SINGEN_CTRL0_SPEC>`"]
pub type SINGEN_CTRL0 = crate::Reg<singen_ctrl0::SINGEN_CTRL0_SPEC>;
#[doc = "singen_ctrl0."]
pub mod singen_ctrl0;
#[doc = "singen_ctrl1 (rw) register accessor: an alias for `Reg<SINGEN_CTRL1_SPEC>`"]
pub type SINGEN_CTRL1 = crate::Reg<singen_ctrl1::SINGEN_CTRL1_SPEC>;
#[doc = "singen_ctrl1."]
pub mod singen_ctrl1;
#[doc = "singen_ctrl2 (rw) register accessor: an alias for `Reg<SINGEN_CTRL2_SPEC>`"]
pub type SINGEN_CTRL2 = crate::Reg<singen_ctrl2::SINGEN_CTRL2_SPEC>;
#[doc = "singen_ctrl2."]
pub mod singen_ctrl2;
#[doc = "singen_ctrl3 (rw) register accessor: an alias for `Reg<SINGEN_CTRL3_SPEC>`"]
pub type SINGEN_CTRL3 = crate::Reg<singen_ctrl3::SINGEN_CTRL3_SPEC>;
#[doc = "singen_ctrl3."]
pub mod singen_ctrl3;
#[doc = "singen_ctrl4 (rw) register accessor: an alias for `Reg<SINGEN_CTRL4_SPEC>`"]
pub type SINGEN_CTRL4 = crate::Reg<singen_ctrl4::SINGEN_CTRL4_SPEC>;
#[doc = "singen_ctrl4."]
pub mod singen_ctrl4;
#[doc = "rfif_dfe_ctrl0 (rw) register accessor: an alias for `Reg<RFIF_DFE_CTRL0_SPEC>`"]
pub type RFIF_DFE_CTRL0 = crate::Reg<rfif_dfe_ctrl0::RFIF_DFE_CTRL0_SPEC>;
#[doc = "rfif_dfe_ctrl0."]
pub mod rfif_dfe_ctrl0;
#[doc = "rfif_test_read (rw) register accessor: an alias for `Reg<RFIF_TEST_READ_SPEC>`"]
pub type RFIF_TEST_READ = crate::Reg<rfif_test_read::RFIF_TEST_READ_SPEC>;
#[doc = "rfif_test_read."]
pub mod rfif_test_read;
#[doc = "rfif_dig_ctrl (rw) register accessor: an alias for `Reg<RFIF_DIG_CTRL_SPEC>`"]
pub type RFIF_DIG_CTRL = crate::Reg<rfif_dig_ctrl::RFIF_DIG_CTRL_SPEC>;
#[doc = "rfif_dig_ctrl."]
pub mod rfif_dig_ctrl;
#[doc = "rf_data_temp_0 (rw) register accessor: an alias for `Reg<RF_DATA_TEMP_0_SPEC>`"]
pub type RF_DATA_TEMP_0 = crate::Reg<rf_data_temp_0::RF_DATA_TEMP_0_SPEC>;
#[doc = "rf_data_temp_0."]
pub mod rf_data_temp_0;
#[doc = "rf_data_temp_1 (rw) register accessor: an alias for `Reg<RF_DATA_TEMP_1_SPEC>`"]
pub type RF_DATA_TEMP_1 = crate::Reg<rf_data_temp_1::RF_DATA_TEMP_1_SPEC>;
#[doc = "rf_data_temp_1."]
pub mod rf_data_temp_1;
#[doc = "rf_data_temp_2 (rw) register accessor: an alias for `Reg<RF_DATA_TEMP_2_SPEC>`"]
pub type RF_DATA_TEMP_2 = crate::Reg<rf_data_temp_2::RF_DATA_TEMP_2_SPEC>;
#[doc = "rf_data_temp_2."]
pub mod rf_data_temp_2;
#[doc = "rf_data_temp_3 (rw) register accessor: an alias for `Reg<RF_DATA_TEMP_3_SPEC>`"]
pub type RF_DATA_TEMP_3 = crate::Reg<rf_data_temp_3::RF_DATA_TEMP_3_SPEC>;
#[doc = "rf_data_temp_3."]
pub mod rf_data_temp_3;
#[doc = "rf_sram_ctrl0 (rw) register accessor: an alias for `Reg<RF_SRAM_CTRL0_SPEC>`"]
pub type RF_SRAM_CTRL0 = crate::Reg<rf_sram_ctrl0::RF_SRAM_CTRL0_SPEC>;
#[doc = "rf_sram_ctrl0."]
pub mod rf_sram_ctrl0;
#[doc = "rf_sram_ctrl1 (rw) register accessor: an alias for `Reg<RF_SRAM_CTRL1_SPEC>`"]
pub type RF_SRAM_CTRL1 = crate::Reg<rf_sram_ctrl1::RF_SRAM_CTRL1_SPEC>;
#[doc = "rf_sram_ctrl1."]
pub mod rf_sram_ctrl1;
#[doc = "rf_sram_ctrl2 (rw) register accessor: an alias for `Reg<RF_SRAM_CTRL2_SPEC>`"]
pub type RF_SRAM_CTRL2 = crate::Reg<rf_sram_ctrl2::RF_SRAM_CTRL2_SPEC>;
#[doc = "rf_sram_ctrl2."]
pub mod rf_sram_ctrl2;
#[doc = "rf_sram_ctrl3 (rw) register accessor: an alias for `Reg<RF_SRAM_CTRL3_SPEC>`"]
pub type RF_SRAM_CTRL3 = crate::Reg<rf_sram_ctrl3::RF_SRAM_CTRL3_SPEC>;
#[doc = "rf_sram_ctrl3."]
pub mod rf_sram_ctrl3;
#[doc = "rf_sram_ctrl4 (rw) register accessor: an alias for `Reg<RF_SRAM_CTRL4_SPEC>`"]
pub type RF_SRAM_CTRL4 = crate::Reg<rf_sram_ctrl4::RF_SRAM_CTRL4_SPEC>;
#[doc = "rf_sram_ctrl4."]
pub mod rf_sram_ctrl4;
#[doc = "rf_sram_ctrl5 (rw) register accessor: an alias for `Reg<RF_SRAM_CTRL5_SPEC>`"]
pub type RF_SRAM_CTRL5 = crate::Reg<rf_sram_ctrl5::RF_SRAM_CTRL5_SPEC>;
#[doc = "rf_sram_ctrl5."]
pub mod rf_sram_ctrl5;
#[doc = "rf_sram_ctrl6 (rw) register accessor: an alias for `Reg<RF_SRAM_CTRL6_SPEC>`"]
pub type RF_SRAM_CTRL6 = crate::Reg<rf_sram_ctrl6::RF_SRAM_CTRL6_SPEC>;
#[doc = "rf_sram_ctrl6."]
pub mod rf_sram_ctrl6;
#[doc = "rf_ical_ctrl0 (rw) register accessor: an alias for `Reg<RF_ICAL_CTRL0_SPEC>`"]
pub type RF_ICAL_CTRL0 = crate::Reg<rf_ical_ctrl0::RF_ICAL_CTRL0_SPEC>;
#[doc = "rf_ical_ctrl0."]
pub mod rf_ical_ctrl0;
#[doc = "rf_ical_ctrl1 (rw) register accessor: an alias for `Reg<RF_ICAL_CTRL1_SPEC>`"]
pub type RF_ICAL_CTRL1 = crate::Reg<rf_ical_ctrl1::RF_ICAL_CTRL1_SPEC>;
#[doc = "rf_ical_ctrl1."]
pub mod rf_ical_ctrl1;
#[doc = "rf_ical_ctrl2 (rw) register accessor: an alias for `Reg<RF_ICAL_CTRL2_SPEC>`"]
pub type RF_ICAL_CTRL2 = crate::Reg<rf_ical_ctrl2::RF_ICAL_CTRL2_SPEC>;
#[doc = "rf_ical_ctrl2."]
pub mod rf_ical_ctrl2;
#[doc = "rf_fsm_ctrl0 (rw) register accessor: an alias for `Reg<RF_FSM_CTRL0_SPEC>`"]
pub type RF_FSM_CTRL0 = crate::Reg<rf_fsm_ctrl0::RF_FSM_CTRL0_SPEC>;
#[doc = "rf_fsm_ctrl0."]
pub mod rf_fsm_ctrl0;
#[doc = "rf_fsm_ctrl1 (rw) register accessor: an alias for `Reg<RF_FSM_CTRL1_SPEC>`"]
pub type RF_FSM_CTRL1 = crate::Reg<rf_fsm_ctrl1::RF_FSM_CTRL1_SPEC>;
#[doc = "rf_fsm_ctrl1."]
pub mod rf_fsm_ctrl1;
#[doc = "rf_fsm_ctrl2 (rw) register accessor: an alias for `Reg<RF_FSM_CTRL2_SPEC>`"]
pub type RF_FSM_CTRL2 = crate::Reg<rf_fsm_ctrl2::RF_FSM_CTRL2_SPEC>;
#[doc = "rf_fsm_ctrl2."]
pub mod rf_fsm_ctrl2;
#[doc = "rf_pkdet_ctrl0 (rw) register accessor: an alias for `Reg<RF_PKDET_CTRL0_SPEC>`"]
pub type RF_PKDET_CTRL0 = crate::Reg<rf_pkdet_ctrl0::RF_PKDET_CTRL0_SPEC>;
#[doc = "rf_pkdet_ctrl0."]
pub mod rf_pkdet_ctrl0;
#[doc = "dfe_ctrl_0 (rw) register accessor: an alias for `Reg<DFE_CTRL_0_SPEC>`"]
pub type DFE_CTRL_0 = crate::Reg<dfe_ctrl_0::DFE_CTRL_0_SPEC>;
#[doc = "dfe_ctrl_0."]
pub mod dfe_ctrl_0;
#[doc = "dfe_ctrl_1 (rw) register accessor: an alias for `Reg<DFE_CTRL_1_SPEC>`"]
pub type DFE_CTRL_1 = crate::Reg<dfe_ctrl_1::DFE_CTRL_1_SPEC>;
#[doc = "dfe_ctrl_1."]
pub mod dfe_ctrl_1;
#[doc = "dfe_ctrl_2 (rw) register accessor: an alias for `Reg<DFE_CTRL_2_SPEC>`"]
pub type DFE_CTRL_2 = crate::Reg<dfe_ctrl_2::DFE_CTRL_2_SPEC>;
#[doc = "dfe_ctrl_2."]
pub mod dfe_ctrl_2;
#[doc = "dfe_ctrl_3 (rw) register accessor: an alias for `Reg<DFE_CTRL_3_SPEC>`"]
pub type DFE_CTRL_3 = crate::Reg<dfe_ctrl_3::DFE_CTRL_3_SPEC>;
#[doc = "dfe_ctrl_3."]
pub mod dfe_ctrl_3;
#[doc = "dfe_ctrl_4 (rw) register accessor: an alias for `Reg<DFE_CTRL_4_SPEC>`"]
pub type DFE_CTRL_4 = crate::Reg<dfe_ctrl_4::DFE_CTRL_4_SPEC>;
#[doc = "dfe_ctrl_4."]
pub mod dfe_ctrl_4;
#[doc = "dfe_ctrl_5 (rw) register accessor: an alias for `Reg<DFE_CTRL_5_SPEC>`"]
pub type DFE_CTRL_5 = crate::Reg<dfe_ctrl_5::DFE_CTRL_5_SPEC>;
#[doc = "dfe_ctrl_5."]
pub mod dfe_ctrl_5;
#[doc = "dfe_ctrl_6 (rw) register accessor: an alias for `Reg<DFE_CTRL_6_SPEC>`"]
pub type DFE_CTRL_6 = crate::Reg<dfe_ctrl_6::DFE_CTRL_6_SPEC>;
#[doc = "dfe_ctrl_6."]
pub mod dfe_ctrl_6;
#[doc = "dfe_ctrl_7 (rw) register accessor: an alias for `Reg<DFE_CTRL_7_SPEC>`"]
pub type DFE_CTRL_7 = crate::Reg<dfe_ctrl_7::DFE_CTRL_7_SPEC>;
#[doc = "dfe_ctrl_7."]
pub mod dfe_ctrl_7;
#[doc = "dfe_ctrl_8 (rw) register accessor: an alias for `Reg<DFE_CTRL_8_SPEC>`"]
pub type DFE_CTRL_8 = crate::Reg<dfe_ctrl_8::DFE_CTRL_8_SPEC>;
#[doc = "dfe_ctrl_8."]
pub mod dfe_ctrl_8;
#[doc = "dfe_ctrl_9 (rw) register accessor: an alias for `Reg<DFE_CTRL_9_SPEC>`"]
pub type DFE_CTRL_9 = crate::Reg<dfe_ctrl_9::DFE_CTRL_9_SPEC>;
#[doc = "dfe_ctrl_9."]
pub mod dfe_ctrl_9;
#[doc = "dfe_ctrl_10 (rw) register accessor: an alias for `Reg<DFE_CTRL_10_SPEC>`"]
pub type DFE_CTRL_10 = crate::Reg<dfe_ctrl_10::DFE_CTRL_10_SPEC>;
#[doc = "dfe_ctrl_10."]
pub mod dfe_ctrl_10;
#[doc = "dfe_ctrl_11 (rw) register accessor: an alias for `Reg<DFE_CTRL_11_SPEC>`"]
pub type DFE_CTRL_11 = crate::Reg<dfe_ctrl_11::DFE_CTRL_11_SPEC>;
#[doc = "dfe_ctrl_11."]
pub mod dfe_ctrl_11;
#[doc = "dfe_ctrl_12 (rw) register accessor: an alias for `Reg<DFE_CTRL_12_SPEC>`"]
pub type DFE_CTRL_12 = crate::Reg<dfe_ctrl_12::DFE_CTRL_12_SPEC>;
#[doc = "dfe_ctrl_12."]
pub mod dfe_ctrl_12;
#[doc = "dfe_ctrl_13 (rw) register accessor: an alias for `Reg<DFE_CTRL_13_SPEC>`"]
pub type DFE_CTRL_13 = crate::Reg<dfe_ctrl_13::DFE_CTRL_13_SPEC>;
#[doc = "dfe_ctrl_13."]
pub mod dfe_ctrl_13;
#[doc = "dfe_ctrl_14 (rw) register accessor: an alias for `Reg<DFE_CTRL_14_SPEC>`"]
pub type DFE_CTRL_14 = crate::Reg<dfe_ctrl_14::DFE_CTRL_14_SPEC>;
#[doc = "dfe_ctrl_14."]
pub mod dfe_ctrl_14;
#[doc = "dfe_ctrl_15 (rw) register accessor: an alias for `Reg<DFE_CTRL_15_SPEC>`"]
pub type DFE_CTRL_15 = crate::Reg<dfe_ctrl_15::DFE_CTRL_15_SPEC>;
#[doc = "dfe_ctrl_15."]
pub mod dfe_ctrl_15;
#[doc = "dfe_ctrl_16 (rw) register accessor: an alias for `Reg<DFE_CTRL_16_SPEC>`"]
pub type DFE_CTRL_16 = crate::Reg<dfe_ctrl_16::DFE_CTRL_16_SPEC>;
#[doc = "dfe_ctrl_16."]
pub mod dfe_ctrl_16;
#[doc = "dfe_ctrl_17 (rw) register accessor: an alias for `Reg<DFE_CTRL_17_SPEC>`"]
pub type DFE_CTRL_17 = crate::Reg<dfe_ctrl_17::DFE_CTRL_17_SPEC>;
#[doc = "dfe_ctrl_17."]
pub mod dfe_ctrl_17;
#[doc = "dfe_ctrl_18 (rw) register accessor: an alias for `Reg<DFE_CTRL_18_SPEC>`"]
pub type DFE_CTRL_18 = crate::Reg<dfe_ctrl_18::DFE_CTRL_18_SPEC>;
#[doc = "dfe_ctrl_18."]
pub mod dfe_ctrl_18;

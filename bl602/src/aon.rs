#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - aon."]
    pub aon: AON,
    #[doc = "0x04 - aon_common."]
    pub aon_common: AON_COMMON,
    #[doc = "0x08 - aon_misc."]
    pub aon_misc: AON_MISC,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - bg_sys_top."]
    pub bg_sys_top: BG_SYS_TOP,
    #[doc = "0x14 - dcdc18_top_0."]
    pub dcdc18_top_0: DCDC18_TOP_0,
    #[doc = "0x18 - dcdc18_top_1."]
    pub dcdc18_top_1: DCDC18_TOP_1,
    #[doc = "0x1c - ldo11soc_and_dctest."]
    pub ldo11soc_and_dctest: LDO11SOC_AND_DCTEST,
    #[doc = "0x20 - psw_irrcv."]
    pub psw_irrcv: PSW_IRRCV,
    _reserved8: [u8; 0x5c],
    #[doc = "0x80 - rf_top_aon."]
    pub rf_top_aon: RF_TOP_AON,
    #[doc = "0x84 - xtal_cfg."]
    pub xtal_cfg: XTAL_CFG,
    #[doc = "0x88 - tsen."]
    pub tsen: TSEN,
    _reserved11: [u8; 0x74],
    #[doc = "0x100 - acomp0_ctrl."]
    pub acomp0_ctrl: ACOMP0_CTRL,
    #[doc = "0x104 - acomp1_ctrl."]
    pub acomp1_ctrl: ACOMP1_CTRL,
    #[doc = "0x108 - acomp_ctrl."]
    pub acomp_ctrl: ACOMP_CTRL,
    #[doc = "0x10c - gpadc_reg_cmd."]
    pub gpadc_reg_cmd: GPADC_REG_CMD,
    #[doc = "0x110 - gpadc_reg_config1."]
    pub gpadc_reg_config1: GPADC_REG_CONFIG1,
    #[doc = "0x114 - gpadc_reg_config2."]
    pub gpadc_reg_config2: GPADC_REG_CONFIG2,
    #[doc = "0x118 - adc converation sequence 1"]
    pub gpadc_reg_scn_pos1: GPADC_REG_SCN_POS1,
    #[doc = "0x11c - adc converation sequence 2"]
    pub gpadc_reg_scn_pos2: GPADC_REG_SCN_POS2,
    #[doc = "0x120 - adc converation sequence 3"]
    pub gpadc_reg_scn_neg1: GPADC_REG_SCN_NEG1,
    #[doc = "0x124 - adc converation sequence 4"]
    pub gpadc_reg_scn_neg2: GPADC_REG_SCN_NEG2,
    #[doc = "0x128 - gpadc_reg_status."]
    pub gpadc_reg_status: GPADC_REG_STATUS,
    #[doc = "0x12c - gpadc_reg_isr."]
    pub gpadc_reg_isr: GPADC_REG_ISR,
    #[doc = "0x130 - gpadc_reg_result."]
    pub gpadc_reg_result: GPADC_REG_RESULT,
    #[doc = "0x134 - gpadc_reg_raw_result."]
    pub gpadc_reg_raw_result: GPADC_REG_RAW_RESULT,
    #[doc = "0x138 - gpadc_reg_define."]
    pub gpadc_reg_define: GPADC_REG_DEFINE,
    #[doc = "0x13c - hbncore_resv0."]
    pub hbncore_resv0: HBNCORE_RESV0,
    #[doc = "0x140 - hbncore_resv1."]
    pub hbncore_resv1: HBNCORE_RESV1,
}
#[doc = "aon (rw) register accessor: an alias for `Reg<AON_SPEC>`"]
pub type AON = crate::Reg<aon::AON_SPEC>;
#[doc = "aon."]
pub mod aon;
#[doc = "aon_common (rw) register accessor: an alias for `Reg<AON_COMMON_SPEC>`"]
pub type AON_COMMON = crate::Reg<aon_common::AON_COMMON_SPEC>;
#[doc = "aon_common."]
pub mod aon_common;
#[doc = "aon_misc (rw) register accessor: an alias for `Reg<AON_MISC_SPEC>`"]
pub type AON_MISC = crate::Reg<aon_misc::AON_MISC_SPEC>;
#[doc = "aon_misc."]
pub mod aon_misc;
#[doc = "bg_sys_top (rw) register accessor: an alias for `Reg<BG_SYS_TOP_SPEC>`"]
pub type BG_SYS_TOP = crate::Reg<bg_sys_top::BG_SYS_TOP_SPEC>;
#[doc = "bg_sys_top."]
pub mod bg_sys_top;
#[doc = "dcdc18_top_0 (rw) register accessor: an alias for `Reg<DCDC18_TOP_0_SPEC>`"]
pub type DCDC18_TOP_0 = crate::Reg<dcdc18_top_0::DCDC18_TOP_0_SPEC>;
#[doc = "dcdc18_top_0."]
pub mod dcdc18_top_0;
#[doc = "dcdc18_top_1 (rw) register accessor: an alias for `Reg<DCDC18_TOP_1_SPEC>`"]
pub type DCDC18_TOP_1 = crate::Reg<dcdc18_top_1::DCDC18_TOP_1_SPEC>;
#[doc = "dcdc18_top_1."]
pub mod dcdc18_top_1;
#[doc = "ldo11soc_and_dctest (rw) register accessor: an alias for `Reg<LDO11SOC_AND_DCTEST_SPEC>`"]
pub type LDO11SOC_AND_DCTEST = crate::Reg<ldo11soc_and_dctest::LDO11SOC_AND_DCTEST_SPEC>;
#[doc = "ldo11soc_and_dctest."]
pub mod ldo11soc_and_dctest;
#[doc = "psw_irrcv (rw) register accessor: an alias for `Reg<PSW_IRRCV_SPEC>`"]
pub type PSW_IRRCV = crate::Reg<psw_irrcv::PSW_IRRCV_SPEC>;
#[doc = "psw_irrcv."]
pub mod psw_irrcv;
#[doc = "rf_top_aon (rw) register accessor: an alias for `Reg<RF_TOP_AON_SPEC>`"]
pub type RF_TOP_AON = crate::Reg<rf_top_aon::RF_TOP_AON_SPEC>;
#[doc = "rf_top_aon."]
pub mod rf_top_aon;
#[doc = "xtal_cfg (rw) register accessor: an alias for `Reg<XTAL_CFG_SPEC>`"]
pub type XTAL_CFG = crate::Reg<xtal_cfg::XTAL_CFG_SPEC>;
#[doc = "xtal_cfg."]
pub mod xtal_cfg;
#[doc = "tsen (rw) register accessor: an alias for `Reg<TSEN_SPEC>`"]
pub type TSEN = crate::Reg<tsen::TSEN_SPEC>;
#[doc = "tsen."]
pub mod tsen;
#[doc = "acomp0_ctrl (rw) register accessor: an alias for `Reg<ACOMP0_CTRL_SPEC>`"]
pub type ACOMP0_CTRL = crate::Reg<acomp0_ctrl::ACOMP0_CTRL_SPEC>;
#[doc = "acomp0_ctrl."]
pub mod acomp0_ctrl;
#[doc = "acomp1_ctrl (rw) register accessor: an alias for `Reg<ACOMP1_CTRL_SPEC>`"]
pub type ACOMP1_CTRL = crate::Reg<acomp1_ctrl::ACOMP1_CTRL_SPEC>;
#[doc = "acomp1_ctrl."]
pub mod acomp1_ctrl;
#[doc = "acomp_ctrl (rw) register accessor: an alias for `Reg<ACOMP_CTRL_SPEC>`"]
pub type ACOMP_CTRL = crate::Reg<acomp_ctrl::ACOMP_CTRL_SPEC>;
#[doc = "acomp_ctrl."]
pub mod acomp_ctrl;
#[doc = "gpadc_reg_cmd (rw) register accessor: an alias for `Reg<GPADC_REG_CMD_SPEC>`"]
pub type GPADC_REG_CMD = crate::Reg<gpadc_reg_cmd::GPADC_REG_CMD_SPEC>;
#[doc = "gpadc_reg_cmd."]
pub mod gpadc_reg_cmd;
#[doc = "gpadc_reg_config1 (rw) register accessor: an alias for `Reg<GPADC_REG_CONFIG1_SPEC>`"]
pub type GPADC_REG_CONFIG1 = crate::Reg<gpadc_reg_config1::GPADC_REG_CONFIG1_SPEC>;
#[doc = "gpadc_reg_config1."]
pub mod gpadc_reg_config1;
#[doc = "gpadc_reg_config2 (rw) register accessor: an alias for `Reg<GPADC_REG_CONFIG2_SPEC>`"]
pub type GPADC_REG_CONFIG2 = crate::Reg<gpadc_reg_config2::GPADC_REG_CONFIG2_SPEC>;
#[doc = "gpadc_reg_config2."]
pub mod gpadc_reg_config2;
#[doc = "gpadc_reg_scn_pos1 (rw) register accessor: an alias for `Reg<GPADC_REG_SCN_POS1_SPEC>`"]
pub type GPADC_REG_SCN_POS1 = crate::Reg<gpadc_reg_scn_pos1::GPADC_REG_SCN_POS1_SPEC>;
#[doc = "adc converation sequence 1"]
pub mod gpadc_reg_scn_pos1;
#[doc = "gpadc_reg_scn_pos2 (rw) register accessor: an alias for `Reg<GPADC_REG_SCN_POS2_SPEC>`"]
pub type GPADC_REG_SCN_POS2 = crate::Reg<gpadc_reg_scn_pos2::GPADC_REG_SCN_POS2_SPEC>;
#[doc = "adc converation sequence 2"]
pub mod gpadc_reg_scn_pos2;
#[doc = "gpadc_reg_scn_neg1 (rw) register accessor: an alias for `Reg<GPADC_REG_SCN_NEG1_SPEC>`"]
pub type GPADC_REG_SCN_NEG1 = crate::Reg<gpadc_reg_scn_neg1::GPADC_REG_SCN_NEG1_SPEC>;
#[doc = "adc converation sequence 3"]
pub mod gpadc_reg_scn_neg1;
#[doc = "gpadc_reg_scn_neg2 (rw) register accessor: an alias for `Reg<GPADC_REG_SCN_NEG2_SPEC>`"]
pub type GPADC_REG_SCN_NEG2 = crate::Reg<gpadc_reg_scn_neg2::GPADC_REG_SCN_NEG2_SPEC>;
#[doc = "adc converation sequence 4"]
pub mod gpadc_reg_scn_neg2;
#[doc = "gpadc_reg_status (rw) register accessor: an alias for `Reg<GPADC_REG_STATUS_SPEC>`"]
pub type GPADC_REG_STATUS = crate::Reg<gpadc_reg_status::GPADC_REG_STATUS_SPEC>;
#[doc = "gpadc_reg_status."]
pub mod gpadc_reg_status;
#[doc = "gpadc_reg_isr (rw) register accessor: an alias for `Reg<GPADC_REG_ISR_SPEC>`"]
pub type GPADC_REG_ISR = crate::Reg<gpadc_reg_isr::GPADC_REG_ISR_SPEC>;
#[doc = "gpadc_reg_isr."]
pub mod gpadc_reg_isr;
#[doc = "gpadc_reg_result (rw) register accessor: an alias for `Reg<GPADC_REG_RESULT_SPEC>`"]
pub type GPADC_REG_RESULT = crate::Reg<gpadc_reg_result::GPADC_REG_RESULT_SPEC>;
#[doc = "gpadc_reg_result."]
pub mod gpadc_reg_result;
#[doc = "gpadc_reg_raw_result (rw) register accessor: an alias for `Reg<GPADC_REG_RAW_RESULT_SPEC>`"]
pub type GPADC_REG_RAW_RESULT = crate::Reg<gpadc_reg_raw_result::GPADC_REG_RAW_RESULT_SPEC>;
#[doc = "gpadc_reg_raw_result."]
pub mod gpadc_reg_raw_result;
#[doc = "gpadc_reg_define (rw) register accessor: an alias for `Reg<GPADC_REG_DEFINE_SPEC>`"]
pub type GPADC_REG_DEFINE = crate::Reg<gpadc_reg_define::GPADC_REG_DEFINE_SPEC>;
#[doc = "gpadc_reg_define."]
pub mod gpadc_reg_define;
#[doc = "hbncore_resv0 (rw) register accessor: an alias for `Reg<HBNCORE_RESV0_SPEC>`"]
pub type HBNCORE_RESV0 = crate::Reg<hbncore_resv0::HBNCORE_RESV0_SPEC>;
#[doc = "hbncore_resv0."]
pub mod hbncore_resv0;
#[doc = "hbncore_resv1 (rw) register accessor: an alias for `Reg<HBNCORE_RESV1_SPEC>`"]
pub type HBNCORE_RESV1 = crate::Reg<hbncore_resv1::HBNCORE_RESV1_SPEC>;
#[doc = "hbncore_resv1."]
pub mod hbncore_resv1;

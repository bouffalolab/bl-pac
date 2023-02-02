#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDS_CTL."]
    pub pds_ctl: PDS_CTL,
    #[doc = "0x04 - PDS_TIME1."]
    pub pds_time1: PDS_TIME1,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - PDS_INT."]
    pub pds_int: PDS_INT,
    #[doc = "0x10 - PDS_CTL2."]
    pub pds_ctl2: PDS_CTL2,
    #[doc = "0x14 - PDS_CTL3."]
    pub pds_ctl3: PDS_CTL3,
    #[doc = "0x18 - PDS_CTL4."]
    pub pds_ctl4: PDS_CTL4,
    #[doc = "0x1c - pds_stat."]
    pub pds_stat: PDS_STAT,
    #[doc = "0x20 - pds_ram1."]
    pub pds_ram1: PDS_RAM1,
    _reserved8: [u8; 0x02dc],
    #[doc = "0x300 - rc32m_ctrl0."]
    pub rc32m_ctrl0: RC32M_CTRL0,
    #[doc = "0x304 - rc32m_ctrl1."]
    pub rc32m_ctrl1: RC32M_CTRL1,
    _reserved10: [u8; 0xf8],
    #[doc = "0x400 - pu_rst_clkpll."]
    pub pu_rst_clkpll: PU_RST_CLKPLL,
    #[doc = "0x404 - clkpll_top_ctrl."]
    pub clkpll_top_ctrl: CLKPLL_TOP_CTRL,
    #[doc = "0x408 - clkpll_cp."]
    pub clkpll_cp: CLKPLL_CP,
    #[doc = "0x40c - clkpll_rz."]
    pub clkpll_rz: CLKPLL_RZ,
    #[doc = "0x410 - clkpll_fbdv."]
    pub clkpll_fbdv: CLKPLL_FBDV,
    #[doc = "0x414 - clkpll_vco."]
    pub clkpll_vco: CLKPLL_VCO,
    #[doc = "0x418 - clkpll_sdm."]
    pub clkpll_sdm: CLKPLL_SDM,
    #[doc = "0x41c - clkpll_output_en."]
    pub clkpll_output_en: CLKPLL_OUTPUT_EN,
}
#[doc = "PDS_CTL (rw) register accessor: an alias for `Reg<PDS_CTL_SPEC>`"]
pub type PDS_CTL = crate::Reg<pds_ctl::PDS_CTL_SPEC>;
#[doc = "PDS_CTL."]
pub mod pds_ctl;
#[doc = "PDS_TIME1 (rw) register accessor: an alias for `Reg<PDS_TIME1_SPEC>`"]
pub type PDS_TIME1 = crate::Reg<pds_time1::PDS_TIME1_SPEC>;
#[doc = "PDS_TIME1."]
pub mod pds_time1;
#[doc = "PDS_INT (rw) register accessor: an alias for `Reg<PDS_INT_SPEC>`"]
pub type PDS_INT = crate::Reg<pds_int::PDS_INT_SPEC>;
#[doc = "PDS_INT."]
pub mod pds_int;
#[doc = "PDS_CTL2 (rw) register accessor: an alias for `Reg<PDS_CTL2_SPEC>`"]
pub type PDS_CTL2 = crate::Reg<pds_ctl2::PDS_CTL2_SPEC>;
#[doc = "PDS_CTL2."]
pub mod pds_ctl2;
#[doc = "PDS_CTL3 (rw) register accessor: an alias for `Reg<PDS_CTL3_SPEC>`"]
pub type PDS_CTL3 = crate::Reg<pds_ctl3::PDS_CTL3_SPEC>;
#[doc = "PDS_CTL3."]
pub mod pds_ctl3;
#[doc = "PDS_CTL4 (rw) register accessor: an alias for `Reg<PDS_CTL4_SPEC>`"]
pub type PDS_CTL4 = crate::Reg<pds_ctl4::PDS_CTL4_SPEC>;
#[doc = "PDS_CTL4."]
pub mod pds_ctl4;
#[doc = "pds_stat (r) register accessor: an alias for `Reg<PDS_STAT_SPEC>`"]
pub type PDS_STAT = crate::Reg<pds_stat::PDS_STAT_SPEC>;
#[doc = "pds_stat."]
pub mod pds_stat;
#[doc = "pds_ram1 (rw) register accessor: an alias for `Reg<PDS_RAM1_SPEC>`"]
pub type PDS_RAM1 = crate::Reg<pds_ram1::PDS_RAM1_SPEC>;
#[doc = "pds_ram1."]
pub mod pds_ram1;
#[doc = "rc32m_ctrl0 (rw) register accessor: an alias for `Reg<RC32M_CTRL0_SPEC>`"]
pub type RC32M_CTRL0 = crate::Reg<rc32m_ctrl0::RC32M_CTRL0_SPEC>;
#[doc = "rc32m_ctrl0."]
pub mod rc32m_ctrl0;
#[doc = "rc32m_ctrl1 (rw) register accessor: an alias for `Reg<RC32M_CTRL1_SPEC>`"]
pub type RC32M_CTRL1 = crate::Reg<rc32m_ctrl1::RC32M_CTRL1_SPEC>;
#[doc = "rc32m_ctrl1."]
pub mod rc32m_ctrl1;
#[doc = "pu_rst_clkpll (rw) register accessor: an alias for `Reg<PU_RST_CLKPLL_SPEC>`"]
pub type PU_RST_CLKPLL = crate::Reg<pu_rst_clkpll::PU_RST_CLKPLL_SPEC>;
#[doc = "pu_rst_clkpll."]
pub mod pu_rst_clkpll;
#[doc = "clkpll_top_ctrl (rw) register accessor: an alias for `Reg<CLKPLL_TOP_CTRL_SPEC>`"]
pub type CLKPLL_TOP_CTRL = crate::Reg<clkpll_top_ctrl::CLKPLL_TOP_CTRL_SPEC>;
#[doc = "clkpll_top_ctrl."]
pub mod clkpll_top_ctrl;
#[doc = "clkpll_cp (rw) register accessor: an alias for `Reg<CLKPLL_CP_SPEC>`"]
pub type CLKPLL_CP = crate::Reg<clkpll_cp::CLKPLL_CP_SPEC>;
#[doc = "clkpll_cp."]
pub mod clkpll_cp;
#[doc = "clkpll_rz (rw) register accessor: an alias for `Reg<CLKPLL_RZ_SPEC>`"]
pub type CLKPLL_RZ = crate::Reg<clkpll_rz::CLKPLL_RZ_SPEC>;
#[doc = "clkpll_rz."]
pub mod clkpll_rz;
#[doc = "clkpll_fbdv (rw) register accessor: an alias for `Reg<CLKPLL_FBDV_SPEC>`"]
pub type CLKPLL_FBDV = crate::Reg<clkpll_fbdv::CLKPLL_FBDV_SPEC>;
#[doc = "clkpll_fbdv."]
pub mod clkpll_fbdv;
#[doc = "clkpll_vco (rw) register accessor: an alias for `Reg<CLKPLL_VCO_SPEC>`"]
pub type CLKPLL_VCO = crate::Reg<clkpll_vco::CLKPLL_VCO_SPEC>;
#[doc = "clkpll_vco."]
pub mod clkpll_vco;
#[doc = "clkpll_sdm (rw) register accessor: an alias for `Reg<CLKPLL_SDM_SPEC>`"]
pub type CLKPLL_SDM = crate::Reg<clkpll_sdm::CLKPLL_SDM_SPEC>;
#[doc = "clkpll_sdm."]
pub mod clkpll_sdm;
#[doc = "clkpll_output_en (rw) register accessor: an alias for `Reg<CLKPLL_OUTPUT_EN_SPEC>`"]
pub type CLKPLL_OUTPUT_EN = crate::Reg<clkpll_output_en::CLKPLL_OUTPUT_EN_SPEC>;
#[doc = "clkpll_output_en."]
pub mod clkpll_output_en;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - cci_cfg."]
    pub cci_cfg: CCI_CFG,
    #[doc = "0x04 - cci_addr."]
    pub cci_addr: CCI_ADDR,
    #[doc = "0x08 - cci_wdata."]
    pub cci_wdata: CCI_WDATA,
    #[doc = "0x0c - cci_rdata."]
    pub cci_rdata: CCI_RDATA,
    #[doc = "0x10 - cci_ctl."]
    pub cci_ctl: CCI_CTL,
}
#[doc = "cci_cfg (rw) register accessor: an alias for `Reg<CCI_CFG_SPEC>`"]
pub type CCI_CFG = crate::Reg<cci_cfg::CCI_CFG_SPEC>;
#[doc = "cci_cfg."]
pub mod cci_cfg;
#[doc = "cci_addr (rw) register accessor: an alias for `Reg<CCI_ADDR_SPEC>`"]
pub type CCI_ADDR = crate::Reg<cci_addr::CCI_ADDR_SPEC>;
#[doc = "cci_addr."]
pub mod cci_addr;
#[doc = "cci_wdata (rw) register accessor: an alias for `Reg<CCI_WDATA_SPEC>`"]
pub type CCI_WDATA = crate::Reg<cci_wdata::CCI_WDATA_SPEC>;
#[doc = "cci_wdata."]
pub mod cci_wdata;
#[doc = "cci_rdata (rw) register accessor: an alias for `Reg<CCI_RDATA_SPEC>`"]
pub type CCI_RDATA = crate::Reg<cci_rdata::CCI_RDATA_SPEC>;
#[doc = "cci_rdata."]
pub mod cci_rdata;
#[doc = "cci_ctl (rw) register accessor: an alias for `Reg<CCI_CTL_SPEC>`"]
pub type CCI_CTL = crate::Reg<cci_ctl::CCI_CTL_SPEC>;
#[doc = "cci_ctl."]
pub mod cci_ctl;

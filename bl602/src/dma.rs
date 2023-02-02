#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA_IntStatus."]
    pub dma_int_status: DMA_INT_STATUS,
    #[doc = "0x04 - DMA_IntTCStatus."]
    pub dma_int_tcstatus: DMA_INT_TCSTATUS,
    #[doc = "0x08 - DMA_IntTCClear."]
    pub dma_int_tcclear: DMA_INT_TCCLEAR,
    #[doc = "0x0c - DMA_IntErrorStatus."]
    pub dma_int_error_status: DMA_INT_ERROR_STATUS,
    #[doc = "0x10 - DMA_IntErrClr."]
    pub dma_int_err_clr: DMA_INT_ERR_CLR,
    #[doc = "0x14 - DMA_RawIntTCStatus."]
    pub dma_raw_int_tcstatus: DMA_RAW_INT_TCSTATUS,
    #[doc = "0x18 - DMA_RawIntErrorStatus."]
    pub dma_raw_int_error_status: DMA_RAW_INT_ERROR_STATUS,
    #[doc = "0x1c - DMA_EnbldChns."]
    pub dma_enbld_chns: DMA_ENBLD_CHNS,
    #[doc = "0x20 - DMA_SoftBReq."]
    pub dma_soft_breq: DMA_SOFT_BREQ,
    #[doc = "0x24 - DMA_SoftSReq."]
    pub dma_soft_sreq: DMA_SOFT_SREQ,
    #[doc = "0x28 - DMA_SoftLBReq."]
    pub dma_soft_lbreq: DMA_SOFT_LBREQ,
    #[doc = "0x2c - DMA_SoftLSReq."]
    pub dma_soft_lsreq: DMA_SOFT_LSREQ,
    #[doc = "0x30 - DMA_Top_Config."]
    pub dma_top_config: DMA_TOP_CONFIG,
    #[doc = "0x34 - DMA_Sync."]
    pub dma_sync: DMA_SYNC,
    _reserved14: [u8; 0xc8],
    #[doc = "0x100 - DMA_C0SrcAddr."]
    pub dma_c0src_addr: DMA_C0SRC_ADDR,
    #[doc = "0x104 - DMA_C0DstAddr."]
    pub dma_c0dst_addr: DMA_C0DST_ADDR,
    #[doc = "0x108 - DMA_C0LLI."]
    pub dma_c0lli: DMA_C0LLI,
    #[doc = "0x10c - DMA_C0Control."]
    pub dma_c0control: DMA_C0CONTROL,
    #[doc = "0x110 - DMA_C0Config."]
    pub dma_c0config: DMA_C0CONFIG,
    _reserved19: [u8; 0xec],
    #[doc = "0x200 - DMA_C1SrcAddr."]
    pub dma_c1src_addr: DMA_C1SRC_ADDR,
    #[doc = "0x204 - DMA_C1DstAddr."]
    pub dma_c1dst_addr: DMA_C1DST_ADDR,
    #[doc = "0x208 - DMA_C1LLI."]
    pub dma_c1lli: DMA_C1LLI,
    #[doc = "0x20c - DMA_C1Control."]
    pub dma_c1control: DMA_C1CONTROL,
    #[doc = "0x210 - DMA_C1Config."]
    pub dma_c1config: DMA_C1CONFIG,
    _reserved24: [u8; 0xec],
    #[doc = "0x300 - DMA_C2SrcAddr."]
    pub dma_c2src_addr: DMA_C2SRC_ADDR,
    #[doc = "0x304 - DMA_C2DstAddr."]
    pub dma_c2dst_addr: DMA_C2DST_ADDR,
    #[doc = "0x308 - DMA_C2LLI."]
    pub dma_c2lli: DMA_C2LLI,
    #[doc = "0x30c - DMA_C2Control."]
    pub dma_c2control: DMA_C2CONTROL,
    #[doc = "0x310 - DMA_C2Config."]
    pub dma_c2config: DMA_C2CONFIG,
    _reserved29: [u8; 0xec],
    #[doc = "0x400 - DMA_C3SrcAddr."]
    pub dma_c3src_addr: DMA_C3SRC_ADDR,
    #[doc = "0x404 - DMA_C3DstAddr."]
    pub dma_c3dst_addr: DMA_C3DST_ADDR,
    #[doc = "0x408 - DMA_C3LLI."]
    pub dma_c3lli: DMA_C3LLI,
    #[doc = "0x40c - DMA_C3Control."]
    pub dma_c3control: DMA_C3CONTROL,
    #[doc = "0x410 - DMA_C3Config."]
    pub dma_c3config: DMA_C3CONFIG,
}
#[doc = "DMA_IntStatus (rw) register accessor: an alias for `Reg<DMA_INT_STATUS_SPEC>`"]
pub type DMA_INT_STATUS = crate::Reg<dma_int_status::DMA_INT_STATUS_SPEC>;
#[doc = "DMA_IntStatus."]
pub mod dma_int_status;
#[doc = "DMA_IntTCStatus (rw) register accessor: an alias for `Reg<DMA_INT_TCSTATUS_SPEC>`"]
pub type DMA_INT_TCSTATUS = crate::Reg<dma_int_tcstatus::DMA_INT_TCSTATUS_SPEC>;
#[doc = "DMA_IntTCStatus."]
pub mod dma_int_tcstatus;
#[doc = "DMA_IntTCClear (rw) register accessor: an alias for `Reg<DMA_INT_TCCLEAR_SPEC>`"]
pub type DMA_INT_TCCLEAR = crate::Reg<dma_int_tcclear::DMA_INT_TCCLEAR_SPEC>;
#[doc = "DMA_IntTCClear."]
pub mod dma_int_tcclear;
#[doc = "DMA_IntErrorStatus (rw) register accessor: an alias for `Reg<DMA_INT_ERROR_STATUS_SPEC>`"]
pub type DMA_INT_ERROR_STATUS = crate::Reg<dma_int_error_status::DMA_INT_ERROR_STATUS_SPEC>;
#[doc = "DMA_IntErrorStatus."]
pub mod dma_int_error_status;
#[doc = "DMA_IntErrClr (rw) register accessor: an alias for `Reg<DMA_INT_ERR_CLR_SPEC>`"]
pub type DMA_INT_ERR_CLR = crate::Reg<dma_int_err_clr::DMA_INT_ERR_CLR_SPEC>;
#[doc = "DMA_IntErrClr."]
pub mod dma_int_err_clr;
#[doc = "DMA_RawIntTCStatus (rw) register accessor: an alias for `Reg<DMA_RAW_INT_TCSTATUS_SPEC>`"]
pub type DMA_RAW_INT_TCSTATUS = crate::Reg<dma_raw_int_tcstatus::DMA_RAW_INT_TCSTATUS_SPEC>;
#[doc = "DMA_RawIntTCStatus."]
pub mod dma_raw_int_tcstatus;
#[doc = "DMA_RawIntErrorStatus (rw) register accessor: an alias for `Reg<DMA_RAW_INT_ERROR_STATUS_SPEC>`"]
pub type DMA_RAW_INT_ERROR_STATUS =
    crate::Reg<dma_raw_int_error_status::DMA_RAW_INT_ERROR_STATUS_SPEC>;
#[doc = "DMA_RawIntErrorStatus."]
pub mod dma_raw_int_error_status;
#[doc = "DMA_EnbldChns (rw) register accessor: an alias for `Reg<DMA_ENBLD_CHNS_SPEC>`"]
pub type DMA_ENBLD_CHNS = crate::Reg<dma_enbld_chns::DMA_ENBLD_CHNS_SPEC>;
#[doc = "DMA_EnbldChns."]
pub mod dma_enbld_chns;
#[doc = "DMA_SoftBReq (rw) register accessor: an alias for `Reg<DMA_SOFT_BREQ_SPEC>`"]
pub type DMA_SOFT_BREQ = crate::Reg<dma_soft_breq::DMA_SOFT_BREQ_SPEC>;
#[doc = "DMA_SoftBReq."]
pub mod dma_soft_breq;
#[doc = "DMA_SoftSReq (rw) register accessor: an alias for `Reg<DMA_SOFT_SREQ_SPEC>`"]
pub type DMA_SOFT_SREQ = crate::Reg<dma_soft_sreq::DMA_SOFT_SREQ_SPEC>;
#[doc = "DMA_SoftSReq."]
pub mod dma_soft_sreq;
#[doc = "DMA_SoftLBReq (rw) register accessor: an alias for `Reg<DMA_SOFT_LBREQ_SPEC>`"]
pub type DMA_SOFT_LBREQ = crate::Reg<dma_soft_lbreq::DMA_SOFT_LBREQ_SPEC>;
#[doc = "DMA_SoftLBReq."]
pub mod dma_soft_lbreq;
#[doc = "DMA_SoftLSReq (rw) register accessor: an alias for `Reg<DMA_SOFT_LSREQ_SPEC>`"]
pub type DMA_SOFT_LSREQ = crate::Reg<dma_soft_lsreq::DMA_SOFT_LSREQ_SPEC>;
#[doc = "DMA_SoftLSReq."]
pub mod dma_soft_lsreq;
#[doc = "DMA_Top_Config (rw) register accessor: an alias for `Reg<DMA_TOP_CONFIG_SPEC>`"]
pub type DMA_TOP_CONFIG = crate::Reg<dma_top_config::DMA_TOP_CONFIG_SPEC>;
#[doc = "DMA_Top_Config."]
pub mod dma_top_config;
#[doc = "DMA_Sync (rw) register accessor: an alias for `Reg<DMA_SYNC_SPEC>`"]
pub type DMA_SYNC = crate::Reg<dma_sync::DMA_SYNC_SPEC>;
#[doc = "DMA_Sync."]
pub mod dma_sync;
#[doc = "DMA_C0SrcAddr (rw) register accessor: an alias for `Reg<DMA_C0SRC_ADDR_SPEC>`"]
pub type DMA_C0SRC_ADDR = crate::Reg<dma_c0src_addr::DMA_C0SRC_ADDR_SPEC>;
#[doc = "DMA_C0SrcAddr."]
pub mod dma_c0src_addr;
#[doc = "DMA_C0DstAddr (rw) register accessor: an alias for `Reg<DMA_C0DST_ADDR_SPEC>`"]
pub type DMA_C0DST_ADDR = crate::Reg<dma_c0dst_addr::DMA_C0DST_ADDR_SPEC>;
#[doc = "DMA_C0DstAddr."]
pub mod dma_c0dst_addr;
#[doc = "DMA_C0LLI (rw) register accessor: an alias for `Reg<DMA_C0LLI_SPEC>`"]
pub type DMA_C0LLI = crate::Reg<dma_c0lli::DMA_C0LLI_SPEC>;
#[doc = "DMA_C0LLI."]
pub mod dma_c0lli;
#[doc = "DMA_C0Control (rw) register accessor: an alias for `Reg<DMA_C0CONTROL_SPEC>`"]
pub type DMA_C0CONTROL = crate::Reg<dma_c0control::DMA_C0CONTROL_SPEC>;
#[doc = "DMA_C0Control."]
pub mod dma_c0control;
#[doc = "DMA_C0Config (rw) register accessor: an alias for `Reg<DMA_C0CONFIG_SPEC>`"]
pub type DMA_C0CONFIG = crate::Reg<dma_c0config::DMA_C0CONFIG_SPEC>;
#[doc = "DMA_C0Config."]
pub mod dma_c0config;
#[doc = "DMA_C1SrcAddr (rw) register accessor: an alias for `Reg<DMA_C1SRC_ADDR_SPEC>`"]
pub type DMA_C1SRC_ADDR = crate::Reg<dma_c1src_addr::DMA_C1SRC_ADDR_SPEC>;
#[doc = "DMA_C1SrcAddr."]
pub mod dma_c1src_addr;
#[doc = "DMA_C1DstAddr (rw) register accessor: an alias for `Reg<DMA_C1DST_ADDR_SPEC>`"]
pub type DMA_C1DST_ADDR = crate::Reg<dma_c1dst_addr::DMA_C1DST_ADDR_SPEC>;
#[doc = "DMA_C1DstAddr."]
pub mod dma_c1dst_addr;
#[doc = "DMA_C1LLI (rw) register accessor: an alias for `Reg<DMA_C1LLI_SPEC>`"]
pub type DMA_C1LLI = crate::Reg<dma_c1lli::DMA_C1LLI_SPEC>;
#[doc = "DMA_C1LLI."]
pub mod dma_c1lli;
#[doc = "DMA_C1Control (rw) register accessor: an alias for `Reg<DMA_C1CONTROL_SPEC>`"]
pub type DMA_C1CONTROL = crate::Reg<dma_c1control::DMA_C1CONTROL_SPEC>;
#[doc = "DMA_C1Control."]
pub mod dma_c1control;
#[doc = "DMA_C1Config (rw) register accessor: an alias for `Reg<DMA_C1CONFIG_SPEC>`"]
pub type DMA_C1CONFIG = crate::Reg<dma_c1config::DMA_C1CONFIG_SPEC>;
#[doc = "DMA_C1Config."]
pub mod dma_c1config;
#[doc = "DMA_C2SrcAddr (rw) register accessor: an alias for `Reg<DMA_C2SRC_ADDR_SPEC>`"]
pub type DMA_C2SRC_ADDR = crate::Reg<dma_c2src_addr::DMA_C2SRC_ADDR_SPEC>;
#[doc = "DMA_C2SrcAddr."]
pub mod dma_c2src_addr;
#[doc = "DMA_C2DstAddr (rw) register accessor: an alias for `Reg<DMA_C2DST_ADDR_SPEC>`"]
pub type DMA_C2DST_ADDR = crate::Reg<dma_c2dst_addr::DMA_C2DST_ADDR_SPEC>;
#[doc = "DMA_C2DstAddr."]
pub mod dma_c2dst_addr;
#[doc = "DMA_C2LLI (rw) register accessor: an alias for `Reg<DMA_C2LLI_SPEC>`"]
pub type DMA_C2LLI = crate::Reg<dma_c2lli::DMA_C2LLI_SPEC>;
#[doc = "DMA_C2LLI."]
pub mod dma_c2lli;
#[doc = "DMA_C2Control (rw) register accessor: an alias for `Reg<DMA_C2CONTROL_SPEC>`"]
pub type DMA_C2CONTROL = crate::Reg<dma_c2control::DMA_C2CONTROL_SPEC>;
#[doc = "DMA_C2Control."]
pub mod dma_c2control;
#[doc = "DMA_C2Config (rw) register accessor: an alias for `Reg<DMA_C2CONFIG_SPEC>`"]
pub type DMA_C2CONFIG = crate::Reg<dma_c2config::DMA_C2CONFIG_SPEC>;
#[doc = "DMA_C2Config."]
pub mod dma_c2config;
#[doc = "DMA_C3SrcAddr (rw) register accessor: an alias for `Reg<DMA_C3SRC_ADDR_SPEC>`"]
pub type DMA_C3SRC_ADDR = crate::Reg<dma_c3src_addr::DMA_C3SRC_ADDR_SPEC>;
#[doc = "DMA_C3SrcAddr."]
pub mod dma_c3src_addr;
#[doc = "DMA_C3DstAddr (rw) register accessor: an alias for `Reg<DMA_C3DST_ADDR_SPEC>`"]
pub type DMA_C3DST_ADDR = crate::Reg<dma_c3dst_addr::DMA_C3DST_ADDR_SPEC>;
#[doc = "DMA_C3DstAddr."]
pub mod dma_c3dst_addr;
#[doc = "DMA_C3LLI (rw) register accessor: an alias for `Reg<DMA_C3LLI_SPEC>`"]
pub type DMA_C3LLI = crate::Reg<dma_c3lli::DMA_C3LLI_SPEC>;
#[doc = "DMA_C3LLI."]
pub mod dma_c3lli;
#[doc = "DMA_C3Control (rw) register accessor: an alias for `Reg<DMA_C3CONTROL_SPEC>`"]
pub type DMA_C3CONTROL = crate::Reg<dma_c3control::DMA_C3CONTROL_SPEC>;
#[doc = "DMA_C3Control."]
pub mod dma_c3control;
#[doc = "DMA_C3Config (rw) register accessor: an alias for `Reg<DMA_C3CONFIG_SPEC>`"]
pub type DMA_C3CONFIG = crate::Reg<dma_c3config::DMA_C3CONFIG_SPEC>;
#[doc = "DMA_C3Config."]
pub mod dma_c3config;

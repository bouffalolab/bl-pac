#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - utx_config."]
    pub utx_config: UTX_CONFIG,
    #[doc = "0x04 - urx_config."]
    pub urx_config: URX_CONFIG,
    #[doc = "0x08 - uart_bit_prd."]
    pub uart_bit_prd: UART_BIT_PRD,
    #[doc = "0x0c - data_config."]
    pub data_config: DATA_CONFIG,
    #[doc = "0x10 - utx_ir_position."]
    pub utx_ir_position: UTX_IR_POSITION,
    #[doc = "0x14 - urx_ir_position."]
    pub urx_ir_position: URX_IR_POSITION,
    #[doc = "0x18 - urx_rto_timer."]
    pub urx_rto_timer: URX_RTO_TIMER,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - UART interrupt status"]
    pub uart_int_sts: UART_INT_STS,
    #[doc = "0x24 - UART interrupt mask"]
    pub uart_int_mask: UART_INT_MASK,
    #[doc = "0x28 - UART interrupt clear"]
    pub uart_int_clear: UART_INT_CLEAR,
    #[doc = "0x2c - UART interrupt enable"]
    pub uart_int_en: UART_INT_EN,
    #[doc = "0x30 - uart_status."]
    pub uart_status: UART_STATUS,
    #[doc = "0x34 - sts_urx_abr_prd."]
    pub sts_urx_abr_prd: STS_URX_ABR_PRD,
    _reserved13: [u8; 0x48],
    #[doc = "0x80 - uart_fifo_config_0."]
    pub uart_fifo_config_0: UART_FIFO_CONFIG_0,
    #[doc = "0x84 - uart_fifo_config_1."]
    pub uart_fifo_config_1: UART_FIFO_CONFIG_1,
    #[doc = "0x88 - uart_fifo_wdata."]
    pub uart_fifo_wdata: UART_FIFO_WDATA,
    #[doc = "0x8c - uart_fifo_rdata."]
    pub uart_fifo_rdata: UART_FIFO_RDATA,
}
#[doc = "utx_config (rw) register accessor: an alias for `Reg<UTX_CONFIG_SPEC>`"]
pub type UTX_CONFIG = crate::Reg<utx_config::UTX_CONFIG_SPEC>;
#[doc = "utx_config."]
pub mod utx_config;
#[doc = "urx_config (rw) register accessor: an alias for `Reg<URX_CONFIG_SPEC>`"]
pub type URX_CONFIG = crate::Reg<urx_config::URX_CONFIG_SPEC>;
#[doc = "urx_config."]
pub mod urx_config;
#[doc = "uart_bit_prd (rw) register accessor: an alias for `Reg<UART_BIT_PRD_SPEC>`"]
pub type UART_BIT_PRD = crate::Reg<uart_bit_prd::UART_BIT_PRD_SPEC>;
#[doc = "uart_bit_prd."]
pub mod uart_bit_prd;
#[doc = "data_config (rw) register accessor: an alias for `Reg<DATA_CONFIG_SPEC>`"]
pub type DATA_CONFIG = crate::Reg<data_config::DATA_CONFIG_SPEC>;
#[doc = "data_config."]
pub mod data_config;
#[doc = "utx_ir_position (rw) register accessor: an alias for `Reg<UTX_IR_POSITION_SPEC>`"]
pub type UTX_IR_POSITION = crate::Reg<utx_ir_position::UTX_IR_POSITION_SPEC>;
#[doc = "utx_ir_position."]
pub mod utx_ir_position;
#[doc = "urx_ir_position (rw) register accessor: an alias for `Reg<URX_IR_POSITION_SPEC>`"]
pub type URX_IR_POSITION = crate::Reg<urx_ir_position::URX_IR_POSITION_SPEC>;
#[doc = "urx_ir_position."]
pub mod urx_ir_position;
#[doc = "urx_rto_timer (rw) register accessor: an alias for `Reg<URX_RTO_TIMER_SPEC>`"]
pub type URX_RTO_TIMER = crate::Reg<urx_rto_timer::URX_RTO_TIMER_SPEC>;
#[doc = "urx_rto_timer."]
pub mod urx_rto_timer;
#[doc = "uart_int_sts (r) register accessor: an alias for `Reg<UART_INT_STS_SPEC>`"]
pub type UART_INT_STS = crate::Reg<uart_int_sts::UART_INT_STS_SPEC>;
#[doc = "UART interrupt status"]
pub mod uart_int_sts;
#[doc = "uart_int_mask (rw) register accessor: an alias for `Reg<UART_INT_MASK_SPEC>`"]
pub type UART_INT_MASK = crate::Reg<uart_int_mask::UART_INT_MASK_SPEC>;
#[doc = "UART interrupt mask"]
pub mod uart_int_mask;
#[doc = "uart_int_clear (rw) register accessor: an alias for `Reg<UART_INT_CLEAR_SPEC>`"]
pub type UART_INT_CLEAR = crate::Reg<uart_int_clear::UART_INT_CLEAR_SPEC>;
#[doc = "UART interrupt clear"]
pub mod uart_int_clear;
#[doc = "uart_int_en (rw) register accessor: an alias for `Reg<UART_INT_EN_SPEC>`"]
pub type UART_INT_EN = crate::Reg<uart_int_en::UART_INT_EN_SPEC>;
#[doc = "UART interrupt enable"]
pub mod uart_int_en;
#[doc = "uart_status (r) register accessor: an alias for `Reg<UART_STATUS_SPEC>`"]
pub type UART_STATUS = crate::Reg<uart_status::UART_STATUS_SPEC>;
#[doc = "uart_status."]
pub mod uart_status;
#[doc = "sts_urx_abr_prd (r) register accessor: an alias for `Reg<STS_URX_ABR_PRD_SPEC>`"]
pub type STS_URX_ABR_PRD = crate::Reg<sts_urx_abr_prd::STS_URX_ABR_PRD_SPEC>;
#[doc = "sts_urx_abr_prd."]
pub mod sts_urx_abr_prd;
#[doc = "uart_fifo_config_0 (rw) register accessor: an alias for `Reg<UART_FIFO_CONFIG_0_SPEC>`"]
pub type UART_FIFO_CONFIG_0 = crate::Reg<uart_fifo_config_0::UART_FIFO_CONFIG_0_SPEC>;
#[doc = "uart_fifo_config_0."]
pub mod uart_fifo_config_0;
#[doc = "uart_fifo_config_1 (rw) register accessor: an alias for `Reg<UART_FIFO_CONFIG_1_SPEC>`"]
pub type UART_FIFO_CONFIG_1 = crate::Reg<uart_fifo_config_1::UART_FIFO_CONFIG_1_SPEC>;
#[doc = "uart_fifo_config_1."]
pub mod uart_fifo_config_1;
#[doc = "uart_fifo_wdata (w) register accessor: an alias for `Reg<UART_FIFO_WDATA_SPEC>`"]
pub type UART_FIFO_WDATA = crate::Reg<uart_fifo_wdata::UART_FIFO_WDATA_SPEC>;
#[doc = "uart_fifo_wdata."]
pub mod uart_fifo_wdata;
#[doc = "uart_fifo_rdata (r) register accessor: an alias for `Reg<UART_FIFO_RDATA_SPEC>`"]
pub type UART_FIFO_RDATA = crate::Reg<uart_fifo_rdata::UART_FIFO_RDATA_SPEC>;
#[doc = "uart_fifo_rdata."]
pub mod uart_fifo_rdata;

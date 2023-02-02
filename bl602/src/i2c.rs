#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - i2c_config."]
    pub i2c_config: I2C_CONFIG,
    #[doc = "0x04 - i2c_int_sts."]
    pub i2c_int_sts: I2C_INT_STS,
    #[doc = "0x08 - i2c_sub_addr."]
    pub i2c_sub_addr: I2C_SUB_ADDR,
    #[doc = "0x0c - i2c_bus_busy."]
    pub i2c_bus_busy: I2C_BUS_BUSY,
    #[doc = "0x10 - i2c_prd_start."]
    pub i2c_prd_start: I2C_PRD_START,
    #[doc = "0x14 - i2c_prd_stop."]
    pub i2c_prd_stop: I2C_PRD_STOP,
    #[doc = "0x18 - i2c_prd_data."]
    pub i2c_prd_data: I2C_PRD_DATA,
    _reserved7: [u8; 0x64],
    #[doc = "0x80 - i2c_fifo_config_0."]
    pub i2c_fifo_config_0: I2C_FIFO_CONFIG_0,
    #[doc = "0x84 - i2c_fifo_config_1."]
    pub i2c_fifo_config_1: I2C_FIFO_CONFIG_1,
    #[doc = "0x88 - i2c_fifo_wdata."]
    pub i2c_fifo_wdata: I2C_FIFO_WDATA,
    #[doc = "0x8c - i2c_fifo_rdata."]
    pub i2c_fifo_rdata: I2C_FIFO_RDATA,
}
#[doc = "i2c_config (rw) register accessor: an alias for `Reg<I2C_CONFIG_SPEC>`"]
pub type I2C_CONFIG = crate::Reg<i2c_config::I2C_CONFIG_SPEC>;
#[doc = "i2c_config."]
pub mod i2c_config;
#[doc = "i2c_int_sts (rw) register accessor: an alias for `Reg<I2C_INT_STS_SPEC>`"]
pub type I2C_INT_STS = crate::Reg<i2c_int_sts::I2C_INT_STS_SPEC>;
#[doc = "i2c_int_sts."]
pub mod i2c_int_sts;
#[doc = "i2c_sub_addr (rw) register accessor: an alias for `Reg<I2C_SUB_ADDR_SPEC>`"]
pub type I2C_SUB_ADDR = crate::Reg<i2c_sub_addr::I2C_SUB_ADDR_SPEC>;
#[doc = "i2c_sub_addr."]
pub mod i2c_sub_addr;
#[doc = "i2c_bus_busy (rw) register accessor: an alias for `Reg<I2C_BUS_BUSY_SPEC>`"]
pub type I2C_BUS_BUSY = crate::Reg<i2c_bus_busy::I2C_BUS_BUSY_SPEC>;
#[doc = "i2c_bus_busy."]
pub mod i2c_bus_busy;
#[doc = "i2c_prd_start (rw) register accessor: an alias for `Reg<I2C_PRD_START_SPEC>`"]
pub type I2C_PRD_START = crate::Reg<i2c_prd_start::I2C_PRD_START_SPEC>;
#[doc = "i2c_prd_start."]
pub mod i2c_prd_start;
#[doc = "i2c_prd_stop (rw) register accessor: an alias for `Reg<I2C_PRD_STOP_SPEC>`"]
pub type I2C_PRD_STOP = crate::Reg<i2c_prd_stop::I2C_PRD_STOP_SPEC>;
#[doc = "i2c_prd_stop."]
pub mod i2c_prd_stop;
#[doc = "i2c_prd_data (rw) register accessor: an alias for `Reg<I2C_PRD_DATA_SPEC>`"]
pub type I2C_PRD_DATA = crate::Reg<i2c_prd_data::I2C_PRD_DATA_SPEC>;
#[doc = "i2c_prd_data."]
pub mod i2c_prd_data;
#[doc = "i2c_fifo_config_0 (rw) register accessor: an alias for `Reg<I2C_FIFO_CONFIG_0_SPEC>`"]
pub type I2C_FIFO_CONFIG_0 = crate::Reg<i2c_fifo_config_0::I2C_FIFO_CONFIG_0_SPEC>;
#[doc = "i2c_fifo_config_0."]
pub mod i2c_fifo_config_0;
#[doc = "i2c_fifo_config_1 (rw) register accessor: an alias for `Reg<I2C_FIFO_CONFIG_1_SPEC>`"]
pub type I2C_FIFO_CONFIG_1 = crate::Reg<i2c_fifo_config_1::I2C_FIFO_CONFIG_1_SPEC>;
#[doc = "i2c_fifo_config_1."]
pub mod i2c_fifo_config_1;
#[doc = "i2c_fifo_wdata (w) register accessor: an alias for `Reg<I2C_FIFO_WDATA_SPEC>`"]
pub type I2C_FIFO_WDATA = crate::Reg<i2c_fifo_wdata::I2C_FIFO_WDATA_SPEC>;
#[doc = "i2c_fifo_wdata."]
pub mod i2c_fifo_wdata;
#[doc = "i2c_fifo_rdata (r) register accessor: an alias for `Reg<I2C_FIFO_RDATA_SPEC>`"]
pub type I2C_FIFO_RDATA = crate::Reg<i2c_fifo_rdata::I2C_FIFO_RDATA_SPEC>;
#[doc = "i2c_fifo_rdata."]
pub mod i2c_fifo_rdata;

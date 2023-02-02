#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - gpadc_config."]
    pub gpadc_config: GPADC_CONFIG,
    #[doc = "0x04 - gpadc_dma_rdata."]
    pub gpadc_dma_rdata: GPADC_DMA_RDATA,
    _reserved2: [u8; 0x38],
    #[doc = "0x40 - gpdac_config."]
    pub gpdac_config: GPDAC_CONFIG,
    #[doc = "0x44 - gpdac_dma_config."]
    pub gpdac_dma_config: GPDAC_DMA_CONFIG,
    #[doc = "0x48 - gpdac_dma_wdata."]
    pub gpdac_dma_wdata: GPDAC_DMA_WDATA,
    #[doc = "0x4c - gpdac_tx_fifo_status."]
    pub gpdac_tx_fifo_status: GPDAC_TX_FIFO_STATUS,
}
#[doc = "gpadc_config (rw) register accessor: an alias for `Reg<GPADC_CONFIG_SPEC>`"]
pub type GPADC_CONFIG = crate::Reg<gpadc_config::GPADC_CONFIG_SPEC>;
#[doc = "gpadc_config."]
pub mod gpadc_config;
#[doc = "gpadc_dma_rdata (r) register accessor: an alias for `Reg<GPADC_DMA_RDATA_SPEC>`"]
pub type GPADC_DMA_RDATA = crate::Reg<gpadc_dma_rdata::GPADC_DMA_RDATA_SPEC>;
#[doc = "gpadc_dma_rdata."]
pub mod gpadc_dma_rdata;
#[doc = "gpdac_config (rw) register accessor: an alias for `Reg<GPDAC_CONFIG_SPEC>`"]
pub type GPDAC_CONFIG = crate::Reg<gpdac_config::GPDAC_CONFIG_SPEC>;
#[doc = "gpdac_config."]
pub mod gpdac_config;
#[doc = "gpdac_dma_config (rw) register accessor: an alias for `Reg<GPDAC_DMA_CONFIG_SPEC>`"]
pub type GPDAC_DMA_CONFIG = crate::Reg<gpdac_dma_config::GPDAC_DMA_CONFIG_SPEC>;
#[doc = "gpdac_dma_config."]
pub mod gpdac_dma_config;
#[doc = "gpdac_dma_wdata (w) register accessor: an alias for `Reg<GPDAC_DMA_WDATA_SPEC>`"]
pub type GPDAC_DMA_WDATA = crate::Reg<gpdac_dma_wdata::GPDAC_DMA_WDATA_SPEC>;
#[doc = "gpdac_dma_wdata."]
pub mod gpdac_dma_wdata;
#[doc = "gpdac_tx_fifo_status (r) register accessor: an alias for `Reg<GPDAC_TX_FIFO_STATUS_SPEC>`"]
pub type GPDAC_TX_FIFO_STATUS = crate::Reg<gpdac_tx_fifo_status::GPDAC_TX_FIFO_STATUS_SPEC>;
#[doc = "gpdac_tx_fifo_status."]
pub mod gpdac_tx_fifo_status;

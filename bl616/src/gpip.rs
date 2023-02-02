#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Generic Analog-to-Digital Converter register"]
    pub gpadc_config: GPADC_CONFIG,
    #[doc = "0x04 - DMA data output of Generic Analog-to-Digital Converter"]
    pub gpadc_dma_read: GPADC_DMA_READ,
    _reserved2: [u8; 0x38],
    #[doc = "0x40 - Generic Digital-to-Analog Converter register"]
    pub gpdac_config: GPDAC_CONFIG,
    #[doc = "0x44 - Digital-to-Analog Converter DMA configuration"]
    pub gpdac_dma_config: GPDAC_DMA_CONFIG,
    #[doc = "0x48 - DMA data input of Generic Digital-to-Analog Converter"]
    pub gpdac_dma_write: GPDAC_DMA_WRITE,
    #[doc = "0x4c - Transmit FIFO state of Generic Digital-to-Analog Converter"]
    pub gpdac_fifo_state: GPDAC_FIFO_STATE,
}
#[doc = "gpadc_config (rw) register accessor: an alias for `Reg<GPADC_CONFIG_SPEC>`"]
pub type GPADC_CONFIG = crate::Reg<gpadc_config::GPADC_CONFIG_SPEC>;
#[doc = "Generic Analog-to-Digital Converter register"]
pub mod gpadc_config;
#[doc = "gpadc_dma_read (rw) register accessor: an alias for `Reg<GPADC_DMA_READ_SPEC>`"]
pub type GPADC_DMA_READ = crate::Reg<gpadc_dma_read::GPADC_DMA_READ_SPEC>;
#[doc = "DMA data output of Generic Analog-to-Digital Converter"]
pub mod gpadc_dma_read;
#[doc = "gpdac_config (rw) register accessor: an alias for `Reg<GPDAC_CONFIG_SPEC>`"]
pub type GPDAC_CONFIG = crate::Reg<gpdac_config::GPDAC_CONFIG_SPEC>;
#[doc = "Generic Digital-to-Analog Converter register"]
pub mod gpdac_config;
#[doc = "gpdac_dma_config (rw) register accessor: an alias for `Reg<GPDAC_DMA_CONFIG_SPEC>`"]
pub type GPDAC_DMA_CONFIG = crate::Reg<gpdac_dma_config::GPDAC_DMA_CONFIG_SPEC>;
#[doc = "Digital-to-Analog Converter DMA configuration"]
pub mod gpdac_dma_config;
#[doc = "gpdac_dma_write (rw) register accessor: an alias for `Reg<GPDAC_DMA_WRITE_SPEC>`"]
pub type GPDAC_DMA_WRITE = crate::Reg<gpdac_dma_write::GPDAC_DMA_WRITE_SPEC>;
#[doc = "DMA data input of Generic Digital-to-Analog Converter"]
pub mod gpdac_dma_write;
#[doc = "gpdac_fifo_state (rw) register accessor: an alias for `Reg<GPDAC_FIFO_STATE_SPEC>`"]
pub type GPDAC_FIFO_STATE = crate::Reg<gpdac_fifo_state::GPDAC_FIFO_STATE_SPEC>;
#[doc = "Transmit FIFO state of Generic Digital-to-Analog Converter"]
pub mod gpdac_fifo_state;

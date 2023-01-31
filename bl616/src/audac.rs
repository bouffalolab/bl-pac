#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub clock: CLOCK,
    #[doc = "0x04 - Peripheral state register"]
    pub state: STATE,
    #[doc = "0x08 - Volume control register 0"]
    pub volume_0: VOLUME_0,
    #[doc = "0x0c - Volume control register 1"]
    pub volume_1: VOLUME_1,
    #[doc = "0x10 - Zero signal detection"]
    pub zero_signal: ZERO_SIGNAL,
    #[doc = "0x14 - Delta-Sigma and mixer control"]
    pub config: CONFIG,
    _reserved6: [u8; 0x74],
    #[doc = "0x8c - Controls audio output FIFO"]
    pub fifo_control: FIFO_CONTROL,
    #[doc = "0x90 - Gets states of audio output FIFO"]
    pub fifo_state: FIFO_STATE,
    #[doc = "0x94 - Writes data into audio output FIFO"]
    pub fifo_data: FIFO_DATA,
}
#[doc = "clock (rw) register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "Clock control register"]
pub mod clock;
#[doc = "state (rw) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Peripheral state register"]
pub mod state;
#[doc = "volume_0 (rw) register accessor: an alias for `Reg<VOLUME_0_SPEC>`"]
pub type VOLUME_0 = crate::Reg<volume_0::VOLUME_0_SPEC>;
#[doc = "Volume control register 0"]
pub mod volume_0;
#[doc = "volume_1 (rw) register accessor: an alias for `Reg<VOLUME_1_SPEC>`"]
pub type VOLUME_1 = crate::Reg<volume_1::VOLUME_1_SPEC>;
#[doc = "Volume control register 1"]
pub mod volume_1;
#[doc = "zero_signal (rw) register accessor: an alias for `Reg<ZERO_SIGNAL_SPEC>`"]
pub type ZERO_SIGNAL = crate::Reg<zero_signal::ZERO_SIGNAL_SPEC>;
#[doc = "Zero signal detection"]
pub mod zero_signal;
#[doc = "config (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Delta-Sigma and mixer control"]
pub mod config;
#[doc = "fifo_control (rw) register accessor: an alias for `Reg<FIFO_CONTROL_SPEC>`"]
pub type FIFO_CONTROL = crate::Reg<fifo_control::FIFO_CONTROL_SPEC>;
#[doc = "Controls audio output FIFO"]
pub mod fifo_control;
#[doc = "fifo_state (rw) register accessor: an alias for `Reg<FIFO_STATE_SPEC>`"]
pub type FIFO_STATE = crate::Reg<fifo_state::FIFO_STATE_SPEC>;
#[doc = "Gets states of audio output FIFO"]
pub mod fifo_state;
#[doc = "fifo_data (rw) register accessor: an alias for `Reg<FIFO_DATA_SPEC>`"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "Writes data into audio output FIFO"]
pub mod fifo_data;

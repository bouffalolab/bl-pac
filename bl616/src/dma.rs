#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ??"]
    pub interrupt_state: INTERRUPT_STATE,
    #[doc = "0x04 - ??"]
    pub terminate_state: TERMINATE_STATE,
    #[doc = "0x08 - ??"]
    pub terminate_clear: TERMINATE_CLEAR,
    #[doc = "0x0c - ??"]
    pub error_state: ERROR_STATE,
    #[doc = "0x10 - ??"]
    pub error_clear: ERROR_CLEAR,
    #[doc = "0x14 - ??"]
    pub terminate_state_raw: TERMINATE_STATE_RAW,
    #[doc = "0x18 - ??"]
    pub error_state_raw: ERROR_STATE_RAW,
    #[doc = "0x1c - ??"]
    pub channel_state: CHANNEL_STATE,
    #[doc = "0x20 - ??"]
    pub burst_request: BURST_REQUEST,
    #[doc = "0x24 - ??"]
    pub single_request: SINGLE_REQUEST,
    #[doc = "0x28 - ??"]
    pub last_burst_request: LAST_BURST_REQUEST,
    #[doc = "0x2c - ??"]
    pub last_single_request: LAST_SINGLE_REQUEST,
    #[doc = "0x30 - ??"]
    pub config: CONFIG,
    #[doc = "0x34 - ??"]
    pub synchronize: SYNCHRONIZE,
    _reserved14: [u8; 0xc8],
    #[doc = "0x100..0x114 - Direct Memory Access channel"]
    pub channel0: CHANNEL,
    _reserved15: [u8; 0xec],
    #[doc = "0x200..0x214 - Direct Memory Access channel"]
    pub channel1: CHANNEL,
    _reserved16: [u8; 0xec],
    #[doc = "0x300..0x314 - Direct Memory Access channel"]
    pub channel2: CHANNEL,
    _reserved17: [u8; 0xec],
    #[doc = "0x400..0x414 - Direct Memory Access channel"]
    pub channel3: CHANNEL,
}
#[doc = "interrupt_state (rw) register accessor: an alias for `Reg<INTERRUPT_STATE_SPEC>`"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "??"]
pub mod interrupt_state;
#[doc = "terminate_state (rw) register accessor: an alias for `Reg<TERMINATE_STATE_SPEC>`"]
pub type TERMINATE_STATE = crate::Reg<terminate_state::TERMINATE_STATE_SPEC>;
#[doc = "??"]
pub mod terminate_state;
#[doc = "terminate_clear (rw) register accessor: an alias for `Reg<TERMINATE_CLEAR_SPEC>`"]
pub type TERMINATE_CLEAR = crate::Reg<terminate_clear::TERMINATE_CLEAR_SPEC>;
#[doc = "??"]
pub mod terminate_clear;
#[doc = "error_state (rw) register accessor: an alias for `Reg<ERROR_STATE_SPEC>`"]
pub type ERROR_STATE = crate::Reg<error_state::ERROR_STATE_SPEC>;
#[doc = "??"]
pub mod error_state;
#[doc = "error_clear (rw) register accessor: an alias for `Reg<ERROR_CLEAR_SPEC>`"]
pub type ERROR_CLEAR = crate::Reg<error_clear::ERROR_CLEAR_SPEC>;
#[doc = "??"]
pub mod error_clear;
#[doc = "terminate_state_raw (rw) register accessor: an alias for `Reg<TERMINATE_STATE_RAW_SPEC>`"]
pub type TERMINATE_STATE_RAW = crate::Reg<terminate_state_raw::TERMINATE_STATE_RAW_SPEC>;
#[doc = "??"]
pub mod terminate_state_raw;
#[doc = "error_state_raw (rw) register accessor: an alias for `Reg<ERROR_STATE_RAW_SPEC>`"]
pub type ERROR_STATE_RAW = crate::Reg<error_state_raw::ERROR_STATE_RAW_SPEC>;
#[doc = "??"]
pub mod error_state_raw;
#[doc = "channel_state (rw) register accessor: an alias for `Reg<CHANNEL_STATE_SPEC>`"]
pub type CHANNEL_STATE = crate::Reg<channel_state::CHANNEL_STATE_SPEC>;
#[doc = "??"]
pub mod channel_state;
#[doc = "burst_request (rw) register accessor: an alias for `Reg<BURST_REQUEST_SPEC>`"]
pub type BURST_REQUEST = crate::Reg<burst_request::BURST_REQUEST_SPEC>;
#[doc = "??"]
pub mod burst_request;
#[doc = "single_request (rw) register accessor: an alias for `Reg<SINGLE_REQUEST_SPEC>`"]
pub type SINGLE_REQUEST = crate::Reg<single_request::SINGLE_REQUEST_SPEC>;
#[doc = "??"]
pub mod single_request;
#[doc = "last_burst_request (rw) register accessor: an alias for `Reg<LAST_BURST_REQUEST_SPEC>`"]
pub type LAST_BURST_REQUEST = crate::Reg<last_burst_request::LAST_BURST_REQUEST_SPEC>;
#[doc = "??"]
pub mod last_burst_request;
#[doc = "last_single_request (rw) register accessor: an alias for `Reg<LAST_SINGLE_REQUEST_SPEC>`"]
pub type LAST_SINGLE_REQUEST = crate::Reg<last_single_request::LAST_SINGLE_REQUEST_SPEC>;
#[doc = "??"]
pub mod last_single_request;
#[doc = "config (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "??"]
pub mod config;
#[doc = "synchronize (rw) register accessor: an alias for `Reg<SYNCHRONIZE_SPEC>`"]
pub type SYNCHRONIZE = crate::Reg<synchronize::SYNCHRONIZE_SPEC>;
#[doc = "??"]
pub mod synchronize;
#[doc = "Direct Memory Access channel"]
pub use self::channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "Direct Memory Access channel"]
pub mod channel;

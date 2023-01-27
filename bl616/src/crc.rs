#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - Data input register"]
    pub input: INPUT,
    #[doc = "0x08 - Checksum output register"]
    pub output: OUTPUT,
}
#[doc = "config (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "input (w) register accessor: an alias for `Reg<INPUT_SPEC>`"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "Data input register"]
pub mod input;
#[doc = "output (r) register accessor: an alias for `Reg<OUTPUT_SPEC>`"]
pub type OUTPUT = crate::Reg<output::OUTPUT_SPEC>;
#[doc = "Checksum output register"]
pub mod output;

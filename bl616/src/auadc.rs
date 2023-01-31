#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral clock control register"]
    pub clock: CLOCK,
    #[doc = "0x04 - Interface control register 0"]
    pub interface_0: INTERFACE_0,
    #[doc = "0x08 - Finite Impulse Response control"]
    pub finite_impulse: FINITE_IMPULSE,
    #[doc = "0x0c - High-Pass Filter control register"]
    pub high_pass: HIGH_PASS,
    #[doc = "0x10 - Interface control register 1"]
    pub interface_1: INTERFACE_1,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - Pulse-Width Modulator control"]
    pub pulse_width: PULSE_WIDTH,
    _reserved6: [u8; 0x18],
    #[doc = "0x38 - Volume control register"]
    pub volume: VOLUME,
    _reserved7: [u8; 0x24],
    #[doc = "0x60 - Analog signal configuration 0"]
    pub analog_0: ANALOG_0,
    #[doc = "0x64 - Analog signal configuration 1"]
    pub analog_1: ANALOG_1,
    #[doc = "0x68 - Analog-to-Digital Converter commands"]
    pub command: COMMAND,
    #[doc = "0x6c - Analog-to-Digital Converter sample output"]
    pub sample_data: SAMPLE_DATA,
    _reserved11: [u8; 0x10],
    #[doc = "0x80 - Controls audio input FIFO"]
    pub fifo_control: FIFO_CONTROL,
    #[doc = "0x84 - Gets states of audio input FIFO"]
    pub fifo_state: FIFO_STATE,
    #[doc = "0x88 - Reads data from audio input FIFO"]
    pub fifo_data: FIFO_DATA,
}
#[doc = "clock (rw) register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "Peripheral clock control register"]
pub mod clock;
#[doc = "interface_0 (rw) register accessor: an alias for `Reg<INTERFACE_0_SPEC>`"]
pub type INTERFACE_0 = crate::Reg<interface_0::INTERFACE_0_SPEC>;
#[doc = "Interface control register 0"]
pub mod interface_0;
#[doc = "finite_impulse (rw) register accessor: an alias for `Reg<FINITE_IMPULSE_SPEC>`"]
pub type FINITE_IMPULSE = crate::Reg<finite_impulse::FINITE_IMPULSE_SPEC>;
#[doc = "Finite Impulse Response control"]
pub mod finite_impulse;
#[doc = "high_pass (rw) register accessor: an alias for `Reg<HIGH_PASS_SPEC>`"]
pub type HIGH_PASS = crate::Reg<high_pass::HIGH_PASS_SPEC>;
#[doc = "High-Pass Filter control register"]
pub mod high_pass;
#[doc = "interface_1 (rw) register accessor: an alias for `Reg<INTERFACE_1_SPEC>`"]
pub type INTERFACE_1 = crate::Reg<interface_1::INTERFACE_1_SPEC>;
#[doc = "Interface control register 1"]
pub mod interface_1;
#[doc = "pulse_width (rw) register accessor: an alias for `Reg<PULSE_WIDTH_SPEC>`"]
pub type PULSE_WIDTH = crate::Reg<pulse_width::PULSE_WIDTH_SPEC>;
#[doc = "Pulse-Width Modulator control"]
pub mod pulse_width;
#[doc = "volume (rw) register accessor: an alias for `Reg<VOLUME_SPEC>`"]
pub type VOLUME = crate::Reg<volume::VOLUME_SPEC>;
#[doc = "Volume control register"]
pub mod volume;
#[doc = "analog_0 (rw) register accessor: an alias for `Reg<ANALOG_0_SPEC>`"]
pub type ANALOG_0 = crate::Reg<analog_0::ANALOG_0_SPEC>;
#[doc = "Analog signal configuration 0"]
pub mod analog_0;
#[doc = "analog_1 (rw) register accessor: an alias for `Reg<ANALOG_1_SPEC>`"]
pub type ANALOG_1 = crate::Reg<analog_1::ANALOG_1_SPEC>;
#[doc = "Analog signal configuration 1"]
pub mod analog_1;
#[doc = "command (rw) register accessor: an alias for `Reg<COMMAND_SPEC>`"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Analog-to-Digital Converter commands"]
pub mod command;
#[doc = "sample_data (rw) register accessor: an alias for `Reg<SAMPLE_DATA_SPEC>`"]
pub type SAMPLE_DATA = crate::Reg<sample_data::SAMPLE_DATA_SPEC>;
#[doc = "Analog-to-Digital Converter sample output"]
pub mod sample_data;
#[doc = "fifo_control (rw) register accessor: an alias for `Reg<FIFO_CONTROL_SPEC>`"]
pub type FIFO_CONTROL = crate::Reg<fifo_control::FIFO_CONTROL_SPEC>;
#[doc = "Controls audio input FIFO"]
pub mod fifo_control;
#[doc = "fifo_state (rw) register accessor: an alias for `Reg<FIFO_STATE_SPEC>`"]
pub type FIFO_STATE = crate::Reg<fifo_state::FIFO_STATE_SPEC>;
#[doc = "Gets states of audio input FIFO"]
pub mod fifo_state;
#[doc = "fifo_data (rw) register accessor: an alias for `Reg<FIFO_DATA_SPEC>`"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "Reads data from audio input FIFO"]
pub mod fifo_data;

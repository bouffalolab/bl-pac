#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08c4],
    #[doc = "0x8c4..0x950 - Generic Purpose Input/Output config"]
    pub gpio_config: [GPIO_CONFIG; 35],
    _reserved1: [u8; 0x0174],
    #[doc = "0xac4..0xacc - Read value from Generic Purpose Input/Output pins"]
    pub gpio_input: [GPIO_INPUT; 2],
    _reserved2: [u8; 0x18],
    #[doc = "0xae4..0xaec - Write value to Generic Purpose Input/Output pins"]
    pub gpio_output: [GPIO_OUTPUT; 2],
    #[doc = "0xaec..0xaf4 - Set pin output value to high"]
    pub gpio_set: [GPIO_SET; 2],
    #[doc = "0xaf4..0xafc - Set pin output value to low"]
    pub gpio_clear: [GPIO_CLEAR; 2],
}
#[doc = "gpio_config (rw) register accessor: an alias for `Reg<GPIO_CONFIG_SPEC>`"]
pub type GPIO_CONFIG = crate::Reg<gpio_config::GPIO_CONFIG_SPEC>;
#[doc = "Generic Purpose Input/Output config"]
pub mod gpio_config;
#[doc = "gpio_input (rw) register accessor: an alias for `Reg<GPIO_INPUT_SPEC>`"]
pub type GPIO_INPUT = crate::Reg<gpio_input::GPIO_INPUT_SPEC>;
#[doc = "Read value from Generic Purpose Input/Output pins"]
pub mod gpio_input;
#[doc = "gpio_output (rw) register accessor: an alias for `Reg<GPIO_OUTPUT_SPEC>`"]
pub type GPIO_OUTPUT = crate::Reg<gpio_output::GPIO_OUTPUT_SPEC>;
#[doc = "Write value to Generic Purpose Input/Output pins"]
pub mod gpio_output;
#[doc = "gpio_set (rw) register accessor: an alias for `Reg<GPIO_SET_SPEC>`"]
pub type GPIO_SET = crate::Reg<gpio_set::GPIO_SET_SPEC>;
#[doc = "Set pin output value to high"]
pub mod gpio_set;
#[doc = "gpio_clear (rw) register accessor: an alias for `Reg<GPIO_CLEAR_SPEC>`"]
pub type GPIO_CLEAR = crate::Reg<gpio_clear::GPIO_CLEAR_SPEC>;
#[doc = "Set pin output value to low"]
pub mod gpio_clear;

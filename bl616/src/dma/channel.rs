#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Source address"]
    pub source: SOURCE,
    #[doc = "0x04 - Destination address"]
    pub destination: DESTINATION,
    #[doc = "0x08 - Linked list buffer base address"]
    pub linked_list: LINKED_LIST,
    #[doc = "0x0c - Control register"]
    pub control: CONTROL,
    #[doc = "0x10 - Configuration register"]
    pub config: CONFIG,
}
#[doc = "source (rw) register accessor: an alias for `Reg<SOURCE_SPEC>`"]
pub type SOURCE = crate::Reg<source::SOURCE_SPEC>;
#[doc = "Source address"]
pub mod source;
#[doc = "destination (rw) register accessor: an alias for `Reg<DESTINATION_SPEC>`"]
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
#[doc = "Destination address"]
pub mod destination;
#[doc = "linked_list (rw) register accessor: an alias for `Reg<LINKED_LIST_SPEC>`"]
pub type LINKED_LIST = crate::Reg<linked_list::LINKED_LIST_SPEC>;
#[doc = "Linked list buffer base address"]
pub mod linked_list;
#[doc = "control (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control register"]
pub mod control;
#[doc = "config (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;

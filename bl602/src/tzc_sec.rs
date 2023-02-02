#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - tzc_rom_ctrl."]
    pub tzc_rom_ctrl: TZC_ROM_CTRL,
    #[doc = "0x44 - tzc_rom0_r0."]
    pub tzc_rom0_r0: TZC_ROM0_R0,
    #[doc = "0x48 - tzc_rom0_r1."]
    pub tzc_rom0_r1: TZC_ROM0_R1,
    #[doc = "0x4c - tzc_rom1_r0."]
    pub tzc_rom1_r0: TZC_ROM1_R0,
    #[doc = "0x50 - tzc_rom1_r1."]
    pub tzc_rom1_r1: TZC_ROM1_R1,
}
#[doc = "tzc_rom_ctrl (rw) register accessor: an alias for `Reg<TZC_ROM_CTRL_SPEC>`"]
pub type TZC_ROM_CTRL = crate::Reg<tzc_rom_ctrl::TZC_ROM_CTRL_SPEC>;
#[doc = "tzc_rom_ctrl."]
pub mod tzc_rom_ctrl;
#[doc = "tzc_rom0_r0 (rw) register accessor: an alias for `Reg<TZC_ROM0_R0_SPEC>`"]
pub type TZC_ROM0_R0 = crate::Reg<tzc_rom0_r0::TZC_ROM0_R0_SPEC>;
#[doc = "tzc_rom0_r0."]
pub mod tzc_rom0_r0;
#[doc = "tzc_rom0_r1 (rw) register accessor: an alias for `Reg<TZC_ROM0_R1_SPEC>`"]
pub type TZC_ROM0_R1 = crate::Reg<tzc_rom0_r1::TZC_ROM0_R1_SPEC>;
#[doc = "tzc_rom0_r1."]
pub mod tzc_rom0_r1;
#[doc = "tzc_rom1_r0 (rw) register accessor: an alias for `Reg<TZC_ROM1_R0_SPEC>`"]
pub type TZC_ROM1_R0 = crate::Reg<tzc_rom1_r0::TZC_ROM1_R0_SPEC>;
#[doc = "tzc_rom1_r0."]
pub mod tzc_rom1_r0;
#[doc = "tzc_rom1_r1 (rw) register accessor: an alias for `Reg<TZC_ROM1_R1_SPEC>`"]
pub type TZC_ROM1_R1 = crate::Reg<tzc_rom1_r1::TZC_ROM1_R1_SPEC>;
#[doc = "tzc_rom1_r1."]
pub mod tzc_rom1_r1;

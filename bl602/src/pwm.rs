#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - pwm_int_config."]
    pub pwm_int_config: PWM_INT_CONFIG,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - pwm0_clkdiv."]
    pub pwm0_clkdiv: PWM0_CLKDIV,
    #[doc = "0x24 - pwm0_thre1."]
    pub pwm0_thre1: PWM0_THRE1,
    #[doc = "0x28 - pwm0_thre2."]
    pub pwm0_thre2: PWM0_THRE2,
    #[doc = "0x2c - pwm0_period."]
    pub pwm0_period: PWM0_PERIOD,
    #[doc = "0x30 - pwm0_config."]
    pub pwm0_config: PWM0_CONFIG,
    #[doc = "0x34 - pwm0_interrupt."]
    pub pwm0_interrupt: PWM0_INTERRUPT,
    _reserved7: [u8; 0x08],
    #[doc = "0x40 - pwm1_clkdiv."]
    pub pwm1_clkdiv: PWM1_CLKDIV,
    #[doc = "0x44 - pwm1_thre1."]
    pub pwm1_thre1: PWM1_THRE1,
    #[doc = "0x48 - pwm1_thre2."]
    pub pwm1_thre2: PWM1_THRE2,
    #[doc = "0x4c - pwm1_period."]
    pub pwm1_period: PWM1_PERIOD,
    #[doc = "0x50 - pwm1_config."]
    pub pwm1_config: PWM1_CONFIG,
    #[doc = "0x54 - pwm1_interrupt."]
    pub pwm1_interrupt: PWM1_INTERRUPT,
    _reserved13: [u8; 0x08],
    #[doc = "0x60 - pwm2_clkdiv."]
    pub pwm2_clkdiv: PWM2_CLKDIV,
    #[doc = "0x64 - pwm2_thre1."]
    pub pwm2_thre1: PWM2_THRE1,
    #[doc = "0x68 - pwm2_thre2."]
    pub pwm2_thre2: PWM2_THRE2,
    #[doc = "0x6c - pwm2_period."]
    pub pwm2_period: PWM2_PERIOD,
    #[doc = "0x70 - pwm2_config."]
    pub pwm2_config: PWM2_CONFIG,
    #[doc = "0x74 - pwm2_interrupt."]
    pub pwm2_interrupt: PWM2_INTERRUPT,
    _reserved19: [u8; 0x08],
    #[doc = "0x80 - pwm3_clkdiv."]
    pub pwm3_clkdiv: PWM3_CLKDIV,
    #[doc = "0x84 - pwm3_thre1."]
    pub pwm3_thre1: PWM3_THRE1,
    #[doc = "0x88 - pwm3_thre2."]
    pub pwm3_thre2: PWM3_THRE2,
    #[doc = "0x8c - pwm3_period."]
    pub pwm3_period: PWM3_PERIOD,
    #[doc = "0x90 - pwm3_config."]
    pub pwm3_config: PWM3_CONFIG,
    #[doc = "0x94 - pwm3_interrupt."]
    pub pwm3_interrupt: PWM3_INTERRUPT,
    _reserved25: [u8; 0x08],
    #[doc = "0xa0 - pwm4_clkdiv."]
    pub pwm4_clkdiv: PWM4_CLKDIV,
    #[doc = "0xa4 - pwm4_thre1."]
    pub pwm4_thre1: PWM4_THRE1,
    #[doc = "0xa8 - pwm4_thre2."]
    pub pwm4_thre2: PWM4_THRE2,
    #[doc = "0xac - pwm4_period."]
    pub pwm4_period: PWM4_PERIOD,
    #[doc = "0xb0 - pwm4_config."]
    pub pwm4_config: PWM4_CONFIG,
    #[doc = "0xb4 - pwm4_interrupt."]
    pub pwm4_interrupt: PWM4_INTERRUPT,
}
#[doc = "pwm_int_config (rw) register accessor: an alias for `Reg<PWM_INT_CONFIG_SPEC>`"]
pub type PWM_INT_CONFIG = crate::Reg<pwm_int_config::PWM_INT_CONFIG_SPEC>;
#[doc = "pwm_int_config."]
pub mod pwm_int_config;
#[doc = "pwm0_clkdiv (rw) register accessor: an alias for `Reg<PWM0_CLKDIV_SPEC>`"]
pub type PWM0_CLKDIV = crate::Reg<pwm0_clkdiv::PWM0_CLKDIV_SPEC>;
#[doc = "pwm0_clkdiv."]
pub mod pwm0_clkdiv;
#[doc = "pwm0_thre1 (rw) register accessor: an alias for `Reg<PWM0_THRE1_SPEC>`"]
pub type PWM0_THRE1 = crate::Reg<pwm0_thre1::PWM0_THRE1_SPEC>;
#[doc = "pwm0_thre1."]
pub mod pwm0_thre1;
#[doc = "pwm0_thre2 (rw) register accessor: an alias for `Reg<PWM0_THRE2_SPEC>`"]
pub type PWM0_THRE2 = crate::Reg<pwm0_thre2::PWM0_THRE2_SPEC>;
#[doc = "pwm0_thre2."]
pub mod pwm0_thre2;
#[doc = "pwm0_period (rw) register accessor: an alias for `Reg<PWM0_PERIOD_SPEC>`"]
pub type PWM0_PERIOD = crate::Reg<pwm0_period::PWM0_PERIOD_SPEC>;
#[doc = "pwm0_period."]
pub mod pwm0_period;
#[doc = "pwm0_config (rw) register accessor: an alias for `Reg<PWM0_CONFIG_SPEC>`"]
pub type PWM0_CONFIG = crate::Reg<pwm0_config::PWM0_CONFIG_SPEC>;
#[doc = "pwm0_config."]
pub mod pwm0_config;
#[doc = "pwm0_interrupt (rw) register accessor: an alias for `Reg<PWM0_INTERRUPT_SPEC>`"]
pub type PWM0_INTERRUPT = crate::Reg<pwm0_interrupt::PWM0_INTERRUPT_SPEC>;
#[doc = "pwm0_interrupt."]
pub mod pwm0_interrupt;
#[doc = "pwm1_clkdiv (rw) register accessor: an alias for `Reg<PWM1_CLKDIV_SPEC>`"]
pub type PWM1_CLKDIV = crate::Reg<pwm1_clkdiv::PWM1_CLKDIV_SPEC>;
#[doc = "pwm1_clkdiv."]
pub mod pwm1_clkdiv;
#[doc = "pwm1_thre1 (rw) register accessor: an alias for `Reg<PWM1_THRE1_SPEC>`"]
pub type PWM1_THRE1 = crate::Reg<pwm1_thre1::PWM1_THRE1_SPEC>;
#[doc = "pwm1_thre1."]
pub mod pwm1_thre1;
#[doc = "pwm1_thre2 (rw) register accessor: an alias for `Reg<PWM1_THRE2_SPEC>`"]
pub type PWM1_THRE2 = crate::Reg<pwm1_thre2::PWM1_THRE2_SPEC>;
#[doc = "pwm1_thre2."]
pub mod pwm1_thre2;
#[doc = "pwm1_period (rw) register accessor: an alias for `Reg<PWM1_PERIOD_SPEC>`"]
pub type PWM1_PERIOD = crate::Reg<pwm1_period::PWM1_PERIOD_SPEC>;
#[doc = "pwm1_period."]
pub mod pwm1_period;
#[doc = "pwm1_config (rw) register accessor: an alias for `Reg<PWM1_CONFIG_SPEC>`"]
pub type PWM1_CONFIG = crate::Reg<pwm1_config::PWM1_CONFIG_SPEC>;
#[doc = "pwm1_config."]
pub mod pwm1_config;
#[doc = "pwm1_interrupt (rw) register accessor: an alias for `Reg<PWM1_INTERRUPT_SPEC>`"]
pub type PWM1_INTERRUPT = crate::Reg<pwm1_interrupt::PWM1_INTERRUPT_SPEC>;
#[doc = "pwm1_interrupt."]
pub mod pwm1_interrupt;
#[doc = "pwm2_clkdiv (rw) register accessor: an alias for `Reg<PWM2_CLKDIV_SPEC>`"]
pub type PWM2_CLKDIV = crate::Reg<pwm2_clkdiv::PWM2_CLKDIV_SPEC>;
#[doc = "pwm2_clkdiv."]
pub mod pwm2_clkdiv;
#[doc = "pwm2_thre1 (rw) register accessor: an alias for `Reg<PWM2_THRE1_SPEC>`"]
pub type PWM2_THRE1 = crate::Reg<pwm2_thre1::PWM2_THRE1_SPEC>;
#[doc = "pwm2_thre1."]
pub mod pwm2_thre1;
#[doc = "pwm2_thre2 (rw) register accessor: an alias for `Reg<PWM2_THRE2_SPEC>`"]
pub type PWM2_THRE2 = crate::Reg<pwm2_thre2::PWM2_THRE2_SPEC>;
#[doc = "pwm2_thre2."]
pub mod pwm2_thre2;
#[doc = "pwm2_period (rw) register accessor: an alias for `Reg<PWM2_PERIOD_SPEC>`"]
pub type PWM2_PERIOD = crate::Reg<pwm2_period::PWM2_PERIOD_SPEC>;
#[doc = "pwm2_period."]
pub mod pwm2_period;
#[doc = "pwm2_config (rw) register accessor: an alias for `Reg<PWM2_CONFIG_SPEC>`"]
pub type PWM2_CONFIG = crate::Reg<pwm2_config::PWM2_CONFIG_SPEC>;
#[doc = "pwm2_config."]
pub mod pwm2_config;
#[doc = "pwm2_interrupt (rw) register accessor: an alias for `Reg<PWM2_INTERRUPT_SPEC>`"]
pub type PWM2_INTERRUPT = crate::Reg<pwm2_interrupt::PWM2_INTERRUPT_SPEC>;
#[doc = "pwm2_interrupt."]
pub mod pwm2_interrupt;
#[doc = "pwm3_clkdiv (rw) register accessor: an alias for `Reg<PWM3_CLKDIV_SPEC>`"]
pub type PWM3_CLKDIV = crate::Reg<pwm3_clkdiv::PWM3_CLKDIV_SPEC>;
#[doc = "pwm3_clkdiv."]
pub mod pwm3_clkdiv;
#[doc = "pwm3_thre1 (rw) register accessor: an alias for `Reg<PWM3_THRE1_SPEC>`"]
pub type PWM3_THRE1 = crate::Reg<pwm3_thre1::PWM3_THRE1_SPEC>;
#[doc = "pwm3_thre1."]
pub mod pwm3_thre1;
#[doc = "pwm3_thre2 (rw) register accessor: an alias for `Reg<PWM3_THRE2_SPEC>`"]
pub type PWM3_THRE2 = crate::Reg<pwm3_thre2::PWM3_THRE2_SPEC>;
#[doc = "pwm3_thre2."]
pub mod pwm3_thre2;
#[doc = "pwm3_period (rw) register accessor: an alias for `Reg<PWM3_PERIOD_SPEC>`"]
pub type PWM3_PERIOD = crate::Reg<pwm3_period::PWM3_PERIOD_SPEC>;
#[doc = "pwm3_period."]
pub mod pwm3_period;
#[doc = "pwm3_config (rw) register accessor: an alias for `Reg<PWM3_CONFIG_SPEC>`"]
pub type PWM3_CONFIG = crate::Reg<pwm3_config::PWM3_CONFIG_SPEC>;
#[doc = "pwm3_config."]
pub mod pwm3_config;
#[doc = "pwm3_interrupt (rw) register accessor: an alias for `Reg<PWM3_INTERRUPT_SPEC>`"]
pub type PWM3_INTERRUPT = crate::Reg<pwm3_interrupt::PWM3_INTERRUPT_SPEC>;
#[doc = "pwm3_interrupt."]
pub mod pwm3_interrupt;
#[doc = "pwm4_clkdiv (rw) register accessor: an alias for `Reg<PWM4_CLKDIV_SPEC>`"]
pub type PWM4_CLKDIV = crate::Reg<pwm4_clkdiv::PWM4_CLKDIV_SPEC>;
#[doc = "pwm4_clkdiv."]
pub mod pwm4_clkdiv;
#[doc = "pwm4_thre1 (rw) register accessor: an alias for `Reg<PWM4_THRE1_SPEC>`"]
pub type PWM4_THRE1 = crate::Reg<pwm4_thre1::PWM4_THRE1_SPEC>;
#[doc = "pwm4_thre1."]
pub mod pwm4_thre1;
#[doc = "pwm4_thre2 (rw) register accessor: an alias for `Reg<PWM4_THRE2_SPEC>`"]
pub type PWM4_THRE2 = crate::Reg<pwm4_thre2::PWM4_THRE2_SPEC>;
#[doc = "pwm4_thre2."]
pub mod pwm4_thre2;
#[doc = "pwm4_period (rw) register accessor: an alias for `Reg<PWM4_PERIOD_SPEC>`"]
pub type PWM4_PERIOD = crate::Reg<pwm4_period::PWM4_PERIOD_SPEC>;
#[doc = "pwm4_period."]
pub mod pwm4_period;
#[doc = "pwm4_config (rw) register accessor: an alias for `Reg<PWM4_CONFIG_SPEC>`"]
pub type PWM4_CONFIG = crate::Reg<pwm4_config::PWM4_CONFIG_SPEC>;
#[doc = "pwm4_config."]
pub mod pwm4_config;
#[doc = "pwm4_interrupt (rw) register accessor: an alias for `Reg<PWM4_INTERRUPT_SPEC>`"]
pub type PWM4_INTERRUPT = crate::Reg<pwm4_interrupt::PWM4_INTERRUPT_SPEC>;
#[doc = "pwm4_interrupt."]
pub mod pwm4_interrupt;

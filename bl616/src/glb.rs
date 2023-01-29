#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip information register"]
    pub chip_inform: CHIP_INFORM,
    _reserved1: [u8; 0x4c],
    #[doc = "0x50..0x58 - Chip interrupt state register"]
    pub interrupt_state: [INTERRUPT_STATE; 2],
    #[doc = "0x58..0x60 - Chip interrupt mask register"]
    pub interrupt_mask: [INTERRUPT_MASK; 2],
    #[doc = "0x60..0x68 - Chip clear interrupt register"]
    pub interrupt_clear: [INTERRUPT_CLEAR; 2],
    _reserved4: [u8; 0x28],
    #[doc = "0x90 - System clock configuration register 0"]
    pub clock_config_0: CLOCK_CONFIG_0,
    #[doc = "0x94 - System clock configuration register 1"]
    pub clock_config_1: CLOCK_CONFIG_1,
    _reserved6: [u8; 0x08],
    #[doc = "0xa0 - Bus configuration register 0"]
    pub bus_config_0: BUS_CONFIG_0,
    _reserved7: [u8; 0x6c],
    #[doc = "0x110 - General Purpose Analog-to-digital convert configuration"]
    pub gpadc_config: GPADC_CONFIG,
    _reserved8: [u8; 0x0c],
    #[doc = "0x120 - General Purpose Digital-to-analog convert configuration 0"]
    pub gpdac_config_0: GPDAC_CONFIG_0,
    #[doc = "0x124 - General Purpose Digital-to-analog convert configuration 1"]
    pub gpdac_config_1: GPDAC_CONFIG_1,
    #[doc = "0x128 - General Purpose Digital-to-analog convert configuration 2"]
    pub gpdac_config_2: GPDAC_CONFIG_2,
    #[doc = "0x12c - General Purpose Digital-to-analog convert configuration 3"]
    pub gpdac_config_3: GPDAC_CONFIG_3,
    #[doc = "0x130 - Direct Memory Access configuration 0"]
    pub dma_config_0: DMA_CONFIG_0,
    #[doc = "0x134 - Direct Memory Access configuration 1"]
    pub dma_config_1: DMA_CONFIG_1,
    #[doc = "0x138 - Direct Memory Access configuration 2"]
    pub dma_config_2: DMA_CONFIG_2,
    _reserved15: [u8; 0x04],
    #[doc = "0x140 - Infrared configuration register 0"]
    pub ir_config_0: IR_CONFIG_0,
    #[doc = "0x144 - Infrared configuration register 1"]
    pub ir_config_1: IR_CONFIG_1,
    _reserved17: [u8; 0x08],
    #[doc = "0x150 - Universal Asynchronous Receiver/Transmitter configuration"]
    pub uart_config: UART_CONFIG,
    #[doc = "0x154 - Universal Asynchronous Receiver/Transmitter signal configuration 0"]
    pub uart_signal_0: UART_SIGNAL_0,
    #[doc = "0x158 - Universal Asynchronous Receiver/Transmitter signal configuration 1"]
    pub uart_signal_1: UART_SIGNAL_1,
    _reserved20: [u8; 0x14],
    #[doc = "0x170 - Serial flash configuration"]
    pub flash_config: FLASH_CONFIG,
    _reserved21: [u8; 0x0c],
    #[doc = "0x180 - Inter-Integrated Circuit bus configuration"]
    pub i2c_config: I2C_CONFIG,
    _reserved22: [u8; 0x0c],
    #[doc = "0x190 - Inter-IC Sound configuration"]
    pub i2s_config: I2S_CONFIG,
    _reserved23: [u8; 0x1c],
    #[doc = "0x1b0 - Serial Peripheral Interface configuration"]
    pub spi_config: SPI_CONFIG,
    _reserved24: [u8; 0x2c],
    #[doc = "0x1e0 - Pulse-Width configuration"]
    pub pwm_config: PWM_CONFIG,
    _reserved25: [u8; 0x0c],
    #[doc = "0x1f0 - MIPI Display Bus Interface clock configuration"]
    pub dbi_config: DBI_CONFIG,
    _reserved26: [u8; 0x5c],
    #[doc = "0x250 - Digital clock configuration 0"]
    pub digit_clock_0: DIGIT_CLOCK_0,
    #[doc = "0x254 - Digital clock configuration 1"]
    pub digit_clock_1: DIGIT_CLOCK_1,
    #[doc = "0x258 - Digital clock configuration 2"]
    pub digit_clock_2: DIGIT_CLOCK_2,
    _reserved29: [u8; 0x04],
    #[doc = "0x260 - Radio frequency configuration register"]
    pub radio_config: RADIO_CONFIG,
    _reserved30: [u8; 0x7c],
    #[doc = "0x2e0 - Debug configuration register 0"]
    pub debug_config_0: DEBUG_CONFIG_0,
    #[doc = "0x2e4 - Debug configuration register 1"]
    pub debug_config_1: DEBUG_CONFIG_1,
    #[doc = "0x2e8 - Debug configuration register 2"]
    pub debug_config_2: DEBUG_CONFIG_2,
    #[doc = "0x2ec - Debug configuration register 3"]
    pub debug_config_3: DEBUG_CONFIG_3,
    #[doc = "0x2f0 - Debug configuration register 4"]
    pub debug_config_4: DEBUG_CONFIG_4,
    _reserved35: [u8; 0x0c],
    #[doc = "0x300 - Machine Built-in Self Test register 0"]
    pub self_test_0: SELF_TEST_0,
    #[doc = "0x304 - Machine Built-in Self Test register 1"]
    pub self_test_1: SELF_TEST_1,
    _reserved37: [u8; 0x38],
    #[doc = "0x340 - Audio configuration register 0"]
    pub audio_config_0: AUDIO_CONFIG_0,
    #[doc = "0x344 - Audio configuration register 1"]
    pub audio_config_1: AUDIO_CONFIG_1,
    _reserved39: [u8; 0x04c8],
    #[doc = "0x810 - Wireless Fidelity Phase-Locked Loop configuration 0"]
    pub wifi_pll_config_0: WIFI_PLL_CONFIG_0,
    #[doc = "0x814 - Wireless Fidelity Phase-Locked Loop configuration 1"]
    pub wifi_pll_config_1: WIFI_PLL_CONFIG_1,
    #[doc = "0x818 - Wireless Fidelity Phase-Locked Loop configuration 2"]
    pub wifi_pll_config_2: WIFI_PLL_CONFIG_2,
    #[doc = "0x81c - Wireless Fidelity Phase-Locked Loop configuration 3"]
    pub wifi_pll_config_3: WIFI_PLL_CONFIG_3,
    #[doc = "0x820 - Wireless Fidelity Phase-Locked Loop configuration 4"]
    pub wifi_pll_config_4: WIFI_PLL_CONFIG_4,
    #[doc = "0x824 - Wireless Fidelity Phase-Locked Loop configuration 5"]
    pub wifi_pll_config_5: WIFI_PLL_CONFIG_5,
    #[doc = "0x828 - Wireless Fidelity Phase-Locked Loop configuration 6"]
    pub wifi_pll_config_6: WIFI_PLL_CONFIG_6,
    #[doc = "0x82c - Wireless Fidelity Phase-Locked Loop configuration 7"]
    pub wifi_pll_config_7: WIFI_PLL_CONFIG_7,
    #[doc = "0x830 - Wireless Fidelity Phase-Locked Loop configuration 8"]
    pub wifi_pll_config_8: WIFI_PLL_CONFIG_8,
    #[doc = "0x834 - Wireless Fidelity Phase-Locked Loop configuration 9"]
    pub wifi_pll_config_9: WIFI_PLL_CONFIG_9,
    #[doc = "0x838 - Wireless Fidelity Phase-Locked Loop configuration 10"]
    pub wifi_pll_config_10: WIFI_PLL_CONFIG_10,
    #[doc = "0x83c - Wireless Fidelity Phase-Locked Loop configuration 11"]
    pub wifi_pll_config_11: WIFI_PLL_CONFIG_11,
    #[doc = "0x840 - Wireless Fidelity Phase-Locked Loop configuration 12"]
    pub wifi_pll_config_12: WIFI_PLL_CONFIG_12,
    #[doc = "0x844 - Wireless Fidelity Phase-Locked Loop configuration 13"]
    pub wifi_pll_config_13: WIFI_PLL_CONFIG_13,
    #[doc = "0x848 - Wireless Fidelity Phase-Locked Loop configuration 14"]
    pub wifi_pll_config_14: WIFI_PLL_CONFIG_14,
    _reserved54: [u8; 0x38],
    #[doc = "0x884 - 1.8-V Low Dropout Linear Regulator configuration"]
    pub ldo18: LDO18,
    _reserved55: [u8; 0x3c],
    #[doc = "0x8c4..0x950 - Generic Purpose Input/Output config"]
    pub gpio_config: [GPIO_CONFIG; 35],
    _reserved56: [u8; 0x0174],
    #[doc = "0xac4..0xacc - Read value from Generic Purpose Input/Output pins"]
    pub gpio_input: [GPIO_INPUT; 2],
    _reserved57: [u8; 0x18],
    #[doc = "0xae4..0xaec - Write value to Generic Purpose Input/Output pins"]
    pub gpio_output: [GPIO_OUTPUT; 2],
    #[doc = "0xaec..0xaf4 - Set pin output value to high"]
    pub gpio_set: [GPIO_SET; 2],
    #[doc = "0xaf4..0xafc - Set pin output value to low"]
    pub gpio_clear: [GPIO_CLEAR; 2],
}
#[doc = "chip_inform (rw) register accessor: an alias for `Reg<CHIP_INFORM_SPEC>`"]
pub type CHIP_INFORM = crate::Reg<chip_inform::CHIP_INFORM_SPEC>;
#[doc = "Chip information register"]
pub mod chip_inform;
#[doc = "interrupt_state (rw) register accessor: an alias for `Reg<INTERRUPT_STATE_SPEC>`"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Chip interrupt state register"]
pub mod interrupt_state;
#[doc = "interrupt_mask (rw) register accessor: an alias for `Reg<INTERRUPT_MASK_SPEC>`"]
pub type INTERRUPT_MASK = crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>;
#[doc = "Chip interrupt mask register"]
pub mod interrupt_mask;
#[doc = "interrupt_clear (rw) register accessor: an alias for `Reg<INTERRUPT_CLEAR_SPEC>`"]
pub type INTERRUPT_CLEAR = crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>;
#[doc = "Chip clear interrupt register"]
pub mod interrupt_clear;
#[doc = "clock_config_0 (rw) register accessor: an alias for `Reg<CLOCK_CONFIG_0_SPEC>`"]
pub type CLOCK_CONFIG_0 = crate::Reg<clock_config_0::CLOCK_CONFIG_0_SPEC>;
#[doc = "System clock configuration register 0"]
pub mod clock_config_0;
#[doc = "clock_config_1 (rw) register accessor: an alias for `Reg<CLOCK_CONFIG_1_SPEC>`"]
pub type CLOCK_CONFIG_1 = crate::Reg<clock_config_1::CLOCK_CONFIG_1_SPEC>;
#[doc = "System clock configuration register 1"]
pub mod clock_config_1;
#[doc = "bus_config_0 (rw) register accessor: an alias for `Reg<BUS_CONFIG_0_SPEC>`"]
pub type BUS_CONFIG_0 = crate::Reg<bus_config_0::BUS_CONFIG_0_SPEC>;
#[doc = "Bus configuration register 0"]
pub mod bus_config_0;
#[doc = "gpadc_config (rw) register accessor: an alias for `Reg<GPADC_CONFIG_SPEC>`"]
pub type GPADC_CONFIG = crate::Reg<gpadc_config::GPADC_CONFIG_SPEC>;
#[doc = "General Purpose Analog-to-digital convert configuration"]
pub mod gpadc_config;
#[doc = "gpdac_config_0 (rw) register accessor: an alias for `Reg<GPDAC_CONFIG_0_SPEC>`"]
pub type GPDAC_CONFIG_0 = crate::Reg<gpdac_config_0::GPDAC_CONFIG_0_SPEC>;
#[doc = "General Purpose Digital-to-analog convert configuration 0"]
pub mod gpdac_config_0;
#[doc = "gpdac_config_1 (rw) register accessor: an alias for `Reg<GPDAC_CONFIG_1_SPEC>`"]
pub type GPDAC_CONFIG_1 = crate::Reg<gpdac_config_1::GPDAC_CONFIG_1_SPEC>;
#[doc = "General Purpose Digital-to-analog convert configuration 1"]
pub mod gpdac_config_1;
#[doc = "gpdac_config_2 (rw) register accessor: an alias for `Reg<GPDAC_CONFIG_2_SPEC>`"]
pub type GPDAC_CONFIG_2 = crate::Reg<gpdac_config_2::GPDAC_CONFIG_2_SPEC>;
#[doc = "General Purpose Digital-to-analog convert configuration 2"]
pub mod gpdac_config_2;
#[doc = "gpdac_config_3 (rw) register accessor: an alias for `Reg<GPDAC_CONFIG_3_SPEC>`"]
pub type GPDAC_CONFIG_3 = crate::Reg<gpdac_config_3::GPDAC_CONFIG_3_SPEC>;
#[doc = "General Purpose Digital-to-analog convert configuration 3"]
pub mod gpdac_config_3;
#[doc = "dma_config_0 (rw) register accessor: an alias for `Reg<DMA_CONFIG_0_SPEC>`"]
pub type DMA_CONFIG_0 = crate::Reg<dma_config_0::DMA_CONFIG_0_SPEC>;
#[doc = "Direct Memory Access configuration 0"]
pub mod dma_config_0;
#[doc = "dma_config_1 (rw) register accessor: an alias for `Reg<DMA_CONFIG_1_SPEC>`"]
pub type DMA_CONFIG_1 = crate::Reg<dma_config_1::DMA_CONFIG_1_SPEC>;
#[doc = "Direct Memory Access configuration 1"]
pub mod dma_config_1;
#[doc = "dma_config_2 (rw) register accessor: an alias for `Reg<DMA_CONFIG_2_SPEC>`"]
pub type DMA_CONFIG_2 = crate::Reg<dma_config_2::DMA_CONFIG_2_SPEC>;
#[doc = "Direct Memory Access configuration 2"]
pub mod dma_config_2;
#[doc = "ir_config_0 (rw) register accessor: an alias for `Reg<IR_CONFIG_0_SPEC>`"]
pub type IR_CONFIG_0 = crate::Reg<ir_config_0::IR_CONFIG_0_SPEC>;
#[doc = "Infrared configuration register 0"]
pub mod ir_config_0;
#[doc = "ir_config_1 (rw) register accessor: an alias for `Reg<IR_CONFIG_1_SPEC>`"]
pub type IR_CONFIG_1 = crate::Reg<ir_config_1::IR_CONFIG_1_SPEC>;
#[doc = "Infrared configuration register 1"]
pub mod ir_config_1;
#[doc = "uart_config (rw) register accessor: an alias for `Reg<UART_CONFIG_SPEC>`"]
pub type UART_CONFIG = crate::Reg<uart_config::UART_CONFIG_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter configuration"]
pub mod uart_config;
#[doc = "uart_signal_0 (rw) register accessor: an alias for `Reg<UART_SIGNAL_0_SPEC>`"]
pub type UART_SIGNAL_0 = crate::Reg<uart_signal_0::UART_SIGNAL_0_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter signal configuration 0"]
pub mod uart_signal_0;
#[doc = "uart_signal_1 (rw) register accessor: an alias for `Reg<UART_SIGNAL_1_SPEC>`"]
pub type UART_SIGNAL_1 = crate::Reg<uart_signal_1::UART_SIGNAL_1_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter signal configuration 1"]
pub mod uart_signal_1;
#[doc = "flash_config (rw) register accessor: an alias for `Reg<FLASH_CONFIG_SPEC>`"]
pub type FLASH_CONFIG = crate::Reg<flash_config::FLASH_CONFIG_SPEC>;
#[doc = "Serial flash configuration"]
pub mod flash_config;
#[doc = "i2c_config (rw) register accessor: an alias for `Reg<I2C_CONFIG_SPEC>`"]
pub type I2C_CONFIG = crate::Reg<i2c_config::I2C_CONFIG_SPEC>;
#[doc = "Inter-Integrated Circuit bus configuration"]
pub mod i2c_config;
#[doc = "i2s_config (rw) register accessor: an alias for `Reg<I2S_CONFIG_SPEC>`"]
pub type I2S_CONFIG = crate::Reg<i2s_config::I2S_CONFIG_SPEC>;
#[doc = "Inter-IC Sound configuration"]
pub mod i2s_config;
#[doc = "spi_config (rw) register accessor: an alias for `Reg<SPI_CONFIG_SPEC>`"]
pub type SPI_CONFIG = crate::Reg<spi_config::SPI_CONFIG_SPEC>;
#[doc = "Serial Peripheral Interface configuration"]
pub mod spi_config;
#[doc = "pwm_config (rw) register accessor: an alias for `Reg<PWM_CONFIG_SPEC>`"]
pub type PWM_CONFIG = crate::Reg<pwm_config::PWM_CONFIG_SPEC>;
#[doc = "Pulse-Width configuration"]
pub mod pwm_config;
#[doc = "dbi_config (rw) register accessor: an alias for `Reg<DBI_CONFIG_SPEC>`"]
pub type DBI_CONFIG = crate::Reg<dbi_config::DBI_CONFIG_SPEC>;
#[doc = "MIPI Display Bus Interface clock configuration"]
pub mod dbi_config;
#[doc = "digit_clock_0 (rw) register accessor: an alias for `Reg<DIGIT_CLOCK_0_SPEC>`"]
pub type DIGIT_CLOCK_0 = crate::Reg<digit_clock_0::DIGIT_CLOCK_0_SPEC>;
#[doc = "Digital clock configuration 0"]
pub mod digit_clock_0;
#[doc = "digit_clock_1 (rw) register accessor: an alias for `Reg<DIGIT_CLOCK_1_SPEC>`"]
pub type DIGIT_CLOCK_1 = crate::Reg<digit_clock_1::DIGIT_CLOCK_1_SPEC>;
#[doc = "Digital clock configuration 1"]
pub mod digit_clock_1;
#[doc = "digit_clock_2 (rw) register accessor: an alias for `Reg<DIGIT_CLOCK_2_SPEC>`"]
pub type DIGIT_CLOCK_2 = crate::Reg<digit_clock_2::DIGIT_CLOCK_2_SPEC>;
#[doc = "Digital clock configuration 2"]
pub mod digit_clock_2;
#[doc = "radio_config (rw) register accessor: an alias for `Reg<RADIO_CONFIG_SPEC>`"]
pub type RADIO_CONFIG = crate::Reg<radio_config::RADIO_CONFIG_SPEC>;
#[doc = "Radio frequency configuration register"]
pub mod radio_config;
#[doc = "debug_config_0 (rw) register accessor: an alias for `Reg<DEBUG_CONFIG_0_SPEC>`"]
pub type DEBUG_CONFIG_0 = crate::Reg<debug_config_0::DEBUG_CONFIG_0_SPEC>;
#[doc = "Debug configuration register 0"]
pub mod debug_config_0;
#[doc = "debug_config_1 (rw) register accessor: an alias for `Reg<DEBUG_CONFIG_1_SPEC>`"]
pub type DEBUG_CONFIG_1 = crate::Reg<debug_config_1::DEBUG_CONFIG_1_SPEC>;
#[doc = "Debug configuration register 1"]
pub mod debug_config_1;
#[doc = "debug_config_2 (rw) register accessor: an alias for `Reg<DEBUG_CONFIG_2_SPEC>`"]
pub type DEBUG_CONFIG_2 = crate::Reg<debug_config_2::DEBUG_CONFIG_2_SPEC>;
#[doc = "Debug configuration register 2"]
pub mod debug_config_2;
#[doc = "debug_config_3 (rw) register accessor: an alias for `Reg<DEBUG_CONFIG_3_SPEC>`"]
pub type DEBUG_CONFIG_3 = crate::Reg<debug_config_3::DEBUG_CONFIG_3_SPEC>;
#[doc = "Debug configuration register 3"]
pub mod debug_config_3;
#[doc = "debug_config_4 (rw) register accessor: an alias for `Reg<DEBUG_CONFIG_4_SPEC>`"]
pub type DEBUG_CONFIG_4 = crate::Reg<debug_config_4::DEBUG_CONFIG_4_SPEC>;
#[doc = "Debug configuration register 4"]
pub mod debug_config_4;
#[doc = "self_test_0 (rw) register accessor: an alias for `Reg<SELF_TEST_0_SPEC>`"]
pub type SELF_TEST_0 = crate::Reg<self_test_0::SELF_TEST_0_SPEC>;
#[doc = "Machine Built-in Self Test register 0"]
pub mod self_test_0;
#[doc = "self_test_1 (rw) register accessor: an alias for `Reg<SELF_TEST_1_SPEC>`"]
pub type SELF_TEST_1 = crate::Reg<self_test_1::SELF_TEST_1_SPEC>;
#[doc = "Machine Built-in Self Test register 1"]
pub mod self_test_1;
#[doc = "audio_config_0 (rw) register accessor: an alias for `Reg<AUDIO_CONFIG_0_SPEC>`"]
pub type AUDIO_CONFIG_0 = crate::Reg<audio_config_0::AUDIO_CONFIG_0_SPEC>;
#[doc = "Audio configuration register 0"]
pub mod audio_config_0;
#[doc = "audio_config_1 (rw) register accessor: an alias for `Reg<AUDIO_CONFIG_1_SPEC>`"]
pub type AUDIO_CONFIG_1 = crate::Reg<audio_config_1::AUDIO_CONFIG_1_SPEC>;
#[doc = "Audio configuration register 1"]
pub mod audio_config_1;
#[doc = "wifi_pll_config_0 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_0_SPEC>`"]
pub type WIFI_PLL_CONFIG_0 = crate::Reg<wifi_pll_config_0::WIFI_PLL_CONFIG_0_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 0"]
pub mod wifi_pll_config_0;
#[doc = "wifi_pll_config_1 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_1_SPEC>`"]
pub type WIFI_PLL_CONFIG_1 = crate::Reg<wifi_pll_config_1::WIFI_PLL_CONFIG_1_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 1"]
pub mod wifi_pll_config_1;
#[doc = "wifi_pll_config_2 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_2_SPEC>`"]
pub type WIFI_PLL_CONFIG_2 = crate::Reg<wifi_pll_config_2::WIFI_PLL_CONFIG_2_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 2"]
pub mod wifi_pll_config_2;
#[doc = "wifi_pll_config_3 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_3_SPEC>`"]
pub type WIFI_PLL_CONFIG_3 = crate::Reg<wifi_pll_config_3::WIFI_PLL_CONFIG_3_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 3"]
pub mod wifi_pll_config_3;
#[doc = "wifi_pll_config_4 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_4_SPEC>`"]
pub type WIFI_PLL_CONFIG_4 = crate::Reg<wifi_pll_config_4::WIFI_PLL_CONFIG_4_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 4"]
pub mod wifi_pll_config_4;
#[doc = "wifi_pll_config_5 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_5_SPEC>`"]
pub type WIFI_PLL_CONFIG_5 = crate::Reg<wifi_pll_config_5::WIFI_PLL_CONFIG_5_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 5"]
pub mod wifi_pll_config_5;
#[doc = "wifi_pll_config_6 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_6_SPEC>`"]
pub type WIFI_PLL_CONFIG_6 = crate::Reg<wifi_pll_config_6::WIFI_PLL_CONFIG_6_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 6"]
pub mod wifi_pll_config_6;
#[doc = "wifi_pll_config_7 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_7_SPEC>`"]
pub type WIFI_PLL_CONFIG_7 = crate::Reg<wifi_pll_config_7::WIFI_PLL_CONFIG_7_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 7"]
pub mod wifi_pll_config_7;
#[doc = "wifi_pll_config_8 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_8_SPEC>`"]
pub type WIFI_PLL_CONFIG_8 = crate::Reg<wifi_pll_config_8::WIFI_PLL_CONFIG_8_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 8"]
pub mod wifi_pll_config_8;
#[doc = "wifi_pll_config_9 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_9_SPEC>`"]
pub type WIFI_PLL_CONFIG_9 = crate::Reg<wifi_pll_config_9::WIFI_PLL_CONFIG_9_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 9"]
pub mod wifi_pll_config_9;
#[doc = "wifi_pll_config_10 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_10_SPEC>`"]
pub type WIFI_PLL_CONFIG_10 = crate::Reg<wifi_pll_config_10::WIFI_PLL_CONFIG_10_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 10"]
pub mod wifi_pll_config_10;
#[doc = "wifi_pll_config_11 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_11_SPEC>`"]
pub type WIFI_PLL_CONFIG_11 = crate::Reg<wifi_pll_config_11::WIFI_PLL_CONFIG_11_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 11"]
pub mod wifi_pll_config_11;
#[doc = "wifi_pll_config_12 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_12_SPEC>`"]
pub type WIFI_PLL_CONFIG_12 = crate::Reg<wifi_pll_config_12::WIFI_PLL_CONFIG_12_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 12"]
pub mod wifi_pll_config_12;
#[doc = "wifi_pll_config_13 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_13_SPEC>`"]
pub type WIFI_PLL_CONFIG_13 = crate::Reg<wifi_pll_config_13::WIFI_PLL_CONFIG_13_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 13"]
pub mod wifi_pll_config_13;
#[doc = "wifi_pll_config_14 (rw) register accessor: an alias for `Reg<WIFI_PLL_CONFIG_14_SPEC>`"]
pub type WIFI_PLL_CONFIG_14 = crate::Reg<wifi_pll_config_14::WIFI_PLL_CONFIG_14_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 14"]
pub mod wifi_pll_config_14;
#[doc = "ldo18 (rw) register accessor: an alias for `Reg<LDO18_SPEC>`"]
pub type LDO18 = crate::Reg<ldo18::LDO18_SPEC>;
#[doc = "1.8-V Low Dropout Linear Regulator configuration"]
pub mod ldo18;
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

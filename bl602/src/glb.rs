#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock configuration for processor and bus"]
    pub clk_cfg0: CLK_CFG0,
    #[doc = "0x04 - clk_cfg1."]
    pub clk_cfg1: CLK_CFG1,
    #[doc = "0x08 - Clock configuration for UART and Flash"]
    pub clk_cfg2: CLK_CFG2,
    #[doc = "0x0c - Clock configuration for I2C and SPI"]
    pub clk_cfg3: CLK_CFG3,
    #[doc = "0x10 - swrst_cfg0."]
    pub swrst_cfg0: SWRST_CFG0,
    #[doc = "0x14 - swrst_cfg1."]
    pub swrst_cfg1: SWRST_CFG1,
    #[doc = "0x18 - swrst_cfg2."]
    pub swrst_cfg2: SWRST_CFG2,
    #[doc = "0x1c - swrst_cfg3."]
    pub swrst_cfg3: SWRST_CFG3,
    #[doc = "0x20 - cgen_cfg0."]
    pub cgen_cfg0: CGEN_CFG0,
    #[doc = "0x24 - cgen_cfg1."]
    pub cgen_cfg1: CGEN_CFG1,
    #[doc = "0x28 - cgen_cfg2."]
    pub cgen_cfg2: CGEN_CFG2,
    #[doc = "0x2c - cgen_cfg3."]
    pub cgen_cfg3: CGEN_CFG3,
    #[doc = "0x30 - MBIST_CTL."]
    pub mbist_ctl: MBIST_CTL,
    #[doc = "0x34 - MBIST_STAT."]
    pub mbist_stat: MBIST_STAT,
    _reserved14: [u8; 0x18],
    #[doc = "0x50 - bmx_cfg1."]
    pub bmx_cfg1: BMX_CFG1,
    #[doc = "0x54 - bmx_cfg2."]
    pub bmx_cfg2: BMX_CFG2,
    #[doc = "0x58 - bmx_err_addr."]
    pub bmx_err_addr: BMX_ERR_ADDR,
    #[doc = "0x5c - bmx_dbg_out."]
    pub bmx_dbg_out: BMX_DBG_OUT,
    #[doc = "0x60 - rsv0."]
    pub rsv0: RSV0,
    #[doc = "0x64 - rsv1."]
    pub rsv1: RSV1,
    #[doc = "0x68 - rsv2."]
    pub rsv2: RSV2,
    #[doc = "0x6c - rsv3."]
    pub rsv3: RSV3,
    #[doc = "0x70 - sram_ret."]
    pub sram_ret: SRAM_RET,
    #[doc = "0x74 - sram_slp."]
    pub sram_slp: SRAM_SLP,
    #[doc = "0x78 - sram_parm."]
    pub sram_parm: SRAM_PARM,
    #[doc = "0x7c - seam_misc."]
    pub seam_misc: SEAM_MISC,
    #[doc = "0x80 - glb_parm."]
    pub glb_parm: GLB_PARM,
    _reserved27: [u8; 0x0c],
    #[doc = "0x90 - CPU_CLK_CFG."]
    pub cpu_clk_cfg: CPU_CLK_CFG,
    _reserved28: [u8; 0x10],
    #[doc = "0xa4 - Clock configuration for GPADC"]
    pub gpadc_32m_src_ctrl: GPADC_32M_SRC_CTRL,
    #[doc = "0xa8 - DIG32K_WAKEUP_CTRL."]
    pub dig32k_wakeup_ctrl: DIG32K_WAKEUP_CTRL,
    #[doc = "0xac - WIFI_BT_COEX_CTRL."]
    pub wifi_bt_coex_ctrl: WIFI_BT_COEX_CTRL,
    _reserved31: [u8; 0x10],
    #[doc = "0xc0 - UART_SIG_SEL_0."]
    pub uart_sig_sel_0: UART_SIG_SEL_0,
    _reserved32: [u8; 0x0c],
    #[doc = "0xd0 - DBG_SEL_LL."]
    pub dbg_sel_ll: DBG_SEL_LL,
    #[doc = "0xd4 - DBG_SEL_LH."]
    pub dbg_sel_lh: DBG_SEL_LH,
    #[doc = "0xd8 - DBG_SEL_HL."]
    pub dbg_sel_hl: DBG_SEL_HL,
    #[doc = "0xdc - DBG_SEL_HH."]
    pub dbg_sel_hh: DBG_SEL_HH,
    #[doc = "0xe0 - debug."]
    pub debug: DEBUG,
    _reserved37: [u8; 0x1c],
    #[doc = "0x100 - GPIO0, GPIO1 configuration"]
    pub gpio_cfgctl0: GPIO_CFGCTL0,
    #[doc = "0x104 - GPIO2, GPIO3 configuration"]
    pub gpio_cfgctl1: GPIO_CFGCTL1,
    #[doc = "0x108 - GPIO4, GPIO5 configuration"]
    pub gpio_cfgctl2: GPIO_CFGCTL2,
    #[doc = "0x10c - GPIO6, GPIO7 configuration"]
    pub gpio_cfgctl3: GPIO_CFGCTL3,
    #[doc = "0x110 - GPIO8, GPIO9 configuration"]
    pub gpio_cfgctl4: GPIO_CFGCTL4,
    #[doc = "0x114 - GPIO10, GPIO11 configuration"]
    pub gpio_cfgctl5: GPIO_CFGCTL5,
    #[doc = "0x118 - GPIO12, GPIO13 configuration"]
    pub gpio_cfgctl6: GPIO_CFGCTL6,
    #[doc = "0x11c - GPIO14, GPIO15 configuration"]
    pub gpio_cfgctl7: GPIO_CFGCTL7,
    #[doc = "0x120 - GPIO16, GPIO17 configuration"]
    pub gpio_cfgctl8: GPIO_CFGCTL8,
    #[doc = "0x124 - GPIO18, GPIO19 configuration"]
    pub gpio_cfgctl9: GPIO_CFGCTL9,
    #[doc = "0x128 - GPIO20, GPIO21 configuration"]
    pub gpio_cfgctl10: GPIO_CFGCTL10,
    #[doc = "0x12c - GPIO22, GPIO23 configuration"]
    pub gpio_cfgctl11: GPIO_CFGCTL11,
    #[doc = "0x130 - GPIO24, GPIO25 configuration"]
    pub gpio_cfgctl12: GPIO_CFGCTL12,
    #[doc = "0x134 - GPIO26, GPIO27 configuration"]
    pub gpio_cfgctl13: GPIO_CFGCTL13,
    #[doc = "0x138 - GPIO28 configuration"]
    pub gpio_cfgctl14: GPIO_CFGCTL14,
    _reserved52: [u8; 0x44],
    #[doc = "0x180 - Input register for all GPIO pins. Input Enabled bit must be set in configuration register to work."]
    pub gpio_cfgctl30: GPIO_CFGCTL30,
    #[doc = "0x184 - Reserved according to SDK."]
    pub gpio_cfgctl31: GPIO_CFGCTL31,
    #[doc = "0x188 - Output register for all GPIO pins. Output Enabled bit must be set in Output Enable register to work."]
    pub gpio_cfgctl32: GPIO_CFGCTL32,
    #[doc = "0x18c - Reserved according to SDK."]
    pub gpio_cfgctl33: GPIO_CFGCTL33,
    #[doc = "0x190 - Output enable register for GPIO."]
    pub gpio_cfgctl34: GPIO_CFGCTL34,
    #[doc = "0x194 - Reserved according to SDK."]
    pub gpio_cfgctl35: GPIO_CFGCTL35,
    _reserved58: [u8; 0x08],
    #[doc = "0x1a0 - Interrupt masking register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins."]
    pub gpio_int_mask1: GPIO_INT_MASK1,
    _reserved59: [u8; 0x04],
    #[doc = "0x1a8 - Interrupt status register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins."]
    pub gpio_int_stat1: GPIO_INT_STAT1,
    _reserved60: [u8; 0x04],
    #[doc = "0x1b0 - Interrupt clearing register."]
    pub gpio_int_clr1: GPIO_INT_CLR1,
    _reserved61: [u8; 0x0c],
    #[doc = "0x1c0 - GPIO interrupt trigger and control register for GPIO0-GPIO9."]
    pub gpio_int_mode_set1: GPIO_INT_MODE_SET1,
    #[doc = "0x1c4 - GPIO interrupt trigger and control register for GPIO10-GPIO19."]
    pub gpio_int_mode_set2: GPIO_INT_MODE_SET2,
    #[doc = "0x1c8 - GPIO interrupt trigger and control register for GPIO20-GPIO29."]
    pub gpio_int_mode_set3: GPIO_INT_MODE_SET3,
    _reserved64: [u8; 0x58],
    #[doc = "0x224 - led_driver."]
    pub led_driver: LED_DRIVER,
    _reserved65: [u8; 0xe0],
    #[doc = "0x308 - gpdac_ctrl."]
    pub gpdac_ctrl: GPDAC_CTRL,
    #[doc = "0x30c - gpdac_actrl."]
    pub gpdac_actrl: GPDAC_ACTRL,
    #[doc = "0x310 - gpdac_bctrl."]
    pub gpdac_bctrl: GPDAC_BCTRL,
    #[doc = "0x314 - gpdac_data."]
    pub gpdac_data: GPDAC_DATA,
    _reserved69: [u8; 0x0be8],
    #[doc = "0xf00 - tzc_glb_ctrl_0."]
    pub tzc_glb_ctrl_0: TZC_GLB_CTRL_0,
    #[doc = "0xf04 - tzc_glb_ctrl_1."]
    pub tzc_glb_ctrl_1: TZC_GLB_CTRL_1,
    #[doc = "0xf08 - tzc_glb_ctrl_2."]
    pub tzc_glb_ctrl_2: TZC_GLB_CTRL_2,
    #[doc = "0xf0c - Reserved according to SDK."]
    pub tzc_glb_ctrl_3: TZC_GLB_CTRL_3,
}
#[doc = "clk_cfg0 (rw) register accessor: an alias for `Reg<CLK_CFG0_SPEC>`"]
pub type CLK_CFG0 = crate::Reg<clk_cfg0::CLK_CFG0_SPEC>;
#[doc = "Clock configuration for processor and bus"]
pub mod clk_cfg0;
#[doc = "clk_cfg1 (rw) register accessor: an alias for `Reg<CLK_CFG1_SPEC>`"]
pub type CLK_CFG1 = crate::Reg<clk_cfg1::CLK_CFG1_SPEC>;
#[doc = "clk_cfg1."]
pub mod clk_cfg1;
#[doc = "clk_cfg2 (rw) register accessor: an alias for `Reg<CLK_CFG2_SPEC>`"]
pub type CLK_CFG2 = crate::Reg<clk_cfg2::CLK_CFG2_SPEC>;
#[doc = "Clock configuration for UART and Flash"]
pub mod clk_cfg2;
#[doc = "clk_cfg3 (rw) register accessor: an alias for `Reg<CLK_CFG3_SPEC>`"]
pub type CLK_CFG3 = crate::Reg<clk_cfg3::CLK_CFG3_SPEC>;
#[doc = "Clock configuration for I2C and SPI"]
pub mod clk_cfg3;
#[doc = "swrst_cfg0 (rw) register accessor: an alias for `Reg<SWRST_CFG0_SPEC>`"]
pub type SWRST_CFG0 = crate::Reg<swrst_cfg0::SWRST_CFG0_SPEC>;
#[doc = "swrst_cfg0."]
pub mod swrst_cfg0;
#[doc = "swrst_cfg1 (rw) register accessor: an alias for `Reg<SWRST_CFG1_SPEC>`"]
pub type SWRST_CFG1 = crate::Reg<swrst_cfg1::SWRST_CFG1_SPEC>;
#[doc = "swrst_cfg1."]
pub mod swrst_cfg1;
#[doc = "swrst_cfg2 (rw) register accessor: an alias for `Reg<SWRST_CFG2_SPEC>`"]
pub type SWRST_CFG2 = crate::Reg<swrst_cfg2::SWRST_CFG2_SPEC>;
#[doc = "swrst_cfg2."]
pub mod swrst_cfg2;
#[doc = "swrst_cfg3 (rw) register accessor: an alias for `Reg<SWRST_CFG3_SPEC>`"]
pub type SWRST_CFG3 = crate::Reg<swrst_cfg3::SWRST_CFG3_SPEC>;
#[doc = "swrst_cfg3."]
pub mod swrst_cfg3;
#[doc = "cgen_cfg0 (rw) register accessor: an alias for `Reg<CGEN_CFG0_SPEC>`"]
pub type CGEN_CFG0 = crate::Reg<cgen_cfg0::CGEN_CFG0_SPEC>;
#[doc = "cgen_cfg0."]
pub mod cgen_cfg0;
#[doc = "cgen_cfg1 (rw) register accessor: an alias for `Reg<CGEN_CFG1_SPEC>`"]
pub type CGEN_CFG1 = crate::Reg<cgen_cfg1::CGEN_CFG1_SPEC>;
#[doc = "cgen_cfg1."]
pub mod cgen_cfg1;
#[doc = "cgen_cfg2 (rw) register accessor: an alias for `Reg<CGEN_CFG2_SPEC>`"]
pub type CGEN_CFG2 = crate::Reg<cgen_cfg2::CGEN_CFG2_SPEC>;
#[doc = "cgen_cfg2."]
pub mod cgen_cfg2;
#[doc = "cgen_cfg3 (rw) register accessor: an alias for `Reg<CGEN_CFG3_SPEC>`"]
pub type CGEN_CFG3 = crate::Reg<cgen_cfg3::CGEN_CFG3_SPEC>;
#[doc = "cgen_cfg3."]
pub mod cgen_cfg3;
#[doc = "MBIST_CTL (rw) register accessor: an alias for `Reg<MBIST_CTL_SPEC>`"]
pub type MBIST_CTL = crate::Reg<mbist_ctl::MBIST_CTL_SPEC>;
#[doc = "MBIST_CTL."]
pub mod mbist_ctl;
#[doc = "MBIST_STAT (r) register accessor: an alias for `Reg<MBIST_STAT_SPEC>`"]
pub type MBIST_STAT = crate::Reg<mbist_stat::MBIST_STAT_SPEC>;
#[doc = "MBIST_STAT."]
pub mod mbist_stat;
#[doc = "bmx_cfg1 (rw) register accessor: an alias for `Reg<BMX_CFG1_SPEC>`"]
pub type BMX_CFG1 = crate::Reg<bmx_cfg1::BMX_CFG1_SPEC>;
#[doc = "bmx_cfg1."]
pub mod bmx_cfg1;
#[doc = "bmx_cfg2 (rw) register accessor: an alias for `Reg<BMX_CFG2_SPEC>`"]
pub type BMX_CFG2 = crate::Reg<bmx_cfg2::BMX_CFG2_SPEC>;
#[doc = "bmx_cfg2."]
pub mod bmx_cfg2;
#[doc = "bmx_err_addr (r) register accessor: an alias for `Reg<BMX_ERR_ADDR_SPEC>`"]
pub type BMX_ERR_ADDR = crate::Reg<bmx_err_addr::BMX_ERR_ADDR_SPEC>;
#[doc = "bmx_err_addr."]
pub mod bmx_err_addr;
#[doc = "bmx_dbg_out (r) register accessor: an alias for `Reg<BMX_DBG_OUT_SPEC>`"]
pub type BMX_DBG_OUT = crate::Reg<bmx_dbg_out::BMX_DBG_OUT_SPEC>;
#[doc = "bmx_dbg_out."]
pub mod bmx_dbg_out;
#[doc = "rsv0 (rw) register accessor: an alias for `Reg<RSV0_SPEC>`"]
pub type RSV0 = crate::Reg<rsv0::RSV0_SPEC>;
#[doc = "rsv0."]
pub mod rsv0;
#[doc = "rsv1 (rw) register accessor: an alias for `Reg<RSV1_SPEC>`"]
pub type RSV1 = crate::Reg<rsv1::RSV1_SPEC>;
#[doc = "rsv1."]
pub mod rsv1;
#[doc = "rsv2 (rw) register accessor: an alias for `Reg<RSV2_SPEC>`"]
pub type RSV2 = crate::Reg<rsv2::RSV2_SPEC>;
#[doc = "rsv2."]
pub mod rsv2;
#[doc = "rsv3 (rw) register accessor: an alias for `Reg<RSV3_SPEC>`"]
pub type RSV3 = crate::Reg<rsv3::RSV3_SPEC>;
#[doc = "rsv3."]
pub mod rsv3;
#[doc = "sram_ret (rw) register accessor: an alias for `Reg<SRAM_RET_SPEC>`"]
pub type SRAM_RET = crate::Reg<sram_ret::SRAM_RET_SPEC>;
#[doc = "sram_ret."]
pub mod sram_ret;
#[doc = "sram_slp (rw) register accessor: an alias for `Reg<SRAM_SLP_SPEC>`"]
pub type SRAM_SLP = crate::Reg<sram_slp::SRAM_SLP_SPEC>;
#[doc = "sram_slp."]
pub mod sram_slp;
#[doc = "sram_parm (rw) register accessor: an alias for `Reg<SRAM_PARM_SPEC>`"]
pub type SRAM_PARM = crate::Reg<sram_parm::SRAM_PARM_SPEC>;
#[doc = "sram_parm."]
pub mod sram_parm;
#[doc = "seam_misc (rw) register accessor: an alias for `Reg<SEAM_MISC_SPEC>`"]
pub type SEAM_MISC = crate::Reg<seam_misc::SEAM_MISC_SPEC>;
#[doc = "seam_misc."]
pub mod seam_misc;
#[doc = "glb_parm (rw) register accessor: an alias for `Reg<GLB_PARM_SPEC>`"]
pub type GLB_PARM = crate::Reg<glb_parm::GLB_PARM_SPEC>;
#[doc = "glb_parm."]
pub mod glb_parm;
#[doc = "CPU_CLK_CFG (rw) register accessor: an alias for `Reg<CPU_CLK_CFG_SPEC>`"]
pub type CPU_CLK_CFG = crate::Reg<cpu_clk_cfg::CPU_CLK_CFG_SPEC>;
#[doc = "CPU_CLK_CFG."]
pub mod cpu_clk_cfg;
#[doc = "GPADC_32M_SRC_CTRL (rw) register accessor: an alias for `Reg<GPADC_32M_SRC_CTRL_SPEC>`"]
pub type GPADC_32M_SRC_CTRL = crate::Reg<gpadc_32m_src_ctrl::GPADC_32M_SRC_CTRL_SPEC>;
#[doc = "Clock configuration for GPADC"]
pub mod gpadc_32m_src_ctrl;
#[doc = "DIG32K_WAKEUP_CTRL (rw) register accessor: an alias for `Reg<DIG32K_WAKEUP_CTRL_SPEC>`"]
pub type DIG32K_WAKEUP_CTRL = crate::Reg<dig32k_wakeup_ctrl::DIG32K_WAKEUP_CTRL_SPEC>;
#[doc = "DIG32K_WAKEUP_CTRL."]
pub mod dig32k_wakeup_ctrl;
#[doc = "WIFI_BT_COEX_CTRL (rw) register accessor: an alias for `Reg<WIFI_BT_COEX_CTRL_SPEC>`"]
pub type WIFI_BT_COEX_CTRL = crate::Reg<wifi_bt_coex_ctrl::WIFI_BT_COEX_CTRL_SPEC>;
#[doc = "WIFI_BT_COEX_CTRL."]
pub mod wifi_bt_coex_ctrl;
#[doc = "UART_SIG_SEL_0 (rw) register accessor: an alias for `Reg<UART_SIG_SEL_0_SPEC>`"]
pub type UART_SIG_SEL_0 = crate::Reg<uart_sig_sel_0::UART_SIG_SEL_0_SPEC>;
#[doc = "UART_SIG_SEL_0."]
pub mod uart_sig_sel_0;
#[doc = "DBG_SEL_LL (rw) register accessor: an alias for `Reg<DBG_SEL_LL_SPEC>`"]
pub type DBG_SEL_LL = crate::Reg<dbg_sel_ll::DBG_SEL_LL_SPEC>;
#[doc = "DBG_SEL_LL."]
pub mod dbg_sel_ll;
#[doc = "DBG_SEL_LH (rw) register accessor: an alias for `Reg<DBG_SEL_LH_SPEC>`"]
pub type DBG_SEL_LH = crate::Reg<dbg_sel_lh::DBG_SEL_LH_SPEC>;
#[doc = "DBG_SEL_LH."]
pub mod dbg_sel_lh;
#[doc = "DBG_SEL_HL (rw) register accessor: an alias for `Reg<DBG_SEL_HL_SPEC>`"]
pub type DBG_SEL_HL = crate::Reg<dbg_sel_hl::DBG_SEL_HL_SPEC>;
#[doc = "DBG_SEL_HL."]
pub mod dbg_sel_hl;
#[doc = "DBG_SEL_HH (rw) register accessor: an alias for `Reg<DBG_SEL_HH_SPEC>`"]
pub type DBG_SEL_HH = crate::Reg<dbg_sel_hh::DBG_SEL_HH_SPEC>;
#[doc = "DBG_SEL_HH."]
pub mod dbg_sel_hh;
#[doc = "debug (rw) register accessor: an alias for `Reg<DEBUG_SPEC>`"]
pub type DEBUG = crate::Reg<debug::DEBUG_SPEC>;
#[doc = "debug."]
pub mod debug;
#[doc = "GPIO_CFGCTL0 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL0_SPEC>`"]
pub type GPIO_CFGCTL0 = crate::Reg<gpio_cfgctl0::GPIO_CFGCTL0_SPEC>;
#[doc = "GPIO0, GPIO1 configuration"]
pub mod gpio_cfgctl0;
#[doc = "GPIO_CFGCTL1 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL1_SPEC>`"]
pub type GPIO_CFGCTL1 = crate::Reg<gpio_cfgctl1::GPIO_CFGCTL1_SPEC>;
#[doc = "GPIO2, GPIO3 configuration"]
pub mod gpio_cfgctl1;
#[doc = "GPIO_CFGCTL2 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL2_SPEC>`"]
pub type GPIO_CFGCTL2 = crate::Reg<gpio_cfgctl2::GPIO_CFGCTL2_SPEC>;
#[doc = "GPIO4, GPIO5 configuration"]
pub mod gpio_cfgctl2;
#[doc = "GPIO_CFGCTL3 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL3_SPEC>`"]
pub type GPIO_CFGCTL3 = crate::Reg<gpio_cfgctl3::GPIO_CFGCTL3_SPEC>;
#[doc = "GPIO6, GPIO7 configuration"]
pub mod gpio_cfgctl3;
#[doc = "GPIO_CFGCTL4 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL4_SPEC>`"]
pub type GPIO_CFGCTL4 = crate::Reg<gpio_cfgctl4::GPIO_CFGCTL4_SPEC>;
#[doc = "GPIO8, GPIO9 configuration"]
pub mod gpio_cfgctl4;
#[doc = "GPIO_CFGCTL5 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL5_SPEC>`"]
pub type GPIO_CFGCTL5 = crate::Reg<gpio_cfgctl5::GPIO_CFGCTL5_SPEC>;
#[doc = "GPIO10, GPIO11 configuration"]
pub mod gpio_cfgctl5;
#[doc = "GPIO_CFGCTL6 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL6_SPEC>`"]
pub type GPIO_CFGCTL6 = crate::Reg<gpio_cfgctl6::GPIO_CFGCTL6_SPEC>;
#[doc = "GPIO12, GPIO13 configuration"]
pub mod gpio_cfgctl6;
#[doc = "GPIO_CFGCTL7 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL7_SPEC>`"]
pub type GPIO_CFGCTL7 = crate::Reg<gpio_cfgctl7::GPIO_CFGCTL7_SPEC>;
#[doc = "GPIO14, GPIO15 configuration"]
pub mod gpio_cfgctl7;
#[doc = "GPIO_CFGCTL8 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL8_SPEC>`"]
pub type GPIO_CFGCTL8 = crate::Reg<gpio_cfgctl8::GPIO_CFGCTL8_SPEC>;
#[doc = "GPIO16, GPIO17 configuration"]
pub mod gpio_cfgctl8;
#[doc = "GPIO_CFGCTL9 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL9_SPEC>`"]
pub type GPIO_CFGCTL9 = crate::Reg<gpio_cfgctl9::GPIO_CFGCTL9_SPEC>;
#[doc = "GPIO18, GPIO19 configuration"]
pub mod gpio_cfgctl9;
#[doc = "GPIO_CFGCTL10 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL10_SPEC>`"]
pub type GPIO_CFGCTL10 = crate::Reg<gpio_cfgctl10::GPIO_CFGCTL10_SPEC>;
#[doc = "GPIO20, GPIO21 configuration"]
pub mod gpio_cfgctl10;
#[doc = "GPIO_CFGCTL11 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL11_SPEC>`"]
pub type GPIO_CFGCTL11 = crate::Reg<gpio_cfgctl11::GPIO_CFGCTL11_SPEC>;
#[doc = "GPIO22, GPIO23 configuration"]
pub mod gpio_cfgctl11;
#[doc = "GPIO_CFGCTL12 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL12_SPEC>`"]
pub type GPIO_CFGCTL12 = crate::Reg<gpio_cfgctl12::GPIO_CFGCTL12_SPEC>;
#[doc = "GPIO24, GPIO25 configuration"]
pub mod gpio_cfgctl12;
#[doc = "GPIO_CFGCTL13 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL13_SPEC>`"]
pub type GPIO_CFGCTL13 = crate::Reg<gpio_cfgctl13::GPIO_CFGCTL13_SPEC>;
#[doc = "GPIO26, GPIO27 configuration"]
pub mod gpio_cfgctl13;
#[doc = "GPIO_CFGCTL14 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL14_SPEC>`"]
pub type GPIO_CFGCTL14 = crate::Reg<gpio_cfgctl14::GPIO_CFGCTL14_SPEC>;
#[doc = "GPIO28 configuration"]
pub mod gpio_cfgctl14;
#[doc = "GPIO_CFGCTL30 (r) register accessor: an alias for `Reg<GPIO_CFGCTL30_SPEC>`"]
pub type GPIO_CFGCTL30 = crate::Reg<gpio_cfgctl30::GPIO_CFGCTL30_SPEC>;
#[doc = "Input register for all GPIO pins. Input Enabled bit must be set in configuration register to work."]
pub mod gpio_cfgctl30;
#[doc = "GPIO_CFGCTL31 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL31_SPEC>`"]
pub type GPIO_CFGCTL31 = crate::Reg<gpio_cfgctl31::GPIO_CFGCTL31_SPEC>;
#[doc = "Reserved according to SDK."]
pub mod gpio_cfgctl31;
#[doc = "GPIO_CFGCTL32 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL32_SPEC>`"]
pub type GPIO_CFGCTL32 = crate::Reg<gpio_cfgctl32::GPIO_CFGCTL32_SPEC>;
#[doc = "Output register for all GPIO pins. Output Enabled bit must be set in Output Enable register to work."]
pub mod gpio_cfgctl32;
#[doc = "GPIO_CFGCTL33 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL33_SPEC>`"]
pub type GPIO_CFGCTL33 = crate::Reg<gpio_cfgctl33::GPIO_CFGCTL33_SPEC>;
#[doc = "Reserved according to SDK."]
pub mod gpio_cfgctl33;
#[doc = "GPIO_CFGCTL34 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL34_SPEC>`"]
pub type GPIO_CFGCTL34 = crate::Reg<gpio_cfgctl34::GPIO_CFGCTL34_SPEC>;
#[doc = "Output enable register for GPIO."]
pub mod gpio_cfgctl34;
#[doc = "GPIO_CFGCTL35 (rw) register accessor: an alias for `Reg<GPIO_CFGCTL35_SPEC>`"]
pub type GPIO_CFGCTL35 = crate::Reg<gpio_cfgctl35::GPIO_CFGCTL35_SPEC>;
#[doc = "Reserved according to SDK."]
pub mod gpio_cfgctl35;
#[doc = "GPIO_INT_MASK1 (rw) register accessor: an alias for `Reg<GPIO_INT_MASK1_SPEC>`"]
pub type GPIO_INT_MASK1 = crate::Reg<gpio_int_mask1::GPIO_INT_MASK1_SPEC>;
#[doc = "Interrupt masking register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins."]
pub mod gpio_int_mask1;
#[doc = "GPIO_INT_STAT1 (r) register accessor: an alias for `Reg<GPIO_INT_STAT1_SPEC>`"]
pub type GPIO_INT_STAT1 = crate::Reg<gpio_int_stat1::GPIO_INT_STAT1_SPEC>;
#[doc = "Interrupt status register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins."]
pub mod gpio_int_stat1;
#[doc = "GPIO_INT_CLR1 (rw) register accessor: an alias for `Reg<GPIO_INT_CLR1_SPEC>`"]
pub type GPIO_INT_CLR1 = crate::Reg<gpio_int_clr1::GPIO_INT_CLR1_SPEC>;
#[doc = "Interrupt clearing register."]
pub mod gpio_int_clr1;
#[doc = "GPIO_INT_MODE_SET1 (rw) register accessor: an alias for `Reg<GPIO_INT_MODE_SET1_SPEC>`"]
pub type GPIO_INT_MODE_SET1 = crate::Reg<gpio_int_mode_set1::GPIO_INT_MODE_SET1_SPEC>;
#[doc = "GPIO interrupt trigger and control register for GPIO0-GPIO9."]
pub mod gpio_int_mode_set1;
#[doc = "GPIO_INT_MODE_SET2 (rw) register accessor: an alias for `Reg<GPIO_INT_MODE_SET2_SPEC>`"]
pub type GPIO_INT_MODE_SET2 = crate::Reg<gpio_int_mode_set2::GPIO_INT_MODE_SET2_SPEC>;
#[doc = "GPIO interrupt trigger and control register for GPIO10-GPIO19."]
pub mod gpio_int_mode_set2;
#[doc = "GPIO_INT_MODE_SET3 (rw) register accessor: an alias for `Reg<GPIO_INT_MODE_SET3_SPEC>`"]
pub type GPIO_INT_MODE_SET3 = crate::Reg<gpio_int_mode_set3::GPIO_INT_MODE_SET3_SPEC>;
#[doc = "GPIO interrupt trigger and control register for GPIO20-GPIO29."]
pub mod gpio_int_mode_set3;
#[doc = "led_driver (rw) register accessor: an alias for `Reg<LED_DRIVER_SPEC>`"]
pub type LED_DRIVER = crate::Reg<led_driver::LED_DRIVER_SPEC>;
#[doc = "led_driver."]
pub mod led_driver;
#[doc = "gpdac_ctrl (rw) register accessor: an alias for `Reg<GPDAC_CTRL_SPEC>`"]
pub type GPDAC_CTRL = crate::Reg<gpdac_ctrl::GPDAC_CTRL_SPEC>;
#[doc = "gpdac_ctrl."]
pub mod gpdac_ctrl;
#[doc = "gpdac_actrl (rw) register accessor: an alias for `Reg<GPDAC_ACTRL_SPEC>`"]
pub type GPDAC_ACTRL = crate::Reg<gpdac_actrl::GPDAC_ACTRL_SPEC>;
#[doc = "gpdac_actrl."]
pub mod gpdac_actrl;
#[doc = "gpdac_bctrl (rw) register accessor: an alias for `Reg<GPDAC_BCTRL_SPEC>`"]
pub type GPDAC_BCTRL = crate::Reg<gpdac_bctrl::GPDAC_BCTRL_SPEC>;
#[doc = "gpdac_bctrl."]
pub mod gpdac_bctrl;
#[doc = "gpdac_data (rw) register accessor: an alias for `Reg<GPDAC_DATA_SPEC>`"]
pub type GPDAC_DATA = crate::Reg<gpdac_data::GPDAC_DATA_SPEC>;
#[doc = "gpdac_data."]
pub mod gpdac_data;
#[doc = "tzc_glb_ctrl_0 (r) register accessor: an alias for `Reg<TZC_GLB_CTRL_0_SPEC>`"]
pub type TZC_GLB_CTRL_0 = crate::Reg<tzc_glb_ctrl_0::TZC_GLB_CTRL_0_SPEC>;
#[doc = "tzc_glb_ctrl_0."]
pub mod tzc_glb_ctrl_0;
#[doc = "tzc_glb_ctrl_1 (r) register accessor: an alias for `Reg<TZC_GLB_CTRL_1_SPEC>`"]
pub type TZC_GLB_CTRL_1 = crate::Reg<tzc_glb_ctrl_1::TZC_GLB_CTRL_1_SPEC>;
#[doc = "tzc_glb_ctrl_1."]
pub mod tzc_glb_ctrl_1;
#[doc = "tzc_glb_ctrl_2 (r) register accessor: an alias for `Reg<TZC_GLB_CTRL_2_SPEC>`"]
pub type TZC_GLB_CTRL_2 = crate::Reg<tzc_glb_ctrl_2::TZC_GLB_CTRL_2_SPEC>;
#[doc = "tzc_glb_ctrl_2."]
pub mod tzc_glb_ctrl_2;
#[doc = "tzc_glb_ctrl_3 (rw) register accessor: an alias for `Reg<TZC_GLB_CTRL_3_SPEC>`"]
pub type TZC_GLB_CTRL_3 = crate::Reg<tzc_glb_ctrl_3::TZC_GLB_CTRL_3_SPEC>;
#[doc = "Reserved according to SDK."]
pub mod tzc_glb_ctrl_3;

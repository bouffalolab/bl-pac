#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - sf_ctrl_0."]
    pub sf_ctrl_0: SF_CTRL_0,
    #[doc = "0x04 - sf_ctrl_1."]
    pub sf_ctrl_1: SF_CTRL_1,
    #[doc = "0x08 - sf_if_sahb_0."]
    pub sf_if_sahb_0: SF_IF_SAHB_0,
    #[doc = "0x0c - sf_if_sahb_1."]
    pub sf_if_sahb_1: SF_IF_SAHB_1,
    #[doc = "0x10 - sf_if_sahb_2."]
    pub sf_if_sahb_2: SF_IF_SAHB_2,
    #[doc = "0x14 - sf_if_iahb_0."]
    pub sf_if_iahb_0: SF_IF_IAHB_0,
    #[doc = "0x18 - sf_if_iahb_1."]
    pub sf_if_iahb_1: SF_IF_IAHB_1,
    #[doc = "0x1c - sf_if_iahb_2."]
    pub sf_if_iahb_2: SF_IF_IAHB_2,
    #[doc = "0x20 - sf_if_status_0."]
    pub sf_if_status_0: SF_IF_STATUS_0,
    #[doc = "0x24 - sf_if_status_1."]
    pub sf_if_status_1: SF_IF_STATUS_1,
    #[doc = "0x28 - sf_aes."]
    pub sf_aes: SF_AES,
    #[doc = "0x2c - sf_ahb2sif_status."]
    pub sf_ahb2sif_status: SF_AHB2SIF_STATUS,
    #[doc = "0x30 - sf_if_io_dly_0."]
    pub sf_if_io_dly_0: SF_IF_IO_DLY_0,
    #[doc = "0x34 - sf_if_io_dly_1."]
    pub sf_if_io_dly_1: SF_IF_IO_DLY_1,
    #[doc = "0x38 - sf_if_io_dly_2."]
    pub sf_if_io_dly_2: SF_IF_IO_DLY_2,
    #[doc = "0x3c - sf_if_io_dly_3."]
    pub sf_if_io_dly_3: SF_IF_IO_DLY_3,
    #[doc = "0x40 - sf_if_io_dly_4."]
    pub sf_if_io_dly_4: SF_IF_IO_DLY_4,
    #[doc = "0x44 - sf_reserved."]
    pub sf_reserved: SF_RESERVED,
    #[doc = "0x48 - sf2_if_io_dly_0."]
    pub sf2_if_io_dly_0: SF2_IF_IO_DLY_0,
    #[doc = "0x4c - sf2_if_io_dly_1."]
    pub sf2_if_io_dly_1: SF2_IF_IO_DLY_1,
    #[doc = "0x50 - sf2_if_io_dly_2."]
    pub sf2_if_io_dly_2: SF2_IF_IO_DLY_2,
    #[doc = "0x54 - sf2_if_io_dly_3."]
    pub sf2_if_io_dly_3: SF2_IF_IO_DLY_3,
    #[doc = "0x58 - sf2_if_io_dly_4."]
    pub sf2_if_io_dly_4: SF2_IF_IO_DLY_4,
    #[doc = "0x5c - sf3_if_io_dly_0."]
    pub sf3_if_io_dly_0: SF3_IF_IO_DLY_0,
    #[doc = "0x60 - sf3_if_io_dly_1."]
    pub sf3_if_io_dly_1: SF3_IF_IO_DLY_1,
    #[doc = "0x64 - sf3_if_io_dly_2."]
    pub sf3_if_io_dly_2: SF3_IF_IO_DLY_2,
    #[doc = "0x68 - sf3_if_io_dly_3."]
    pub sf3_if_io_dly_3: SF3_IF_IO_DLY_3,
    #[doc = "0x6c - sf3_if_io_dly_4."]
    pub sf3_if_io_dly_4: SF3_IF_IO_DLY_4,
    #[doc = "0x70 - sf_ctrl_2."]
    pub sf_ctrl_2: SF_CTRL_2,
    #[doc = "0x74 - sf_ctrl_3."]
    pub sf_ctrl_3: SF_CTRL_3,
    #[doc = "0x78 - sf_if_iahb_3."]
    pub sf_if_iahb_3: SF_IF_IAHB_3,
    #[doc = "0x7c - sf_if_iahb_4."]
    pub sf_if_iahb_4: SF_IF_IAHB_4,
    #[doc = "0x80 - sf_if_iahb_5."]
    pub sf_if_iahb_5: SF_IF_IAHB_5,
    #[doc = "0x84 - sf_if_iahb_6."]
    pub sf_if_iahb_6: SF_IF_IAHB_6,
    #[doc = "0x88 - sf_if_iahb_7."]
    pub sf_if_iahb_7: SF_IF_IAHB_7,
    _reserved35: [u8; 0x74],
    #[doc = "0x100 - sf_ctrl_prot_en_rd."]
    pub sf_ctrl_prot_en_rd: SF_CTRL_PROT_EN_RD,
    #[doc = "0x104 - sf_ctrl_prot_en."]
    pub sf_ctrl_prot_en: SF_CTRL_PROT_EN,
    _reserved37: [u8; 0xf8],
    #[doc = "0x200 - sf_aes_key_r0_0."]
    pub sf_aes_key_r0_0: SF_AES_KEY_R0_0,
    #[doc = "0x204 - sf_aes_key_r0_1."]
    pub sf_aes_key_r0_1: SF_AES_KEY_R0_1,
    #[doc = "0x208 - sf_aes_key_r0_2."]
    pub sf_aes_key_r0_2: SF_AES_KEY_R0_2,
    #[doc = "0x20c - sf_aes_key_r0_3."]
    pub sf_aes_key_r0_3: SF_AES_KEY_R0_3,
    #[doc = "0x210 - sf_aes_key_r0_4."]
    pub sf_aes_key_r0_4: SF_AES_KEY_R0_4,
    #[doc = "0x214 - sf_aes_key_r0_5."]
    pub sf_aes_key_r0_5: SF_AES_KEY_R0_5,
    #[doc = "0x218 - sf_aes_key_r0_6."]
    pub sf_aes_key_r0_6: SF_AES_KEY_R0_6,
    #[doc = "0x21c - sf_aes_key_r0_7."]
    pub sf_aes_key_r0_7: SF_AES_KEY_R0_7,
    #[doc = "0x220 - sf_aes_iv_r0_w0."]
    pub sf_aes_iv_r0_w0: SF_AES_IV_R0_W0,
    #[doc = "0x224 - sf_aes_iv_r0_w1."]
    pub sf_aes_iv_r0_w1: SF_AES_IV_R0_W1,
    #[doc = "0x228 - sf_aes_iv_r0_w2."]
    pub sf_aes_iv_r0_w2: SF_AES_IV_R0_W2,
    #[doc = "0x22c - sf_aes_iv_r0_w3."]
    pub sf_aes_iv_r0_w3: SF_AES_IV_R0_W3,
    #[doc = "0x230 - sf_aes_cfg_r0."]
    pub sf_aes_cfg_r0: SF_AES_CFG_R0,
    _reserved50: [u8; 0xcc],
    #[doc = "0x300 - sf_aes_key_r1_0."]
    pub sf_aes_key_r1_0: SF_AES_KEY_R1_0,
    #[doc = "0x304 - sf_aes_key_r1_1."]
    pub sf_aes_key_r1_1: SF_AES_KEY_R1_1,
    #[doc = "0x308 - sf_aes_key_r1_2."]
    pub sf_aes_key_r1_2: SF_AES_KEY_R1_2,
    #[doc = "0x30c - sf_aes_key_r1_3."]
    pub sf_aes_key_r1_3: SF_AES_KEY_R1_3,
    #[doc = "0x310 - sf_aes_key_r1_4."]
    pub sf_aes_key_r1_4: SF_AES_KEY_R1_4,
    #[doc = "0x314 - sf_aes_key_r1_5."]
    pub sf_aes_key_r1_5: SF_AES_KEY_R1_5,
    #[doc = "0x318 - sf_aes_key_r1_6."]
    pub sf_aes_key_r1_6: SF_AES_KEY_R1_6,
    #[doc = "0x31c - sf_aes_key_r1_7."]
    pub sf_aes_key_r1_7: SF_AES_KEY_R1_7,
    #[doc = "0x320 - sf_aes_iv_r1_w0."]
    pub sf_aes_iv_r1_w0: SF_AES_IV_R1_W0,
    #[doc = "0x324 - sf_aes_iv_r1_w1."]
    pub sf_aes_iv_r1_w1: SF_AES_IV_R1_W1,
    #[doc = "0x328 - sf_aes_iv_r1_w2."]
    pub sf_aes_iv_r1_w2: SF_AES_IV_R1_W2,
    #[doc = "0x32c - sf_aes_iv_r1_w3."]
    pub sf_aes_iv_r1_w3: SF_AES_IV_R1_W3,
    #[doc = "0x330 - sf_aes_r1."]
    pub sf_aes_r1: SF_AES_R1,
    _reserved63: [u8; 0xcc],
    #[doc = "0x400 - sf_aes_key_r2_0."]
    pub sf_aes_key_r2_0: SF_AES_KEY_R2_0,
    #[doc = "0x404 - sf_aes_key_r2_1."]
    pub sf_aes_key_r2_1: SF_AES_KEY_R2_1,
    #[doc = "0x408 - sf_aes_key_r2_2."]
    pub sf_aes_key_r2_2: SF_AES_KEY_R2_2,
    #[doc = "0x40c - sf_aes_key_r2_3."]
    pub sf_aes_key_r2_3: SF_AES_KEY_R2_3,
    #[doc = "0x410 - sf_aes_key_r2_4."]
    pub sf_aes_key_r2_4: SF_AES_KEY_R2_4,
    #[doc = "0x414 - sf_aes_key_r2_5."]
    pub sf_aes_key_r2_5: SF_AES_KEY_R2_5,
    #[doc = "0x418 - sf_aes_key_r2_6."]
    pub sf_aes_key_r2_6: SF_AES_KEY_R2_6,
    #[doc = "0x41c - sf_aes_key_r2_7."]
    pub sf_aes_key_r2_7: SF_AES_KEY_R2_7,
    #[doc = "0x420 - sf_aes_iv_r2_w0."]
    pub sf_aes_iv_r2_w0: SF_AES_IV_R2_W0,
    #[doc = "0x424 - sf_aes_iv_r2_w1."]
    pub sf_aes_iv_r2_w1: SF_AES_IV_R2_W1,
    #[doc = "0x428 - sf_aes_iv_r2_w2."]
    pub sf_aes_iv_r2_w2: SF_AES_IV_R2_W2,
    #[doc = "0x42c - sf_aes_iv_r2_w3."]
    pub sf_aes_iv_r2_w3: SF_AES_IV_R2_W3,
    #[doc = "0x430 - sf_aes_r2."]
    pub sf_aes_r2: SF_AES_R2,
    #[doc = "0x434 - sf_id0_offset."]
    pub sf_id0_offset: SF_ID0_OFFSET,
    #[doc = "0x438 - sf_id1_offset."]
    pub sf_id1_offset: SF_ID1_OFFSET,
}
#[doc = "sf_ctrl_0 (rw) register accessor: an alias for `Reg<SF_CTRL_0_SPEC>`"]
pub type SF_CTRL_0 = crate::Reg<sf_ctrl_0::SF_CTRL_0_SPEC>;
#[doc = "sf_ctrl_0."]
pub mod sf_ctrl_0;
#[doc = "sf_ctrl_1 (rw) register accessor: an alias for `Reg<SF_CTRL_1_SPEC>`"]
pub type SF_CTRL_1 = crate::Reg<sf_ctrl_1::SF_CTRL_1_SPEC>;
#[doc = "sf_ctrl_1."]
pub mod sf_ctrl_1;
#[doc = "sf_if_sahb_0 (rw) register accessor: an alias for `Reg<SF_IF_SAHB_0_SPEC>`"]
pub type SF_IF_SAHB_0 = crate::Reg<sf_if_sahb_0::SF_IF_SAHB_0_SPEC>;
#[doc = "sf_if_sahb_0."]
pub mod sf_if_sahb_0;
#[doc = "sf_if_sahb_1 (rw) register accessor: an alias for `Reg<SF_IF_SAHB_1_SPEC>`"]
pub type SF_IF_SAHB_1 = crate::Reg<sf_if_sahb_1::SF_IF_SAHB_1_SPEC>;
#[doc = "sf_if_sahb_1."]
pub mod sf_if_sahb_1;
#[doc = "sf_if_sahb_2 (rw) register accessor: an alias for `Reg<SF_IF_SAHB_2_SPEC>`"]
pub type SF_IF_SAHB_2 = crate::Reg<sf_if_sahb_2::SF_IF_SAHB_2_SPEC>;
#[doc = "sf_if_sahb_2."]
pub mod sf_if_sahb_2;
#[doc = "sf_if_iahb_0 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_0_SPEC>`"]
pub type SF_IF_IAHB_0 = crate::Reg<sf_if_iahb_0::SF_IF_IAHB_0_SPEC>;
#[doc = "sf_if_iahb_0."]
pub mod sf_if_iahb_0;
#[doc = "sf_if_iahb_1 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_1_SPEC>`"]
pub type SF_IF_IAHB_1 = crate::Reg<sf_if_iahb_1::SF_IF_IAHB_1_SPEC>;
#[doc = "sf_if_iahb_1."]
pub mod sf_if_iahb_1;
#[doc = "sf_if_iahb_2 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_2_SPEC>`"]
pub type SF_IF_IAHB_2 = crate::Reg<sf_if_iahb_2::SF_IF_IAHB_2_SPEC>;
#[doc = "sf_if_iahb_2."]
pub mod sf_if_iahb_2;
#[doc = "sf_if_status_0 (r) register accessor: an alias for `Reg<SF_IF_STATUS_0_SPEC>`"]
pub type SF_IF_STATUS_0 = crate::Reg<sf_if_status_0::SF_IF_STATUS_0_SPEC>;
#[doc = "sf_if_status_0."]
pub mod sf_if_status_0;
#[doc = "sf_if_status_1 (r) register accessor: an alias for `Reg<SF_IF_STATUS_1_SPEC>`"]
pub type SF_IF_STATUS_1 = crate::Reg<sf_if_status_1::SF_IF_STATUS_1_SPEC>;
#[doc = "sf_if_status_1."]
pub mod sf_if_status_1;
#[doc = "sf_aes (rw) register accessor: an alias for `Reg<SF_AES_SPEC>`"]
pub type SF_AES = crate::Reg<sf_aes::SF_AES_SPEC>;
#[doc = "sf_aes."]
pub mod sf_aes;
#[doc = "sf_ahb2sif_status (r) register accessor: an alias for `Reg<SF_AHB2SIF_STATUS_SPEC>`"]
pub type SF_AHB2SIF_STATUS = crate::Reg<sf_ahb2sif_status::SF_AHB2SIF_STATUS_SPEC>;
#[doc = "sf_ahb2sif_status."]
pub mod sf_ahb2sif_status;
#[doc = "sf_if_io_dly_0 (rw) register accessor: an alias for `Reg<SF_IF_IO_DLY_0_SPEC>`"]
pub type SF_IF_IO_DLY_0 = crate::Reg<sf_if_io_dly_0::SF_IF_IO_DLY_0_SPEC>;
#[doc = "sf_if_io_dly_0."]
pub mod sf_if_io_dly_0;
#[doc = "sf_if_io_dly_1 (rw) register accessor: an alias for `Reg<SF_IF_IO_DLY_1_SPEC>`"]
pub type SF_IF_IO_DLY_1 = crate::Reg<sf_if_io_dly_1::SF_IF_IO_DLY_1_SPEC>;
#[doc = "sf_if_io_dly_1."]
pub mod sf_if_io_dly_1;
#[doc = "sf_if_io_dly_2 (rw) register accessor: an alias for `Reg<SF_IF_IO_DLY_2_SPEC>`"]
pub type SF_IF_IO_DLY_2 = crate::Reg<sf_if_io_dly_2::SF_IF_IO_DLY_2_SPEC>;
#[doc = "sf_if_io_dly_2."]
pub mod sf_if_io_dly_2;
#[doc = "sf_if_io_dly_3 (rw) register accessor: an alias for `Reg<SF_IF_IO_DLY_3_SPEC>`"]
pub type SF_IF_IO_DLY_3 = crate::Reg<sf_if_io_dly_3::SF_IF_IO_DLY_3_SPEC>;
#[doc = "sf_if_io_dly_3."]
pub mod sf_if_io_dly_3;
#[doc = "sf_if_io_dly_4 (rw) register accessor: an alias for `Reg<SF_IF_IO_DLY_4_SPEC>`"]
pub type SF_IF_IO_DLY_4 = crate::Reg<sf_if_io_dly_4::SF_IF_IO_DLY_4_SPEC>;
#[doc = "sf_if_io_dly_4."]
pub mod sf_if_io_dly_4;
#[doc = "sf_reserved (rw) register accessor: an alias for `Reg<SF_RESERVED_SPEC>`"]
pub type SF_RESERVED = crate::Reg<sf_reserved::SF_RESERVED_SPEC>;
#[doc = "sf_reserved."]
pub mod sf_reserved;
#[doc = "sf2_if_io_dly_0 (rw) register accessor: an alias for `Reg<SF2_IF_IO_DLY_0_SPEC>`"]
pub type SF2_IF_IO_DLY_0 = crate::Reg<sf2_if_io_dly_0::SF2_IF_IO_DLY_0_SPEC>;
#[doc = "sf2_if_io_dly_0."]
pub mod sf2_if_io_dly_0;
#[doc = "sf2_if_io_dly_1 (rw) register accessor: an alias for `Reg<SF2_IF_IO_DLY_1_SPEC>`"]
pub type SF2_IF_IO_DLY_1 = crate::Reg<sf2_if_io_dly_1::SF2_IF_IO_DLY_1_SPEC>;
#[doc = "sf2_if_io_dly_1."]
pub mod sf2_if_io_dly_1;
#[doc = "sf2_if_io_dly_2 (rw) register accessor: an alias for `Reg<SF2_IF_IO_DLY_2_SPEC>`"]
pub type SF2_IF_IO_DLY_2 = crate::Reg<sf2_if_io_dly_2::SF2_IF_IO_DLY_2_SPEC>;
#[doc = "sf2_if_io_dly_2."]
pub mod sf2_if_io_dly_2;
#[doc = "sf2_if_io_dly_3 (rw) register accessor: an alias for `Reg<SF2_IF_IO_DLY_3_SPEC>`"]
pub type SF2_IF_IO_DLY_3 = crate::Reg<sf2_if_io_dly_3::SF2_IF_IO_DLY_3_SPEC>;
#[doc = "sf2_if_io_dly_3."]
pub mod sf2_if_io_dly_3;
#[doc = "sf2_if_io_dly_4 (rw) register accessor: an alias for `Reg<SF2_IF_IO_DLY_4_SPEC>`"]
pub type SF2_IF_IO_DLY_4 = crate::Reg<sf2_if_io_dly_4::SF2_IF_IO_DLY_4_SPEC>;
#[doc = "sf2_if_io_dly_4."]
pub mod sf2_if_io_dly_4;
#[doc = "sf3_if_io_dly_0 (rw) register accessor: an alias for `Reg<SF3_IF_IO_DLY_0_SPEC>`"]
pub type SF3_IF_IO_DLY_0 = crate::Reg<sf3_if_io_dly_0::SF3_IF_IO_DLY_0_SPEC>;
#[doc = "sf3_if_io_dly_0."]
pub mod sf3_if_io_dly_0;
#[doc = "sf3_if_io_dly_1 (rw) register accessor: an alias for `Reg<SF3_IF_IO_DLY_1_SPEC>`"]
pub type SF3_IF_IO_DLY_1 = crate::Reg<sf3_if_io_dly_1::SF3_IF_IO_DLY_1_SPEC>;
#[doc = "sf3_if_io_dly_1."]
pub mod sf3_if_io_dly_1;
#[doc = "sf3_if_io_dly_2 (rw) register accessor: an alias for `Reg<SF3_IF_IO_DLY_2_SPEC>`"]
pub type SF3_IF_IO_DLY_2 = crate::Reg<sf3_if_io_dly_2::SF3_IF_IO_DLY_2_SPEC>;
#[doc = "sf3_if_io_dly_2."]
pub mod sf3_if_io_dly_2;
#[doc = "sf3_if_io_dly_3 (rw) register accessor: an alias for `Reg<SF3_IF_IO_DLY_3_SPEC>`"]
pub type SF3_IF_IO_DLY_3 = crate::Reg<sf3_if_io_dly_3::SF3_IF_IO_DLY_3_SPEC>;
#[doc = "sf3_if_io_dly_3."]
pub mod sf3_if_io_dly_3;
#[doc = "sf3_if_io_dly_4 (rw) register accessor: an alias for `Reg<SF3_IF_IO_DLY_4_SPEC>`"]
pub type SF3_IF_IO_DLY_4 = crate::Reg<sf3_if_io_dly_4::SF3_IF_IO_DLY_4_SPEC>;
#[doc = "sf3_if_io_dly_4."]
pub mod sf3_if_io_dly_4;
#[doc = "sf_ctrl_2 (rw) register accessor: an alias for `Reg<SF_CTRL_2_SPEC>`"]
pub type SF_CTRL_2 = crate::Reg<sf_ctrl_2::SF_CTRL_2_SPEC>;
#[doc = "sf_ctrl_2."]
pub mod sf_ctrl_2;
#[doc = "sf_ctrl_3 (rw) register accessor: an alias for `Reg<SF_CTRL_3_SPEC>`"]
pub type SF_CTRL_3 = crate::Reg<sf_ctrl_3::SF_CTRL_3_SPEC>;
#[doc = "sf_ctrl_3."]
pub mod sf_ctrl_3;
#[doc = "sf_if_iahb_3 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_3_SPEC>`"]
pub type SF_IF_IAHB_3 = crate::Reg<sf_if_iahb_3::SF_IF_IAHB_3_SPEC>;
#[doc = "sf_if_iahb_3."]
pub mod sf_if_iahb_3;
#[doc = "sf_if_iahb_4 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_4_SPEC>`"]
pub type SF_IF_IAHB_4 = crate::Reg<sf_if_iahb_4::SF_IF_IAHB_4_SPEC>;
#[doc = "sf_if_iahb_4."]
pub mod sf_if_iahb_4;
#[doc = "sf_if_iahb_5 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_5_SPEC>`"]
pub type SF_IF_IAHB_5 = crate::Reg<sf_if_iahb_5::SF_IF_IAHB_5_SPEC>;
#[doc = "sf_if_iahb_5."]
pub mod sf_if_iahb_5;
#[doc = "sf_if_iahb_6 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_6_SPEC>`"]
pub type SF_IF_IAHB_6 = crate::Reg<sf_if_iahb_6::SF_IF_IAHB_6_SPEC>;
#[doc = "sf_if_iahb_6."]
pub mod sf_if_iahb_6;
#[doc = "sf_if_iahb_7 (rw) register accessor: an alias for `Reg<SF_IF_IAHB_7_SPEC>`"]
pub type SF_IF_IAHB_7 = crate::Reg<sf_if_iahb_7::SF_IF_IAHB_7_SPEC>;
#[doc = "sf_if_iahb_7."]
pub mod sf_if_iahb_7;
#[doc = "sf_ctrl_prot_en_rd (r) register accessor: an alias for `Reg<SF_CTRL_PROT_EN_RD_SPEC>`"]
pub type SF_CTRL_PROT_EN_RD = crate::Reg<sf_ctrl_prot_en_rd::SF_CTRL_PROT_EN_RD_SPEC>;
#[doc = "sf_ctrl_prot_en_rd."]
pub mod sf_ctrl_prot_en_rd;
#[doc = "sf_ctrl_prot_en (rw) register accessor: an alias for `Reg<SF_CTRL_PROT_EN_SPEC>`"]
pub type SF_CTRL_PROT_EN = crate::Reg<sf_ctrl_prot_en::SF_CTRL_PROT_EN_SPEC>;
#[doc = "sf_ctrl_prot_en."]
pub mod sf_ctrl_prot_en;
#[doc = "sf_aes_key_r0_0 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_0_SPEC>`"]
pub type SF_AES_KEY_R0_0 = crate::Reg<sf_aes_key_r0_0::SF_AES_KEY_R0_0_SPEC>;
#[doc = "sf_aes_key_r0_0."]
pub mod sf_aes_key_r0_0;
#[doc = "sf_aes_key_r0_1 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_1_SPEC>`"]
pub type SF_AES_KEY_R0_1 = crate::Reg<sf_aes_key_r0_1::SF_AES_KEY_R0_1_SPEC>;
#[doc = "sf_aes_key_r0_1."]
pub mod sf_aes_key_r0_1;
#[doc = "sf_aes_key_r0_2 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_2_SPEC>`"]
pub type SF_AES_KEY_R0_2 = crate::Reg<sf_aes_key_r0_2::SF_AES_KEY_R0_2_SPEC>;
#[doc = "sf_aes_key_r0_2."]
pub mod sf_aes_key_r0_2;
#[doc = "sf_aes_key_r0_3 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_3_SPEC>`"]
pub type SF_AES_KEY_R0_3 = crate::Reg<sf_aes_key_r0_3::SF_AES_KEY_R0_3_SPEC>;
#[doc = "sf_aes_key_r0_3."]
pub mod sf_aes_key_r0_3;
#[doc = "sf_aes_key_r0_4 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_4_SPEC>`"]
pub type SF_AES_KEY_R0_4 = crate::Reg<sf_aes_key_r0_4::SF_AES_KEY_R0_4_SPEC>;
#[doc = "sf_aes_key_r0_4."]
pub mod sf_aes_key_r0_4;
#[doc = "sf_aes_key_r0_5 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_5_SPEC>`"]
pub type SF_AES_KEY_R0_5 = crate::Reg<sf_aes_key_r0_5::SF_AES_KEY_R0_5_SPEC>;
#[doc = "sf_aes_key_r0_5."]
pub mod sf_aes_key_r0_5;
#[doc = "sf_aes_key_r0_6 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_6_SPEC>`"]
pub type SF_AES_KEY_R0_6 = crate::Reg<sf_aes_key_r0_6::SF_AES_KEY_R0_6_SPEC>;
#[doc = "sf_aes_key_r0_6."]
pub mod sf_aes_key_r0_6;
#[doc = "sf_aes_key_r0_7 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R0_7_SPEC>`"]
pub type SF_AES_KEY_R0_7 = crate::Reg<sf_aes_key_r0_7::SF_AES_KEY_R0_7_SPEC>;
#[doc = "sf_aes_key_r0_7."]
pub mod sf_aes_key_r0_7;
#[doc = "sf_aes_iv_r0_w0 (rw) register accessor: an alias for `Reg<SF_AES_IV_R0_W0_SPEC>`"]
pub type SF_AES_IV_R0_W0 = crate::Reg<sf_aes_iv_r0_w0::SF_AES_IV_R0_W0_SPEC>;
#[doc = "sf_aes_iv_r0_w0."]
pub mod sf_aes_iv_r0_w0;
#[doc = "sf_aes_iv_r0_w1 (rw) register accessor: an alias for `Reg<SF_AES_IV_R0_W1_SPEC>`"]
pub type SF_AES_IV_R0_W1 = crate::Reg<sf_aes_iv_r0_w1::SF_AES_IV_R0_W1_SPEC>;
#[doc = "sf_aes_iv_r0_w1."]
pub mod sf_aes_iv_r0_w1;
#[doc = "sf_aes_iv_r0_w2 (rw) register accessor: an alias for `Reg<SF_AES_IV_R0_W2_SPEC>`"]
pub type SF_AES_IV_R0_W2 = crate::Reg<sf_aes_iv_r0_w2::SF_AES_IV_R0_W2_SPEC>;
#[doc = "sf_aes_iv_r0_w2."]
pub mod sf_aes_iv_r0_w2;
#[doc = "sf_aes_iv_r0_w3 (rw) register accessor: an alias for `Reg<SF_AES_IV_R0_W3_SPEC>`"]
pub type SF_AES_IV_R0_W3 = crate::Reg<sf_aes_iv_r0_w3::SF_AES_IV_R0_W3_SPEC>;
#[doc = "sf_aes_iv_r0_w3."]
pub mod sf_aes_iv_r0_w3;
#[doc = "sf_aes_cfg_r0 (rw) register accessor: an alias for `Reg<SF_AES_CFG_R0_SPEC>`"]
pub type SF_AES_CFG_R0 = crate::Reg<sf_aes_cfg_r0::SF_AES_CFG_R0_SPEC>;
#[doc = "sf_aes_cfg_r0."]
pub mod sf_aes_cfg_r0;
#[doc = "sf_aes_key_r1_0 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_0_SPEC>`"]
pub type SF_AES_KEY_R1_0 = crate::Reg<sf_aes_key_r1_0::SF_AES_KEY_R1_0_SPEC>;
#[doc = "sf_aes_key_r1_0."]
pub mod sf_aes_key_r1_0;
#[doc = "sf_aes_key_r1_1 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_1_SPEC>`"]
pub type SF_AES_KEY_R1_1 = crate::Reg<sf_aes_key_r1_1::SF_AES_KEY_R1_1_SPEC>;
#[doc = "sf_aes_key_r1_1."]
pub mod sf_aes_key_r1_1;
#[doc = "sf_aes_key_r1_2 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_2_SPEC>`"]
pub type SF_AES_KEY_R1_2 = crate::Reg<sf_aes_key_r1_2::SF_AES_KEY_R1_2_SPEC>;
#[doc = "sf_aes_key_r1_2."]
pub mod sf_aes_key_r1_2;
#[doc = "sf_aes_key_r1_3 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_3_SPEC>`"]
pub type SF_AES_KEY_R1_3 = crate::Reg<sf_aes_key_r1_3::SF_AES_KEY_R1_3_SPEC>;
#[doc = "sf_aes_key_r1_3."]
pub mod sf_aes_key_r1_3;
#[doc = "sf_aes_key_r1_4 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_4_SPEC>`"]
pub type SF_AES_KEY_R1_4 = crate::Reg<sf_aes_key_r1_4::SF_AES_KEY_R1_4_SPEC>;
#[doc = "sf_aes_key_r1_4."]
pub mod sf_aes_key_r1_4;
#[doc = "sf_aes_key_r1_5 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_5_SPEC>`"]
pub type SF_AES_KEY_R1_5 = crate::Reg<sf_aes_key_r1_5::SF_AES_KEY_R1_5_SPEC>;
#[doc = "sf_aes_key_r1_5."]
pub mod sf_aes_key_r1_5;
#[doc = "sf_aes_key_r1_6 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_6_SPEC>`"]
pub type SF_AES_KEY_R1_6 = crate::Reg<sf_aes_key_r1_6::SF_AES_KEY_R1_6_SPEC>;
#[doc = "sf_aes_key_r1_6."]
pub mod sf_aes_key_r1_6;
#[doc = "sf_aes_key_r1_7 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R1_7_SPEC>`"]
pub type SF_AES_KEY_R1_7 = crate::Reg<sf_aes_key_r1_7::SF_AES_KEY_R1_7_SPEC>;
#[doc = "sf_aes_key_r1_7."]
pub mod sf_aes_key_r1_7;
#[doc = "sf_aes_iv_r1_w0 (rw) register accessor: an alias for `Reg<SF_AES_IV_R1_W0_SPEC>`"]
pub type SF_AES_IV_R1_W0 = crate::Reg<sf_aes_iv_r1_w0::SF_AES_IV_R1_W0_SPEC>;
#[doc = "sf_aes_iv_r1_w0."]
pub mod sf_aes_iv_r1_w0;
#[doc = "sf_aes_iv_r1_w1 (rw) register accessor: an alias for `Reg<SF_AES_IV_R1_W1_SPEC>`"]
pub type SF_AES_IV_R1_W1 = crate::Reg<sf_aes_iv_r1_w1::SF_AES_IV_R1_W1_SPEC>;
#[doc = "sf_aes_iv_r1_w1."]
pub mod sf_aes_iv_r1_w1;
#[doc = "sf_aes_iv_r1_w2 (rw) register accessor: an alias for `Reg<SF_AES_IV_R1_W2_SPEC>`"]
pub type SF_AES_IV_R1_W2 = crate::Reg<sf_aes_iv_r1_w2::SF_AES_IV_R1_W2_SPEC>;
#[doc = "sf_aes_iv_r1_w2."]
pub mod sf_aes_iv_r1_w2;
#[doc = "sf_aes_iv_r1_w3 (rw) register accessor: an alias for `Reg<SF_AES_IV_R1_W3_SPEC>`"]
pub type SF_AES_IV_R1_W3 = crate::Reg<sf_aes_iv_r1_w3::SF_AES_IV_R1_W3_SPEC>;
#[doc = "sf_aes_iv_r1_w3."]
pub mod sf_aes_iv_r1_w3;
#[doc = "sf_aes_r1 (rw) register accessor: an alias for `Reg<SF_AES_R1_SPEC>`"]
pub type SF_AES_R1 = crate::Reg<sf_aes_r1::SF_AES_R1_SPEC>;
#[doc = "sf_aes_r1."]
pub mod sf_aes_r1;
#[doc = "sf_aes_key_r2_0 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_0_SPEC>`"]
pub type SF_AES_KEY_R2_0 = crate::Reg<sf_aes_key_r2_0::SF_AES_KEY_R2_0_SPEC>;
#[doc = "sf_aes_key_r2_0."]
pub mod sf_aes_key_r2_0;
#[doc = "sf_aes_key_r2_1 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_1_SPEC>`"]
pub type SF_AES_KEY_R2_1 = crate::Reg<sf_aes_key_r2_1::SF_AES_KEY_R2_1_SPEC>;
#[doc = "sf_aes_key_r2_1."]
pub mod sf_aes_key_r2_1;
#[doc = "sf_aes_key_r2_2 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_2_SPEC>`"]
pub type SF_AES_KEY_R2_2 = crate::Reg<sf_aes_key_r2_2::SF_AES_KEY_R2_2_SPEC>;
#[doc = "sf_aes_key_r2_2."]
pub mod sf_aes_key_r2_2;
#[doc = "sf_aes_key_r2_3 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_3_SPEC>`"]
pub type SF_AES_KEY_R2_3 = crate::Reg<sf_aes_key_r2_3::SF_AES_KEY_R2_3_SPEC>;
#[doc = "sf_aes_key_r2_3."]
pub mod sf_aes_key_r2_3;
#[doc = "sf_aes_key_r2_4 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_4_SPEC>`"]
pub type SF_AES_KEY_R2_4 = crate::Reg<sf_aes_key_r2_4::SF_AES_KEY_R2_4_SPEC>;
#[doc = "sf_aes_key_r2_4."]
pub mod sf_aes_key_r2_4;
#[doc = "sf_aes_key_r2_5 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_5_SPEC>`"]
pub type SF_AES_KEY_R2_5 = crate::Reg<sf_aes_key_r2_5::SF_AES_KEY_R2_5_SPEC>;
#[doc = "sf_aes_key_r2_5."]
pub mod sf_aes_key_r2_5;
#[doc = "sf_aes_key_r2_6 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_6_SPEC>`"]
pub type SF_AES_KEY_R2_6 = crate::Reg<sf_aes_key_r2_6::SF_AES_KEY_R2_6_SPEC>;
#[doc = "sf_aes_key_r2_6."]
pub mod sf_aes_key_r2_6;
#[doc = "sf_aes_key_r2_7 (rw) register accessor: an alias for `Reg<SF_AES_KEY_R2_7_SPEC>`"]
pub type SF_AES_KEY_R2_7 = crate::Reg<sf_aes_key_r2_7::SF_AES_KEY_R2_7_SPEC>;
#[doc = "sf_aes_key_r2_7."]
pub mod sf_aes_key_r2_7;
#[doc = "sf_aes_iv_r2_w0 (rw) register accessor: an alias for `Reg<SF_AES_IV_R2_W0_SPEC>`"]
pub type SF_AES_IV_R2_W0 = crate::Reg<sf_aes_iv_r2_w0::SF_AES_IV_R2_W0_SPEC>;
#[doc = "sf_aes_iv_r2_w0."]
pub mod sf_aes_iv_r2_w0;
#[doc = "sf_aes_iv_r2_w1 (rw) register accessor: an alias for `Reg<SF_AES_IV_R2_W1_SPEC>`"]
pub type SF_AES_IV_R2_W1 = crate::Reg<sf_aes_iv_r2_w1::SF_AES_IV_R2_W1_SPEC>;
#[doc = "sf_aes_iv_r2_w1."]
pub mod sf_aes_iv_r2_w1;
#[doc = "sf_aes_iv_r2_w2 (rw) register accessor: an alias for `Reg<SF_AES_IV_R2_W2_SPEC>`"]
pub type SF_AES_IV_R2_W2 = crate::Reg<sf_aes_iv_r2_w2::SF_AES_IV_R2_W2_SPEC>;
#[doc = "sf_aes_iv_r2_w2."]
pub mod sf_aes_iv_r2_w2;
#[doc = "sf_aes_iv_r2_w3 (rw) register accessor: an alias for `Reg<SF_AES_IV_R2_W3_SPEC>`"]
pub type SF_AES_IV_R2_W3 = crate::Reg<sf_aes_iv_r2_w3::SF_AES_IV_R2_W3_SPEC>;
#[doc = "sf_aes_iv_r2_w3."]
pub mod sf_aes_iv_r2_w3;
#[doc = "sf_aes_r2 (rw) register accessor: an alias for `Reg<SF_AES_R2_SPEC>`"]
pub type SF_AES_R2 = crate::Reg<sf_aes_r2::SF_AES_R2_SPEC>;
#[doc = "sf_aes_r2."]
pub mod sf_aes_r2;
#[doc = "sf_id0_offset (rw) register accessor: an alias for `Reg<SF_ID0_OFFSET_SPEC>`"]
pub type SF_ID0_OFFSET = crate::Reg<sf_id0_offset::SF_ID0_OFFSET_SPEC>;
#[doc = "sf_id0_offset."]
pub mod sf_id0_offset;
#[doc = "sf_id1_offset (rw) register accessor: an alias for `Reg<SF_ID1_OFFSET_SPEC>`"]
pub type SF_ID1_OFFSET = crate::Reg<sf_id1_offset::SF_ID1_OFFSET_SPEC>;
#[doc = "sf_id1_offset."]
pub mod sf_id1_offset;

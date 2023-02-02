#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - se_sha_0_ctrl."]
    pub se_sha_0_ctrl: SE_SHA_0_CTRL,
    #[doc = "0x04 - se_sha_0_msa."]
    pub se_sha_0_msa: SE_SHA_0_MSA,
    #[doc = "0x08 - se_sha_0_status."]
    pub se_sha_0_status: SE_SHA_0_STATUS,
    #[doc = "0x0c - se_sha_0_endian."]
    pub se_sha_0_endian: SE_SHA_0_ENDIAN,
    #[doc = "0x10 - se_sha_0_hash_l_0."]
    pub se_sha_0_hash_l_0: SE_SHA_0_HASH_L_0,
    #[doc = "0x14 - se_sha_0_hash_l_1."]
    pub se_sha_0_hash_l_1: SE_SHA_0_HASH_L_1,
    #[doc = "0x18 - se_sha_0_hash_l_2."]
    pub se_sha_0_hash_l_2: SE_SHA_0_HASH_L_2,
    #[doc = "0x1c - se_sha_0_hash_l_3."]
    pub se_sha_0_hash_l_3: SE_SHA_0_HASH_L_3,
    #[doc = "0x20 - se_sha_0_hash_l_4."]
    pub se_sha_0_hash_l_4: SE_SHA_0_HASH_L_4,
    #[doc = "0x24 - se_sha_0_hash_l_5."]
    pub se_sha_0_hash_l_5: SE_SHA_0_HASH_L_5,
    #[doc = "0x28 - se_sha_0_hash_l_6."]
    pub se_sha_0_hash_l_6: SE_SHA_0_HASH_L_6,
    #[doc = "0x2c - se_sha_0_hash_l_7."]
    pub se_sha_0_hash_l_7: SE_SHA_0_HASH_L_7,
    #[doc = "0x30 - se_sha_0_hash_h_0."]
    pub se_sha_0_hash_h_0: SE_SHA_0_HASH_H_0,
    #[doc = "0x34 - se_sha_0_hash_h_1."]
    pub se_sha_0_hash_h_1: SE_SHA_0_HASH_H_1,
    #[doc = "0x38 - se_sha_0_hash_h_2."]
    pub se_sha_0_hash_h_2: SE_SHA_0_HASH_H_2,
    #[doc = "0x3c - se_sha_0_hash_h_3."]
    pub se_sha_0_hash_h_3: SE_SHA_0_HASH_H_3,
    #[doc = "0x40 - se_sha_0_hash_h_4."]
    pub se_sha_0_hash_h_4: SE_SHA_0_HASH_H_4,
    #[doc = "0x44 - se_sha_0_hash_h_5."]
    pub se_sha_0_hash_h_5: SE_SHA_0_HASH_H_5,
    #[doc = "0x48 - se_sha_0_hash_h_6."]
    pub se_sha_0_hash_h_6: SE_SHA_0_HASH_H_6,
    #[doc = "0x4c - se_sha_0_hash_h_7."]
    pub se_sha_0_hash_h_7: SE_SHA_0_HASH_H_7,
    #[doc = "0x50 - se_sha_0_link."]
    pub se_sha_0_link: SE_SHA_0_LINK,
    _reserved21: [u8; 0xa8],
    #[doc = "0xfc - se_sha_0_ctrl_prot."]
    pub se_sha_0_ctrl_prot: SE_SHA_0_CTRL_PROT,
    #[doc = "0x100 - se_aes_0_ctrl."]
    pub se_aes_0_ctrl: SE_AES_0_CTRL,
    #[doc = "0x104 - se_aes_0_msa."]
    pub se_aes_0_msa: SE_AES_0_MSA,
    #[doc = "0x108 - se_aes_0_mda."]
    pub se_aes_0_mda: SE_AES_0_MDA,
    #[doc = "0x10c - se_aes_0_status."]
    pub se_aes_0_status: SE_AES_0_STATUS,
    #[doc = "0x110 - se_aes_0_iv_0."]
    pub se_aes_0_iv_0: SE_AES_0_IV_0,
    #[doc = "0x114 - se_aes_0_iv_1."]
    pub se_aes_0_iv_1: SE_AES_0_IV_1,
    #[doc = "0x118 - se_aes_0_iv_2."]
    pub se_aes_0_iv_2: SE_AES_0_IV_2,
    #[doc = "0x11c - se_aes_0_iv_3."]
    pub se_aes_0_iv_3: SE_AES_0_IV_3,
    #[doc = "0x120 - se_aes_0_key_0."]
    pub se_aes_0_key_0: SE_AES_0_KEY_0,
    #[doc = "0x124 - se_aes_0_key_1."]
    pub se_aes_0_key_1: SE_AES_0_KEY_1,
    #[doc = "0x128 - se_aes_0_key_2."]
    pub se_aes_0_key_2: SE_AES_0_KEY_2,
    #[doc = "0x12c - se_aes_0_key_3."]
    pub se_aes_0_key_3: SE_AES_0_KEY_3,
    #[doc = "0x130 - se_aes_0_key_4."]
    pub se_aes_0_key_4: SE_AES_0_KEY_4,
    #[doc = "0x134 - se_aes_0_key_5."]
    pub se_aes_0_key_5: SE_AES_0_KEY_5,
    #[doc = "0x138 - se_aes_0_key_6."]
    pub se_aes_0_key_6: SE_AES_0_KEY_6,
    #[doc = "0x13c - se_aes_0_key_7."]
    pub se_aes_0_key_7: SE_AES_0_KEY_7,
    #[doc = "0x140 - se_aes_0_key_sel_0."]
    pub se_aes_0_key_sel_0: SE_AES_0_KEY_SEL_0,
    #[doc = "0x144 - se_aes_0_key_sel_1."]
    pub se_aes_0_key_sel_1: SE_AES_0_KEY_SEL_1,
    #[doc = "0x148 - se_aes_0_endian."]
    pub se_aes_0_endian: SE_AES_0_ENDIAN,
    #[doc = "0x14c - se_aes_0_sboot."]
    pub se_aes_0_sboot: SE_AES_0_SBOOT,
    #[doc = "0x150 - se_aes_0_link."]
    pub se_aes_0_link: SE_AES_0_LINK,
    _reserved43: [u8; 0xa8],
    #[doc = "0x1fc - se_aes_0_ctrl_prot."]
    pub se_aes_0_ctrl_prot: SE_AES_0_CTRL_PROT,
    #[doc = "0x200 - se_trng_0_ctrl_0."]
    pub se_trng_0_ctrl_0: SE_TRNG_0_CTRL_0,
    #[doc = "0x204 - se_trng_0_status."]
    pub se_trng_0_status: SE_TRNG_0_STATUS,
    #[doc = "0x208 - se_trng_0_dout_0."]
    pub se_trng_0_dout_0: SE_TRNG_0_DOUT_0,
    #[doc = "0x20c - se_trng_0_dout_1."]
    pub se_trng_0_dout_1: SE_TRNG_0_DOUT_1,
    #[doc = "0x210 - se_trng_0_dout_2."]
    pub se_trng_0_dout_2: SE_TRNG_0_DOUT_2,
    #[doc = "0x214 - se_trng_0_dout_3."]
    pub se_trng_0_dout_3: SE_TRNG_0_DOUT_3,
    #[doc = "0x218 - se_trng_0_dout_4."]
    pub se_trng_0_dout_4: SE_TRNG_0_DOUT_4,
    #[doc = "0x21c - se_trng_0_dout_5."]
    pub se_trng_0_dout_5: SE_TRNG_0_DOUT_5,
    #[doc = "0x220 - se_trng_0_dout_6."]
    pub se_trng_0_dout_6: SE_TRNG_0_DOUT_6,
    #[doc = "0x224 - se_trng_0_dout_7."]
    pub se_trng_0_dout_7: SE_TRNG_0_DOUT_7,
    #[doc = "0x228 - se_trng_0_test."]
    pub se_trng_0_test: SE_TRNG_0_TEST,
    #[doc = "0x22c - se_trng_0_ctrl_1."]
    pub se_trng_0_ctrl_1: SE_TRNG_0_CTRL_1,
    #[doc = "0x230 - se_trng_0_ctrl_2."]
    pub se_trng_0_ctrl_2: SE_TRNG_0_CTRL_2,
    #[doc = "0x234 - se_trng_0_ctrl_3."]
    pub se_trng_0_ctrl_3: SE_TRNG_0_CTRL_3,
    _reserved58: [u8; 0x08],
    #[doc = "0x240 - se_trng_0_test_out_0."]
    pub se_trng_0_test_out_0: SE_TRNG_0_TEST_OUT_0,
    #[doc = "0x244 - se_trng_0_test_out_1."]
    pub se_trng_0_test_out_1: SE_TRNG_0_TEST_OUT_1,
    #[doc = "0x248 - se_trng_0_test_out_2."]
    pub se_trng_0_test_out_2: SE_TRNG_0_TEST_OUT_2,
    #[doc = "0x24c - se_trng_0_test_out_3."]
    pub se_trng_0_test_out_3: SE_TRNG_0_TEST_OUT_3,
    _reserved62: [u8; 0xac],
    #[doc = "0x2fc - se_trng_0_ctrl_prot."]
    pub se_trng_0_ctrl_prot: SE_TRNG_0_CTRL_PROT,
    #[doc = "0x300 - se_pka_0_ctrl_0."]
    pub se_pka_0_ctrl_0: SE_PKA_0_CTRL_0,
    _reserved64: [u8; 0x08],
    #[doc = "0x30c - se_pka_0_seed."]
    pub se_pka_0_seed: SE_PKA_0_SEED,
    #[doc = "0x310 - se_pka_0_ctrl_1."]
    pub se_pka_0_ctrl_1: SE_PKA_0_CTRL_1,
    _reserved66: [u8; 0x2c],
    #[doc = "0x340 - se_pka_0_rw."]
    pub se_pka_0_rw: SE_PKA_0_RW,
    _reserved67: [u8; 0x1c],
    #[doc = "0x360 - se_pka_0_rw_burst."]
    pub se_pka_0_rw_burst: SE_PKA_0_RW_BURST,
    _reserved68: [u8; 0x98],
    #[doc = "0x3fc - se_pka_0_ctrl_prot."]
    pub se_pka_0_ctrl_prot: SE_PKA_0_CTRL_PROT,
    #[doc = "0x400 - se_cdet_0_ctrl_0."]
    pub se_cdet_0_ctrl_0: SE_CDET_0_CTRL_0,
    #[doc = "0x404 - se_cdet_0_ctrl_1."]
    pub se_cdet_0_ctrl_1: SE_CDET_0_CTRL_1,
    _reserved71: [u8; 0xf4],
    #[doc = "0x4fc - se_cdet_0_ctrl_prot."]
    pub se_cdet_0_ctrl_prot: SE_CDET_0_CTRL_PROT,
    #[doc = "0x500 - se_gmac_0_ctrl_0."]
    pub se_gmac_0_ctrl_0: SE_GMAC_0_CTRL_0,
    #[doc = "0x504 - se_gmac_0_lca."]
    pub se_gmac_0_lca: SE_GMAC_0_LCA,
    #[doc = "0x508 - se_gmac_0_status."]
    pub se_gmac_0_status: SE_GMAC_0_STATUS,
    _reserved75: [u8; 0xf0],
    #[doc = "0x5fc - se_gmac_0_ctrl_prot."]
    pub se_gmac_0_ctrl_prot: SE_GMAC_0_CTRL_PROT,
    _reserved76: [u8; 0x0900],
    #[doc = "0xf00 - se_ctrl_prot_rd."]
    pub se_ctrl_prot_rd: SE_CTRL_PROT_RD,
    #[doc = "0xf04 - se_ctrl_reserved_0."]
    pub se_ctrl_reserved_0: SE_CTRL_RESERVED_0,
    #[doc = "0xf08 - se_ctrl_reserved_1."]
    pub se_ctrl_reserved_1: SE_CTRL_RESERVED_1,
    #[doc = "0xf0c - se_ctrl_reserved_2."]
    pub se_ctrl_reserved_2: SE_CTRL_RESERVED_2,
}
#[doc = "se_sha_0_ctrl (rw) register accessor: an alias for `Reg<SE_SHA_0_CTRL_SPEC>`"]
pub type SE_SHA_0_CTRL = crate::Reg<se_sha_0_ctrl::SE_SHA_0_CTRL_SPEC>;
#[doc = "se_sha_0_ctrl."]
pub mod se_sha_0_ctrl;
#[doc = "se_sha_0_msa (rw) register accessor: an alias for `Reg<SE_SHA_0_MSA_SPEC>`"]
pub type SE_SHA_0_MSA = crate::Reg<se_sha_0_msa::SE_SHA_0_MSA_SPEC>;
#[doc = "se_sha_0_msa."]
pub mod se_sha_0_msa;
#[doc = "se_sha_0_status (r) register accessor: an alias for `Reg<SE_SHA_0_STATUS_SPEC>`"]
pub type SE_SHA_0_STATUS = crate::Reg<se_sha_0_status::SE_SHA_0_STATUS_SPEC>;
#[doc = "se_sha_0_status."]
pub mod se_sha_0_status;
#[doc = "se_sha_0_endian (rw) register accessor: an alias for `Reg<SE_SHA_0_ENDIAN_SPEC>`"]
pub type SE_SHA_0_ENDIAN = crate::Reg<se_sha_0_endian::SE_SHA_0_ENDIAN_SPEC>;
#[doc = "se_sha_0_endian."]
pub mod se_sha_0_endian;
#[doc = "se_sha_0_hash_l_0 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_L_0_SPEC>`"]
pub type SE_SHA_0_HASH_L_0 = crate::Reg<se_sha_0_hash_l_0::SE_SHA_0_HASH_L_0_SPEC>;
#[doc = "se_sha_0_hash_l_0."]
pub mod se_sha_0_hash_l_0;
#[doc = "se_sha_0_hash_l_1 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_L_1_SPEC>`"]
pub type SE_SHA_0_HASH_L_1 = crate::Reg<se_sha_0_hash_l_1::SE_SHA_0_HASH_L_1_SPEC>;
#[doc = "se_sha_0_hash_l_1."]
pub mod se_sha_0_hash_l_1;
#[doc = "se_sha_0_hash_l_2 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_L_2_SPEC>`"]
pub type SE_SHA_0_HASH_L_2 = crate::Reg<se_sha_0_hash_l_2::SE_SHA_0_HASH_L_2_SPEC>;
#[doc = "se_sha_0_hash_l_2."]
pub mod se_sha_0_hash_l_2;
#[doc = "se_sha_0_hash_l_3 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_L_3_SPEC>`"]
pub type SE_SHA_0_HASH_L_3 = crate::Reg<se_sha_0_hash_l_3::SE_SHA_0_HASH_L_3_SPEC>;
#[doc = "se_sha_0_hash_l_3."]
pub mod se_sha_0_hash_l_3;
#[doc = "se_sha_0_hash_l_4 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_L_4_SPEC>`"]
pub type SE_SHA_0_HASH_L_4 = crate::Reg<se_sha_0_hash_l_4::SE_SHA_0_HASH_L_4_SPEC>;
#[doc = "se_sha_0_hash_l_4."]
pub mod se_sha_0_hash_l_4;
#[doc = "se_sha_0_hash_l_5 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_L_5_SPEC>`"]
pub type SE_SHA_0_HASH_L_5 = crate::Reg<se_sha_0_hash_l_5::SE_SHA_0_HASH_L_5_SPEC>;
#[doc = "se_sha_0_hash_l_5."]
pub mod se_sha_0_hash_l_5;
#[doc = "se_sha_0_hash_l_6 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_L_6_SPEC>`"]
pub type SE_SHA_0_HASH_L_6 = crate::Reg<se_sha_0_hash_l_6::SE_SHA_0_HASH_L_6_SPEC>;
#[doc = "se_sha_0_hash_l_6."]
pub mod se_sha_0_hash_l_6;
#[doc = "se_sha_0_hash_l_7 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_L_7_SPEC>`"]
pub type SE_SHA_0_HASH_L_7 = crate::Reg<se_sha_0_hash_l_7::SE_SHA_0_HASH_L_7_SPEC>;
#[doc = "se_sha_0_hash_l_7."]
pub mod se_sha_0_hash_l_7;
#[doc = "se_sha_0_hash_h_0 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_H_0_SPEC>`"]
pub type SE_SHA_0_HASH_H_0 = crate::Reg<se_sha_0_hash_h_0::SE_SHA_0_HASH_H_0_SPEC>;
#[doc = "se_sha_0_hash_h_0."]
pub mod se_sha_0_hash_h_0;
#[doc = "se_sha_0_hash_h_1 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_H_1_SPEC>`"]
pub type SE_SHA_0_HASH_H_1 = crate::Reg<se_sha_0_hash_h_1::SE_SHA_0_HASH_H_1_SPEC>;
#[doc = "se_sha_0_hash_h_1."]
pub mod se_sha_0_hash_h_1;
#[doc = "se_sha_0_hash_h_2 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_H_2_SPEC>`"]
pub type SE_SHA_0_HASH_H_2 = crate::Reg<se_sha_0_hash_h_2::SE_SHA_0_HASH_H_2_SPEC>;
#[doc = "se_sha_0_hash_h_2."]
pub mod se_sha_0_hash_h_2;
#[doc = "se_sha_0_hash_h_3 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_H_3_SPEC>`"]
pub type SE_SHA_0_HASH_H_3 = crate::Reg<se_sha_0_hash_h_3::SE_SHA_0_HASH_H_3_SPEC>;
#[doc = "se_sha_0_hash_h_3."]
pub mod se_sha_0_hash_h_3;
#[doc = "se_sha_0_hash_h_4 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_H_4_SPEC>`"]
pub type SE_SHA_0_HASH_H_4 = crate::Reg<se_sha_0_hash_h_4::SE_SHA_0_HASH_H_4_SPEC>;
#[doc = "se_sha_0_hash_h_4."]
pub mod se_sha_0_hash_h_4;
#[doc = "se_sha_0_hash_h_5 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_H_5_SPEC>`"]
pub type SE_SHA_0_HASH_H_5 = crate::Reg<se_sha_0_hash_h_5::SE_SHA_0_HASH_H_5_SPEC>;
#[doc = "se_sha_0_hash_h_5."]
pub mod se_sha_0_hash_h_5;
#[doc = "se_sha_0_hash_h_6 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_H_6_SPEC>`"]
pub type SE_SHA_0_HASH_H_6 = crate::Reg<se_sha_0_hash_h_6::SE_SHA_0_HASH_H_6_SPEC>;
#[doc = "se_sha_0_hash_h_6."]
pub mod se_sha_0_hash_h_6;
#[doc = "se_sha_0_hash_h_7 (r) register accessor: an alias for `Reg<SE_SHA_0_HASH_H_7_SPEC>`"]
pub type SE_SHA_0_HASH_H_7 = crate::Reg<se_sha_0_hash_h_7::SE_SHA_0_HASH_H_7_SPEC>;
#[doc = "se_sha_0_hash_h_7."]
pub mod se_sha_0_hash_h_7;
#[doc = "se_sha_0_link (rw) register accessor: an alias for `Reg<SE_SHA_0_LINK_SPEC>`"]
pub type SE_SHA_0_LINK = crate::Reg<se_sha_0_link::SE_SHA_0_LINK_SPEC>;
#[doc = "se_sha_0_link."]
pub mod se_sha_0_link;
#[doc = "se_sha_0_ctrl_prot (rw) register accessor: an alias for `Reg<SE_SHA_0_CTRL_PROT_SPEC>`"]
pub type SE_SHA_0_CTRL_PROT = crate::Reg<se_sha_0_ctrl_prot::SE_SHA_0_CTRL_PROT_SPEC>;
#[doc = "se_sha_0_ctrl_prot."]
pub mod se_sha_0_ctrl_prot;
#[doc = "se_aes_0_ctrl (rw) register accessor: an alias for `Reg<SE_AES_0_CTRL_SPEC>`"]
pub type SE_AES_0_CTRL = crate::Reg<se_aes_0_ctrl::SE_AES_0_CTRL_SPEC>;
#[doc = "se_aes_0_ctrl."]
pub mod se_aes_0_ctrl;
#[doc = "se_aes_0_msa (rw) register accessor: an alias for `Reg<SE_AES_0_MSA_SPEC>`"]
pub type SE_AES_0_MSA = crate::Reg<se_aes_0_msa::SE_AES_0_MSA_SPEC>;
#[doc = "se_aes_0_msa."]
pub mod se_aes_0_msa;
#[doc = "se_aes_0_mda (rw) register accessor: an alias for `Reg<SE_AES_0_MDA_SPEC>`"]
pub type SE_AES_0_MDA = crate::Reg<se_aes_0_mda::SE_AES_0_MDA_SPEC>;
#[doc = "se_aes_0_mda."]
pub mod se_aes_0_mda;
#[doc = "se_aes_0_status (r) register accessor: an alias for `Reg<SE_AES_0_STATUS_SPEC>`"]
pub type SE_AES_0_STATUS = crate::Reg<se_aes_0_status::SE_AES_0_STATUS_SPEC>;
#[doc = "se_aes_0_status."]
pub mod se_aes_0_status;
#[doc = "se_aes_0_iv_0 (rw) register accessor: an alias for `Reg<SE_AES_0_IV_0_SPEC>`"]
pub type SE_AES_0_IV_0 = crate::Reg<se_aes_0_iv_0::SE_AES_0_IV_0_SPEC>;
#[doc = "se_aes_0_iv_0."]
pub mod se_aes_0_iv_0;
#[doc = "se_aes_0_iv_1 (rw) register accessor: an alias for `Reg<SE_AES_0_IV_1_SPEC>`"]
pub type SE_AES_0_IV_1 = crate::Reg<se_aes_0_iv_1::SE_AES_0_IV_1_SPEC>;
#[doc = "se_aes_0_iv_1."]
pub mod se_aes_0_iv_1;
#[doc = "se_aes_0_iv_2 (rw) register accessor: an alias for `Reg<SE_AES_0_IV_2_SPEC>`"]
pub type SE_AES_0_IV_2 = crate::Reg<se_aes_0_iv_2::SE_AES_0_IV_2_SPEC>;
#[doc = "se_aes_0_iv_2."]
pub mod se_aes_0_iv_2;
#[doc = "se_aes_0_iv_3 (rw) register accessor: an alias for `Reg<SE_AES_0_IV_3_SPEC>`"]
pub type SE_AES_0_IV_3 = crate::Reg<se_aes_0_iv_3::SE_AES_0_IV_3_SPEC>;
#[doc = "se_aes_0_iv_3."]
pub mod se_aes_0_iv_3;
#[doc = "se_aes_0_key_0 (rw) register accessor: an alias for `Reg<SE_AES_0_KEY_0_SPEC>`"]
pub type SE_AES_0_KEY_0 = crate::Reg<se_aes_0_key_0::SE_AES_0_KEY_0_SPEC>;
#[doc = "se_aes_0_key_0."]
pub mod se_aes_0_key_0;
#[doc = "se_aes_0_key_1 (rw) register accessor: an alias for `Reg<SE_AES_0_KEY_1_SPEC>`"]
pub type SE_AES_0_KEY_1 = crate::Reg<se_aes_0_key_1::SE_AES_0_KEY_1_SPEC>;
#[doc = "se_aes_0_key_1."]
pub mod se_aes_0_key_1;
#[doc = "se_aes_0_key_2 (rw) register accessor: an alias for `Reg<SE_AES_0_KEY_2_SPEC>`"]
pub type SE_AES_0_KEY_2 = crate::Reg<se_aes_0_key_2::SE_AES_0_KEY_2_SPEC>;
#[doc = "se_aes_0_key_2."]
pub mod se_aes_0_key_2;
#[doc = "se_aes_0_key_3 (rw) register accessor: an alias for `Reg<SE_AES_0_KEY_3_SPEC>`"]
pub type SE_AES_0_KEY_3 = crate::Reg<se_aes_0_key_3::SE_AES_0_KEY_3_SPEC>;
#[doc = "se_aes_0_key_3."]
pub mod se_aes_0_key_3;
#[doc = "se_aes_0_key_4 (rw) register accessor: an alias for `Reg<SE_AES_0_KEY_4_SPEC>`"]
pub type SE_AES_0_KEY_4 = crate::Reg<se_aes_0_key_4::SE_AES_0_KEY_4_SPEC>;
#[doc = "se_aes_0_key_4."]
pub mod se_aes_0_key_4;
#[doc = "se_aes_0_key_5 (rw) register accessor: an alias for `Reg<SE_AES_0_KEY_5_SPEC>`"]
pub type SE_AES_0_KEY_5 = crate::Reg<se_aes_0_key_5::SE_AES_0_KEY_5_SPEC>;
#[doc = "se_aes_0_key_5."]
pub mod se_aes_0_key_5;
#[doc = "se_aes_0_key_6 (rw) register accessor: an alias for `Reg<SE_AES_0_KEY_6_SPEC>`"]
pub type SE_AES_0_KEY_6 = crate::Reg<se_aes_0_key_6::SE_AES_0_KEY_6_SPEC>;
#[doc = "se_aes_0_key_6."]
pub mod se_aes_0_key_6;
#[doc = "se_aes_0_key_7 (rw) register accessor: an alias for `Reg<SE_AES_0_KEY_7_SPEC>`"]
pub type SE_AES_0_KEY_7 = crate::Reg<se_aes_0_key_7::SE_AES_0_KEY_7_SPEC>;
#[doc = "se_aes_0_key_7."]
pub mod se_aes_0_key_7;
#[doc = "se_aes_0_key_sel_0 (rw) register accessor: an alias for `Reg<SE_AES_0_KEY_SEL_0_SPEC>`"]
pub type SE_AES_0_KEY_SEL_0 = crate::Reg<se_aes_0_key_sel_0::SE_AES_0_KEY_SEL_0_SPEC>;
#[doc = "se_aes_0_key_sel_0."]
pub mod se_aes_0_key_sel_0;
#[doc = "se_aes_0_key_sel_1 (rw) register accessor: an alias for `Reg<SE_AES_0_KEY_SEL_1_SPEC>`"]
pub type SE_AES_0_KEY_SEL_1 = crate::Reg<se_aes_0_key_sel_1::SE_AES_0_KEY_SEL_1_SPEC>;
#[doc = "se_aes_0_key_sel_1."]
pub mod se_aes_0_key_sel_1;
#[doc = "se_aes_0_endian (rw) register accessor: an alias for `Reg<SE_AES_0_ENDIAN_SPEC>`"]
pub type SE_AES_0_ENDIAN = crate::Reg<se_aes_0_endian::SE_AES_0_ENDIAN_SPEC>;
#[doc = "se_aes_0_endian."]
pub mod se_aes_0_endian;
#[doc = "se_aes_0_sboot (rw) register accessor: an alias for `Reg<SE_AES_0_SBOOT_SPEC>`"]
pub type SE_AES_0_SBOOT = crate::Reg<se_aes_0_sboot::SE_AES_0_SBOOT_SPEC>;
#[doc = "se_aes_0_sboot."]
pub mod se_aes_0_sboot;
#[doc = "se_aes_0_link (rw) register accessor: an alias for `Reg<SE_AES_0_LINK_SPEC>`"]
pub type SE_AES_0_LINK = crate::Reg<se_aes_0_link::SE_AES_0_LINK_SPEC>;
#[doc = "se_aes_0_link."]
pub mod se_aes_0_link;
#[doc = "se_aes_0_ctrl_prot (rw) register accessor: an alias for `Reg<SE_AES_0_CTRL_PROT_SPEC>`"]
pub type SE_AES_0_CTRL_PROT = crate::Reg<se_aes_0_ctrl_prot::SE_AES_0_CTRL_PROT_SPEC>;
#[doc = "se_aes_0_ctrl_prot."]
pub mod se_aes_0_ctrl_prot;
#[doc = "se_trng_0_ctrl_0 (rw) register accessor: an alias for `Reg<SE_TRNG_0_CTRL_0_SPEC>`"]
pub type SE_TRNG_0_CTRL_0 = crate::Reg<se_trng_0_ctrl_0::SE_TRNG_0_CTRL_0_SPEC>;
#[doc = "se_trng_0_ctrl_0."]
pub mod se_trng_0_ctrl_0;
#[doc = "se_trng_0_status (r) register accessor: an alias for `Reg<SE_TRNG_0_STATUS_SPEC>`"]
pub type SE_TRNG_0_STATUS = crate::Reg<se_trng_0_status::SE_TRNG_0_STATUS_SPEC>;
#[doc = "se_trng_0_status."]
pub mod se_trng_0_status;
#[doc = "se_trng_0_dout_0 (r) register accessor: an alias for `Reg<SE_TRNG_0_DOUT_0_SPEC>`"]
pub type SE_TRNG_0_DOUT_0 = crate::Reg<se_trng_0_dout_0::SE_TRNG_0_DOUT_0_SPEC>;
#[doc = "se_trng_0_dout_0."]
pub mod se_trng_0_dout_0;
#[doc = "se_trng_0_dout_1 (r) register accessor: an alias for `Reg<SE_TRNG_0_DOUT_1_SPEC>`"]
pub type SE_TRNG_0_DOUT_1 = crate::Reg<se_trng_0_dout_1::SE_TRNG_0_DOUT_1_SPEC>;
#[doc = "se_trng_0_dout_1."]
pub mod se_trng_0_dout_1;
#[doc = "se_trng_0_dout_2 (r) register accessor: an alias for `Reg<SE_TRNG_0_DOUT_2_SPEC>`"]
pub type SE_TRNG_0_DOUT_2 = crate::Reg<se_trng_0_dout_2::SE_TRNG_0_DOUT_2_SPEC>;
#[doc = "se_trng_0_dout_2."]
pub mod se_trng_0_dout_2;
#[doc = "se_trng_0_dout_3 (r) register accessor: an alias for `Reg<SE_TRNG_0_DOUT_3_SPEC>`"]
pub type SE_TRNG_0_DOUT_3 = crate::Reg<se_trng_0_dout_3::SE_TRNG_0_DOUT_3_SPEC>;
#[doc = "se_trng_0_dout_3."]
pub mod se_trng_0_dout_3;
#[doc = "se_trng_0_dout_4 (r) register accessor: an alias for `Reg<SE_TRNG_0_DOUT_4_SPEC>`"]
pub type SE_TRNG_0_DOUT_4 = crate::Reg<se_trng_0_dout_4::SE_TRNG_0_DOUT_4_SPEC>;
#[doc = "se_trng_0_dout_4."]
pub mod se_trng_0_dout_4;
#[doc = "se_trng_0_dout_5 (r) register accessor: an alias for `Reg<SE_TRNG_0_DOUT_5_SPEC>`"]
pub type SE_TRNG_0_DOUT_5 = crate::Reg<se_trng_0_dout_5::SE_TRNG_0_DOUT_5_SPEC>;
#[doc = "se_trng_0_dout_5."]
pub mod se_trng_0_dout_5;
#[doc = "se_trng_0_dout_6 (r) register accessor: an alias for `Reg<SE_TRNG_0_DOUT_6_SPEC>`"]
pub type SE_TRNG_0_DOUT_6 = crate::Reg<se_trng_0_dout_6::SE_TRNG_0_DOUT_6_SPEC>;
#[doc = "se_trng_0_dout_6."]
pub mod se_trng_0_dout_6;
#[doc = "se_trng_0_dout_7 (r) register accessor: an alias for `Reg<SE_TRNG_0_DOUT_7_SPEC>`"]
pub type SE_TRNG_0_DOUT_7 = crate::Reg<se_trng_0_dout_7::SE_TRNG_0_DOUT_7_SPEC>;
#[doc = "se_trng_0_dout_7."]
pub mod se_trng_0_dout_7;
#[doc = "se_trng_0_test (rw) register accessor: an alias for `Reg<SE_TRNG_0_TEST_SPEC>`"]
pub type SE_TRNG_0_TEST = crate::Reg<se_trng_0_test::SE_TRNG_0_TEST_SPEC>;
#[doc = "se_trng_0_test."]
pub mod se_trng_0_test;
#[doc = "se_trng_0_ctrl_1 (rw) register accessor: an alias for `Reg<SE_TRNG_0_CTRL_1_SPEC>`"]
pub type SE_TRNG_0_CTRL_1 = crate::Reg<se_trng_0_ctrl_1::SE_TRNG_0_CTRL_1_SPEC>;
#[doc = "se_trng_0_ctrl_1."]
pub mod se_trng_0_ctrl_1;
#[doc = "se_trng_0_ctrl_2 (rw) register accessor: an alias for `Reg<SE_TRNG_0_CTRL_2_SPEC>`"]
pub type SE_TRNG_0_CTRL_2 = crate::Reg<se_trng_0_ctrl_2::SE_TRNG_0_CTRL_2_SPEC>;
#[doc = "se_trng_0_ctrl_2."]
pub mod se_trng_0_ctrl_2;
#[doc = "se_trng_0_ctrl_3 (rw) register accessor: an alias for `Reg<SE_TRNG_0_CTRL_3_SPEC>`"]
pub type SE_TRNG_0_CTRL_3 = crate::Reg<se_trng_0_ctrl_3::SE_TRNG_0_CTRL_3_SPEC>;
#[doc = "se_trng_0_ctrl_3."]
pub mod se_trng_0_ctrl_3;
#[doc = "se_trng_0_test_out_0 (r) register accessor: an alias for `Reg<SE_TRNG_0_TEST_OUT_0_SPEC>`"]
pub type SE_TRNG_0_TEST_OUT_0 = crate::Reg<se_trng_0_test_out_0::SE_TRNG_0_TEST_OUT_0_SPEC>;
#[doc = "se_trng_0_test_out_0."]
pub mod se_trng_0_test_out_0;
#[doc = "se_trng_0_test_out_1 (r) register accessor: an alias for `Reg<SE_TRNG_0_TEST_OUT_1_SPEC>`"]
pub type SE_TRNG_0_TEST_OUT_1 = crate::Reg<se_trng_0_test_out_1::SE_TRNG_0_TEST_OUT_1_SPEC>;
#[doc = "se_trng_0_test_out_1."]
pub mod se_trng_0_test_out_1;
#[doc = "se_trng_0_test_out_2 (r) register accessor: an alias for `Reg<SE_TRNG_0_TEST_OUT_2_SPEC>`"]
pub type SE_TRNG_0_TEST_OUT_2 = crate::Reg<se_trng_0_test_out_2::SE_TRNG_0_TEST_OUT_2_SPEC>;
#[doc = "se_trng_0_test_out_2."]
pub mod se_trng_0_test_out_2;
#[doc = "se_trng_0_test_out_3 (r) register accessor: an alias for `Reg<SE_TRNG_0_TEST_OUT_3_SPEC>`"]
pub type SE_TRNG_0_TEST_OUT_3 = crate::Reg<se_trng_0_test_out_3::SE_TRNG_0_TEST_OUT_3_SPEC>;
#[doc = "se_trng_0_test_out_3."]
pub mod se_trng_0_test_out_3;
#[doc = "se_trng_0_ctrl_prot (rw) register accessor: an alias for `Reg<SE_TRNG_0_CTRL_PROT_SPEC>`"]
pub type SE_TRNG_0_CTRL_PROT = crate::Reg<se_trng_0_ctrl_prot::SE_TRNG_0_CTRL_PROT_SPEC>;
#[doc = "se_trng_0_ctrl_prot."]
pub mod se_trng_0_ctrl_prot;
#[doc = "se_pka_0_ctrl_0 (rw) register accessor: an alias for `Reg<SE_PKA_0_CTRL_0_SPEC>`"]
pub type SE_PKA_0_CTRL_0 = crate::Reg<se_pka_0_ctrl_0::SE_PKA_0_CTRL_0_SPEC>;
#[doc = "se_pka_0_ctrl_0."]
pub mod se_pka_0_ctrl_0;
#[doc = "se_pka_0_seed (rw) register accessor: an alias for `Reg<SE_PKA_0_SEED_SPEC>`"]
pub type SE_PKA_0_SEED = crate::Reg<se_pka_0_seed::SE_PKA_0_SEED_SPEC>;
#[doc = "se_pka_0_seed."]
pub mod se_pka_0_seed;
#[doc = "se_pka_0_ctrl_1 (rw) register accessor: an alias for `Reg<SE_PKA_0_CTRL_1_SPEC>`"]
pub type SE_PKA_0_CTRL_1 = crate::Reg<se_pka_0_ctrl_1::SE_PKA_0_CTRL_1_SPEC>;
#[doc = "se_pka_0_ctrl_1."]
pub mod se_pka_0_ctrl_1;
#[doc = "se_pka_0_rw (rw) register accessor: an alias for `Reg<SE_PKA_0_RW_SPEC>`"]
pub type SE_PKA_0_RW = crate::Reg<se_pka_0_rw::SE_PKA_0_RW_SPEC>;
#[doc = "se_pka_0_rw."]
pub mod se_pka_0_rw;
#[doc = "se_pka_0_rw_burst (rw) register accessor: an alias for `Reg<SE_PKA_0_RW_BURST_SPEC>`"]
pub type SE_PKA_0_RW_BURST = crate::Reg<se_pka_0_rw_burst::SE_PKA_0_RW_BURST_SPEC>;
#[doc = "se_pka_0_rw_burst."]
pub mod se_pka_0_rw_burst;
#[doc = "se_pka_0_ctrl_prot (rw) register accessor: an alias for `Reg<SE_PKA_0_CTRL_PROT_SPEC>`"]
pub type SE_PKA_0_CTRL_PROT = crate::Reg<se_pka_0_ctrl_prot::SE_PKA_0_CTRL_PROT_SPEC>;
#[doc = "se_pka_0_ctrl_prot."]
pub mod se_pka_0_ctrl_prot;
#[doc = "se_cdet_0_ctrl_0 (rw) register accessor: an alias for `Reg<SE_CDET_0_CTRL_0_SPEC>`"]
pub type SE_CDET_0_CTRL_0 = crate::Reg<se_cdet_0_ctrl_0::SE_CDET_0_CTRL_0_SPEC>;
#[doc = "se_cdet_0_ctrl_0."]
pub mod se_cdet_0_ctrl_0;
#[doc = "se_cdet_0_ctrl_1 (rw) register accessor: an alias for `Reg<SE_CDET_0_CTRL_1_SPEC>`"]
pub type SE_CDET_0_CTRL_1 = crate::Reg<se_cdet_0_ctrl_1::SE_CDET_0_CTRL_1_SPEC>;
#[doc = "se_cdet_0_ctrl_1."]
pub mod se_cdet_0_ctrl_1;
#[doc = "se_cdet_0_ctrl_prot (rw) register accessor: an alias for `Reg<SE_CDET_0_CTRL_PROT_SPEC>`"]
pub type SE_CDET_0_CTRL_PROT = crate::Reg<se_cdet_0_ctrl_prot::SE_CDET_0_CTRL_PROT_SPEC>;
#[doc = "se_cdet_0_ctrl_prot."]
pub mod se_cdet_0_ctrl_prot;
#[doc = "se_gmac_0_ctrl_0 (rw) register accessor: an alias for `Reg<SE_GMAC_0_CTRL_0_SPEC>`"]
pub type SE_GMAC_0_CTRL_0 = crate::Reg<se_gmac_0_ctrl_0::SE_GMAC_0_CTRL_0_SPEC>;
#[doc = "se_gmac_0_ctrl_0."]
pub mod se_gmac_0_ctrl_0;
#[doc = "se_gmac_0_lca (rw) register accessor: an alias for `Reg<SE_GMAC_0_LCA_SPEC>`"]
pub type SE_GMAC_0_LCA = crate::Reg<se_gmac_0_lca::SE_GMAC_0_LCA_SPEC>;
#[doc = "se_gmac_0_lca."]
pub mod se_gmac_0_lca;
#[doc = "se_gmac_0_status (r) register accessor: an alias for `Reg<SE_GMAC_0_STATUS_SPEC>`"]
pub type SE_GMAC_0_STATUS = crate::Reg<se_gmac_0_status::SE_GMAC_0_STATUS_SPEC>;
#[doc = "se_gmac_0_status."]
pub mod se_gmac_0_status;
#[doc = "se_gmac_0_ctrl_prot (rw) register accessor: an alias for `Reg<SE_GMAC_0_CTRL_PROT_SPEC>`"]
pub type SE_GMAC_0_CTRL_PROT = crate::Reg<se_gmac_0_ctrl_prot::SE_GMAC_0_CTRL_PROT_SPEC>;
#[doc = "se_gmac_0_ctrl_prot."]
pub mod se_gmac_0_ctrl_prot;
#[doc = "se_ctrl_prot_rd (r) register accessor: an alias for `Reg<SE_CTRL_PROT_RD_SPEC>`"]
pub type SE_CTRL_PROT_RD = crate::Reg<se_ctrl_prot_rd::SE_CTRL_PROT_RD_SPEC>;
#[doc = "se_ctrl_prot_rd."]
pub mod se_ctrl_prot_rd;
#[doc = "se_ctrl_reserved_0 (rw) register accessor: an alias for `Reg<SE_CTRL_RESERVED_0_SPEC>`"]
pub type SE_CTRL_RESERVED_0 = crate::Reg<se_ctrl_reserved_0::SE_CTRL_RESERVED_0_SPEC>;
#[doc = "se_ctrl_reserved_0."]
pub mod se_ctrl_reserved_0;
#[doc = "se_ctrl_reserved_1 (rw) register accessor: an alias for `Reg<SE_CTRL_RESERVED_1_SPEC>`"]
pub type SE_CTRL_RESERVED_1 = crate::Reg<se_ctrl_reserved_1::SE_CTRL_RESERVED_1_SPEC>;
#[doc = "se_ctrl_reserved_1."]
pub mod se_ctrl_reserved_1;
#[doc = "se_ctrl_reserved_2 (rw) register accessor: an alias for `Reg<SE_CTRL_RESERVED_2_SPEC>`"]
pub type SE_CTRL_RESERVED_2 = crate::Reg<se_ctrl_reserved_2::SE_CTRL_RESERVED_2_SPEC>;
#[doc = "se_ctrl_reserved_2."]
pub mod se_ctrl_reserved_2;

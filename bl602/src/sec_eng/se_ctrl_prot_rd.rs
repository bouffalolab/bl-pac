#[doc = "Register `se_ctrl_prot_rd` reader"]
pub struct R(crate::R<SE_CTRL_PROT_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_CTRL_PROT_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_CTRL_PROT_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_CTRL_PROT_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `se_sha_prot_en_rd` reader - "]
pub type SE_SHA_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_sha_id0_en_rd` reader - "]
pub type SE_SHA_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_sha_id1_en_rd` reader - "]
pub type SE_SHA_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_prot_en_rd` reader - "]
pub type SE_AES_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_id0_en_rd` reader - "]
pub type SE_AES_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_id1_en_rd` reader - "]
pub type SE_AES_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_prot_en_rd` reader - "]
pub type SE_TRNG_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_id0_en_rd` reader - "]
pub type SE_TRNG_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_id1_en_rd` reader - "]
pub type SE_TRNG_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_prot_en_rd` reader - "]
pub type SE_PKA_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_id0_en_rd` reader - "]
pub type SE_PKA_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_id1_en_rd` reader - "]
pub type SE_PKA_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_cdet_prot_en_rd` reader - "]
pub type SE_CDET_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_cdet_id0_en_rd` reader - "]
pub type SE_CDET_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_cdet_id1_en_rd` reader - "]
pub type SE_CDET_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_prot_en_rd` reader - "]
pub type SE_GMAC_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_id0_en_rd` reader - "]
pub type SE_GMAC_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_id1_en_rd` reader - "]
pub type SE_GMAC_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_dbg_dis` reader - "]
pub type SE_DBG_DIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_prot_en_rd(&self) -> SE_SHA_PROT_EN_RD_R {
        SE_SHA_PROT_EN_RD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_id0_en_rd(&self) -> SE_SHA_ID0_EN_RD_R {
        SE_SHA_ID0_EN_RD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_sha_id1_en_rd(&self) -> SE_SHA_ID1_EN_RD_R {
        SE_SHA_ID1_EN_RD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_aes_prot_en_rd(&self) -> SE_AES_PROT_EN_RD_R {
        SE_AES_PROT_EN_RD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_aes_id0_en_rd(&self) -> SE_AES_ID0_EN_RD_R {
        SE_AES_ID0_EN_RD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_id1_en_rd(&self) -> SE_AES_ID1_EN_RD_R {
        SE_AES_ID1_EN_RD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_prot_en_rd(&self) -> SE_TRNG_PROT_EN_RD_R {
        SE_TRNG_PROT_EN_RD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_id0_en_rd(&self) -> SE_TRNG_ID0_EN_RD_R {
        SE_TRNG_ID0_EN_RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_id1_en_rd(&self) -> SE_TRNG_ID1_EN_RD_R {
        SE_TRNG_ID1_EN_RD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_prot_en_rd(&self) -> SE_PKA_PROT_EN_RD_R {
        SE_PKA_PROT_EN_RD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_id0_en_rd(&self) -> SE_PKA_ID0_EN_RD_R {
        SE_PKA_ID0_EN_RD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_pka_id1_en_rd(&self) -> SE_PKA_ID1_EN_RD_R {
        SE_PKA_ID1_EN_RD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_cdet_prot_en_rd(&self) -> SE_CDET_PROT_EN_RD_R {
        SE_CDET_PROT_EN_RD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn se_cdet_id0_en_rd(&self) -> SE_CDET_ID0_EN_RD_R {
        SE_CDET_ID0_EN_RD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn se_cdet_id1_en_rd(&self) -> SE_CDET_ID1_EN_RD_R {
        SE_CDET_ID1_EN_RD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn se_gmac_prot_en_rd(&self) -> SE_GMAC_PROT_EN_RD_R {
        SE_GMAC_PROT_EN_RD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn se_gmac_id0_en_rd(&self) -> SE_GMAC_ID0_EN_RD_R {
        SE_GMAC_ID0_EN_RD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn se_gmac_id1_en_rd(&self) -> SE_GMAC_ID1_EN_RD_R {
        SE_GMAC_ID1_EN_RD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn se_dbg_dis(&self) -> SE_DBG_DIS_R {
        SE_DBG_DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "se_ctrl_prot_rd.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_ctrl_prot_rd](index.html) module"]
pub struct SE_CTRL_PROT_RD_SPEC;
impl crate::RegisterSpec for SE_CTRL_PROT_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_ctrl_prot_rd::R](R) reader structure"]
impl crate::Readable for SE_CTRL_PROT_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets se_ctrl_prot_rd to value 0x0077_7777"]
impl crate::Resettable for SE_CTRL_PROT_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0077_7777;
}

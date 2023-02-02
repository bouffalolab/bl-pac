#[doc = "Register `sf_ctrl_prot_en_rd` reader"]
pub struct R(crate::R<SF_CTRL_PROT_EN_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_PROT_EN_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CTRL_PROT_EN_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CTRL_PROT_EN_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sf_ctrl_prot_en_rd` reader - "]
pub type SF_CTRL_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `sf_ctrl_id0_en_rd` reader - "]
pub type SF_CTRL_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `sf_ctrl_id1_en_rd` reader - "]
pub type SF_CTRL_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_0_trig_wr_lock` reader - "]
pub type SF_IF_0_TRIG_WR_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `sf_dbg_dis` reader - "]
pub type SF_DBG_DIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en_rd(&self) -> SF_CTRL_PROT_EN_RD_R {
        SF_CTRL_PROT_EN_RD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en_rd(&self) -> SF_CTRL_ID0_EN_RD_R {
        SF_CTRL_ID0_EN_RD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en_rd(&self) -> SF_CTRL_ID1_EN_RD_R {
        SF_CTRL_ID1_EN_RD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_0_trig_wr_lock(&self) -> SF_IF_0_TRIG_WR_LOCK_R {
        SF_IF_0_TRIG_WR_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_dbg_dis(&self) -> SF_DBG_DIS_R {
        SF_DBG_DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "sf_ctrl_prot_en_rd.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_prot_en_rd](index.html) module"]
pub struct SF_CTRL_PROT_EN_RD_SPEC;
impl crate::RegisterSpec for SF_CTRL_PROT_EN_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_prot_en_rd::R](R) reader structure"]
impl crate::Readable for SF_CTRL_PROT_EN_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sf_ctrl_prot_en_rd to value 0x07"]
impl crate::Resettable for SF_CTRL_PROT_EN_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}

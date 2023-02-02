#[doc = "Register `gpdac_tx_fifo_status` reader"]
pub struct R(crate::R<GPDAC_TX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_TX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_TX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_TX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tx_fifo_empty` reader - "]
pub type TX_FIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_full` reader - "]
pub type TX_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `tx_cs` reader - "]
pub type TX_CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TxFifoRdPtr` reader - "]
pub type TX_FIFO_RD_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TxFifoWrPtr` reader - "]
pub type TX_FIFO_WR_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTY_R {
        TX_FIFO_EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TX_FIFO_FULL_R {
        TX_FIFO_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_cs(&self) -> TX_CS_R {
        TX_CS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_fifo_rd_ptr(&self) -> TX_FIFO_RD_PTR_R {
        TX_FIFO_RD_PTR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_fifo_wr_ptr(&self) -> TX_FIFO_WR_PTR_R {
        TX_FIFO_WR_PTR_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "gpdac_tx_fifo_status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_tx_fifo_status](index.html) module"]
pub struct GPDAC_TX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for GPDAC_TX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_tx_fifo_status::R](R) reader structure"]
impl crate::Readable for GPDAC_TX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets gpdac_tx_fifo_status to value 0x40"]
impl crate::Resettable for GPDAC_TX_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}

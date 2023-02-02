#[doc = "Register `HBN_IRQ_STAT` reader"]
pub struct R(crate::R<HBN_IRQ_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_IRQ_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_IRQ_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_IRQ_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `irq_stat` reader - "]
pub type IRQ_STAT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_stat(&self) -> IRQ_STAT_R {
        IRQ_STAT_R::new(self.bits)
    }
}
#[doc = "HBN_IRQ_STAT.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_irq_stat](index.html) module"]
pub struct HBN_IRQ_STAT_SPEC;
impl crate::RegisterSpec for HBN_IRQ_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_irq_stat::R](R) reader structure"]
impl crate::Readable for HBN_IRQ_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HBN_IRQ_STAT to value 0"]
impl crate::Resettable for HBN_IRQ_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

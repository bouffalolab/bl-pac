#[doc = "Register `spi_int_sts` reader"]
pub struct R(crate::R<SPI_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_INT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_int_sts` writer"]
pub struct W(crate::W<SPI_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_INT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_end_int` reader - "]
pub type SPI_END_INT_R = crate::BitReader<bool>;
#[doc = "Field `spi_txf_int` reader - "]
pub type SPI_TXF_INT_R = crate::BitReader<bool>;
#[doc = "Field `spi_rxf_int` reader - "]
pub type SPI_RXF_INT_R = crate::BitReader<bool>;
#[doc = "Field `spi_sto_int` reader - "]
pub type SPI_STO_INT_R = crate::BitReader<bool>;
#[doc = "Field `spi_txu_int` reader - "]
pub type SPI_TXU_INT_R = crate::BitReader<bool>;
#[doc = "Field `spi_fer_int` reader - "]
pub type SPI_FER_INT_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_end_mask` reader - "]
pub type CR_SPI_END_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_end_mask` writer - "]
pub type CR_SPI_END_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_txf_mask` reader - "]
pub type CR_SPI_TXF_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_txf_mask` writer - "]
pub type CR_SPI_TXF_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_rxf_mask` reader - "]
pub type CR_SPI_RXF_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_rxf_mask` writer - "]
pub type CR_SPI_RXF_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_sto_mask` reader - "]
pub type CR_SPI_STO_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_sto_mask` writer - "]
pub type CR_SPI_STO_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_txu_mask` reader - "]
pub type CR_SPI_TXU_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_txu_mask` writer - "]
pub type CR_SPI_TXU_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_fer_mask` reader - "]
pub type CR_SPI_FER_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_fer_mask` writer - "]
pub type CR_SPI_FER_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_end_clr` reader - "]
pub type CR_SPI_END_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_end_clr` writer - "]
pub type CR_SPI_END_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `rsvd_17` reader - "]
pub type RSVD_17_R = crate::BitReader<bool>;
#[doc = "Field `rsvd_17` writer - "]
pub type RSVD_17_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `rsvd_18` reader - "]
pub type RSVD_18_R = crate::BitReader<bool>;
#[doc = "Field `rsvd_18` writer - "]
pub type RSVD_18_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_sto_clr` reader - "]
pub type CR_SPI_STO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_sto_clr` writer - "]
pub type CR_SPI_STO_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_txu_clr` reader - "]
pub type CR_SPI_TXU_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_txu_clr` writer - "]
pub type CR_SPI_TXU_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `rsvd_21` reader - "]
pub type RSVD_21_R = crate::BitReader<bool>;
#[doc = "Field `rsvd_21` writer - "]
pub type RSVD_21_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_end_en` reader - "]
pub type CR_SPI_END_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_end_en` writer - "]
pub type CR_SPI_END_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_txf_en` reader - "]
pub type CR_SPI_TXF_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_txf_en` writer - "]
pub type CR_SPI_TXF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_rxf_en` reader - "]
pub type CR_SPI_RXF_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_rxf_en` writer - "]
pub type CR_SPI_RXF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_sto_en` reader - "]
pub type CR_SPI_STO_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_sto_en` writer - "]
pub type CR_SPI_STO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_txu_en` reader - "]
pub type CR_SPI_TXU_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_txu_en` writer - "]
pub type CR_SPI_TXU_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
#[doc = "Field `cr_spi_fer_en` reader - "]
pub type CR_SPI_FER_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_spi_fer_en` writer - "]
pub type CR_SPI_FER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_INT_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_end_int(&self) -> SPI_END_INT_R {
        SPI_END_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_txf_int(&self) -> SPI_TXF_INT_R {
        SPI_TXF_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_rxf_int(&self) -> SPI_RXF_INT_R {
        SPI_RXF_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_sto_int(&self) -> SPI_STO_INT_R {
        SPI_STO_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_txu_int(&self) -> SPI_TXU_INT_R {
        SPI_TXU_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_fer_int(&self) -> SPI_FER_INT_R {
        SPI_FER_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_spi_end_mask(&self) -> CR_SPI_END_MASK_R {
        CR_SPI_END_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_spi_txf_mask(&self) -> CR_SPI_TXF_MASK_R {
        CR_SPI_TXF_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_spi_rxf_mask(&self) -> CR_SPI_RXF_MASK_R {
        CR_SPI_RXF_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_spi_sto_mask(&self) -> CR_SPI_STO_MASK_R {
        CR_SPI_STO_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_spi_txu_mask(&self) -> CR_SPI_TXU_MASK_R {
        CR_SPI_TXU_MASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_spi_fer_mask(&self) -> CR_SPI_FER_MASK_R {
        CR_SPI_FER_MASK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_spi_end_clr(&self) -> CR_SPI_END_CLR_R {
        CR_SPI_END_CLR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rsvd_17(&self) -> RSVD_17_R {
        RSVD_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rsvd_18(&self) -> RSVD_18_R {
        RSVD_18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_spi_sto_clr(&self) -> CR_SPI_STO_CLR_R {
        CR_SPI_STO_CLR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_spi_txu_clr(&self) -> CR_SPI_TXU_CLR_R {
        CR_SPI_TXU_CLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rsvd_21(&self) -> RSVD_21_R {
        RSVD_21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_spi_end_en(&self) -> CR_SPI_END_EN_R {
        CR_SPI_END_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_spi_txf_en(&self) -> CR_SPI_TXF_EN_R {
        CR_SPI_TXF_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_spi_rxf_en(&self) -> CR_SPI_RXF_EN_R {
        CR_SPI_RXF_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_spi_sto_en(&self) -> CR_SPI_STO_EN_R {
        CR_SPI_STO_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_spi_txu_en(&self) -> CR_SPI_TXU_EN_R {
        CR_SPI_TXU_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_spi_fer_en(&self) -> CR_SPI_FER_EN_R {
        CR_SPI_FER_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_end_mask(&mut self) -> CR_SPI_END_MASK_W<8> {
        CR_SPI_END_MASK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_txf_mask(&mut self) -> CR_SPI_TXF_MASK_W<9> {
        CR_SPI_TXF_MASK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_rxf_mask(&mut self) -> CR_SPI_RXF_MASK_W<10> {
        CR_SPI_RXF_MASK_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_sto_mask(&mut self) -> CR_SPI_STO_MASK_W<11> {
        CR_SPI_STO_MASK_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_txu_mask(&mut self) -> CR_SPI_TXU_MASK_W<12> {
        CR_SPI_TXU_MASK_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_fer_mask(&mut self) -> CR_SPI_FER_MASK_W<13> {
        CR_SPI_FER_MASK_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_end_clr(&mut self) -> CR_SPI_END_CLR_W<16> {
        CR_SPI_END_CLR_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_17(&mut self) -> RSVD_17_W<17> {
        RSVD_17_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_18(&mut self) -> RSVD_18_W<18> {
        RSVD_18_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_sto_clr(&mut self) -> CR_SPI_STO_CLR_W<19> {
        CR_SPI_STO_CLR_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_txu_clr(&mut self) -> CR_SPI_TXU_CLR_W<20> {
        CR_SPI_TXU_CLR_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_21(&mut self) -> RSVD_21_W<21> {
        RSVD_21_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_end_en(&mut self) -> CR_SPI_END_EN_W<24> {
        CR_SPI_END_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_txf_en(&mut self) -> CR_SPI_TXF_EN_W<25> {
        CR_SPI_TXF_EN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_rxf_en(&mut self) -> CR_SPI_RXF_EN_W<26> {
        CR_SPI_RXF_EN_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_sto_en(&mut self) -> CR_SPI_STO_EN_W<27> {
        CR_SPI_STO_EN_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_txu_en(&mut self) -> CR_SPI_TXU_EN_W<28> {
        CR_SPI_TXU_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_fer_en(&mut self) -> CR_SPI_FER_EN_W<29> {
        CR_SPI_FER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_int_sts](index.html) module"]
pub struct SPI_INT_STS_SPEC;
impl crate::RegisterSpec for SPI_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_int_sts::R](R) reader structure"]
impl crate::Readable for SPI_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_int_sts::W](W) writer structure"]
impl crate::Writable for SPI_INT_STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_int_sts to value 0x3f00_3f00"]
impl crate::Resettable for SPI_INT_STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f00_3f00;
}

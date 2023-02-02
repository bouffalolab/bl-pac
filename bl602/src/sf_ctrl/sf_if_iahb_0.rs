#[doc = "Register `sf_if_iahb_0` reader"]
pub struct R(crate::R<SF_IF_IAHB_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_IF_IAHB_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_IF_IAHB_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_IF_IAHB_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_if_iahb_0` writer"]
pub struct W(crate::W<SF_IF_IAHB_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_IF_IAHB_0_SPEC>;
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
impl From<crate::W<SF_IF_IAHB_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_IF_IAHB_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_if_1_dmy_byte` reader - "]
pub type SF_IF_1_DMY_BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if_1_dmy_byte` writer - "]
pub type SF_IF_1_DMY_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_IF_IAHB_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `sf_if_1_adr_byte` reader - "]
pub type SF_IF_1_ADR_BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if_1_adr_byte` writer - "]
pub type SF_IF_1_ADR_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_IF_IAHB_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `sf_if_1_cmd_byte` reader - "]
pub type SF_IF_1_CMD_BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if_1_cmd_byte` writer - "]
pub type SF_IF_1_CMD_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_IF_IAHB_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `sf_if_1_dat_rw` reader - "]
pub type SF_IF_1_DAT_RW_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_1_dat_rw` writer - "]
pub type SF_IF_1_DAT_RW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_IF_IAHB_0_SPEC, bool, O>;
#[doc = "Field `sf_if_1_dat_en` reader - "]
pub type SF_IF_1_DAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_1_dat_en` writer - "]
pub type SF_IF_1_DAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_IF_IAHB_0_SPEC, bool, O>;
#[doc = "Field `sf_if_1_dmy_en` reader - "]
pub type SF_IF_1_DMY_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_1_dmy_en` writer - "]
pub type SF_IF_1_DMY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_IF_IAHB_0_SPEC, bool, O>;
#[doc = "Field `sf_if_1_adr_en` reader - "]
pub type SF_IF_1_ADR_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_1_adr_en` writer - "]
pub type SF_IF_1_ADR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_IF_IAHB_0_SPEC, bool, O>;
#[doc = "Field `sf_if_1_cmd_en` reader - "]
pub type SF_IF_1_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_1_cmd_en` writer - "]
pub type SF_IF_1_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_IF_IAHB_0_SPEC, bool, O>;
#[doc = "Field `sf_if_1_spi_mode` reader - "]
pub type SF_IF_1_SPI_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if_1_spi_mode` writer - "]
pub type SF_IF_1_SPI_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_IF_IAHB_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `sf_if_1_qpi_mode_en` reader - "]
pub type SF_IF_1_QPI_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_1_qpi_mode_en` writer - "]
pub type SF_IF_1_QPI_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_IF_IAHB_0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_1_dmy_byte(&self) -> SF_IF_1_DMY_BYTE_R {
        SF_IF_1_DMY_BYTE_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_1_adr_byte(&self) -> SF_IF_1_ADR_BYTE_R {
        SF_IF_1_ADR_BYTE_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_1_cmd_byte(&self) -> SF_IF_1_CMD_BYTE_R {
        SF_IF_1_CMD_BYTE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_1_dat_rw(&self) -> SF_IF_1_DAT_RW_R {
        SF_IF_1_DAT_RW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_1_dat_en(&self) -> SF_IF_1_DAT_EN_R {
        SF_IF_1_DAT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_1_dmy_en(&self) -> SF_IF_1_DMY_EN_R {
        SF_IF_1_DMY_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_1_adr_en(&self) -> SF_IF_1_ADR_EN_R {
        SF_IF_1_ADR_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_1_cmd_en(&self) -> SF_IF_1_CMD_EN_R {
        SF_IF_1_CMD_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_1_spi_mode(&self) -> SF_IF_1_SPI_MODE_R {
        SF_IF_1_SPI_MODE_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_1_qpi_mode_en(&self) -> SF_IF_1_QPI_MODE_EN_R {
        SF_IF_1_QPI_MODE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:16"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_dmy_byte(&mut self) -> SF_IF_1_DMY_BYTE_W<12> {
        SF_IF_1_DMY_BYTE_W::new(self)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_adr_byte(&mut self) -> SF_IF_1_ADR_BYTE_W<17> {
        SF_IF_1_ADR_BYTE_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_cmd_byte(&mut self) -> SF_IF_1_CMD_BYTE_W<20> {
        SF_IF_1_CMD_BYTE_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_dat_rw(&mut self) -> SF_IF_1_DAT_RW_W<23> {
        SF_IF_1_DAT_RW_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_dat_en(&mut self) -> SF_IF_1_DAT_EN_W<24> {
        SF_IF_1_DAT_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_dmy_en(&mut self) -> SF_IF_1_DMY_EN_W<25> {
        SF_IF_1_DMY_EN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_adr_en(&mut self) -> SF_IF_1_ADR_EN_W<26> {
        SF_IF_1_ADR_EN_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_cmd_en(&mut self) -> SF_IF_1_CMD_EN_W<27> {
        SF_IF_1_CMD_EN_W::new(self)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_spi_mode(&mut self) -> SF_IF_1_SPI_MODE_W<28> {
        SF_IF_1_SPI_MODE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_qpi_mode_en(&mut self) -> SF_IF_1_QPI_MODE_EN_W<31> {
        SF_IF_1_QPI_MODE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_if_iahb_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_0](index.html) module"]
pub struct SF_IF_IAHB_0_SPEC;
impl crate::RegisterSpec for SF_IF_IAHB_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_if_iahb_0::R](R) reader structure"]
impl crate::Readable for SF_IF_IAHB_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_0::W](W) writer structure"]
impl crate::Writable for SF_IF_IAHB_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_if_iahb_0 to value 0x0d04_0000"]
impl crate::Resettable for SF_IF_IAHB_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d04_0000;
}

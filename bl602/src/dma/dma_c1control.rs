#[doc = "Register `DMA_C1Control` reader"]
pub struct R(crate::R<DMA_C1CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_C1CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_C1CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_C1CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_C1Control` writer"]
pub struct W(crate::W<DMA_C1CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_C1CONTROL_SPEC>;
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
impl From<crate::W<DMA_C1CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_C1CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TransferSize` reader - "]
pub type TRANSFER_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TransferSize` writer - "]
pub type TRANSFER_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_C1CONTROL_SPEC, u16, u16, 12, O>;
#[doc = "Field `SBSize` reader - "]
pub type SBSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBSize` writer - "]
pub type SBSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_C1CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBSize` reader - "]
pub type DBSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBSize` writer - "]
pub type DBSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_C1CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SWidth` reader - "]
pub type SWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWidth` writer - "]
pub type SWIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_C1CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DWidth` reader - "]
pub type DWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DWidth` writer - "]
pub type DWIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_C1CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SI` reader - "]
pub type SI_R = crate::BitReader<bool>;
#[doc = "Field `SI` writer - "]
pub type SI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_C1CONTROL_SPEC, bool, O>;
#[doc = "Field `DI` reader - "]
pub type DI_R = crate::BitReader<bool>;
#[doc = "Field `DI` writer - "]
pub type DI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_C1CONTROL_SPEC, bool, O>;
#[doc = "Field `Prot` reader - "]
pub type PROT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Prot` writer - "]
pub type PROT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_C1CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `I` reader - "]
pub type I_R = crate::BitReader<bool>;
#[doc = "Field `I` writer - "]
pub type I_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_C1CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn transfer_size(&self) -> TRANSFER_SIZE_R {
        TRANSFER_SIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sbsize(&self) -> SBSIZE_R {
        SBSIZE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dbsize(&self) -> DBSIZE_R {
        DBSIZE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn swidth(&self) -> SWIDTH_R {
        SWIDTH_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_size(&mut self) -> TRANSFER_SIZE_W<0> {
        TRANSFER_SIZE_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn sbsize(&mut self) -> SBSIZE_W<12> {
        SBSIZE_W::new(self)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    #[must_use]
    pub fn dbsize(&mut self) -> DBSIZE_W<15> {
        DBSIZE_W::new(self)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    #[must_use]
    pub fn swidth(&mut self) -> SWIDTH_W<18> {
        SWIDTH_W::new(self)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    #[must_use]
    pub fn dwidth(&mut self) -> DWIDTH_W<21> {
        DWIDTH_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn si(&mut self) -> SI_W<26> {
        SI_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn di(&mut self) -> DI_W<27> {
        DI_W::new(self)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn prot(&mut self) -> PROT_W<28> {
        PROT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn i(&mut self) -> I_W<31> {
        I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_C1Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c1control](index.html) module"]
pub struct DMA_C1CONTROL_SPEC;
impl crate::RegisterSpec for DMA_C1CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_c1control::R](R) reader structure"]
impl crate::Readable for DMA_C1CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_c1control::W](W) writer structure"]
impl crate::Writable for DMA_C1CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_C1Control to value 0x0c48_9000"]
impl crate::Resettable for DMA_C1CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c48_9000;
}

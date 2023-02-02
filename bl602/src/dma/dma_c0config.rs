#[doc = "Register `DMA_C0Config` reader"]
pub struct R(crate::R<DMA_C0CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_C0CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_C0CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_C0CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_C0Config` writer"]
pub struct W(crate::W<DMA_C0CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_C0CONFIG_SPEC>;
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
impl From<crate::W<DMA_C0CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_C0CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `E` reader - "]
pub type E_R = crate::BitReader<bool>;
#[doc = "Field `E` writer - "]
pub type E_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_C0CONFIG_SPEC, bool, O>;
#[doc = "Field `SrcPeripheral` reader - "]
pub type SRC_PERIPHERAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SrcPeripheral` writer - "]
pub type SRC_PERIPHERAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_C0CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `DstPeripheral` reader - "]
pub type DST_PERIPHERAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DstPeripheral` writer - "]
pub type DST_PERIPHERAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_C0CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `FlowCntrl` reader - "]
pub type FLOW_CNTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FlowCntrl` writer - "]
pub type FLOW_CNTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_C0CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `IE` reader - "]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - "]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_C0CONFIG_SPEC, bool, O>;
#[doc = "Field `ITC` reader - "]
pub type ITC_R = crate::BitReader<bool>;
#[doc = "Field `ITC` writer - "]
pub type ITC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_C0CONFIG_SPEC, bool, O>;
#[doc = "Field `L` reader - "]
pub type L_R = crate::BitReader<bool>;
#[doc = "Field `L` writer - "]
pub type L_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_C0CONFIG_SPEC, bool, O>;
#[doc = "Field `A` reader - "]
pub type A_R = crate::BitReader<bool>;
#[doc = "Field `H` reader - "]
pub type H_R = crate::BitReader<bool>;
#[doc = "Field `H` writer - "]
pub type H_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_C0CONFIG_SPEC, bool, O>;
#[doc = "Field `LLICounter` reader - "]
pub type LLICOUNTER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&self) -> SRC_PERIPHERAL_R {
        SRC_PERIPHERAL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&self) -> DST_PERIPHERAL_R {
        DST_PERIPHERAL_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn flow_cntrl(&self) -> FLOW_CNTRL_R {
        FLOW_CNTRL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn itc(&self) -> ITC_R {
        ITC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn llicounter(&self) -> LLICOUNTER_R {
        LLICOUNTER_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn e(&mut self) -> E_W<0> {
        E_W::new(self)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    #[must_use]
    pub fn src_peripheral(&mut self) -> SRC_PERIPHERAL_W<1> {
        SRC_PERIPHERAL_W::new(self)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    #[must_use]
    pub fn dst_peripheral(&mut self) -> DST_PERIPHERAL_W<6> {
        DST_PERIPHERAL_W::new(self)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    #[must_use]
    pub fn flow_cntrl(&mut self) -> FLOW_CNTRL_W<11> {
        FLOW_CNTRL_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<14> {
        IE_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn itc(&mut self) -> ITC_W<15> {
        ITC_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn l(&mut self) -> L_W<16> {
        L_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn h(&mut self) -> H_W<18> {
        H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_C0Config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c0config](index.html) module"]
pub struct DMA_C0CONFIG_SPEC;
impl crate::RegisterSpec for DMA_C0CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_c0config::R](R) reader structure"]
impl crate::Readable for DMA_C0CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_c0config::W](W) writer structure"]
impl crate::Writable for DMA_C0CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_C0Config to value 0"]
impl crate::Resettable for DMA_C0CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

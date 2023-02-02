#[doc = "Register `tx_iq_gain_hw5` reader"]
pub struct R(crate::R<TX_IQ_GAIN_HW5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_IQ_GAIN_HW5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_IQ_GAIN_HW5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_IQ_GAIN_HW5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tx_iq_gain_hw5` writer"]
pub struct W(crate::W<TX_IQ_GAIN_HW5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_IQ_GAIN_HW5_SPEC>;
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
impl From<crate::W<TX_IQ_GAIN_HW5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_IQ_GAIN_HW5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_iq_phase_comp_gc5` reader - "]
pub type TX_IQ_PHASE_COMP_GC5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tx_iq_phase_comp_gc5` writer - "]
pub type TX_IQ_PHASE_COMP_GC5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_IQ_GAIN_HW5_SPEC, u16, u16, 10, O>;
#[doc = "Field `tx_iq_gain_comp_gc5` reader - "]
pub type TX_IQ_GAIN_COMP_GC5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tx_iq_gain_comp_gc5` writer - "]
pub type TX_IQ_GAIN_COMP_GC5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_IQ_GAIN_HW5_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc5(&self) -> TX_IQ_PHASE_COMP_GC5_R {
        TX_IQ_PHASE_COMP_GC5_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc5(&self) -> TX_IQ_GAIN_COMP_GC5_R {
        TX_IQ_GAIN_COMP_GC5_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn tx_iq_phase_comp_gc5(&mut self) -> TX_IQ_PHASE_COMP_GC5_W<0> {
        TX_IQ_PHASE_COMP_GC5_W::new(self)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    #[must_use]
    pub fn tx_iq_gain_comp_gc5(&mut self) -> TX_IQ_GAIN_COMP_GC5_W<16> {
        TX_IQ_GAIN_COMP_GC5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tx_iq_gain_hw5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_iq_gain_hw5](index.html) module"]
pub struct TX_IQ_GAIN_HW5_SPEC;
impl crate::RegisterSpec for TX_IQ_GAIN_HW5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_iq_gain_hw5::R](R) reader structure"]
impl crate::Readable for TX_IQ_GAIN_HW5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_iq_gain_hw5::W](W) writer structure"]
impl crate::Writable for TX_IQ_GAIN_HW5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tx_iq_gain_hw5 to value 0"]
impl crate::Resettable for TX_IQ_GAIN_HW5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

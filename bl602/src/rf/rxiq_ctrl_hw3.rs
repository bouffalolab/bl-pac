#[doc = "Register `rxiq_ctrl_hw3` reader"]
pub struct R(crate::R<RXIQ_CTRL_HW3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIQ_CTRL_HW3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIQ_CTRL_HW3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIQ_CTRL_HW3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxiq_ctrl_hw3` writer"]
pub struct W(crate::W<RXIQ_CTRL_HW3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXIQ_CTRL_HW3_SPEC>;
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
impl From<crate::W<RXIQ_CTRL_HW3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXIQ_CTRL_HW3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_iq_phase_comp_gc2` reader - "]
pub type RX_IQ_PHASE_COMP_GC2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_iq_phase_comp_gc2` writer - "]
pub type RX_IQ_PHASE_COMP_GC2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXIQ_CTRL_HW3_SPEC, u16, u16, 10, O>;
#[doc = "Field `rx_iq_gain_comp_gc2` reader - "]
pub type RX_IQ_GAIN_COMP_GC2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_iq_gain_comp_gc2` writer - "]
pub type RX_IQ_GAIN_COMP_GC2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXIQ_CTRL_HW3_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc2(&self) -> RX_IQ_PHASE_COMP_GC2_R {
        RX_IQ_PHASE_COMP_GC2_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc2(&self) -> RX_IQ_GAIN_COMP_GC2_R {
        RX_IQ_GAIN_COMP_GC2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_iq_phase_comp_gc2(&mut self) -> RX_IQ_PHASE_COMP_GC2_W<0> {
        RX_IQ_PHASE_COMP_GC2_W::new(self)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    #[must_use]
    pub fn rx_iq_gain_comp_gc2(&mut self) -> RX_IQ_GAIN_COMP_GC2_W<16> {
        RX_IQ_GAIN_COMP_GC2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rxiq_ctrl_hw3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxiq_ctrl_hw3](index.html) module"]
pub struct RXIQ_CTRL_HW3_SPEC;
impl crate::RegisterSpec for RXIQ_CTRL_HW3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxiq_ctrl_hw3::R](R) reader structure"]
impl crate::Readable for RXIQ_CTRL_HW3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxiq_ctrl_hw3::W](W) writer structure"]
impl crate::Writable for RXIQ_CTRL_HW3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxiq_ctrl_hw3 to value 0"]
impl crate::Resettable for RXIQ_CTRL_HW3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

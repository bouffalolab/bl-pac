#[doc = "Register `cci_ctl` reader"]
pub struct R(crate::R<CCI_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCI_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCI_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCI_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cci_ctl` writer"]
pub struct W(crate::W<CCI_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCI_CTL_SPEC>;
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
impl From<crate::W<CCI_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCI_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cci_write_flag` reader - "]
pub type CCI_WRITE_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `cci_read_flag` reader - "]
pub type CCI_READ_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `ahb_state` reader - "]
pub type AHB_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_write_flag(&self) -> CCI_WRITE_FLAG_R {
        CCI_WRITE_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_read_flag(&self) -> CCI_READ_FLAG_R {
        CCI_READ_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ahb_state(&self) -> AHB_STATE_R {
        AHB_STATE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cci_ctl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_ctl](index.html) module"]
pub struct CCI_CTL_SPEC;
impl crate::RegisterSpec for CCI_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cci_ctl::R](R) reader structure"]
impl crate::Readable for CCI_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cci_ctl::W](W) writer structure"]
impl crate::Writable for CCI_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cci_ctl to value 0"]
impl crate::Resettable for CCI_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `ef_crc_ctrl_5` reader"]
pub struct R(crate::R<EF_CRC_CTRL_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_CRC_CTRL_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_CRC_CTRL_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_CRC_CTRL_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_crc_ctrl_5` writer"]
pub struct W(crate::W<EF_CRC_CTRL_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_CRC_CTRL_5_SPEC>;
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
impl From<crate::W<EF_CRC_CTRL_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_CRC_CTRL_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_crc_dout` reader - "]
pub type EF_CRC_DOUT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_crc_dout(&self) -> EF_CRC_DOUT_R {
        EF_CRC_DOUT_R::new(self.bits)
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
#[doc = "ef_crc_ctrl_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_5](index.html) module"]
pub struct EF_CRC_CTRL_5_SPEC;
impl crate::RegisterSpec for EF_CRC_CTRL_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_crc_ctrl_5::R](R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_5::W](W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_crc_ctrl_5 to value 0xffff_ffff"]
impl crate::Resettable for EF_CRC_CTRL_5_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

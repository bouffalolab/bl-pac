#[doc = "Register `pa_reg_ctrl_hw1` reader"]
pub struct R(crate::R<PA_REG_CTRL_HW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_REG_CTRL_HW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA_REG_CTRL_HW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA_REG_CTRL_HW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pa_reg_ctrl_hw1` writer"]
pub struct W(crate::W<PA_REG_CTRL_HW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_REG_CTRL_HW1_SPEC>;
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
impl From<crate::W<PA_REG_CTRL_HW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_REG_CTRL_HW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_iet_11n` reader - "]
pub type PA_IET_11N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_iet_11n` writer - "]
pub type PA_IET_11N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_REG_CTRL_HW1_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vbcore_11n` reader - "]
pub type PA_VBCORE_11N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vbcore_11n` writer - "]
pub type PA_VBCORE_11N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_REG_CTRL_HW1_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vbcas_11n` reader - "]
pub type PA_VBCAS_11N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vbcas_11n` writer - "]
pub type PA_VBCAS_11N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_REG_CTRL_HW1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11n(&self) -> PA_IET_11N_R {
        PA_IET_11N_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11n(&self) -> PA_VBCORE_11N_R {
        PA_VBCORE_11N_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11n(&self) -> PA_VBCAS_11N_R {
        PA_VBCAS_11N_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn pa_iet_11n(&mut self) -> PA_IET_11N_W<12> {
        PA_IET_11N_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn pa_vbcore_11n(&mut self) -> PA_VBCORE_11N_W<16> {
        PA_VBCORE_11N_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn pa_vbcas_11n(&mut self) -> PA_VBCAS_11N_W<20> {
        PA_VBCAS_11N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pa_reg_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_reg_ctrl_hw1](index.html) module"]
pub struct PA_REG_CTRL_HW1_SPEC;
impl crate::RegisterSpec for PA_REG_CTRL_HW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_reg_ctrl_hw1::R](R) reader structure"]
impl crate::Readable for PA_REG_CTRL_HW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_reg_ctrl_hw1::W](W) writer structure"]
impl crate::Writable for PA_REG_CTRL_HW1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pa_reg_ctrl_hw1 to value 0"]
impl crate::Resettable for PA_REG_CTRL_HW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

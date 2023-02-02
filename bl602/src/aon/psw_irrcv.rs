#[doc = "Register `psw_irrcv` reader"]
pub struct R(crate::R<PSW_IRRCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSW_IRRCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSW_IRRCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSW_IRRCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psw_irrcv` writer"]
pub struct W(crate::W<PSW_IRRCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSW_IRRCV_SPEC>;
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
impl From<crate::W<PSW_IRRCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSW_IRRCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_ir_psw_aon` reader - "]
pub type PU_IR_PSW_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_ir_psw_aon` writer - "]
pub type PU_IR_PSW_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSW_IRRCV_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ir_psw_aon(&self) -> PU_IR_PSW_AON_R {
        PU_IR_PSW_AON_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ir_psw_aon(&mut self) -> PU_IR_PSW_AON_W<0> {
        PU_IR_PSW_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psw_irrcv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psw_irrcv](index.html) module"]
pub struct PSW_IRRCV_SPEC;
impl crate::RegisterSpec for PSW_IRRCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psw_irrcv::R](R) reader structure"]
impl crate::Readable for PSW_IRRCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psw_irrcv::W](W) writer structure"]
impl crate::Writable for PSW_IRRCV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psw_irrcv to value 0"]
impl crate::Resettable for PSW_IRRCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

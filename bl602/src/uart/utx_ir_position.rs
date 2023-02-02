#[doc = "Register `utx_ir_position` reader"]
pub struct R(crate::R<UTX_IR_POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UTX_IR_POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UTX_IR_POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UTX_IR_POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `utx_ir_position` writer"]
pub struct W(crate::W<UTX_IR_POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UTX_IR_POSITION_SPEC>;
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
impl From<crate::W<UTX_IR_POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UTX_IR_POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_utx_ir_pos_s` reader - "]
pub type CR_UTX_IR_POS_S_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_utx_ir_pos_s` writer - "]
pub type CR_UTX_IR_POS_S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UTX_IR_POSITION_SPEC, u16, u16, 16, O>;
#[doc = "Field `cr_utx_ir_pos_p` reader - "]
pub type CR_UTX_IR_POS_P_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_utx_ir_pos_p` writer - "]
pub type CR_UTX_IR_POS_P_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UTX_IR_POSITION_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_s(&self) -> CR_UTX_IR_POS_S_R {
        CR_UTX_IR_POS_S_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_p(&self) -> CR_UTX_IR_POS_P_R {
        CR_UTX_IR_POS_P_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_ir_pos_s(&mut self) -> CR_UTX_IR_POS_S_W<0> {
        CR_UTX_IR_POS_S_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_ir_pos_p(&mut self) -> CR_UTX_IR_POS_P_W<16> {
        CR_UTX_IR_POS_P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "utx_ir_position.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utx_ir_position](index.html) module"]
pub struct UTX_IR_POSITION_SPEC;
impl crate::RegisterSpec for UTX_IR_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [utx_ir_position::R](R) reader structure"]
impl crate::Readable for UTX_IR_POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [utx_ir_position::W](W) writer structure"]
impl crate::Writable for UTX_IR_POSITION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets utx_ir_position to value 0x009f_0070"]
impl crate::Resettable for UTX_IR_POSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0x009f_0070;
}

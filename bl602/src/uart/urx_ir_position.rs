#[doc = "Register `urx_ir_position` reader"]
pub struct R(crate::R<URX_IR_POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<URX_IR_POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<URX_IR_POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<URX_IR_POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `urx_ir_position` writer"]
pub struct W(crate::W<URX_IR_POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<URX_IR_POSITION_SPEC>;
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
impl From<crate::W<URX_IR_POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<URX_IR_POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_urx_ir_pos_s` reader - "]
pub type CR_URX_IR_POS_S_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_urx_ir_pos_s` writer - "]
pub type CR_URX_IR_POS_S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, URX_IR_POSITION_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_urx_ir_pos_s(&self) -> CR_URX_IR_POS_S_R {
        CR_URX_IR_POS_S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_ir_pos_s(&mut self) -> CR_URX_IR_POS_S_W<0> {
        CR_URX_IR_POS_S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "urx_ir_position.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [urx_ir_position](index.html) module"]
pub struct URX_IR_POSITION_SPEC;
impl crate::RegisterSpec for URX_IR_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [urx_ir_position::R](R) reader structure"]
impl crate::Readable for URX_IR_POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [urx_ir_position::W](W) writer structure"]
impl crate::Writable for URX_IR_POSITION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets urx_ir_position to value 0x6f"]
impl crate::Resettable for URX_IR_POSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0x6f;
}

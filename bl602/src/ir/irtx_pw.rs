#[doc = "Register `irtx_pw` reader"]
pub struct R(crate::R<IRTX_PW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRTX_PW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRTX_PW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRTX_PW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irtx_pw` writer"]
pub struct W(crate::W<IRTX_PW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRTX_PW_SPEC>;
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
impl From<crate::W<IRTX_PW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRTX_PW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irtx_logic0_ph0_w` reader - "]
pub type CR_IRTX_LOGIC0_PH0_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_logic0_ph0_w` writer - "]
pub type CR_IRTX_LOGIC0_PH0_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PW_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_irtx_logic0_ph1_w` reader - "]
pub type CR_IRTX_LOGIC0_PH1_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_logic0_ph1_w` writer - "]
pub type CR_IRTX_LOGIC0_PH1_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PW_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_irtx_logic1_ph0_w` reader - "]
pub type CR_IRTX_LOGIC1_PH0_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_logic1_ph0_w` writer - "]
pub type CR_IRTX_LOGIC1_PH0_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PW_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_irtx_logic1_ph1_w` reader - "]
pub type CR_IRTX_LOGIC1_PH1_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_logic1_ph1_w` writer - "]
pub type CR_IRTX_LOGIC1_PH1_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PW_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_irtx_head_ph0_w` reader - "]
pub type CR_IRTX_HEAD_PH0_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_head_ph0_w` writer - "]
pub type CR_IRTX_HEAD_PH0_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PW_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_irtx_head_ph1_w` reader - "]
pub type CR_IRTX_HEAD_PH1_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_head_ph1_w` writer - "]
pub type CR_IRTX_HEAD_PH1_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PW_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_irtx_tail_ph0_w` reader - "]
pub type CR_IRTX_TAIL_PH0_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_tail_ph0_w` writer - "]
pub type CR_IRTX_TAIL_PH0_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PW_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_irtx_tail_ph1_w` reader - "]
pub type CR_IRTX_TAIL_PH1_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_irtx_tail_ph1_w` writer - "]
pub type CR_IRTX_TAIL_PH1_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_PW_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph0_w(&self) -> CR_IRTX_LOGIC0_PH0_W_R {
        CR_IRTX_LOGIC0_PH0_W_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph1_w(&self) -> CR_IRTX_LOGIC0_PH1_W_R {
        CR_IRTX_LOGIC0_PH1_W_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph0_w(&self) -> CR_IRTX_LOGIC1_PH0_W_R {
        CR_IRTX_LOGIC1_PH0_W_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph1_w(&self) -> CR_IRTX_LOGIC1_PH1_W_R {
        CR_IRTX_LOGIC1_PH1_W_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_irtx_head_ph0_w(&self) -> CR_IRTX_HEAD_PH0_W_R {
        CR_IRTX_HEAD_PH0_W_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_irtx_head_ph1_w(&self) -> CR_IRTX_HEAD_PH1_W_R {
        CR_IRTX_HEAD_PH1_W_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph0_w(&self) -> CR_IRTX_TAIL_PH0_W_R {
        CR_IRTX_TAIL_PH0_W_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph1_w(&self) -> CR_IRTX_TAIL_PH1_W_R {
        CR_IRTX_TAIL_PH1_W_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_logic0_ph0_w(&mut self) -> CR_IRTX_LOGIC0_PH0_W_W<0> {
        CR_IRTX_LOGIC0_PH0_W_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_logic0_ph1_w(&mut self) -> CR_IRTX_LOGIC0_PH1_W_W<4> {
        CR_IRTX_LOGIC0_PH1_W_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_logic1_ph0_w(&mut self) -> CR_IRTX_LOGIC1_PH0_W_W<8> {
        CR_IRTX_LOGIC1_PH0_W_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_logic1_ph1_w(&mut self) -> CR_IRTX_LOGIC1_PH1_W_W<12> {
        CR_IRTX_LOGIC1_PH1_W_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_head_ph0_w(&mut self) -> CR_IRTX_HEAD_PH0_W_W<16> {
        CR_IRTX_HEAD_PH0_W_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_head_ph1_w(&mut self) -> CR_IRTX_HEAD_PH1_W_W<20> {
        CR_IRTX_HEAD_PH1_W_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_tail_ph0_w(&mut self) -> CR_IRTX_TAIL_PH0_W_W<24> {
        CR_IRTX_TAIL_PH0_W_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_irtx_tail_ph1_w(&mut self) -> CR_IRTX_TAIL_PH1_W_W<28> {
        CR_IRTX_TAIL_PH1_W_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irtx_pw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_pw](index.html) module"]
pub struct IRTX_PW_SPEC;
impl crate::RegisterSpec for IRTX_PW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irtx_pw::R](R) reader structure"]
impl crate::Readable for IRTX_PW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irtx_pw::W](W) writer structure"]
impl crate::Writable for IRTX_PW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irtx_pw to value 0x007f_2000"]
impl crate::Resettable for IRTX_PW_SPEC {
    const RESET_VALUE: Self::Ux = 0x007f_2000;
}

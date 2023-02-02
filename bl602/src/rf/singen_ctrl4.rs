#[doc = "Register `singen_ctrl4` reader"]
pub struct R(crate::R<SINGEN_CTRL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGEN_CTRL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGEN_CTRL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGEN_CTRL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `singen_ctrl4` writer"]
pub struct W(crate::W<SINGEN_CTRL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGEN_CTRL4_SPEC>;
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
impl From<crate::W<SINGEN_CTRL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGEN_CTRL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `singen_fix_q` reader - "]
pub type SINGEN_FIX_Q_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_fix_q` writer - "]
pub type SINGEN_FIX_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGEN_CTRL4_SPEC, u16, u16, 12, O>;
#[doc = "Field `singen_fix_en_q` reader - "]
pub type SINGEN_FIX_EN_Q_R = crate::BitReader<bool>;
#[doc = "Field `singen_fix_en_q` writer - "]
pub type SINGEN_FIX_EN_Q_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINGEN_CTRL4_SPEC, bool, O>;
#[doc = "Field `singen_fix_i` reader - "]
pub type SINGEN_FIX_I_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_fix_i` writer - "]
pub type SINGEN_FIX_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGEN_CTRL4_SPEC, u16, u16, 12, O>;
#[doc = "Field `singen_fix_en_i` reader - "]
pub type SINGEN_FIX_EN_I_R = crate::BitReader<bool>;
#[doc = "Field `singen_fix_en_i` writer - "]
pub type SINGEN_FIX_EN_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINGEN_CTRL4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn singen_fix_q(&self) -> SINGEN_FIX_Q_R {
        SINGEN_FIX_Q_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn singen_fix_en_q(&self) -> SINGEN_FIX_EN_Q_R {
        SINGEN_FIX_EN_Q_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn singen_fix_i(&self) -> SINGEN_FIX_I_R {
        SINGEN_FIX_I_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_fix_en_i(&self) -> SINGEN_FIX_EN_I_R {
        SINGEN_FIX_EN_I_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn singen_fix_q(&mut self) -> SINGEN_FIX_Q_W<0> {
        SINGEN_FIX_Q_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn singen_fix_en_q(&mut self) -> SINGEN_FIX_EN_Q_W<12> {
        SINGEN_FIX_EN_Q_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn singen_fix_i(&mut self) -> SINGEN_FIX_I_W<16> {
        SINGEN_FIX_I_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn singen_fix_en_i(&mut self) -> SINGEN_FIX_EN_I_W<28> {
        SINGEN_FIX_EN_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "singen_ctrl4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl4](index.html) module"]
pub struct SINGEN_CTRL4_SPEC;
impl crate::RegisterSpec for SINGEN_CTRL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singen_ctrl4::R](R) reader structure"]
impl crate::Readable for SINGEN_CTRL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singen_ctrl4::W](W) writer structure"]
impl crate::Writable for SINGEN_CTRL4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets singen_ctrl4 to value 0"]
impl crate::Resettable for SINGEN_CTRL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

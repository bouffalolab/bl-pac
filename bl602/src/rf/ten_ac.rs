#[doc = "Register `ten_ac` reader"]
pub struct R(crate::R<TEN_AC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEN_AC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEN_AC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEN_AC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ten_ac` writer"]
pub struct W(crate::W<TEN_AC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEN_AC_SPEC>;
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
impl From<crate::W<TEN_AC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEN_AC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `atest_op_cc` reader - "]
pub type ATEST_OP_CC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `atest_op_cc` writer - "]
pub type ATEST_OP_CC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEN_AC_SPEC, u8, u8, 4, O>;
#[doc = "Field `atest_dac_en` reader - "]
pub type ATEST_DAC_EN_R = crate::BitReader<bool>;
#[doc = "Field `atest_dac_en` writer - "]
pub type ATEST_DAC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_AC_SPEC, bool, O>;
#[doc = "Field `atest_in_trx_sw` reader - "]
pub type ATEST_IN_TRX_SW_R = crate::BitReader<bool>;
#[doc = "Field `atest_in_trx_sw` writer - "]
pub type ATEST_IN_TRX_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_AC_SPEC, bool, O>;
#[doc = "Field `atest_in_en` reader - "]
pub type ATEST_IN_EN_R = crate::BitReader<bool>;
#[doc = "Field `atest_in_en` writer - "]
pub type ATEST_IN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_AC_SPEC, bool, O>;
#[doc = "Field `atest_gain_r9` reader - "]
pub type ATEST_GAIN_R9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `atest_gain_r9` writer - "]
pub type ATEST_GAIN_R9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEN_AC_SPEC, u8, u8, 2, O>;
#[doc = "Field `atest_gain_r8` reader - "]
pub type ATEST_GAIN_R8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `atest_gain_r8` writer - "]
pub type ATEST_GAIN_R8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEN_AC_SPEC, u8, u8, 2, O>;
#[doc = "Field `atest_gain_r7` reader - "]
pub type ATEST_GAIN_R7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `atest_gain_r7` writer - "]
pub type ATEST_GAIN_R7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEN_AC_SPEC, u8, u8, 2, O>;
#[doc = "Field `atest_gain_r6` reader - "]
pub type ATEST_GAIN_R6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `atest_gain_r6` writer - "]
pub type ATEST_GAIN_R6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEN_AC_SPEC, u8, u8, 2, O>;
#[doc = "Field `atest_gain_r5` reader - "]
pub type ATEST_GAIN_R5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `atest_gain_r5` writer - "]
pub type ATEST_GAIN_R5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEN_AC_SPEC, u8, u8, 3, O>;
#[doc = "Field `atest_out_en_q` reader - "]
pub type ATEST_OUT_EN_Q_R = crate::BitReader<bool>;
#[doc = "Field `atest_out_en_q` writer - "]
pub type ATEST_OUT_EN_Q_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_AC_SPEC, bool, O>;
#[doc = "Field `atest_out_en_i` reader - "]
pub type ATEST_OUT_EN_I_R = crate::BitReader<bool>;
#[doc = "Field `atest_out_en_i` writer - "]
pub type ATEST_OUT_EN_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_AC_SPEC, bool, O>;
#[doc = "Field `atest_in_en_q` reader - "]
pub type ATEST_IN_EN_Q_R = crate::BitReader<bool>;
#[doc = "Field `atest_in_en_q` writer - "]
pub type ATEST_IN_EN_Q_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_AC_SPEC, bool, O>;
#[doc = "Field `atest_in_en_i` reader - "]
pub type ATEST_IN_EN_I_R = crate::BitReader<bool>;
#[doc = "Field `atest_in_en_i` writer - "]
pub type ATEST_IN_EN_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEN_AC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn atest_op_cc(&self) -> ATEST_OP_CC_R {
        ATEST_OP_CC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn atest_dac_en(&self) -> ATEST_DAC_EN_R {
        ATEST_DAC_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn atest_in_trx_sw(&self) -> ATEST_IN_TRX_SW_R {
        ATEST_IN_TRX_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn atest_in_en(&self) -> ATEST_IN_EN_R {
        ATEST_IN_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn atest_gain_r9(&self) -> ATEST_GAIN_R9_R {
        ATEST_GAIN_R9_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn atest_gain_r8(&self) -> ATEST_GAIN_R8_R {
        ATEST_GAIN_R8_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn atest_gain_r7(&self) -> ATEST_GAIN_R7_R {
        ATEST_GAIN_R7_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn atest_gain_r6(&self) -> ATEST_GAIN_R6_R {
        ATEST_GAIN_R6_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn atest_gain_r5(&self) -> ATEST_GAIN_R5_R {
        ATEST_GAIN_R5_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn atest_out_en_q(&self) -> ATEST_OUT_EN_Q_R {
        ATEST_OUT_EN_Q_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn atest_out_en_i(&self) -> ATEST_OUT_EN_I_R {
        ATEST_OUT_EN_I_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn atest_in_en_q(&self) -> ATEST_IN_EN_Q_R {
        ATEST_IN_EN_Q_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn atest_in_en_i(&self) -> ATEST_IN_EN_I_R {
        ATEST_IN_EN_I_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn atest_op_cc(&mut self) -> ATEST_OP_CC_W<0> {
        ATEST_OP_CC_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn atest_dac_en(&mut self) -> ATEST_DAC_EN_W<4> {
        ATEST_DAC_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn atest_in_trx_sw(&mut self) -> ATEST_IN_TRX_SW_W<5> {
        ATEST_IN_TRX_SW_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn atest_in_en(&mut self) -> ATEST_IN_EN_W<6> {
        ATEST_IN_EN_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn atest_gain_r9(&mut self) -> ATEST_GAIN_R9_W<8> {
        ATEST_GAIN_R9_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn atest_gain_r8(&mut self) -> ATEST_GAIN_R8_W<10> {
        ATEST_GAIN_R8_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn atest_gain_r7(&mut self) -> ATEST_GAIN_R7_W<12> {
        ATEST_GAIN_R7_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn atest_gain_r6(&mut self) -> ATEST_GAIN_R6_W<14> {
        ATEST_GAIN_R6_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn atest_gain_r5(&mut self) -> ATEST_GAIN_R5_W<16> {
        ATEST_GAIN_R5_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn atest_out_en_q(&mut self) -> ATEST_OUT_EN_Q_W<20> {
        ATEST_OUT_EN_Q_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn atest_out_en_i(&mut self) -> ATEST_OUT_EN_I_W<21> {
        ATEST_OUT_EN_I_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn atest_in_en_q(&mut self) -> ATEST_IN_EN_Q_W<22> {
        ATEST_IN_EN_Q_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn atest_in_en_i(&mut self) -> ATEST_IN_EN_I_W<23> {
        ATEST_IN_EN_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ac test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ten_ac](index.html) module"]
pub struct TEN_AC_SPEC;
impl crate::RegisterSpec for TEN_AC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ten_ac::R](R) reader structure"]
impl crate::Readable for TEN_AC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ten_ac::W](W) writer structure"]
impl crate::Writable for TEN_AC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ten_ac to value 0"]
impl crate::Resettable for TEN_AC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

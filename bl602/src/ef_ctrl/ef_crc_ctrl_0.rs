#[doc = "Register `ef_crc_ctrl_0` reader"]
pub struct R(crate::R<EF_CRC_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_CRC_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_CRC_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_CRC_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_crc_ctrl_0` writer"]
pub struct W(crate::W<EF_CRC_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_CRC_CTRL_0_SPEC>;
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
impl From<crate::W<EF_CRC_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_CRC_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_crc_busy` reader - "]
pub type EF_CRC_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_trig` reader - "]
pub type EF_CRC_TRIG_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_trig` writer - "]
pub type EF_CRC_TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CRC_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_crc_en` reader - "]
pub type EF_CRC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_en` writer - "]
pub type EF_CRC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CRC_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_crc_mode` reader - "]
pub type EF_CRC_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_mode` writer - "]
pub type EF_CRC_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CRC_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_crc_error` reader - "]
pub type EF_CRC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_dout_inv_en` reader - "]
pub type EF_CRC_DOUT_INV_EN_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_dout_inv_en` writer - "]
pub type EF_CRC_DOUT_INV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_CRC_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_crc_dout_endian` reader - "]
pub type EF_CRC_DOUT_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_dout_endian` writer - "]
pub type EF_CRC_DOUT_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_CRC_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_crc_din_endian` reader - "]
pub type EF_CRC_DIN_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_din_endian` writer - "]
pub type EF_CRC_DIN_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_CRC_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_crc_int` reader - "]
pub type EF_CRC_INT_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_int_clr` reader - "]
pub type EF_CRC_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_int_clr` writer - "]
pub type EF_CRC_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CRC_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_crc_int_set` reader - "]
pub type EF_CRC_INT_SET_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_int_set` writer - "]
pub type EF_CRC_INT_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CRC_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_crc_lock` reader - "]
pub type EF_CRC_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `ef_crc_lock` writer - "]
pub type EF_CRC_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CRC_CTRL_0_SPEC, bool, O>;
#[doc = "Field `ef_crc_slp_n` reader - "]
pub type EF_CRC_SLP_N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ef_crc_slp_n` writer - "]
pub type EF_CRC_SLP_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_CRC_CTRL_0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ef_crc_busy(&self) -> EF_CRC_BUSY_R {
        EF_CRC_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_crc_trig(&self) -> EF_CRC_TRIG_R {
        EF_CRC_TRIG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_crc_en(&self) -> EF_CRC_EN_R {
        EF_CRC_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_crc_mode(&self) -> EF_CRC_MODE_R {
        EF_CRC_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_crc_error(&self) -> EF_CRC_ERROR_R {
        EF_CRC_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_crc_dout_inv_en(&self) -> EF_CRC_DOUT_INV_EN_R {
        EF_CRC_DOUT_INV_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_crc_dout_endian(&self) -> EF_CRC_DOUT_ENDIAN_R {
        EF_CRC_DOUT_ENDIAN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_crc_din_endian(&self) -> EF_CRC_DIN_ENDIAN_R {
        EF_CRC_DIN_ENDIAN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ef_crc_int(&self) -> EF_CRC_INT_R {
        EF_CRC_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ef_crc_int_clr(&self) -> EF_CRC_INT_CLR_R {
        EF_CRC_INT_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_crc_int_set(&self) -> EF_CRC_INT_SET_R {
        EF_CRC_INT_SET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_crc_lock(&self) -> EF_CRC_LOCK_R {
        EF_CRC_LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ef_crc_slp_n(&self) -> EF_CRC_SLP_N_R {
        EF_CRC_SLP_N_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ef_crc_trig(&mut self) -> EF_CRC_TRIG_W<1> {
        EF_CRC_TRIG_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ef_crc_en(&mut self) -> EF_CRC_EN_W<2> {
        EF_CRC_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ef_crc_mode(&mut self) -> EF_CRC_MODE_W<3> {
        EF_CRC_MODE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ef_crc_dout_inv_en(&mut self) -> EF_CRC_DOUT_INV_EN_W<5> {
        EF_CRC_DOUT_INV_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ef_crc_dout_endian(&mut self) -> EF_CRC_DOUT_ENDIAN_W<6> {
        EF_CRC_DOUT_ENDIAN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ef_crc_din_endian(&mut self) -> EF_CRC_DIN_ENDIAN_W<7> {
        EF_CRC_DIN_ENDIAN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ef_crc_int_clr(&mut self) -> EF_CRC_INT_CLR_W<9> {
        EF_CRC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ef_crc_int_set(&mut self) -> EF_CRC_INT_SET_W<10> {
        EF_CRC_INT_SET_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ef_crc_lock(&mut self) -> EF_CRC_LOCK_W<11> {
        EF_CRC_LOCK_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn ef_crc_slp_n(&mut self) -> EF_CRC_SLP_N_W<16> {
        EF_CRC_SLP_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_crc_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_0](index.html) module"]
pub struct EF_CRC_CTRL_0_SPEC;
impl crate::RegisterSpec for EF_CRC_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_crc_ctrl_0::R](R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_0::W](W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_crc_ctrl_0 to value 0x00ff_0224"]
impl crate::Resettable for EF_CRC_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_0224;
}

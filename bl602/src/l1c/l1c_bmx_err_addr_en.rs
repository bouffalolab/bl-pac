#[doc = "Register `l1c_bmx_err_addr_en` reader"]
pub struct R(crate::R<L1C_BMX_ERR_ADDR_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1C_BMX_ERR_ADDR_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1C_BMX_ERR_ADDR_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1C_BMX_ERR_ADDR_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `l1c_bmx_err_addr_en` writer"]
pub struct W(crate::W<L1C_BMX_ERR_ADDR_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1C_BMX_ERR_ADDR_EN_SPEC>;
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
impl From<crate::W<L1C_BMX_ERR_ADDR_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1C_BMX_ERR_ADDR_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `l1c_bmx_err_addr_dis` reader - "]
pub type L1C_BMX_ERR_ADDR_DIS_R = crate::BitReader<bool>;
#[doc = "Field `l1c_bmx_err_addr_dis` writer - "]
pub type L1C_BMX_ERR_ADDR_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1C_BMX_ERR_ADDR_EN_SPEC, bool, O>;
#[doc = "Field `l1c_bmx_err_dec` reader - "]
pub type L1C_BMX_ERR_DEC_R = crate::BitReader<bool>;
#[doc = "Field `l1c_bmx_err_tz` reader - "]
pub type L1C_BMX_ERR_TZ_R = crate::BitReader<bool>;
#[doc = "Field `l1c_hsel_option` reader - "]
pub type L1C_HSEL_OPTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `l1c_hsel_option` writer - "]
pub type L1C_HSEL_OPTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L1C_BMX_ERR_ADDR_EN_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr_dis(&self) -> L1C_BMX_ERR_ADDR_DIS_R {
        L1C_BMX_ERR_ADDR_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn l1c_bmx_err_dec(&self) -> L1C_BMX_ERR_DEC_R {
        L1C_BMX_ERR_DEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn l1c_bmx_err_tz(&self) -> L1C_BMX_ERR_TZ_R {
        L1C_BMX_ERR_TZ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn l1c_hsel_option(&self) -> L1C_HSEL_OPTION_R {
        L1C_HSEL_OPTION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_bmx_err_addr_dis(&mut self) -> L1C_BMX_ERR_ADDR_DIS_W<0> {
        L1C_BMX_ERR_ADDR_DIS_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_hsel_option(&mut self) -> L1C_HSEL_OPTION_W<16> {
        L1C_HSEL_OPTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "l1c_bmx_err_addr_en.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_bmx_err_addr_en](index.html) module"]
pub struct L1C_BMX_ERR_ADDR_EN_SPEC;
impl crate::RegisterSpec for L1C_BMX_ERR_ADDR_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1c_bmx_err_addr_en::R](R) reader structure"]
impl crate::Readable for L1C_BMX_ERR_ADDR_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1c_bmx_err_addr_en::W](W) writer structure"]
impl crate::Writable for L1C_BMX_ERR_ADDR_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets l1c_bmx_err_addr_en to value 0"]
impl crate::Resettable for L1C_BMX_ERR_ADDR_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

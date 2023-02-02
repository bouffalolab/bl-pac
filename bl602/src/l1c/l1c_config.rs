#[doc = "Register `l1c_config` reader"]
pub struct R(crate::R<L1C_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1C_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1C_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1C_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `l1c_config` writer"]
pub struct W(crate::W<L1C_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1C_CONFIG_SPEC>;
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
impl From<crate::W<L1C_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1C_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `l1c_cacheable` reader - "]
pub type L1C_CACHEABLE_R = crate::BitReader<bool>;
#[doc = "Field `l1c_cacheable` writer - "]
pub type L1C_CACHEABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1C_CONFIG_SPEC, bool, O>;
#[doc = "Field `l1c_cnt_en` reader - "]
pub type L1C_CNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `l1c_cnt_en` writer - "]
pub type L1C_CNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1C_CONFIG_SPEC, bool, O>;
#[doc = "Field `l1c_invalid_en` reader - "]
pub type L1C_INVALID_EN_R = crate::BitReader<bool>;
#[doc = "Field `l1c_invalid_en` writer - "]
pub type L1C_INVALID_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1C_CONFIG_SPEC, bool, O>;
#[doc = "Field `l1c_invalid_done` reader - "]
pub type L1C_INVALID_DONE_R = crate::BitReader<bool>;
#[doc = "Field `l1c_way_dis` reader - "]
pub type L1C_WAY_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `l1c_way_dis` writer - "]
pub type L1C_WAY_DIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L1C_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `irom_2t_access` reader - "]
pub type IROM_2T_ACCESS_R = crate::BitReader<bool>;
#[doc = "Field `irom_2t_access` writer - "]
pub type IROM_2T_ACCESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1C_CONFIG_SPEC, bool, O>;
#[doc = "Field `l1c_bypass` reader - "]
pub type L1C_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `l1c_bypass` writer - "]
pub type L1C_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1C_CONFIG_SPEC, bool, O>;
#[doc = "Field `l1c_bmx_err_en` reader - "]
pub type L1C_BMX_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `l1c_bmx_err_en` writer - "]
pub type L1C_BMX_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1C_CONFIG_SPEC, bool, O>;
#[doc = "Field `l1c_bmx_arb_mode` reader - "]
pub type L1C_BMX_ARB_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `l1c_bmx_arb_mode` writer - "]
pub type L1C_BMX_ARB_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L1C_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `l1c_bmx_timeout_en` reader - "]
pub type L1C_BMX_TIMEOUT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `l1c_bmx_timeout_en` writer - "]
pub type L1C_BMX_TIMEOUT_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L1C_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `l1c_bmx_busy_option_dis` reader - "]
pub type L1C_BMX_BUSY_OPTION_DIS_R = crate::BitReader<bool>;
#[doc = "Field `l1c_bmx_busy_option_dis` writer - "]
pub type L1C_BMX_BUSY_OPTION_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1C_CONFIG_SPEC, bool, O>;
#[doc = "Field `early_resp_dis` reader - "]
pub type EARLY_RESP_DIS_R = crate::BitReader<bool>;
#[doc = "Field `early_resp_dis` writer - "]
pub type EARLY_RESP_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1C_CONFIG_SPEC, bool, O>;
#[doc = "Field `wrap_dis` reader - "]
pub type WRAP_DIS_R = crate::BitReader<bool>;
#[doc = "Field `wrap_dis` writer - "]
pub type WRAP_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, L1C_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_cacheable(&self) -> L1C_CACHEABLE_R {
        L1C_CACHEABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn l1c_cnt_en(&self) -> L1C_CNT_EN_R {
        L1C_CNT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn l1c_invalid_en(&self) -> L1C_INVALID_EN_R {
        L1C_INVALID_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn l1c_invalid_done(&self) -> L1C_INVALID_DONE_R {
        L1C_INVALID_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn l1c_way_dis(&self) -> L1C_WAY_DIS_R {
        L1C_WAY_DIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irom_2t_access(&self) -> IROM_2T_ACCESS_R {
        IROM_2T_ACCESS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn l1c_bypass(&self) -> L1C_BYPASS_R {
        L1C_BYPASS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn l1c_bmx_err_en(&self) -> L1C_BMX_ERR_EN_R {
        L1C_BMX_ERR_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn l1c_bmx_arb_mode(&self) -> L1C_BMX_ARB_MODE_R {
        L1C_BMX_ARB_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn l1c_bmx_timeout_en(&self) -> L1C_BMX_TIMEOUT_EN_R {
        L1C_BMX_TIMEOUT_EN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn l1c_bmx_busy_option_dis(&self) -> L1C_BMX_BUSY_OPTION_DIS_R {
        L1C_BMX_BUSY_OPTION_DIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn early_resp_dis(&self) -> EARLY_RESP_DIS_R {
        EARLY_RESP_DIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wrap_dis(&self) -> WRAP_DIS_R {
        WRAP_DIS_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_cacheable(&mut self) -> L1C_CACHEABLE_W<0> {
        L1C_CACHEABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_cnt_en(&mut self) -> L1C_CNT_EN_W<1> {
        L1C_CNT_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_invalid_en(&mut self) -> L1C_INVALID_EN_W<2> {
        L1C_INVALID_EN_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_way_dis(&mut self) -> L1C_WAY_DIS_W<8> {
        L1C_WAY_DIS_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn irom_2t_access(&mut self) -> IROM_2T_ACCESS_W<12> {
        IROM_2T_ACCESS_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_bypass(&mut self) -> L1C_BYPASS_W<14> {
        L1C_BYPASS_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_bmx_err_en(&mut self) -> L1C_BMX_ERR_EN_W<15> {
        L1C_BMX_ERR_EN_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_bmx_arb_mode(&mut self) -> L1C_BMX_ARB_MODE_W<16> {
        L1C_BMX_ARB_MODE_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_bmx_timeout_en(&mut self) -> L1C_BMX_TIMEOUT_EN_W<20> {
        L1C_BMX_TIMEOUT_EN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_bmx_busy_option_dis(&mut self) -> L1C_BMX_BUSY_OPTION_DIS_W<24> {
        L1C_BMX_BUSY_OPTION_DIS_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn early_resp_dis(&mut self) -> EARLY_RESP_DIS_W<25> {
        EARLY_RESP_DIS_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_dis(&mut self) -> WRAP_DIS_W<26> {
        WRAP_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "l1c_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_config](index.html) module"]
pub struct L1C_CONFIG_SPEC;
impl crate::RegisterSpec for L1C_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1c_config::R](R) reader structure"]
impl crate::Readable for L1C_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1c_config::W](W) writer structure"]
impl crate::Writable for L1C_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets l1c_config to value 0x0600_0f00"]
impl crate::Resettable for L1C_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600_0f00;
}

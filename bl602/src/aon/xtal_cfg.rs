#[doc = "Register `xtal_cfg` reader"]
pub struct R(crate::R<XTAL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `xtal_cfg` writer"]
pub struct W(crate::W<XTAL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_CFG_SPEC>;
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
impl From<crate::W<XTAL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `xtal_bk_aon` reader - "]
pub type XTAL_BK_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal_bk_aon` writer - "]
pub type XTAL_BK_AON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTAL_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `xtal_capcode_extra_aon` reader - "]
pub type XTAL_CAPCODE_EXTRA_AON_R = crate::BitReader<bool>;
#[doc = "Field `xtal_capcode_extra_aon` writer - "]
pub type XTAL_CAPCODE_EXTRA_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XTAL_CFG_SPEC, bool, O>;
#[doc = "Field `xtal_ext_sel_aon` reader - "]
pub type XTAL_EXT_SEL_AON_R = crate::BitReader<bool>;
#[doc = "Field `xtal_ext_sel_aon` writer - "]
pub type XTAL_EXT_SEL_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTAL_CFG_SPEC, bool, O>;
#[doc = "Field `xtal_buf_en_aon` reader - "]
pub type XTAL_BUF_EN_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal_buf_en_aon` writer - "]
pub type XTAL_BUF_EN_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `xtal_buf_hp_aon` reader - "]
pub type XTAL_BUF_HP_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal_buf_hp_aon` writer - "]
pub type XTAL_BUF_HP_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `xtal_fast_startup_aon` reader - "]
pub type XTAL_FAST_STARTUP_AON_R = crate::BitReader<bool>;
#[doc = "Field `xtal_fast_startup_aon` writer - "]
pub type XTAL_FAST_STARTUP_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XTAL_CFG_SPEC, bool, O>;
#[doc = "Field `xtal_sleep_aon` reader - "]
pub type XTAL_SLEEP_AON_R = crate::BitReader<bool>;
#[doc = "Field `xtal_sleep_aon` writer - "]
pub type XTAL_SLEEP_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTAL_CFG_SPEC, bool, O>;
#[doc = "Field `xtal_amp_ctrl_aon` reader - "]
pub type XTAL_AMP_CTRL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal_amp_ctrl_aon` writer - "]
pub type XTAL_AMP_CTRL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `xtal_capcode_out_aon` reader - "]
pub type XTAL_CAPCODE_OUT_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal_capcode_out_aon` writer - "]
pub type XTAL_CAPCODE_OUT_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `xtal_capcode_in_aon` reader - "]
pub type XTAL_CAPCODE_IN_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal_capcode_in_aon` writer - "]
pub type XTAL_CAPCODE_IN_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `xtal_gm_boost_aon` reader - "]
pub type XTAL_GM_BOOST_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal_gm_boost_aon` writer - "]
pub type XTAL_GM_BOOST_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `xtal_rdy_sel_aon` reader - "]
pub type XTAL_RDY_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal_rdy_sel_aon` writer - "]
pub type XTAL_RDY_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn xtal_bk_aon(&self) -> XTAL_BK_AON_R {
        XTAL_BK_AON_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal_capcode_extra_aon(&self) -> XTAL_CAPCODE_EXTRA_AON_R {
        XTAL_CAPCODE_EXTRA_AON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xtal_ext_sel_aon(&self) -> XTAL_EXT_SEL_AON_R {
        XTAL_EXT_SEL_AON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn xtal_buf_en_aon(&self) -> XTAL_BUF_EN_AON_R {
        XTAL_BUF_EN_AON_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn xtal_buf_hp_aon(&self) -> XTAL_BUF_HP_AON_R {
        XTAL_BUF_HP_AON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xtal_fast_startup_aon(&self) -> XTAL_FAST_STARTUP_AON_R {
        XTAL_FAST_STARTUP_AON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn xtal_sleep_aon(&self) -> XTAL_SLEEP_AON_R {
        XTAL_SLEEP_AON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn xtal_amp_ctrl_aon(&self) -> XTAL_AMP_CTRL_AON_R {
        XTAL_AMP_CTRL_AON_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn xtal_capcode_out_aon(&self) -> XTAL_CAPCODE_OUT_AON_R {
        XTAL_CAPCODE_OUT_AON_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn xtal_capcode_in_aon(&self) -> XTAL_CAPCODE_IN_AON_R {
        XTAL_CAPCODE_IN_AON_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn xtal_gm_boost_aon(&self) -> XTAL_GM_BOOST_AON_R {
        XTAL_GM_BOOST_AON_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_sel_aon(&self) -> XTAL_RDY_SEL_AON_R {
        XTAL_RDY_SEL_AON_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_bk_aon(&mut self) -> XTAL_BK_AON_W<0> {
        XTAL_BK_AON_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_capcode_extra_aon(&mut self) -> XTAL_CAPCODE_EXTRA_AON_W<2> {
        XTAL_CAPCODE_EXTRA_AON_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_ext_sel_aon(&mut self) -> XTAL_EXT_SEL_AON_W<3> {
        XTAL_EXT_SEL_AON_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_buf_en_aon(&mut self) -> XTAL_BUF_EN_AON_W<4> {
        XTAL_BUF_EN_AON_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_buf_hp_aon(&mut self) -> XTAL_BUF_HP_AON_W<8> {
        XTAL_BUF_HP_AON_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_fast_startup_aon(&mut self) -> XTAL_FAST_STARTUP_AON_W<12> {
        XTAL_FAST_STARTUP_AON_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_sleep_aon(&mut self) -> XTAL_SLEEP_AON_W<13> {
        XTAL_SLEEP_AON_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_amp_ctrl_aon(&mut self) -> XTAL_AMP_CTRL_AON_W<14> {
        XTAL_AMP_CTRL_AON_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_capcode_out_aon(&mut self) -> XTAL_CAPCODE_OUT_AON_W<16> {
        XTAL_CAPCODE_OUT_AON_W::new(self)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_capcode_in_aon(&mut self) -> XTAL_CAPCODE_IN_AON_W<22> {
        XTAL_CAPCODE_IN_AON_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_gm_boost_aon(&mut self) -> XTAL_GM_BOOST_AON_W<28> {
        XTAL_GM_BOOST_AON_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_rdy_sel_aon(&mut self) -> XTAL_RDY_SEL_AON_W<30> {
        XTAL_RDY_SEL_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xtal_cfg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_cfg](index.html) module"]
pub struct XTAL_CFG_SPEC;
impl crate::RegisterSpec for XTAL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_cfg::R](R) reader structure"]
impl crate::Readable for XTAL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_cfg::W](W) writer structure"]
impl crate::Writable for XTAL_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets xtal_cfg to value 0xb410_f0f0"]
impl crate::Resettable for XTAL_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0xb410_f0f0;
}

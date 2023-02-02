#[doc = "Register `xtal32k` reader"]
pub struct R(crate::R<XTAL32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `xtal32k` writer"]
pub struct W(crate::W<XTAL32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32K_SPEC>;
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
impl From<crate::W<XTAL32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `xtal32k_ext_sel` reader - "]
pub type XTAL32K_EXT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `xtal32k_ext_sel` writer - "]
pub type XTAL32K_EXT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTAL32K_SPEC, bool, O>;
#[doc = "Field `xtal32k_amp_ctrl` reader - "]
pub type XTAL32K_AMP_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal32k_amp_ctrl` writer - "]
pub type XTAL32K_AMP_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL32K_SPEC, u8, u8, 2, O>;
#[doc = "Field `xtal32k_reg` reader - "]
pub type XTAL32K_REG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal32k_reg` writer - "]
pub type XTAL32K_REG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTAL32K_SPEC, u8, u8, 2, O>;
#[doc = "Field `xtal32k_outbuf_stre` reader - "]
pub type XTAL32K_OUTBUF_STRE_R = crate::BitReader<bool>;
#[doc = "Field `xtal32k_outbuf_stre` writer - "]
pub type XTAL32K_OUTBUF_STRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTAL32K_SPEC, bool, O>;
#[doc = "Field `xtal32k_otf_short` reader - "]
pub type XTAL32K_OTF_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `xtal32k_otf_short` writer - "]
pub type XTAL32K_OTF_SHORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTAL32K_SPEC, bool, O>;
#[doc = "Field `xtal32k_inv_stre` reader - "]
pub type XTAL32K_INV_STRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal32k_inv_stre` writer - "]
pub type XTAL32K_INV_STRE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL32K_SPEC, u8, u8, 2, O>;
#[doc = "Field `xtal32k_capbank` reader - "]
pub type XTAL32K_CAPBANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal32k_capbank` writer - "]
pub type XTAL32K_CAPBANK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL32K_SPEC, u8, u8, 6, O>;
#[doc = "Field `xtal32k_ac_cap_short` reader - "]
pub type XTAL32K_AC_CAP_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `xtal32k_ac_cap_short` writer - "]
pub type XTAL32K_AC_CAP_SHORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTAL32K_SPEC, bool, O>;
#[doc = "Field `pu_xtal32k_buf` reader - "]
pub type PU_XTAL32K_BUF_R = crate::BitReader<bool>;
#[doc = "Field `pu_xtal32k_buf` writer - "]
pub type PU_XTAL32K_BUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTAL32K_SPEC, bool, O>;
#[doc = "Field `pu_xtal32k` reader - "]
pub type PU_XTAL32K_R = crate::BitReader<bool>;
#[doc = "Field `pu_xtal32k` writer - "]
pub type PU_XTAL32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTAL32K_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal32k_ext_sel(&self) -> XTAL32K_EXT_SEL_R {
        XTAL32K_EXT_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn xtal32k_amp_ctrl(&self) -> XTAL32K_AMP_CTRL_R {
        XTAL32K_AMP_CTRL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn xtal32k_reg(&self) -> XTAL32K_REG_R {
        XTAL32K_REG_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal32k_outbuf_stre(&self) -> XTAL32K_OUTBUF_STRE_R {
        XTAL32K_OUTBUF_STRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xtal32k_otf_short(&self) -> XTAL32K_OTF_SHORT_R {
        XTAL32K_OTF_SHORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn xtal32k_inv_stre(&self) -> XTAL32K_INV_STRE_R {
        XTAL32K_INV_STRE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    pub fn xtal32k_capbank(&self) -> XTAL32K_CAPBANK_R {
        XTAL32K_CAPBANK_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn xtal32k_ac_cap_short(&self) -> XTAL32K_AC_CAP_SHORT_R {
        XTAL32K_AC_CAP_SHORT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_xtal32k_buf(&self) -> PU_XTAL32K_BUF_R {
        PU_XTAL32K_BUF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_xtal32k(&self) -> PU_XTAL32K_R {
        PU_XTAL32K_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_ext_sel(&mut self) -> XTAL32K_EXT_SEL_W<2> {
        XTAL32K_EXT_SEL_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_amp_ctrl(&mut self) -> XTAL32K_AMP_CTRL_W<3> {
        XTAL32K_AMP_CTRL_W::new(self)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_reg(&mut self) -> XTAL32K_REG_W<5> {
        XTAL32K_REG_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_outbuf_stre(&mut self) -> XTAL32K_OUTBUF_STRE_W<7> {
        XTAL32K_OUTBUF_STRE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_otf_short(&mut self) -> XTAL32K_OTF_SHORT_W<8> {
        XTAL32K_OTF_SHORT_W::new(self)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_inv_stre(&mut self) -> XTAL32K_INV_STRE_W<9> {
        XTAL32K_INV_STRE_W::new(self)
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_capbank(&mut self) -> XTAL32K_CAPBANK_W<11> {
        XTAL32K_CAPBANK_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_ac_cap_short(&mut self) -> XTAL32K_AC_CAP_SHORT_W<17> {
        XTAL32K_AC_CAP_SHORT_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pu_xtal32k_buf(&mut self) -> PU_XTAL32K_BUF_W<18> {
        PU_XTAL32K_BUF_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pu_xtal32k(&mut self) -> PU_XTAL32K_W<19> {
        PU_XTAL32K_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xtal32k.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k](index.html) module"]
pub struct XTAL32K_SPEC;
impl crate::RegisterSpec for XTAL32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal32k::R](R) reader structure"]
impl crate::Readable for XTAL32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32k::W](W) writer structure"]
impl crate::Writable for XTAL32K_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets xtal32k to value 0x000f_0228"]
impl crate::Resettable for XTAL32K_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_0228;
}

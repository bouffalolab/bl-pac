#[doc = "Register `MBIST_CTL` reader"]
pub struct R(crate::R<MBIST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBIST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBIST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBIST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBIST_CTL` writer"]
pub struct W(crate::W<MBIST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBIST_CTL_SPEC>;
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
impl From<crate::W<MBIST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBIST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irom_mbist_mode` reader - "]
pub type IROM_MBIST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `irom_mbist_mode` writer - "]
pub type IROM_MBIST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_CTL_SPEC, bool, O>;
#[doc = "Field `hsram_mbist_mode` reader - "]
pub type HSRAM_MBIST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `hsram_mbist_mode` writer - "]
pub type HSRAM_MBIST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_CTL_SPEC, bool, O>;
#[doc = "Field `tag_mbist_mode` reader - "]
pub type TAG_MBIST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `tag_mbist_mode` writer - "]
pub type TAG_MBIST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_CTL_SPEC, bool, O>;
#[doc = "Field `ocram_mbist_mode` reader - "]
pub type OCRAM_MBIST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ocram_mbist_mode` writer - "]
pub type OCRAM_MBIST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_CTL_SPEC, bool, O>;
#[doc = "Field `wifi_mbist_mode` reader - "]
pub type WIFI_MBIST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `wifi_mbist_mode` writer - "]
pub type WIFI_MBIST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_CTL_SPEC, bool, O>;
#[doc = "Field `reg_mbist_rst_n` reader - "]
pub type REG_MBIST_RST_N_R = crate::BitReader<bool>;
#[doc = "Field `reg_mbist_rst_n` writer - "]
pub type REG_MBIST_RST_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_mode(&self) -> IROM_MBIST_MODE_R {
        IROM_MBIST_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_mode(&self) -> HSRAM_MBIST_MODE_R {
        HSRAM_MBIST_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_mode(&self) -> TAG_MBIST_MODE_R {
        TAG_MBIST_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_mode(&self) -> OCRAM_MBIST_MODE_R {
        OCRAM_MBIST_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_mode(&self) -> WIFI_MBIST_MODE_R {
        WIFI_MBIST_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_mbist_rst_n(&self) -> REG_MBIST_RST_N_R {
        REG_MBIST_RST_N_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn irom_mbist_mode(&mut self) -> IROM_MBIST_MODE_W<0> {
        IROM_MBIST_MODE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn hsram_mbist_mode(&mut self) -> HSRAM_MBIST_MODE_W<1> {
        HSRAM_MBIST_MODE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tag_mbist_mode(&mut self) -> TAG_MBIST_MODE_W<2> {
        TAG_MBIST_MODE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ocram_mbist_mode(&mut self) -> OCRAM_MBIST_MODE_W<3> {
        OCRAM_MBIST_MODE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_mbist_mode(&mut self) -> WIFI_MBIST_MODE_W<4> {
        WIFI_MBIST_MODE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mbist_rst_n(&mut self) -> REG_MBIST_RST_N_W<31> {
        REG_MBIST_RST_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MBIST_CTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_ctl](index.html) module"]
pub struct MBIST_CTL_SPEC;
impl crate::RegisterSpec for MBIST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbist_ctl::R](R) reader structure"]
impl crate::Readable for MBIST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbist_ctl::W](W) writer structure"]
impl crate::Writable for MBIST_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MBIST_CTL to value 0"]
impl crate::Resettable for MBIST_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `se_pka_0_ctrl_0` reader"]
pub struct R(crate::R<SE_PKA_0_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_PKA_0_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_PKA_0_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_PKA_0_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_pka_0_ctrl_0` writer"]
pub struct W(crate::W<SE_PKA_0_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_PKA_0_CTRL_0_SPEC>;
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
impl From<crate::W<SE_PKA_0_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_PKA_0_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_pka_0_done` reader - "]
pub type SE_PKA_0_DONE_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_done_clr_1t` reader - "]
pub type SE_PKA_0_DONE_CLR_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_done_clr_1t` writer - "]
pub type SE_PKA_0_DONE_CLR_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_PKA_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_pka_0_busy` reader - "]
pub type SE_PKA_0_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_en` reader - "]
pub type SE_PKA_0_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_en` writer - "]
pub type SE_PKA_0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SE_PKA_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_pka_0_prot_md` reader - "]
pub type SE_PKA_0_PROT_MD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_pka_0_prot_md` writer - "]
pub type SE_PKA_0_PROT_MD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_PKA_0_CTRL_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `se_pka_0_int` reader - "]
pub type SE_PKA_0_INT_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_int_clr_1t` reader - "]
pub type SE_PKA_0_INT_CLR_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_int_clr_1t` writer - "]
pub type SE_PKA_0_INT_CLR_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_PKA_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_pka_0_int_set` reader - "]
pub type SE_PKA_0_INT_SET_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_int_set` writer - "]
pub type SE_PKA_0_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_PKA_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_pka_0_int_mask` reader - "]
pub type SE_PKA_0_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_int_mask` writer - "]
pub type SE_PKA_0_INT_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_PKA_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_pka_0_endian` reader - "]
pub type SE_PKA_0_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_endian` writer - "]
pub type SE_PKA_0_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_PKA_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_pka_0_ram_clr_md` reader - "]
pub type SE_PKA_0_RAM_CLR_MD_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_ram_clr_md` writer - "]
pub type SE_PKA_0_RAM_CLR_MD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_PKA_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_pka_0_status_clr_1t` reader - "]
pub type SE_PKA_0_STATUS_CLR_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_0_status_clr_1t` writer - "]
pub type SE_PKA_0_STATUS_CLR_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_PKA_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_pka_0_status` reader - "]
pub type SE_PKA_0_STATUS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_pka_0_done(&self) -> SE_PKA_0_DONE_R {
        SE_PKA_0_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_pka_0_done_clr_1t(&self) -> SE_PKA_0_DONE_CLR_1T_R {
        SE_PKA_0_DONE_CLR_1T_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_pka_0_busy(&self) -> SE_PKA_0_BUSY_R {
        SE_PKA_0_BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_en(&self) -> SE_PKA_0_EN_R {
        SE_PKA_0_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn se_pka_0_prot_md(&self) -> SE_PKA_0_PROT_MD_R {
        SE_PKA_0_PROT_MD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_pka_0_int(&self) -> SE_PKA_0_INT_R {
        SE_PKA_0_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_pka_0_int_clr_1t(&self) -> SE_PKA_0_INT_CLR_1T_R {
        SE_PKA_0_INT_CLR_1T_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_pka_0_int_set(&self) -> SE_PKA_0_INT_SET_R {
        SE_PKA_0_INT_SET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_pka_0_int_mask(&self) -> SE_PKA_0_INT_MASK_R {
        SE_PKA_0_INT_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_0_endian(&self) -> SE_PKA_0_ENDIAN_R {
        SE_PKA_0_ENDIAN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_0_ram_clr_md(&self) -> SE_PKA_0_RAM_CLR_MD_R {
        SE_PKA_0_RAM_CLR_MD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_pka_0_status_clr_1t(&self) -> SE_PKA_0_STATUS_CLR_1T_R {
        SE_PKA_0_STATUS_CLR_1T_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn se_pka_0_status(&self) -> SE_PKA_0_STATUS_R {
        SE_PKA_0_STATUS_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_done_clr_1t(&mut self) -> SE_PKA_0_DONE_CLR_1T_W<1> {
        SE_PKA_0_DONE_CLR_1T_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_en(&mut self) -> SE_PKA_0_EN_W<3> {
        SE_PKA_0_EN_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_prot_md(&mut self) -> SE_PKA_0_PROT_MD_W<4> {
        SE_PKA_0_PROT_MD_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_int_clr_1t(&mut self) -> SE_PKA_0_INT_CLR_1T_W<9> {
        SE_PKA_0_INT_CLR_1T_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_int_set(&mut self) -> SE_PKA_0_INT_SET_W<10> {
        SE_PKA_0_INT_SET_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_int_mask(&mut self) -> SE_PKA_0_INT_MASK_W<11> {
        SE_PKA_0_INT_MASK_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_endian(&mut self) -> SE_PKA_0_ENDIAN_W<12> {
        SE_PKA_0_ENDIAN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_ram_clr_md(&mut self) -> SE_PKA_0_RAM_CLR_MD_W<13> {
        SE_PKA_0_RAM_CLR_MD_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn se_pka_0_status_clr_1t(&mut self) -> SE_PKA_0_STATUS_CLR_1T_W<16> {
        SE_PKA_0_STATUS_CLR_1T_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_pka_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_ctrl_0](index.html) module"]
pub struct SE_PKA_0_CTRL_0_SPEC;
impl crate::RegisterSpec for SE_PKA_0_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_pka_0_ctrl_0::R](R) reader structure"]
impl crate::Readable for SE_PKA_0_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_pka_0_ctrl_0::W](W) writer structure"]
impl crate::Writable for SE_PKA_0_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_pka_0_ctrl_0 to value 0"]
impl crate::Resettable for SE_PKA_0_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

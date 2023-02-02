#[doc = "Register `bg_sys_top` reader"]
pub struct R(crate::R<BG_SYS_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_SYS_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BG_SYS_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BG_SYS_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bg_sys_top` writer"]
pub struct W(crate::W<BG_SYS_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BG_SYS_TOP_SPEC>;
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
impl From<crate::W<BG_SYS_TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BG_SYS_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pmip_resv` reader - "]
pub type PMIP_RESV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pmip_resv` writer - "]
pub type PMIP_RESV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_SYS_TOP_SPEC, u8, u8, 8, O>;
#[doc = "Field `pu_bg_sys_aon` reader - "]
pub type PU_BG_SYS_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_bg_sys_aon` writer - "]
pub type PU_BG_SYS_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BG_SYS_TOP_SPEC, bool, O>;
#[doc = "Field `bg_sys_start_ctrl_aon` reader - "]
pub type BG_SYS_START_CTRL_AON_R = crate::BitReader<bool>;
#[doc = "Field `bg_sys_start_ctrl_aon` writer - "]
pub type BG_SYS_START_CTRL_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BG_SYS_TOP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pmip_resv(&self) -> PMIP_RESV_R {
        PMIP_RESV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&self) -> PU_BG_SYS_AON_R {
        PU_BG_SYS_AON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bg_sys_start_ctrl_aon(&self) -> BG_SYS_START_CTRL_AON_R {
        BG_SYS_START_CTRL_AON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pmip_resv(&mut self) -> PMIP_RESV_W<0> {
        PMIP_RESV_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pu_bg_sys_aon(&mut self) -> PU_BG_SYS_AON_W<8> {
        PU_BG_SYS_AON_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bg_sys_start_ctrl_aon(&mut self) -> BG_SYS_START_CTRL_AON_W<12> {
        BG_SYS_START_CTRL_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bg_sys_top.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_sys_top](index.html) module"]
pub struct BG_SYS_TOP_SPEC;
impl crate::RegisterSpec for BG_SYS_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_sys_top::R](R) reader structure"]
impl crate::Readable for BG_SYS_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bg_sys_top::W](W) writer structure"]
impl crate::Writable for BG_SYS_TOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bg_sys_top to value 0x1100"]
impl crate::Resettable for BG_SYS_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0x1100;
}

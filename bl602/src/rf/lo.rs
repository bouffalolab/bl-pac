#[doc = "Register `lo` reader"]
pub struct R(crate::R<LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo` writer"]
pub struct W(crate::W<LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SPEC>;
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
impl From<crate::W<LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_lf_rz_hw` reader - "]
pub type LO_LF_RZ_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_rz_hw` writer - "]
pub type LO_LF_RZ_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LO_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_r4_hw` reader - "]
pub type LO_LF_R4_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_r4_hw` writer - "]
pub type LO_LF_R4_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LO_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_cz_hw` reader - "]
pub type LO_LF_CZ_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_cz_hw` writer - "]
pub type LO_LF_CZ_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LO_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_rz` reader - "]
pub type LO_LF_RZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_rz` writer - "]
pub type LO_LF_RZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LO_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_cz` reader - "]
pub type LO_LF_CZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_cz` writer - "]
pub type LO_LF_CZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LO_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_r4` reader - "]
pub type LO_LF_R4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_lf_r4` writer - "]
pub type LO_LF_R4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LO_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_lf_r4_short` reader - "]
pub type LO_LF_R4_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `lo_lf_r4_short` writer - "]
pub type LO_LF_R4_SHORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LO_SPEC, bool, O>;
#[doc = "Field `lo_slipped_dn` reader - "]
pub type LO_SLIPPED_DN_R = crate::BitReader<bool>;
#[doc = "Field `lo_slipped_dn` writer - "]
pub type LO_SLIPPED_DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LO_SPEC, bool, O>;
#[doc = "Field `lo_slipped_up` reader - "]
pub type LO_SLIPPED_UP_R = crate::BitReader<bool>;
#[doc = "Field `lo_slipped_up` writer - "]
pub type LO_SLIPPED_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LO_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_lf_rz_hw(&self) -> LO_LF_RZ_HW_R {
        LO_LF_RZ_HW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_r4_hw(&self) -> LO_LF_R4_HW_R {
        LO_LF_R4_HW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_hw(&self) -> LO_LF_CZ_HW_R {
        LO_LF_CZ_HW_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz(&self) -> LO_LF_RZ_R {
        LO_LF_RZ_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_lf_cz(&self) -> LO_LF_CZ_R {
        LO_LF_CZ_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_r4(&self) -> LO_LF_R4_R {
        LO_LF_R4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lo_lf_r4_short(&self) -> LO_LF_R4_SHORT_R {
        LO_LF_R4_SHORT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_slipped_dn(&self) -> LO_SLIPPED_DN_R {
        LO_SLIPPED_DN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_slipped_up(&self) -> LO_SLIPPED_UP_R {
        LO_SLIPPED_UP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_rz_hw(&mut self) -> LO_LF_RZ_HW_W<0> {
        LO_LF_RZ_HW_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_r4_hw(&mut self) -> LO_LF_R4_HW_W<4> {
        LO_LF_R4_HW_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_cz_hw(&mut self) -> LO_LF_CZ_HW_W<8> {
        LO_LF_CZ_HW_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_rz(&mut self) -> LO_LF_RZ_W<12> {
        LO_LF_RZ_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_cz(&mut self) -> LO_LF_CZ_W<14> {
        LO_LF_CZ_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_r4(&mut self) -> LO_LF_R4_W<16> {
        LO_LF_R4_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn lo_lf_r4_short(&mut self) -> LO_LF_R4_SHORT_W<18> {
        LO_LF_R4_SHORT_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn lo_slipped_dn(&mut self) -> LO_SLIPPED_DN_W<20> {
        LO_SLIPPED_DN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn lo_slipped_up(&mut self) -> LO_SLIPPED_UP_W<24> {
        LO_SLIPPED_UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo](index.html) module"]
pub struct LO_SPEC;
impl crate::RegisterSpec for LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo::R](R) reader structure"]
impl crate::Readable for LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo::W](W) writer structure"]
impl crate::Writable for LO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo to value 0"]
impl crate::Resettable for LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

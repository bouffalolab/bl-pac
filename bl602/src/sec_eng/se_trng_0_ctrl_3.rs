#[doc = "Register `se_trng_0_ctrl_3` reader"]
pub struct R(crate::R<SE_TRNG_0_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_TRNG_0_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_TRNG_0_CTRL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_TRNG_0_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_trng_0_ctrl_3` writer"]
pub struct W(crate::W<SE_TRNG_0_CTRL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_TRNG_0_CTRL_3_SPEC>;
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
impl From<crate::W<SE_TRNG_0_CTRL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_TRNG_0_CTRL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_trng_0_cp_ratio` reader - "]
pub type SE_TRNG_0_CP_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_trng_0_cp_ratio` writer - "]
pub type SE_TRNG_0_CP_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_TRNG_0_CTRL_3_SPEC, u8, u8, 8, O>;
#[doc = "Field `se_trng_0_ht_rct_c` reader - "]
pub type SE_TRNG_0_HT_RCT_C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_trng_0_ht_rct_c` writer - "]
pub type SE_TRNG_0_HT_RCT_C_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_TRNG_0_CTRL_3_SPEC, u8, u8, 8, O>;
#[doc = "Field `se_trng_0_ht_apt_c` reader - "]
pub type SE_TRNG_0_HT_APT_C_R = crate::FieldReader<u16, u16>;
#[doc = "Field `se_trng_0_ht_apt_c` writer - "]
pub type SE_TRNG_0_HT_APT_C_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_TRNG_0_CTRL_3_SPEC, u16, u16, 10, O>;
#[doc = "Field `se_trng_0_ht_od_en` reader - "]
pub type SE_TRNG_0_HT_OD_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_ht_od_en` writer - "]
pub type SE_TRNG_0_HT_OD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_3_SPEC, bool, O>;
#[doc = "Field `se_trng_0_rosc_dis` reader - Used to be called 'se_trng_0_rosc_en', but the SDK calls it 'se_trng_0_rosc_dis'."]
pub type SE_TRNG_0_ROSC_DIS_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_0_rosc_dis` writer - Used to be called 'se_trng_0_rosc_en', but the SDK calls it 'se_trng_0_rosc_dis'."]
pub type SE_TRNG_0_ROSC_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_TRNG_0_CTRL_3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_trng_0_cp_ratio(&self) -> SE_TRNG_0_CP_RATIO_R {
        SE_TRNG_0_CP_RATIO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_trng_0_ht_rct_c(&self) -> SE_TRNG_0_HT_RCT_C_R {
        SE_TRNG_0_HT_RCT_C_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn se_trng_0_ht_apt_c(&self) -> SE_TRNG_0_HT_APT_C_R {
        SE_TRNG_0_HT_APT_C_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn se_trng_0_ht_od_en(&self) -> SE_TRNG_0_HT_OD_EN_R {
        SE_TRNG_0_HT_OD_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - Used to be called 'se_trng_0_rosc_en', but the SDK calls it 'se_trng_0_rosc_dis'."]
    #[inline(always)]
    pub fn se_trng_0_rosc_dis(&self) -> SE_TRNG_0_ROSC_DIS_R {
        SE_TRNG_0_ROSC_DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_cp_ratio(&mut self) -> SE_TRNG_0_CP_RATIO_W<0> {
        SE_TRNG_0_CP_RATIO_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_ht_rct_c(&mut self) -> SE_TRNG_0_HT_RCT_C_W<8> {
        SE_TRNG_0_HT_RCT_C_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_ht_apt_c(&mut self) -> SE_TRNG_0_HT_APT_C_W<16> {
        SE_TRNG_0_HT_APT_C_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_ht_od_en(&mut self) -> SE_TRNG_0_HT_OD_EN_W<26> {
        SE_TRNG_0_HT_OD_EN_W::new(self)
    }
    #[doc = "Bit 31 - Used to be called 'se_trng_0_rosc_en', but the SDK calls it 'se_trng_0_rosc_dis'."]
    #[inline(always)]
    #[must_use]
    pub fn se_trng_0_rosc_dis(&mut self) -> SE_TRNG_0_ROSC_DIS_W<31> {
        SE_TRNG_0_ROSC_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_trng_0_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_3](index.html) module"]
pub struct SE_TRNG_0_CTRL_3_SPEC;
impl crate::RegisterSpec for SE_TRNG_0_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_trng_0_ctrl_3::R](R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_3::W](W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_trng_0_ctrl_3 to value 0x837a_4203"]
impl crate::Resettable for SE_TRNG_0_CTRL_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x837a_4203;
}

#[doc = "Register `swrst_cfg1` reader"]
pub struct R(crate::R<SWRST_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRST_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRST_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `swrst_cfg1` writer"]
pub struct W(crate::W<SWRST_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRST_CFG1_SPEC>;
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
impl From<crate::W<SWRST_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRST_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `swrst_s10` reader - "]
pub type SWRST_S10_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s10` writer - "]
pub type SWRST_S10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s11` reader - "]
pub type SWRST_S11_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s11` writer - "]
pub type SWRST_S11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s12` reader - "]
pub type SWRST_S12_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s12` writer - "]
pub type SWRST_S12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s13` reader - "]
pub type SWRST_S13_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s13` writer - "]
pub type SWRST_S13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s14` reader - "]
pub type SWRST_S14_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s14` writer - "]
pub type SWRST_S14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s15` reader - "]
pub type SWRST_S15_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s15` writer - "]
pub type SWRST_S15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s16` reader - "]
pub type SWRST_S16_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s16` writer - "]
pub type SWRST_S16_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s17` reader - "]
pub type SWRST_S17_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s17` writer - "]
pub type SWRST_S17_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s18` reader - "]
pub type SWRST_S18_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s18` writer - "]
pub type SWRST_S18_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s19` reader - "]
pub type SWRST_S19_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s19` writer - "]
pub type SWRST_S19_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1a` reader - "]
pub type SWRST_S1A_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1a` writer - "]
pub type SWRST_S1A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1b` reader - "]
pub type SWRST_S1B_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1b` writer - "]
pub type SWRST_S1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1c` reader - "]
pub type SWRST_S1C_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1c` writer - "]
pub type SWRST_S1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1d` reader - "]
pub type SWRST_S1D_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1d` writer - "]
pub type SWRST_S1D_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1e` reader - "]
pub type SWRST_S1E_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1e` writer - "]
pub type SWRST_S1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1f` reader - "]
pub type SWRST_S1F_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1f` writer - "]
pub type SWRST_S1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1a0` reader - "]
pub type SWRST_S1A0_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1a0` writer - "]
pub type SWRST_S1A0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1a1` reader - "]
pub type SWRST_S1A1_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1a1` writer - "]
pub type SWRST_S1A1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1a2` reader - "]
pub type SWRST_S1A2_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1a2` writer - "]
pub type SWRST_S1A2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1a3` reader - "]
pub type SWRST_S1A3_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1a3` writer - "]
pub type SWRST_S1A3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1a4` reader - "]
pub type SWRST_S1A4_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1a4` writer - "]
pub type SWRST_S1A4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1a5` reader - "]
pub type SWRST_S1A5_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1a5` writer - "]
pub type SWRST_S1A5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1a6` reader - "]
pub type SWRST_S1A6_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1a6` writer - "]
pub type SWRST_S1A6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
#[doc = "Field `swrst_s1a7` reader - "]
pub type SWRST_S1A7_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1a7` writer - "]
pub type SWRST_S1A7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s10(&self) -> SWRST_S10_R {
        SWRST_S10_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s11(&self) -> SWRST_S11_R {
        SWRST_S11_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_s12(&self) -> SWRST_S12_R {
        SWRST_S12_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_s13(&self) -> SWRST_S13_R {
        SWRST_S13_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s14(&self) -> SWRST_S14_R {
        SWRST_S14_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn swrst_s15(&self) -> SWRST_S15_R {
        SWRST_S15_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swrst_s16(&self) -> SWRST_S16_R {
        SWRST_S16_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn swrst_s17(&self) -> SWRST_S17_R {
        SWRST_S17_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s18(&self) -> SWRST_S18_R {
        SWRST_S18_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_s19(&self) -> SWRST_S19_R {
        SWRST_S19_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swrst_s1a(&self) -> SWRST_S1A_R {
        SWRST_S1A_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swrst_s1b(&self) -> SWRST_S1B_R {
        SWRST_S1B_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn swrst_s1c(&self) -> SWRST_S1C_R {
        SWRST_S1C_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn swrst_s1d(&self) -> SWRST_S1D_R {
        SWRST_S1D_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn swrst_s1e(&self) -> SWRST_S1E_R {
        SWRST_S1E_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swrst_s1f(&self) -> SWRST_S1F_R {
        SWRST_S1F_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_s1a0(&self) -> SWRST_S1A0_R {
        SWRST_S1A0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swrst_s1a1(&self) -> SWRST_S1A1_R {
        SWRST_S1A1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn swrst_s1a2(&self) -> SWRST_S1A2_R {
        SWRST_S1A2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn swrst_s1a3(&self) -> SWRST_S1A3_R {
        SWRST_S1A3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn swrst_s1a4(&self) -> SWRST_S1A4_R {
        SWRST_S1A4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn swrst_s1a5(&self) -> SWRST_S1A5_R {
        SWRST_S1A5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn swrst_s1a6(&self) -> SWRST_S1A6_R {
        SWRST_S1A6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swrst_s1a7(&self) -> SWRST_S1A7_R {
        SWRST_S1A7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s10(&mut self) -> SWRST_S10_W<0> {
        SWRST_S10_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s11(&mut self) -> SWRST_S11_W<1> {
        SWRST_S11_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s12(&mut self) -> SWRST_S12_W<2> {
        SWRST_S12_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s13(&mut self) -> SWRST_S13_W<3> {
        SWRST_S13_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s14(&mut self) -> SWRST_S14_W<4> {
        SWRST_S14_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s15(&mut self) -> SWRST_S15_W<5> {
        SWRST_S15_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s16(&mut self) -> SWRST_S16_W<6> {
        SWRST_S16_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s17(&mut self) -> SWRST_S17_W<7> {
        SWRST_S17_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s18(&mut self) -> SWRST_S18_W<8> {
        SWRST_S18_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s19(&mut self) -> SWRST_S19_W<9> {
        SWRST_S19_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1a(&mut self) -> SWRST_S1A_W<10> {
        SWRST_S1A_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1b(&mut self) -> SWRST_S1B_W<11> {
        SWRST_S1B_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1c(&mut self) -> SWRST_S1C_W<12> {
        SWRST_S1C_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1d(&mut self) -> SWRST_S1D_W<13> {
        SWRST_S1D_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1e(&mut self) -> SWRST_S1E_W<14> {
        SWRST_S1E_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1f(&mut self) -> SWRST_S1F_W<15> {
        SWRST_S1F_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1a0(&mut self) -> SWRST_S1A0_W<16> {
        SWRST_S1A0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1a1(&mut self) -> SWRST_S1A1_W<17> {
        SWRST_S1A1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1a2(&mut self) -> SWRST_S1A2_W<18> {
        SWRST_S1A2_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1a3(&mut self) -> SWRST_S1A3_W<19> {
        SWRST_S1A3_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1a4(&mut self) -> SWRST_S1A4_W<20> {
        SWRST_S1A4_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1a5(&mut self) -> SWRST_S1A5_W<21> {
        SWRST_S1A5_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1a6(&mut self) -> SWRST_S1A6_W<22> {
        SWRST_S1A6_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1a7(&mut self) -> SWRST_S1A7_W<23> {
        SWRST_S1A7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "swrst_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg1](index.html) module"]
pub struct SWRST_CFG1_SPEC;
impl crate::RegisterSpec for SWRST_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst_cfg1::R](R) reader structure"]
impl crate::Readable for SWRST_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrst_cfg1::W](W) writer structure"]
impl crate::Writable for SWRST_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets swrst_cfg1 to value 0"]
impl crate::Resettable for SWRST_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

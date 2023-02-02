#[doc = "Register `TICR3` writer"]
pub struct W(crate::W<TICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TICR3_SPEC>;
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
impl From<crate::W<TICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tclr_0` writer - "]
pub type TCLR_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TICR3_SPEC, bool, O>;
#[doc = "Field `tclr_1` writer - "]
pub type TCLR_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TICR3_SPEC, bool, O>;
#[doc = "Field `tclr_2` writer - "]
pub type TCLR_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TICR3_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tclr_0(&mut self) -> TCLR_0_W<0> {
        TCLR_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tclr_1(&mut self) -> TCLR_1_W<1> {
        TCLR_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tclr_2(&mut self) -> TCLR_2_W<2> {
        TCLR_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TICR3.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ticr3](index.html) module"]
pub struct TICR3_SPEC;
impl crate::RegisterSpec for TICR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ticr3::W](W) writer structure"]
impl crate::Writable for TICR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TICR3 to value 0"]
impl crate::Resettable for TICR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

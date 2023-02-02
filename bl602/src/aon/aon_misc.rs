#[doc = "Register `aon_misc` reader"]
pub struct R(crate::R<AON_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AON_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AON_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AON_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `aon_misc` writer"]
pub struct W(crate::W<AON_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AON_MISC_SPEC>;
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
impl From<crate::W<AON_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AON_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sw_soc_en_aon` reader - "]
pub type SW_SOC_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `sw_soc_en_aon` writer - "]
pub type SW_SOC_EN_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_MISC_SPEC, bool, O>;
#[doc = "Field `sw_wb_en_aon` reader - "]
pub type SW_WB_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `sw_wb_en_aon` writer - "]
pub type SW_WB_EN_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_MISC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_soc_en_aon(&self) -> SW_SOC_EN_AON_R {
        SW_SOC_EN_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_wb_en_aon(&self) -> SW_WB_EN_AON_R {
        SW_WB_EN_AON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sw_soc_en_aon(&mut self) -> SW_SOC_EN_AON_W<0> {
        SW_SOC_EN_AON_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_wb_en_aon(&mut self) -> SW_WB_EN_AON_W<1> {
        SW_WB_EN_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "aon_misc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aon_misc](index.html) module"]
pub struct AON_MISC_SPEC;
impl crate::RegisterSpec for AON_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aon_misc::R](R) reader structure"]
impl crate::Readable for AON_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aon_misc::W](W) writer structure"]
impl crate::Writable for AON_MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon_misc to value 0x03"]
impl crate::Resettable for AON_MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}

#[doc = "Register `vco4` reader"]
pub struct R(crate::R<VCO4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCO4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCO4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCO4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vco4` writer"]
pub struct W(crate::W<VCO4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCO4_SPEC>;
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
impl From<crate::W<VCO4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCO4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fcal_cnt_start` reader - "]
pub type FCAL_CNT_START_R = crate::BitReader<bool>;
#[doc = "Field `fcal_cnt_start` writer - "]
pub type FCAL_CNT_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO4_SPEC, bool, O>;
#[doc = "Field `fcal_inc_en_hw` reader - "]
pub type FCAL_INC_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `fcal_inc_en_hw` writer - "]
pub type FCAL_INC_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO4_SPEC, bool, O>;
#[doc = "Field `fcal_inc_large_range` reader - "]
pub type FCAL_INC_LARGE_RANGE_R = crate::BitReader<bool>;
#[doc = "Field `fcal_inc_large_range` writer - "]
pub type FCAL_INC_LARGE_RANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO4_SPEC, bool, O>;
#[doc = "Field `fcal_cnt_rdy` reader - "]
pub type FCAL_CNT_RDY_R = crate::BitReader<bool>;
#[doc = "Field `fcal_cnt_rdy` writer - "]
pub type FCAL_CNT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO4_SPEC, bool, O>;
#[doc = "Field `fcal_inc_vctrl_ud` reader - "]
pub type FCAL_INC_VCTRL_UD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fcal_inc_vctrl_ud` writer - "]
pub type FCAL_INC_VCTRL_UD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VCO4_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fcal_cnt_start(&self) -> FCAL_CNT_START_R {
        FCAL_CNT_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcal_inc_en_hw(&self) -> FCAL_INC_EN_HW_R {
        FCAL_INC_EN_HW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fcal_inc_large_range(&self) -> FCAL_INC_LARGE_RANGE_R {
        FCAL_INC_LARGE_RANGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fcal_cnt_rdy(&self) -> FCAL_CNT_RDY_R {
        FCAL_CNT_RDY_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn fcal_inc_vctrl_ud(&self) -> FCAL_INC_VCTRL_UD_R {
        FCAL_INC_VCTRL_UD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_cnt_start(&mut self) -> FCAL_CNT_START_W<4> {
        FCAL_CNT_START_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_inc_en_hw(&mut self) -> FCAL_INC_EN_HW_W<8> {
        FCAL_INC_EN_HW_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_inc_large_range(&mut self) -> FCAL_INC_LARGE_RANGE_W<16> {
        FCAL_INC_LARGE_RANGE_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_cnt_rdy(&mut self) -> FCAL_CNT_RDY_W<20> {
        FCAL_CNT_RDY_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_inc_vctrl_ud(&mut self) -> FCAL_INC_VCTRL_UD_W<24> {
        FCAL_INC_VCTRL_UD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "vco4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco4](index.html) module"]
pub struct VCO4_SPEC;
impl crate::RegisterSpec for VCO4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vco4::R](R) reader structure"]
impl crate::Readable for VCO4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vco4::W](W) writer structure"]
impl crate::Writable for VCO4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets vco4 to value 0"]
impl crate::Resettable for VCO4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

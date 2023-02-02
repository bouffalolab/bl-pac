#[doc = "Register `pa2` reader"]
pub struct R(crate::R<PA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pa2` writer"]
pub struct W(crate::W<PA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA2_SPEC>;
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
impl From<crate::W<PA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_etb_en_hw` reader - "]
pub type PA_ETB_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `pa_etb_en_hw` writer - "]
pub type PA_ETB_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA2_SPEC, bool, O>;
#[doc = "Field `pa_iet_hw` reader - "]
pub type PA_IET_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_iet_hw` writer - "]
pub type PA_IET_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA2_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vbcore_hw` reader - "]
pub type PA_VBCORE_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vbcore_hw` writer - "]
pub type PA_VBCORE_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA2_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vbcas_hw` reader - "]
pub type PA_VBCAS_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vbcas_hw` writer - "]
pub type PA_VBCAS_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA2_SPEC, u8, u8, 3, O>;
#[doc = "Field `pa_half_on_hw` reader - "]
pub type PA_HALF_ON_HW_R = crate::BitReader<bool>;
#[doc = "Field `pa_half_on_hw` writer - "]
pub type PA_HALF_ON_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA2_SPEC, bool, O>;
#[doc = "Field `pa_ib_fix_hw` reader - "]
pub type PA_IB_FIX_HW_R = crate::BitReader<bool>;
#[doc = "Field `pa_ib_fix_hw` writer - "]
pub type PA_IB_FIX_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en_hw(&self) -> PA_ETB_EN_HW_R {
        PA_ETB_EN_HW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet_hw(&self) -> PA_IET_HW_R {
        PA_IET_HW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore_hw(&self) -> PA_VBCORE_HW_R {
        PA_VBCORE_HW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas_hw(&self) -> PA_VBCAS_HW_R {
        PA_VBCAS_HW_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_half_on_hw(&self) -> PA_HALF_ON_HW_R {
        PA_HALF_ON_HW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_ib_fix_hw(&self) -> PA_IB_FIX_HW_R {
        PA_IB_FIX_HW_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pa_etb_en_hw(&mut self) -> PA_ETB_EN_HW_W<3> {
        PA_ETB_EN_HW_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn pa_iet_hw(&mut self) -> PA_IET_HW_W<4> {
        PA_IET_HW_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn pa_vbcore_hw(&mut self) -> PA_VBCORE_HW_W<8> {
        PA_VBCORE_HW_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn pa_vbcas_hw(&mut self) -> PA_VBCAS_HW_W<12> {
        PA_VBCAS_HW_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pa_half_on_hw(&mut self) -> PA_HALF_ON_HW_W<16> {
        PA_HALF_ON_HW_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pa_ib_fix_hw(&mut self) -> PA_IB_FIX_HW_W<17> {
        PA_IB_FIX_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX normal bias mode registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa2](index.html) module"]
pub struct PA2_SPEC;
impl crate::RegisterSpec for PA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa2::R](R) reader structure"]
impl crate::Readable for PA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa2::W](W) writer structure"]
impl crate::Writable for PA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pa2 to value 0"]
impl crate::Resettable for PA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `rfif_dig_ctrl` reader"]
pub struct R(crate::R<RFIF_DIG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIF_DIG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIF_DIG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIF_DIG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfif_dig_ctrl` writer"]
pub struct W(crate::W<RFIF_DIG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIF_DIG_CTRL_SPEC>;
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
impl From<crate::W<RFIF_DIG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIF_DIG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `test_from_pad_en` reader - "]
pub type TEST_FROM_PAD_EN_R = crate::BitReader<bool>;
#[doc = "Field `test_from_pad_en` writer - "]
pub type TEST_FROM_PAD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DIG_CTRL_SPEC, bool, O>;
#[doc = "Field `test_gc_from_pad_en` reader - "]
pub type TEST_GC_FROM_PAD_EN_R = crate::BitReader<bool>;
#[doc = "Field `test_gc_from_pad_en` writer - "]
pub type TEST_GC_FROM_PAD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DIG_CTRL_SPEC, bool, O>;
#[doc = "Field `rfckg_rxclk_div2_mode` reader - "]
pub type RFCKG_RXCLK_DIV2_MODE_R = crate::BitReader<bool>;
#[doc = "Field `rfckg_rxclk_div2_mode` writer - "]
pub type RFCKG_RXCLK_DIV2_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DIG_CTRL_SPEC, bool, O>;
#[doc = "Field `rfif_int_lo_unlocked_mask` reader - "]
pub type RFIF_INT_LO_UNLOCKED_MASK_R = crate::BitReader<bool>;
#[doc = "Field `rfif_int_lo_unlocked_mask` writer - "]
pub type RFIF_INT_LO_UNLOCKED_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DIG_CTRL_SPEC, bool, O>;
#[doc = "Field `rfif_ppud_cnt2` reader - "]
pub type RFIF_PPUD_CNT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rfif_ppud_cnt2` writer - "]
pub type RFIF_PPUD_CNT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFIF_DIG_CTRL_SPEC, u16, u16, 9, O>;
#[doc = "Field `rfif_ppud_cnt1` reader - "]
pub type RFIF_PPUD_CNT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rfif_ppud_cnt1` writer - "]
pub type RFIF_PPUD_CNT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFIF_DIG_CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `rfif_ppud_manaual_en` reader - "]
pub type RFIF_PPUD_MANAUAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `rfif_ppud_manaual_en` writer - "]
pub type RFIF_PPUD_MANAUAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIF_DIG_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn test_from_pad_en(&self) -> TEST_FROM_PAD_EN_R {
        TEST_FROM_PAD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn test_gc_from_pad_en(&self) -> TEST_GC_FROM_PAD_EN_R {
        TEST_GC_FROM_PAD_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_rxclk_div2_mode(&self) -> RFCKG_RXCLK_DIV2_MODE_R {
        RFCKG_RXCLK_DIV2_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfif_int_lo_unlocked_mask(&self) -> RFIF_INT_LO_UNLOCKED_MASK_R {
        RFIF_INT_LO_UNLOCKED_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rfif_ppud_cnt2(&self) -> RFIF_PPUD_CNT2_R {
        RFIF_PPUD_CNT2_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn rfif_ppud_cnt1(&self) -> RFIF_PPUD_CNT1_R {
        RFIF_PPUD_CNT1_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rfif_ppud_manaual_en(&self) -> RFIF_PPUD_MANAUAL_EN_R {
        RFIF_PPUD_MANAUAL_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn test_from_pad_en(&mut self) -> TEST_FROM_PAD_EN_W<0> {
        TEST_FROM_PAD_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn test_gc_from_pad_en(&mut self) -> TEST_GC_FROM_PAD_EN_W<1> {
        TEST_GC_FROM_PAD_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rfckg_rxclk_div2_mode(&mut self) -> RFCKG_RXCLK_DIV2_MODE_W<2> {
        RFCKG_RXCLK_DIV2_MODE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rfif_int_lo_unlocked_mask(&mut self) -> RFIF_INT_LO_UNLOCKED_MASK_W<3> {
        RFIF_INT_LO_UNLOCKED_MASK_W::new(self)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    #[must_use]
    pub fn rfif_ppud_cnt2(&mut self) -> RFIF_PPUD_CNT2_W<16> {
        RFIF_PPUD_CNT2_W::new(self)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    #[must_use]
    pub fn rfif_ppud_cnt1(&mut self) -> RFIF_PPUD_CNT1_W<25> {
        RFIF_PPUD_CNT1_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rfif_ppud_manaual_en(&mut self) -> RFIF_PPUD_MANAUAL_EN_W<30> {
        RFIF_PPUD_MANAUAL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rfif_dig_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfif_dig_ctrl](index.html) module"]
pub struct RFIF_DIG_CTRL_SPEC;
impl crate::RegisterSpec for RFIF_DIG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfif_dig_ctrl::R](R) reader structure"]
impl crate::Readable for RFIF_DIG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfif_dig_ctrl::W](W) writer structure"]
impl crate::Writable for RFIF_DIG_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rfif_dig_ctrl to value 0"]
impl crate::Resettable for RFIF_DIG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

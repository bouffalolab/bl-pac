#[doc = "Register `HBN_IRQ_MODE` reader"]
pub struct R(crate::R<HBN_IRQ_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_IRQ_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_IRQ_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_IRQ_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_IRQ_MODE` writer"]
pub struct W(crate::W<HBN_IRQ_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_IRQ_MODE_SPEC>;
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
impl From<crate::W<HBN_IRQ_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_IRQ_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hbn_pin_wakeup_mode` reader - "]
pub type HBN_PIN_WAKEUP_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_pin_wakeup_mode` writer - "]
pub type HBN_PIN_WAKEUP_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_IRQ_MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `hbn_pin_wakeup_mask` reader - "]
pub type HBN_PIN_WAKEUP_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_pin_wakeup_mask` writer - "]
pub type HBN_PIN_WAKEUP_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_IRQ_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_aon_pad_ie_smt` reader - "]
pub type REG_AON_PAD_IE_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_aon_pad_ie_smt` writer - "]
pub type REG_AON_PAD_IE_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HBN_IRQ_MODE_SPEC, bool, O>;
#[doc = "Field `reg_en_hw_pu_pd` reader - "]
pub type REG_EN_HW_PU_PD_R = crate::BitReader<bool>;
#[doc = "Field `reg_en_hw_pu_pd` writer - "]
pub type REG_EN_HW_PU_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_IRQ_MODE_SPEC, bool, O>;
#[doc = "Field `irq_bor_en` reader - "]
pub type IRQ_BOR_EN_R = crate::BitReader<bool>;
#[doc = "Field `irq_bor_en` writer - "]
pub type IRQ_BOR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_IRQ_MODE_SPEC, bool, O>;
#[doc = "Field `irq_acomp0_en` reader - "]
pub type IRQ_ACOMP0_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `irq_acomp0_en` writer - "]
pub type IRQ_ACOMP0_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_IRQ_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `irq_acomp1_en` reader - "]
pub type IRQ_ACOMP1_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `irq_acomp1_en` writer - "]
pub type IRQ_ACOMP1_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_IRQ_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `pin_wakeup_sel` reader - "]
pub type PIN_WAKEUP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin_wakeup_sel` writer - "]
pub type PIN_WAKEUP_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_IRQ_MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `pin_wakeup_en` reader - "]
pub type PIN_WAKEUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `pin_wakeup_en` writer - "]
pub type PIN_WAKEUP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_IRQ_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mode(&self) -> HBN_PIN_WAKEUP_MODE_R {
        HBN_PIN_WAKEUP_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mask(&self) -> HBN_PIN_WAKEUP_MASK_R {
        HBN_PIN_WAKEUP_MASK_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_aon_pad_ie_smt(&self) -> REG_AON_PAD_IE_SMT_R {
        REG_AON_PAD_IE_SMT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_en_hw_pu_pd(&self) -> REG_EN_HW_PU_PD_R {
        REG_EN_HW_PU_PD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn irq_bor_en(&self) -> IRQ_BOR_EN_R {
        IRQ_BOR_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn irq_acomp0_en(&self) -> IRQ_ACOMP0_EN_R {
        IRQ_ACOMP0_EN_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn irq_acomp1_en(&self) -> IRQ_ACOMP1_EN_R {
        IRQ_ACOMP1_EN_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pin_wakeup_sel(&self) -> PIN_WAKEUP_SEL_R {
        PIN_WAKEUP_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin_wakeup_en(&self) -> PIN_WAKEUP_EN_R {
        PIN_WAKEUP_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_pin_wakeup_mode(&mut self) -> HBN_PIN_WAKEUP_MODE_W<0> {
        HBN_PIN_WAKEUP_MODE_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_pin_wakeup_mask(&mut self) -> HBN_PIN_WAKEUP_MASK_W<3> {
        HBN_PIN_WAKEUP_MASK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_pad_ie_smt(&mut self) -> REG_AON_PAD_IE_SMT_W<8> {
        REG_AON_PAD_IE_SMT_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_en_hw_pu_pd(&mut self) -> REG_EN_HW_PU_PD_W<16> {
        REG_EN_HW_PU_PD_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn irq_bor_en(&mut self) -> IRQ_BOR_EN_W<18> {
        IRQ_BOR_EN_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn irq_acomp0_en(&mut self) -> IRQ_ACOMP0_EN_W<20> {
        IRQ_ACOMP0_EN_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn irq_acomp1_en(&mut self) -> IRQ_ACOMP1_EN_W<22> {
        IRQ_ACOMP1_EN_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn pin_wakeup_sel(&mut self) -> PIN_WAKEUP_SEL_W<24> {
        PIN_WAKEUP_SEL_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn pin_wakeup_en(&mut self) -> PIN_WAKEUP_EN_W<27> {
        PIN_WAKEUP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_IRQ_MODE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_irq_mode](index.html) module"]
pub struct HBN_IRQ_MODE_SPEC;
impl crate::RegisterSpec for HBN_IRQ_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_irq_mode::R](R) reader structure"]
impl crate::Readable for HBN_IRQ_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_irq_mode::W](W) writer structure"]
impl crate::Writable for HBN_IRQ_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_IRQ_MODE to value 0x0301_0105"]
impl crate::Resettable for HBN_IRQ_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0301_0105;
}

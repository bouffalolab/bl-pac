#[doc = "Register `tsen` reader"]
pub struct R(crate::R<TSEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tsen` writer"]
pub struct W(crate::W<TSEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSEN_SPEC>;
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
impl From<crate::W<TSEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tsen_refcode_corner` reader - "]
pub type TSEN_REFCODE_CORNER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tsen_refcode_corner` writer - "]
pub type TSEN_REFCODE_CORNER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSEN_SPEC, u16, u16, 12, O>;
#[doc = "Field `tsen_refcode_rfcal` reader - "]
pub type TSEN_REFCODE_RFCAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tsen_refcode_rfcal` writer - "]
pub type TSEN_REFCODE_RFCAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSEN_SPEC, u16, u16, 12, O>;
#[doc = "Field `xtal_rdy` reader - "]
pub type XTAL_RDY_R = crate::BitReader<bool>;
#[doc = "Field `xtal_inn_cfg_en_aon` reader - "]
pub type XTAL_INN_CFG_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `xtal_inn_cfg_en_aon` writer - "]
pub type XTAL_INN_CFG_EN_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSEN_SPEC, bool, O>;
#[doc = "Field `xtal_rdy_int_sel_aon` reader - "]
pub type XTAL_RDY_INT_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal_rdy_int_sel_aon` writer - "]
pub type XTAL_RDY_INT_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSEN_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsen_refcode_corner(&self) -> TSEN_REFCODE_CORNER_R {
        TSEN_REFCODE_CORNER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tsen_refcode_rfcal(&self) -> TSEN_REFCODE_RFCAL_R {
        TSEN_REFCODE_RFCAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn xtal_rdy(&self) -> XTAL_RDY_R {
        XTAL_RDY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn xtal_inn_cfg_en_aon(&self) -> XTAL_INN_CFG_EN_AON_R {
        XTAL_INN_CFG_EN_AON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_int_sel_aon(&self) -> XTAL_RDY_INT_SEL_AON_R {
        XTAL_RDY_INT_SEL_AON_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn tsen_refcode_corner(&mut self) -> TSEN_REFCODE_CORNER_W<0> {
        TSEN_REFCODE_CORNER_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn tsen_refcode_rfcal(&mut self) -> TSEN_REFCODE_RFCAL_W<16> {
        TSEN_REFCODE_RFCAL_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_inn_cfg_en_aon(&mut self) -> XTAL_INN_CFG_EN_AON_W<29> {
        XTAL_INN_CFG_EN_AON_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_rdy_int_sel_aon(&mut self) -> XTAL_RDY_INT_SEL_AON_W<30> {
        XTAL_RDY_INT_SEL_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tsen.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsen](index.html) module"]
pub struct TSEN_SPEC;
impl crate::RegisterSpec for TSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsen::R](R) reader structure"]
impl crate::Readable for TSEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsen::W](W) writer structure"]
impl crate::Writable for TSEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tsen to value 0x78ff_08ff"]
impl crate::Resettable for TSEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x78ff_08ff;
}

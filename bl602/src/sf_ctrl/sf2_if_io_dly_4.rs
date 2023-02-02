#[doc = "Register `sf2_if_io_dly_4` reader"]
pub struct R(crate::R<SF2_IF_IO_DLY_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF2_IF_IO_DLY_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF2_IF_IO_DLY_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF2_IF_IO_DLY_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf2_if_io_dly_4` writer"]
pub struct W(crate::W<SF2_IF_IO_DLY_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF2_IF_IO_DLY_4_SPEC>;
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
impl From<crate::W<SF2_IF_IO_DLY_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF2_IF_IO_DLY_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf2_io_3_oe_dly_sel` reader - "]
pub type SF2_IO_3_OE_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf2_io_3_oe_dly_sel` writer - "]
pub type SF2_IO_3_OE_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF2_IF_IO_DLY_4_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf2_io_3_di_dly_sel` reader - "]
pub type SF2_IO_3_DI_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf2_io_3_di_dly_sel` writer - "]
pub type SF2_IO_3_DI_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF2_IF_IO_DLY_4_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf2_io_3_do_dly_sel` reader - "]
pub type SF2_IO_3_DO_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf2_io_3_do_dly_sel` writer - "]
pub type SF2_IO_3_DO_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF2_IF_IO_DLY_4_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf2_io_3_oe_dly_sel(&self) -> SF2_IO_3_OE_DLY_SEL_R {
        SF2_IO_3_OE_DLY_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf2_io_3_di_dly_sel(&self) -> SF2_IO_3_DI_DLY_SEL_R {
        SF2_IO_3_DI_DLY_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf2_io_3_do_dly_sel(&self) -> SF2_IO_3_DO_DLY_SEL_R {
        SF2_IO_3_DO_DLY_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn sf2_io_3_oe_dly_sel(&mut self) -> SF2_IO_3_OE_DLY_SEL_W<0> {
        SF2_IO_3_OE_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn sf2_io_3_di_dly_sel(&mut self) -> SF2_IO_3_DI_DLY_SEL_W<8> {
        SF2_IO_3_DI_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn sf2_io_3_do_dly_sel(&mut self) -> SF2_IO_3_DO_DLY_SEL_W<16> {
        SF2_IO_3_DO_DLY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf2_if_io_dly_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf2_if_io_dly_4](index.html) module"]
pub struct SF2_IF_IO_DLY_4_SPEC;
impl crate::RegisterSpec for SF2_IF_IO_DLY_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf2_if_io_dly_4::R](R) reader structure"]
impl crate::Readable for SF2_IF_IO_DLY_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf2_if_io_dly_4::W](W) writer structure"]
impl crate::Writable for SF2_IF_IO_DLY_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf2_if_io_dly_4 to value 0"]
impl crate::Resettable for SF2_IF_IO_DLY_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

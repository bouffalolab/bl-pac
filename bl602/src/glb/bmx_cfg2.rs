#[doc = "Register `bmx_cfg2` reader"]
pub struct R(crate::R<BMX_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_cfg2` writer"]
pub struct W(crate::W<BMX_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_CFG2_SPEC>;
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
impl From<crate::W<BMX_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMX_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bmx_err_addr_dis` reader - BMX address monitor disable"]
pub type BMX_ERR_ADDR_DIS_R = crate::BitReader<bool>;
#[doc = "Field `bmx_err_addr_dis` writer - BMX address monitor disable"]
pub type BMX_ERR_ADDR_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG2_SPEC, bool, O>;
#[doc = "Field `bmx_err_dec` reader - "]
pub type BMX_ERR_DEC_R = crate::BitReader<bool>;
#[doc = "Field `bmx_err_tz` reader - "]
pub type BMX_ERR_TZ_R = crate::BitReader<bool>;
#[doc = "Field `bmx_dbg_sel` reader - "]
pub type BMX_DBG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bmx_dbg_sel` writer - "]
pub type BMX_DBG_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMX_CFG2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - BMX address monitor disable"]
    #[inline(always)]
    pub fn bmx_err_addr_dis(&self) -> BMX_ERR_ADDR_DIS_R {
        BMX_ERR_ADDR_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bmx_err_dec(&self) -> BMX_ERR_DEC_R {
        BMX_ERR_DEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bmx_err_tz(&self) -> BMX_ERR_TZ_R {
        BMX_ERR_TZ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bmx_dbg_sel(&self) -> BMX_DBG_SEL_R {
        BMX_DBG_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BMX address monitor disable"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_err_addr_dis(&mut self) -> BMX_ERR_ADDR_DIS_W<0> {
        BMX_ERR_ADDR_DIS_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_dbg_sel(&mut self) -> BMX_DBG_SEL_W<28> {
        BMX_DBG_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_cfg2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg2](index.html) module"]
pub struct BMX_CFG2_SPEC;
impl crate::RegisterSpec for BMX_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_cfg2::R](R) reader structure"]
impl crate::Readable for BMX_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_cfg2::W](W) writer structure"]
impl crate::Writable for BMX_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg2 to value 0"]
impl crate::Resettable for BMX_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

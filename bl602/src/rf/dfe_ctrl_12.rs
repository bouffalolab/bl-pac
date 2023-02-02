#[doc = "Register `dfe_ctrl_12` reader"]
pub struct R(crate::R<DFE_CTRL_12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_12` writer"]
pub struct W(crate::W<DFE_CTRL_12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_12_SPEC>;
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
impl From<crate::W<DFE_CTRL_12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_dvga_gain_qdb_gc0` reader - "]
pub type TX_DVGA_GAIN_QDB_GC0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_dvga_gain_qdb_gc0` writer - "]
pub type TX_DVGA_GAIN_QDB_GC0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_12_SPEC, u8, u8, 7, O>;
#[doc = "Field `tx_dvga_gain_qdb_gc1` reader - "]
pub type TX_DVGA_GAIN_QDB_GC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_dvga_gain_qdb_gc1` writer - "]
pub type TX_DVGA_GAIN_QDB_GC1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_12_SPEC, u8, u8, 7, O>;
#[doc = "Field `tx_dvga_gain_qdb_gc2` reader - "]
pub type TX_DVGA_GAIN_QDB_GC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_dvga_gain_qdb_gc2` writer - "]
pub type TX_DVGA_GAIN_QDB_GC2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_12_SPEC, u8, u8, 7, O>;
#[doc = "Field `tx_dvga_gain_qdb_gc3` reader - "]
pub type TX_DVGA_GAIN_QDB_GC3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_dvga_gain_qdb_gc3` writer - "]
pub type TX_DVGA_GAIN_QDB_GC3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_12_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc0(&self) -> TX_DVGA_GAIN_QDB_GC0_R {
        TX_DVGA_GAIN_QDB_GC0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc1(&self) -> TX_DVGA_GAIN_QDB_GC1_R {
        TX_DVGA_GAIN_QDB_GC1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc2(&self) -> TX_DVGA_GAIN_QDB_GC2_R {
        TX_DVGA_GAIN_QDB_GC2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc3(&self) -> TX_DVGA_GAIN_QDB_GC3_R {
        TX_DVGA_GAIN_QDB_GC3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dvga_gain_qdb_gc0(&mut self) -> TX_DVGA_GAIN_QDB_GC0_W<0> {
        TX_DVGA_GAIN_QDB_GC0_W::new(self)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dvga_gain_qdb_gc1(&mut self) -> TX_DVGA_GAIN_QDB_GC1_W<8> {
        TX_DVGA_GAIN_QDB_GC1_W::new(self)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dvga_gain_qdb_gc2(&mut self) -> TX_DVGA_GAIN_QDB_GC2_W<16> {
        TX_DVGA_GAIN_QDB_GC2_W::new(self)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dvga_gain_qdb_gc3(&mut self) -> TX_DVGA_GAIN_QDB_GC3_W<24> {
        TX_DVGA_GAIN_QDB_GC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_12.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_12](index.html) module"]
pub struct DFE_CTRL_12_SPEC;
impl crate::RegisterSpec for DFE_CTRL_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_12::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_12::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_12 to value 0"]
impl crate::Resettable for DFE_CTRL_12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

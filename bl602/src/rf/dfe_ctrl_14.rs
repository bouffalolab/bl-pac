#[doc = "Register `dfe_ctrl_14` reader"]
pub struct R(crate::R<DFE_CTRL_14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_14` writer"]
pub struct W(crate::W<DFE_CTRL_14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_14_SPEC>;
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
impl From<crate::W<DFE_CTRL_14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_dvga_gain_qdb_gc8` reader - "]
pub type TX_DVGA_GAIN_QDB_GC8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_dvga_gain_qdb_gc8` writer - "]
pub type TX_DVGA_GAIN_QDB_GC8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_14_SPEC, u8, u8, 7, O>;
#[doc = "Field `tx_dvga_gain_qdb_gc9` reader - "]
pub type TX_DVGA_GAIN_QDB_GC9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_dvga_gain_qdb_gc9` writer - "]
pub type TX_DVGA_GAIN_QDB_GC9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_14_SPEC, u8, u8, 7, O>;
#[doc = "Field `tx_dvga_gain_qdb_gc10` reader - "]
pub type TX_DVGA_GAIN_QDB_GC10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_dvga_gain_qdb_gc10` writer - "]
pub type TX_DVGA_GAIN_QDB_GC10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_14_SPEC, u8, u8, 7, O>;
#[doc = "Field `tx_dvga_gain_qdb_gc11` reader - "]
pub type TX_DVGA_GAIN_QDB_GC11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_dvga_gain_qdb_gc11` writer - "]
pub type TX_DVGA_GAIN_QDB_GC11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFE_CTRL_14_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc8(&self) -> TX_DVGA_GAIN_QDB_GC8_R {
        TX_DVGA_GAIN_QDB_GC8_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc9(&self) -> TX_DVGA_GAIN_QDB_GC9_R {
        TX_DVGA_GAIN_QDB_GC9_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc10(&self) -> TX_DVGA_GAIN_QDB_GC10_R {
        TX_DVGA_GAIN_QDB_GC10_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc11(&self) -> TX_DVGA_GAIN_QDB_GC11_R {
        TX_DVGA_GAIN_QDB_GC11_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dvga_gain_qdb_gc8(&mut self) -> TX_DVGA_GAIN_QDB_GC8_W<0> {
        TX_DVGA_GAIN_QDB_GC8_W::new(self)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dvga_gain_qdb_gc9(&mut self) -> TX_DVGA_GAIN_QDB_GC9_W<8> {
        TX_DVGA_GAIN_QDB_GC9_W::new(self)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dvga_gain_qdb_gc10(&mut self) -> TX_DVGA_GAIN_QDB_GC10_W<16> {
        TX_DVGA_GAIN_QDB_GC10_W::new(self)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dvga_gain_qdb_gc11(&mut self) -> TX_DVGA_GAIN_QDB_GC11_W<24> {
        TX_DVGA_GAIN_QDB_GC11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_14.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_14](index.html) module"]
pub struct DFE_CTRL_14_SPEC;
impl crate::RegisterSpec for DFE_CTRL_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_14::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_14::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dfe_ctrl_14 to value 0"]
impl crate::Resettable for DFE_CTRL_14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

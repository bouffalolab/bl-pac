#[doc = "Register `DBG_SEL_LL` reader"]
pub struct R(crate::R<DBG_SEL_LL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_SEL_LL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_SEL_LL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_SEL_LL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_SEL_LL` writer"]
pub struct W(crate::W<DBG_SEL_LL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_SEL_LL_SPEC>;
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
impl From<crate::W<DBG_SEL_LL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_SEL_LL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_dbg_ll_ctrl` reader - "]
pub type REG_DBG_LL_CTRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `reg_dbg_ll_ctrl` writer - "]
pub type REG_DBG_LL_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_SEL_LL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_ll_ctrl(&self) -> REG_DBG_LL_CTRL_R {
        REG_DBG_LL_CTRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dbg_ll_ctrl(&mut self) -> REG_DBG_LL_CTRL_W<0> {
        REG_DBG_LL_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBG_SEL_LL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sel_ll](index.html) module"]
pub struct DBG_SEL_LL_SPEC;
impl crate::RegisterSpec for DBG_SEL_LL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_sel_ll::R](R) reader structure"]
impl crate::Readable for DBG_SEL_LL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_sel_ll::W](W) writer structure"]
impl crate::Writable for DBG_SEL_LL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBG_SEL_LL to value 0"]
impl crate::Resettable for DBG_SEL_LL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

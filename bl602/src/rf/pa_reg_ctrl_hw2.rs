#[doc = "Register `pa_reg_ctrl_hw2` reader"]
pub struct R(crate::R<PA_REG_CTRL_HW2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_REG_CTRL_HW2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA_REG_CTRL_HW2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA_REG_CTRL_HW2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pa_reg_ctrl_hw2` writer"]
pub struct W(crate::W<PA_REG_CTRL_HW2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_REG_CTRL_HW2_SPEC>;
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
impl From<crate::W<PA_REG_CTRL_HW2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_REG_CTRL_HW2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_iet_11g` reader - "]
pub type PA_IET_11G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_iet_11g` writer - "]
pub type PA_IET_11G_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_REG_CTRL_HW2_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vbcore_11g` reader - "]
pub type PA_VBCORE_11G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vbcore_11g` writer - "]
pub type PA_VBCORE_11G_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_REG_CTRL_HW2_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vbcas_11g` reader - "]
pub type PA_VBCAS_11G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vbcas_11g` writer - "]
pub type PA_VBCAS_11G_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_REG_CTRL_HW2_SPEC, u8, u8, 3, O>;
#[doc = "Field `pa_iet_11b` reader - "]
pub type PA_IET_11B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_iet_11b` writer - "]
pub type PA_IET_11B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_REG_CTRL_HW2_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vbcore_11b` reader - "]
pub type PA_VBCORE_11B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vbcore_11b` writer - "]
pub type PA_VBCORE_11B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_REG_CTRL_HW2_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vbcas_11b` reader - "]
pub type PA_VBCAS_11B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vbcas_11b` writer - "]
pub type PA_VBCAS_11B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_REG_CTRL_HW2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_iet_11g(&self) -> PA_IET_11G_R {
        PA_IET_11G_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_vbcore_11g(&self) -> PA_VBCORE_11G_R {
        PA_VBCORE_11G_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pa_vbcas_11g(&self) -> PA_VBCAS_11G_R {
        PA_VBCAS_11G_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11b(&self) -> PA_IET_11B_R {
        PA_IET_11B_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11b(&self) -> PA_VBCORE_11B_R {
        PA_VBCORE_11B_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11b(&self) -> PA_VBCAS_11B_R {
        PA_VBCAS_11B_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn pa_iet_11g(&mut self) -> PA_IET_11G_W<0> {
        PA_IET_11G_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn pa_vbcore_11g(&mut self) -> PA_VBCORE_11G_W<4> {
        PA_VBCORE_11G_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pa_vbcas_11g(&mut self) -> PA_VBCAS_11G_W<8> {
        PA_VBCAS_11G_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn pa_iet_11b(&mut self) -> PA_IET_11B_W<12> {
        PA_IET_11B_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn pa_vbcore_11b(&mut self) -> PA_VBCORE_11B_W<16> {
        PA_VBCORE_11B_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn pa_vbcas_11b(&mut self) -> PA_VBCAS_11B_W<20> {
        PA_VBCAS_11B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pa_reg_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_reg_ctrl_hw2](index.html) module"]
pub struct PA_REG_CTRL_HW2_SPEC;
impl crate::RegisterSpec for PA_REG_CTRL_HW2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_reg_ctrl_hw2::R](R) reader structure"]
impl crate::Readable for PA_REG_CTRL_HW2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_reg_ctrl_hw2::W](W) writer structure"]
impl crate::Writable for PA_REG_CTRL_HW2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pa_reg_ctrl_hw2 to value 0"]
impl crate::Resettable for PA_REG_CTRL_HW2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

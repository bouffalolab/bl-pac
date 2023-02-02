#[doc = "Register `se_gmac_0_ctrl_0` reader"]
pub struct R(crate::R<SE_GMAC_0_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_GMAC_0_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_GMAC_0_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_GMAC_0_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_gmac_0_ctrl_0` writer"]
pub struct W(crate::W<SE_GMAC_0_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_GMAC_0_CTRL_0_SPEC>;
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
impl From<crate::W<SE_GMAC_0_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_GMAC_0_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_gmac_0_busy` reader - "]
pub type SE_GMAC_0_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_0_trig_1t` reader - "]
pub type SE_GMAC_0_TRIG_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_0_trig_1t` writer - "]
pub type SE_GMAC_0_TRIG_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_GMAC_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_gmac_0_en` reader - "]
pub type SE_GMAC_0_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_0_en` writer - "]
pub type SE_GMAC_0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_GMAC_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_gmac_0_int` reader - "]
pub type SE_GMAC_0_INT_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_0_int_clr_1t` reader - "]
pub type SE_GMAC_0_INT_CLR_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_0_int_clr_1t` writer - "]
pub type SE_GMAC_0_INT_CLR_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_GMAC_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_gmac_0_int_set_1t` reader - "]
pub type SE_GMAC_0_INT_SET_1T_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_0_int_set_1t` writer - "]
pub type SE_GMAC_0_INT_SET_1T_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_GMAC_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_gmac_0_int_mask` reader - "]
pub type SE_GMAC_0_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_0_int_mask` writer - "]
pub type SE_GMAC_0_INT_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_GMAC_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_gmac_0_t_endian` reader - "]
pub type SE_GMAC_0_T_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_0_t_endian` writer - "]
pub type SE_GMAC_0_T_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_GMAC_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_gmac_0_h_endian` reader - "]
pub type SE_GMAC_0_H_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_0_h_endian` writer - "]
pub type SE_GMAC_0_H_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_GMAC_0_CTRL_0_SPEC, bool, O>;
#[doc = "Field `se_gmac_0_x_endian` reader - "]
pub type SE_GMAC_0_X_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_0_x_endian` writer - "]
pub type SE_GMAC_0_X_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_GMAC_0_CTRL_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_gmac_0_busy(&self) -> SE_GMAC_0_BUSY_R {
        SE_GMAC_0_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_gmac_0_trig_1t(&self) -> SE_GMAC_0_TRIG_1T_R {
        SE_GMAC_0_TRIG_1T_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_gmac_0_en(&self) -> SE_GMAC_0_EN_R {
        SE_GMAC_0_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_gmac_0_int(&self) -> SE_GMAC_0_INT_R {
        SE_GMAC_0_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_gmac_0_int_clr_1t(&self) -> SE_GMAC_0_INT_CLR_1T_R {
        SE_GMAC_0_INT_CLR_1T_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_gmac_0_int_set_1t(&self) -> SE_GMAC_0_INT_SET_1T_R {
        SE_GMAC_0_INT_SET_1T_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_gmac_0_int_mask(&self) -> SE_GMAC_0_INT_MASK_R {
        SE_GMAC_0_INT_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_gmac_0_t_endian(&self) -> SE_GMAC_0_T_ENDIAN_R {
        SE_GMAC_0_T_ENDIAN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_gmac_0_h_endian(&self) -> SE_GMAC_0_H_ENDIAN_R {
        SE_GMAC_0_H_ENDIAN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_gmac_0_x_endian(&self) -> SE_GMAC_0_X_ENDIAN_R {
        SE_GMAC_0_X_ENDIAN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn se_gmac_0_trig_1t(&mut self) -> SE_GMAC_0_TRIG_1T_W<1> {
        SE_GMAC_0_TRIG_1T_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn se_gmac_0_en(&mut self) -> SE_GMAC_0_EN_W<2> {
        SE_GMAC_0_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn se_gmac_0_int_clr_1t(&mut self) -> SE_GMAC_0_INT_CLR_1T_W<9> {
        SE_GMAC_0_INT_CLR_1T_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn se_gmac_0_int_set_1t(&mut self) -> SE_GMAC_0_INT_SET_1T_W<10> {
        SE_GMAC_0_INT_SET_1T_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn se_gmac_0_int_mask(&mut self) -> SE_GMAC_0_INT_MASK_W<11> {
        SE_GMAC_0_INT_MASK_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn se_gmac_0_t_endian(&mut self) -> SE_GMAC_0_T_ENDIAN_W<12> {
        SE_GMAC_0_T_ENDIAN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn se_gmac_0_h_endian(&mut self) -> SE_GMAC_0_H_ENDIAN_W<13> {
        SE_GMAC_0_H_ENDIAN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn se_gmac_0_x_endian(&mut self) -> SE_GMAC_0_X_ENDIAN_W<14> {
        SE_GMAC_0_X_ENDIAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_gmac_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_gmac_0_ctrl_0](index.html) module"]
pub struct SE_GMAC_0_CTRL_0_SPEC;
impl crate::RegisterSpec for SE_GMAC_0_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_gmac_0_ctrl_0::R](R) reader structure"]
impl crate::Readable for SE_GMAC_0_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_gmac_0_ctrl_0::W](W) writer structure"]
impl crate::Writable for SE_GMAC_0_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_gmac_0_ctrl_0 to value 0x7000"]
impl crate::Resettable for SE_GMAC_0_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x7000;
}

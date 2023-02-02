#[doc = "Register `rc32m_ctrl1` reader"]
pub struct R(crate::R<RC32M_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC32M_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC32M_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC32M_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rc32m_ctrl1` writer"]
pub struct W(crate::W<RC32M_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC32M_CTRL1_SPEC>;
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
impl From<crate::W<RC32M_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC32M_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rc32m_test_en` reader - "]
pub type RC32M_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `rc32m_test_en` writer - "]
pub type RC32M_TEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32M_CTRL1_SPEC, bool, O>;
#[doc = "Field `rc32m_soft_rst` reader - "]
pub type RC32M_SOFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `rc32m_soft_rst` writer - "]
pub type RC32M_SOFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32M_CTRL1_SPEC, bool, O>;
#[doc = "Field `rc32m_clk_soft_rst` reader - "]
pub type RC32M_CLK_SOFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `rc32m_clk_soft_rst` writer - "]
pub type RC32M_CLK_SOFT_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RC32M_CTRL1_SPEC, bool, O>;
#[doc = "Field `rc32m_clk_inv` reader - "]
pub type RC32M_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `rc32m_clk_inv` writer - "]
pub type RC32M_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32M_CTRL1_SPEC, bool, O>;
#[doc = "Field `rc32m_clk_force_on` reader - "]
pub type RC32M_CLK_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `rc32m_clk_force_on` writer - "]
pub type RC32M_CLK_FORCE_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RC32M_CTRL1_SPEC, bool, O>;
#[doc = "Field `rc32m_reserved` reader - "]
pub type RC32M_RESERVED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rc32m_reserved` writer - "]
pub type RC32M_RESERVED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RC32M_CTRL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_test_en(&self) -> RC32M_TEST_EN_R {
        RC32M_TEST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_soft_rst(&self) -> RC32M_SOFT_RST_R {
        RC32M_SOFT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_clk_soft_rst(&self) -> RC32M_CLK_SOFT_RST_R {
        RC32M_CLK_SOFT_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rc32m_clk_inv(&self) -> RC32M_CLK_INV_R {
        RC32M_CLK_INV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rc32m_clk_force_on(&self) -> RC32M_CLK_FORCE_ON_R {
        RC32M_CLK_FORCE_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rc32m_reserved(&self) -> RC32M_RESERVED_R {
        RC32M_RESERVED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_test_en(&mut self) -> RC32M_TEST_EN_W<0> {
        RC32M_TEST_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_soft_rst(&mut self) -> RC32M_SOFT_RST_W<1> {
        RC32M_SOFT_RST_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_clk_soft_rst(&mut self) -> RC32M_CLK_SOFT_RST_W<2> {
        RC32M_CLK_SOFT_RST_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_clk_inv(&mut self) -> RC32M_CLK_INV_W<3> {
        RC32M_CLK_INV_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_clk_force_on(&mut self) -> RC32M_CLK_FORCE_ON_W<4> {
        RC32M_CLK_FORCE_ON_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_reserved(&mut self) -> RC32M_RESERVED_W<24> {
        RC32M_RESERVED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rc32m_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32m_ctrl1](index.html) module"]
pub struct RC32M_CTRL1_SPEC;
impl crate::RegisterSpec for RC32M_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc32m_ctrl1::R](R) reader structure"]
impl crate::Readable for RC32M_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc32m_ctrl1::W](W) writer structure"]
impl crate::Writable for RC32M_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rc32m_ctrl1 to value 0"]
impl crate::Resettable for RC32M_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

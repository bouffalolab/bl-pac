#[doc = "Register `i2c_prd_data` reader"]
pub struct R(crate::R<I2C_PRD_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_PRD_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_PRD_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_PRD_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_prd_data` writer"]
pub struct W(crate::W<I2C_PRD_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_PRD_DATA_SPEC>;
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
impl From<crate::W<I2C_PRD_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_PRD_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_i2c_prd_d_ph_0` reader - "]
pub type CR_I2C_PRD_D_PH_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_i2c_prd_d_ph_0` writer - "]
pub type CR_I2C_PRD_D_PH_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_PRD_DATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_i2c_prd_d_ph_1` reader - "]
pub type CR_I2C_PRD_D_PH_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_i2c_prd_d_ph_1` writer - "]
pub type CR_I2C_PRD_D_PH_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_PRD_DATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_i2c_prd_d_ph_2` reader - "]
pub type CR_I2C_PRD_D_PH_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_i2c_prd_d_ph_2` writer - "]
pub type CR_I2C_PRD_D_PH_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_PRD_DATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_i2c_prd_d_ph_3` reader - "]
pub type CR_I2C_PRD_D_PH_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_i2c_prd_d_ph_3` writer - "]
pub type CR_I2C_PRD_D_PH_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_PRD_DATA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_0(&self) -> CR_I2C_PRD_D_PH_0_R {
        CR_I2C_PRD_D_PH_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_1(&self) -> CR_I2C_PRD_D_PH_1_R {
        CR_I2C_PRD_D_PH_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_2(&self) -> CR_I2C_PRD_D_PH_2_R {
        CR_I2C_PRD_D_PH_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_3(&self) -> CR_I2C_PRD_D_PH_3_R {
        CR_I2C_PRD_D_PH_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_prd_d_ph_0(&mut self) -> CR_I2C_PRD_D_PH_0_W<0> {
        CR_I2C_PRD_D_PH_0_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_prd_d_ph_1(&mut self) -> CR_I2C_PRD_D_PH_1_W<8> {
        CR_I2C_PRD_D_PH_1_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_prd_d_ph_2(&mut self) -> CR_I2C_PRD_D_PH_2_W<16> {
        CR_I2C_PRD_D_PH_2_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_prd_d_ph_3(&mut self) -> CR_I2C_PRD_D_PH_3_W<24> {
        CR_I2C_PRD_D_PH_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_prd_data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_prd_data](index.html) module"]
pub struct I2C_PRD_DATA_SPEC;
impl crate::RegisterSpec for I2C_PRD_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_prd_data::R](R) reader structure"]
impl crate::Readable for I2C_PRD_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_prd_data::W](W) writer structure"]
impl crate::Writable for I2C_PRD_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2c_prd_data to value 0x0f0f_0f0f"]
impl crate::Resettable for I2C_PRD_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0f_0f0f;
}

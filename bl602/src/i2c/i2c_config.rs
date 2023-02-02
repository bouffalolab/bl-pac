#[doc = "Register `i2c_config` reader"]
pub struct R(crate::R<I2C_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_config` writer"]
pub struct W(crate::W<I2C_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CONFIG_SPEC>;
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
impl From<crate::W<I2C_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_i2c_m_en` reader - "]
pub type CR_I2C_M_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2c_m_en` writer - "]
pub type CR_I2C_M_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2c_pkt_dir` reader - "]
pub type CR_I2C_PKT_DIR_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2c_pkt_dir` writer - "]
pub type CR_I2C_PKT_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2c_deg_en` reader - "]
pub type CR_I2C_DEG_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2c_deg_en` writer - "]
pub type CR_I2C_DEG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2c_scl_sync_en` reader - "]
pub type CR_I2C_SCL_SYNC_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2c_scl_sync_en` writer - "]
pub type CR_I2C_SCL_SYNC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2C_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2c_sub_addr_en` reader - "]
pub type CR_I2C_SUB_ADDR_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2c_sub_addr_en` writer - "]
pub type CR_I2C_SUB_ADDR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2C_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2c_sub_addr_bc` reader - "]
pub type CR_I2C_SUB_ADDR_BC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_i2c_sub_addr_bc` writer - "]
pub type CR_I2C_SUB_ADDR_BC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_i2c_slv_addr` reader - "]
pub type CR_I2C_SLV_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_i2c_slv_addr` writer - "]
pub type CR_I2C_SLV_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CONFIG_SPEC, u8, u8, 7, O>;
#[doc = "Field `cr_i2c_pkt_len` reader - "]
pub type CR_I2C_PKT_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_i2c_pkt_len` writer - "]
pub type CR_I2C_PKT_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_i2c_deg_cnt` reader - "]
pub type CR_I2C_DEG_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_i2c_deg_cnt` writer - "]
pub type CR_I2C_DEG_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CONFIG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2c_m_en(&self) -> CR_I2C_M_EN_R {
        CR_I2C_M_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_pkt_dir(&self) -> CR_I2C_PKT_DIR_R {
        CR_I2C_PKT_DIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2c_deg_en(&self) -> CR_I2C_DEG_EN_R {
        CR_I2C_DEG_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2c_scl_sync_en(&self) -> CR_I2C_SCL_SYNC_EN_R {
        CR_I2C_SCL_SYNC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_en(&self) -> CR_I2C_SUB_ADDR_EN_R {
        CR_I2C_SUB_ADDR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_bc(&self) -> CR_I2C_SUB_ADDR_BC_R {
        CR_I2C_SUB_ADDR_BC_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn cr_i2c_slv_addr(&self) -> CR_I2C_SLV_ADDR_R {
        CR_I2C_SLV_ADDR_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_pkt_len(&self) -> CR_I2C_PKT_LEN_R {
        CR_I2C_PKT_LEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_i2c_deg_cnt(&self) -> CR_I2C_DEG_CNT_R {
        CR_I2C_DEG_CNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_m_en(&mut self) -> CR_I2C_M_EN_W<0> {
        CR_I2C_M_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_pkt_dir(&mut self) -> CR_I2C_PKT_DIR_W<1> {
        CR_I2C_PKT_DIR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_deg_en(&mut self) -> CR_I2C_DEG_EN_W<2> {
        CR_I2C_DEG_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_scl_sync_en(&mut self) -> CR_I2C_SCL_SYNC_EN_W<3> {
        CR_I2C_SCL_SYNC_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_sub_addr_en(&mut self) -> CR_I2C_SUB_ADDR_EN_W<4> {
        CR_I2C_SUB_ADDR_EN_W::new(self)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_sub_addr_bc(&mut self) -> CR_I2C_SUB_ADDR_BC_W<5> {
        CR_I2C_SUB_ADDR_BC_W::new(self)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_slv_addr(&mut self) -> CR_I2C_SLV_ADDR_W<8> {
        CR_I2C_SLV_ADDR_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_pkt_len(&mut self) -> CR_I2C_PKT_LEN_W<16> {
        CR_I2C_PKT_LEN_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2c_deg_cnt(&mut self) -> CR_I2C_DEG_CNT_W<28> {
        CR_I2C_DEG_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_config](index.html) module"]
pub struct I2C_CONFIG_SPEC;
impl crate::RegisterSpec for I2C_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_config::R](R) reader structure"]
impl crate::Readable for I2C_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_config::W](W) writer structure"]
impl crate::Writable for I2C_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2c_config to value 0x0a"]
impl crate::Resettable for I2C_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}

#[doc = "Register `sf_ctrl_3` reader"]
pub struct R(crate::R<SF_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CTRL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_3` writer"]
pub struct W(crate::W<SF_CTRL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_3_SPEC>;
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
impl From<crate::W<SF_CTRL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_CTRL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_cmds_wrap_len` reader - "]
pub type SF_CMDS_WRAP_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_cmds_wrap_len` writer - "]
pub type SF_CMDS_WRAP_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_CTRL_3_SPEC, u8, u8, 4, O>;
#[doc = "Field `sf_cmds_en` reader - "]
pub type SF_CMDS_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_cmds_en` writer - "]
pub type SF_CMDS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_3_SPEC, bool, O>;
#[doc = "Field `sf_cmds_bt_dly` reader - "]
pub type SF_CMDS_BT_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_cmds_bt_dly` writer - "]
pub type SF_CMDS_BT_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_CTRL_3_SPEC, u8, u8, 3, O>;
#[doc = "Field `sf_cmds_bt_en` reader - "]
pub type SF_CMDS_BT_EN_R = crate::BitReader<bool>;
#[doc = "Field `sf_cmds_bt_en` writer - "]
pub type SF_CMDS_BT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_3_SPEC, bool, O>;
#[doc = "Field `sf_cmds_wrap_q_ini` reader - "]
pub type SF_CMDS_WRAP_Q_INI_R = crate::BitReader<bool>;
#[doc = "Field `sf_cmds_wrap_q_ini` writer - "]
pub type SF_CMDS_WRAP_Q_INI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_3_SPEC, bool, O>;
#[doc = "Field `sf_cmds_wrap_mode` reader - "]
pub type SF_CMDS_WRAP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `sf_cmds_wrap_mode` writer - "]
pub type SF_CMDS_WRAP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SF_CTRL_3_SPEC, bool, O>;
#[doc = "Field `sf_if_1_ack_lat` reader - "]
pub type SF_IF_1_ACK_LAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_if_1_ack_lat` writer - "]
pub type SF_IF_1_ACK_LAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_CTRL_3_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sf_cmds_wrap_len(&self) -> SF_CMDS_WRAP_LEN_R {
        SF_CMDS_WRAP_LEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_cmds_en(&self) -> SF_CMDS_EN_R {
        SF_CMDS_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn sf_cmds_bt_dly(&self) -> SF_CMDS_BT_DLY_R {
        SF_CMDS_BT_DLY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sf_cmds_bt_en(&self) -> SF_CMDS_BT_EN_R {
        SF_CMDS_BT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sf_cmds_wrap_q_ini(&self) -> SF_CMDS_WRAP_Q_INI_R {
        SF_CMDS_WRAP_Q_INI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sf_cmds_wrap_mode(&self) -> SF_CMDS_WRAP_MODE_R {
        SF_CMDS_WRAP_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sf_if_1_ack_lat(&self) -> SF_IF_1_ACK_LAT_R {
        SF_IF_1_ACK_LAT_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn sf_cmds_wrap_len(&mut self) -> SF_CMDS_WRAP_LEN_W<0> {
        SF_CMDS_WRAP_LEN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sf_cmds_en(&mut self) -> SF_CMDS_EN_W<4> {
        SF_CMDS_EN_W::new(self)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    #[must_use]
    pub fn sf_cmds_bt_dly(&mut self) -> SF_CMDS_BT_DLY_W<5> {
        SF_CMDS_BT_DLY_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sf_cmds_bt_en(&mut self) -> SF_CMDS_BT_EN_W<8> {
        SF_CMDS_BT_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sf_cmds_wrap_q_ini(&mut self) -> SF_CMDS_WRAP_Q_INI_W<9> {
        SF_CMDS_WRAP_Q_INI_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sf_cmds_wrap_mode(&mut self) -> SF_CMDS_WRAP_MODE_W<10> {
        SF_CMDS_WRAP_MODE_W::new(self)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_1_ack_lat(&mut self) -> SF_IF_1_ACK_LAT_W<29> {
        SF_IF_1_ACK_LAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_3](index.html) module"]
pub struct SF_CTRL_3_SPEC;
impl crate::RegisterSpec for SF_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_3::R](R) reader structure"]
impl crate::Readable for SF_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_3::W](W) writer structure"]
impl crate::Writable for SF_CTRL_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_ctrl_3 to value 0x2000_0046"]
impl crate::Resettable for SF_CTRL_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0046;
}

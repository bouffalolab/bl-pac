#[doc = "Register `reg_data_1_lock` reader"]
pub struct R(crate::R<REG_DATA_1_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_DATA_1_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_DATA_1_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_DATA_1_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `reg_data_1_lock` writer"]
pub struct W(crate::W<REG_DATA_1_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_DATA_1_LOCK_SPEC>;
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
impl From<crate::W<REG_DATA_1_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_DATA_1_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED_9_0` reader - "]
pub type RESERVED_9_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED_9_0` writer - "]
pub type RESERVED_9_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG_DATA_1_LOCK_SPEC, u16, u16, 10, O>;
#[doc = "Field `wr_lock_key_slot_6` reader - "]
pub type WR_LOCK_KEY_SLOT_6_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_6` writer - "]
pub type WR_LOCK_KEY_SLOT_6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_7` reader - "]
pub type WR_LOCK_KEY_SLOT_7_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_7` writer - "]
pub type WR_LOCK_KEY_SLOT_7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_8` reader - "]
pub type WR_LOCK_KEY_SLOT_8_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_8` writer - "]
pub type WR_LOCK_KEY_SLOT_8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_9` reader - "]
pub type WR_LOCK_KEY_SLOT_9_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_9` writer - "]
pub type WR_LOCK_KEY_SLOT_9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `RESERVED_25_16` reader - "]
pub type RESERVED_25_16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED_25_16` writer - "]
pub type RESERVED_25_16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG_DATA_1_LOCK_SPEC, u16, u16, 10, O>;
#[doc = "Field `rd_lock_key_slot_6` reader - "]
pub type RD_LOCK_KEY_SLOT_6_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_6` writer - "]
pub type RD_LOCK_KEY_SLOT_6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_7` reader - "]
pub type RD_LOCK_KEY_SLOT_7_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_7` writer - "]
pub type RD_LOCK_KEY_SLOT_7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_8` reader - "]
pub type RD_LOCK_KEY_SLOT_8_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_8` writer - "]
pub type RD_LOCK_KEY_SLOT_8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_9` reader - "]
pub type RD_LOCK_KEY_SLOT_9_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_9` writer - "]
pub type RD_LOCK_KEY_SLOT_9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_DATA_1_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reserved_9_0(&self) -> RESERVED_9_0_R {
        RESERVED_9_0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wr_lock_key_slot_6(&self) -> WR_LOCK_KEY_SLOT_6_R {
        WR_LOCK_KEY_SLOT_6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wr_lock_key_slot_7(&self) -> WR_LOCK_KEY_SLOT_7_R {
        WR_LOCK_KEY_SLOT_7_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wr_lock_key_slot_8(&self) -> WR_LOCK_KEY_SLOT_8_R {
        WR_LOCK_KEY_SLOT_8_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_9(&self) -> WR_LOCK_KEY_SLOT_9_R {
        WR_LOCK_KEY_SLOT_9_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn reserved_25_16(&self) -> RESERVED_25_16_R {
        RESERVED_25_16_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_6(&self) -> RD_LOCK_KEY_SLOT_6_R {
        RD_LOCK_KEY_SLOT_6_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_7(&self) -> RD_LOCK_KEY_SLOT_7_R {
        RD_LOCK_KEY_SLOT_7_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_8(&self) -> RD_LOCK_KEY_SLOT_8_R {
        RD_LOCK_KEY_SLOT_8_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_9(&self) -> RD_LOCK_KEY_SLOT_9_R {
        RD_LOCK_KEY_SLOT_9_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_9_0(&mut self) -> RESERVED_9_0_W<0> {
        RESERVED_9_0_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_6(&mut self) -> WR_LOCK_KEY_SLOT_6_W<10> {
        WR_LOCK_KEY_SLOT_6_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_7(&mut self) -> WR_LOCK_KEY_SLOT_7_W<11> {
        WR_LOCK_KEY_SLOT_7_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_8(&mut self) -> WR_LOCK_KEY_SLOT_8_W<12> {
        WR_LOCK_KEY_SLOT_8_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_9(&mut self) -> WR_LOCK_KEY_SLOT_9_W<13> {
        WR_LOCK_KEY_SLOT_9_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_25_16(&mut self) -> RESERVED_25_16_W<16> {
        RESERVED_25_16_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_6(&mut self) -> RD_LOCK_KEY_SLOT_6_W<26> {
        RD_LOCK_KEY_SLOT_6_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_7(&mut self) -> RD_LOCK_KEY_SLOT_7_W<27> {
        RD_LOCK_KEY_SLOT_7_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_8(&mut self) -> RD_LOCK_KEY_SLOT_8_W<28> {
        RD_LOCK_KEY_SLOT_8_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_9(&mut self) -> RD_LOCK_KEY_SLOT_9_W<29> {
        RD_LOCK_KEY_SLOT_9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reg_data_1_lock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_data_1_lock](index.html) module"]
pub struct REG_DATA_1_LOCK_SPEC;
impl crate::RegisterSpec for REG_DATA_1_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_data_1_lock::R](R) reader structure"]
impl crate::Readable for REG_DATA_1_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_data_1_lock::W](W) writer structure"]
impl crate::Writable for REG_DATA_1_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets reg_data_1_lock to value 0"]
impl crate::Resettable for REG_DATA_1_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

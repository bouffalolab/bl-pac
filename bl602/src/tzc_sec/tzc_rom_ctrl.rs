#[doc = "Register `tzc_rom_ctrl` reader"]
pub struct R(crate::R<TZC_ROM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_ROM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_ROM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_ROM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_rom_ctrl` writer"]
pub struct W(crate::W<TZC_ROM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_ROM_CTRL_SPEC>;
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
impl From<crate::W<TZC_ROM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_ROM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_rom0_r0_id0_en` reader - "]
pub type TZC_ROM0_R0_ID0_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom0_r0_id0_en` writer - "]
pub type TZC_ROM0_R0_ID0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom0_r1_id0_en` reader - "]
pub type TZC_ROM0_R1_ID0_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom0_r1_id0_en` writer - "]
pub type TZC_ROM0_R1_ID0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom1_r0_id0_en` reader - "]
pub type TZC_ROM1_R0_ID0_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom1_r0_id0_en` writer - "]
pub type TZC_ROM1_R0_ID0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom1_r1_id0_en` reader - "]
pub type TZC_ROM1_R1_ID0_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom1_r1_id0_en` writer - "]
pub type TZC_ROM1_R1_ID0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom0_r0_id1_en` reader - "]
pub type TZC_ROM0_R0_ID1_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom0_r0_id1_en` writer - "]
pub type TZC_ROM0_R0_ID1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom0_r1_id1_en` reader - "]
pub type TZC_ROM0_R1_ID1_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom0_r1_id1_en` writer - "]
pub type TZC_ROM0_R1_ID1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom1_r0_id1_en` reader - "]
pub type TZC_ROM1_R0_ID1_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom1_r0_id1_en` writer - "]
pub type TZC_ROM1_R0_ID1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom1_r1_id1_en` reader - "]
pub type TZC_ROM1_R1_ID1_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom1_r1_id1_en` writer - "]
pub type TZC_ROM1_R1_ID1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom0_r0_en` reader - "]
pub type TZC_ROM0_R0_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom0_r0_en` writer - "]
pub type TZC_ROM0_R0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom0_r1_en` reader - "]
pub type TZC_ROM0_R1_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom0_r1_en` writer - "]
pub type TZC_ROM0_R1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom1_r0_en` reader - "]
pub type TZC_ROM1_R0_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom1_r0_en` writer - "]
pub type TZC_ROM1_R0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom1_r1_en` reader - "]
pub type TZC_ROM1_R1_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom1_r1_en` writer - "]
pub type TZC_ROM1_R1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom0_r0_lock` reader - "]
pub type TZC_ROM0_R0_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom0_r0_lock` writer - "]
pub type TZC_ROM0_R0_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom0_r1_lock` reader - "]
pub type TZC_ROM0_R1_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom0_r1_lock` writer - "]
pub type TZC_ROM0_R1_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom1_r0_lock` reader - "]
pub type TZC_ROM1_R0_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom1_r0_lock` writer - "]
pub type TZC_ROM1_R0_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_rom1_r1_lock` reader - "]
pub type TZC_ROM1_R1_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_rom1_r1_lock` writer - "]
pub type TZC_ROM1_R1_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_ROM_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_sboot_done` reader - "]
pub type TZC_SBOOT_DONE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_sboot_done` writer - "]
pub type TZC_SBOOT_DONE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_ROM_CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id0_en(&self) -> TZC_ROM0_R0_ID0_EN_R {
        TZC_ROM0_R0_ID0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id0_en(&self) -> TZC_ROM0_R1_ID0_EN_R {
        TZC_ROM0_R1_ID0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id0_en(&self) -> TZC_ROM1_R0_ID0_EN_R {
        TZC_ROM1_R0_ID0_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id0_en(&self) -> TZC_ROM1_R1_ID0_EN_R {
        TZC_ROM1_R1_ID0_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id1_en(&self) -> TZC_ROM0_R0_ID1_EN_R {
        TZC_ROM0_R0_ID1_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id1_en(&self) -> TZC_ROM0_R1_ID1_EN_R {
        TZC_ROM0_R1_ID1_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id1_en(&self) -> TZC_ROM1_R0_ID1_EN_R {
        TZC_ROM1_R0_ID1_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id1_en(&self) -> TZC_ROM1_R1_ID1_EN_R {
        TZC_ROM1_R1_ID1_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_rom0_r0_en(&self) -> TZC_ROM0_R0_EN_R {
        TZC_ROM0_R0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_rom0_r1_en(&self) -> TZC_ROM0_R1_EN_R {
        TZC_ROM0_R1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_rom1_r0_en(&self) -> TZC_ROM1_R0_EN_R {
        TZC_ROM1_R0_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_rom1_r1_en(&self) -> TZC_ROM1_R1_EN_R {
        TZC_ROM1_R1_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_rom0_r0_lock(&self) -> TZC_ROM0_R0_LOCK_R {
        TZC_ROM0_R0_LOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_rom0_r1_lock(&self) -> TZC_ROM0_R1_LOCK_R {
        TZC_ROM0_R1_LOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_rom1_r0_lock(&self) -> TZC_ROM1_R0_LOCK_R {
        TZC_ROM1_R0_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_rom1_r1_lock(&self) -> TZC_ROM1_R1_LOCK_R {
        TZC_ROM1_R1_LOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn tzc_sboot_done(&self) -> TZC_SBOOT_DONE_R {
        TZC_SBOOT_DONE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom0_r0_id0_en(&mut self) -> TZC_ROM0_R0_ID0_EN_W<0> {
        TZC_ROM0_R0_ID0_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom0_r1_id0_en(&mut self) -> TZC_ROM0_R1_ID0_EN_W<1> {
        TZC_ROM0_R1_ID0_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom1_r0_id0_en(&mut self) -> TZC_ROM1_R0_ID0_EN_W<2> {
        TZC_ROM1_R0_ID0_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom1_r1_id0_en(&mut self) -> TZC_ROM1_R1_ID0_EN_W<3> {
        TZC_ROM1_R1_ID0_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom0_r0_id1_en(&mut self) -> TZC_ROM0_R0_ID1_EN_W<8> {
        TZC_ROM0_R0_ID1_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom0_r1_id1_en(&mut self) -> TZC_ROM0_R1_ID1_EN_W<9> {
        TZC_ROM0_R1_ID1_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom1_r0_id1_en(&mut self) -> TZC_ROM1_R0_ID1_EN_W<10> {
        TZC_ROM1_R0_ID1_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom1_r1_id1_en(&mut self) -> TZC_ROM1_R1_ID1_EN_W<11> {
        TZC_ROM1_R1_ID1_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom0_r0_en(&mut self) -> TZC_ROM0_R0_EN_W<16> {
        TZC_ROM0_R0_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom0_r1_en(&mut self) -> TZC_ROM0_R1_EN_W<17> {
        TZC_ROM0_R1_EN_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom1_r0_en(&mut self) -> TZC_ROM1_R0_EN_W<18> {
        TZC_ROM1_R0_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom1_r1_en(&mut self) -> TZC_ROM1_R1_EN_W<19> {
        TZC_ROM1_R1_EN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom0_r0_lock(&mut self) -> TZC_ROM0_R0_LOCK_W<24> {
        TZC_ROM0_R0_LOCK_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom0_r1_lock(&mut self) -> TZC_ROM0_R1_LOCK_W<25> {
        TZC_ROM0_R1_LOCK_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom1_r0_lock(&mut self) -> TZC_ROM1_R0_LOCK_W<26> {
        TZC_ROM1_R0_LOCK_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_rom1_r1_lock(&mut self) -> TZC_ROM1_R1_LOCK_W<27> {
        TZC_ROM1_R1_LOCK_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sboot_done(&mut self) -> TZC_SBOOT_DONE_W<28> {
        TZC_SBOOT_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_rom_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom_ctrl](index.html) module"]
pub struct TZC_ROM_CTRL_SPEC;
impl crate::RegisterSpec for TZC_ROM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_rom_ctrl::R](R) reader structure"]
impl crate::Readable for TZC_ROM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_rom_ctrl::W](W) writer structure"]
impl crate::Writable for TZC_ROM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_rom_ctrl to value 0x0f0f"]
impl crate::Resettable for TZC_ROM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0f;
}

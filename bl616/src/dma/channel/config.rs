#[doc = "Register `config` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `config` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `function` reader - Enable or disable DMA channel"]
pub type FUNCTION_R = crate::BitReader<bool>;
#[doc = "Field `function` writer - Enable or disable DMA channel"]
pub type FUNCTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `source_peripheral` reader - Set source peripheral for this DMA channel"]
pub type SOURCE_PERIPHERAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `source_peripheral` writer - Set source peripheral for this DMA channel"]
pub type SOURCE_PERIPHERAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `destination_peripheral` reader - Set destination peripheral for this DMA channel"]
pub type DESTINATION_PERIPHERAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `destination_peripheral` writer - Set destination peripheral for this DMA channel"]
pub type DESTINATION_PERIPHERAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `flow_control` reader - Set data direction for this channel"]
pub type FLOW_CONTROL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `flow_control` writer - Set data direction for this channel"]
pub type FLOW_CONTROL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `error_mask` reader - Mask error interrupt"]
pub type ERROR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `error_mask` writer - Mask error interrupt"]
pub type ERROR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `terminate_mask` reader - Mask terminal count interrupt"]
pub type TERMINATE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `terminate_mask` writer - Mask terminal count interrupt"]
pub type TERMINATE_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `active` reader - ??"]
pub type ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `lock` reader - ??"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `lock` writer - ??"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `halt` reader - ??"]
pub type HALT_R = crate::BitReader<bool>;
#[doc = "Field `halt` writer - ??"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `linked_list_counter` reader - ??"]
pub type LINKED_LIST_COUNTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `linked_list_counter` writer - ??"]
pub type LINKED_LIST_COUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - Enable or disable DMA channel"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Set source peripheral for this DMA channel"]
    #[inline(always)]
    pub fn source_peripheral(&self) -> SOURCE_PERIPHERAL_R {
        SOURCE_PERIPHERAL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - Set destination peripheral for this DMA channel"]
    #[inline(always)]
    pub fn destination_peripheral(&self) -> DESTINATION_PERIPHERAL_R {
        DESTINATION_PERIPHERAL_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:13 - Set data direction for this channel"]
    #[inline(always)]
    pub fn flow_control(&self) -> FLOW_CONTROL_R {
        FLOW_CONTROL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Mask error interrupt"]
    #[inline(always)]
    pub fn error_mask(&self) -> ERROR_MASK_R {
        ERROR_MASK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask terminal count interrupt"]
    #[inline(always)]
    pub fn terminate_mask(&self) -> TERMINATE_MASK_R {
        TERMINATE_MASK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ??"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 16 - ??"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ??"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:29 - ??"]
    #[inline(always)]
    pub fn linked_list_counter(&self) -> LINKED_LIST_COUNTER_R {
        LINKED_LIST_COUNTER_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FUNCTION_W<0> {
        FUNCTION_W::new(self)
    }
    #[doc = "Bits 1:5 - Set source peripheral for this DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn source_peripheral(&mut self) -> SOURCE_PERIPHERAL_W<1> {
        SOURCE_PERIPHERAL_W::new(self)
    }
    #[doc = "Bits 6:10 - Set destination peripheral for this DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn destination_peripheral(&mut self) -> DESTINATION_PERIPHERAL_W<6> {
        DESTINATION_PERIPHERAL_W::new(self)
    }
    #[doc = "Bits 11:13 - Set data direction for this channel"]
    #[inline(always)]
    #[must_use]
    pub fn flow_control(&mut self) -> FLOW_CONTROL_W<11> {
        FLOW_CONTROL_W::new(self)
    }
    #[doc = "Bit 14 - Mask error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn error_mask(&mut self) -> ERROR_MASK_W<14> {
        ERROR_MASK_W::new(self)
    }
    #[doc = "Bit 15 - Mask terminal count interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn terminate_mask(&mut self) -> TERMINATE_MASK_W<15> {
        TERMINATE_MASK_W::new(self)
    }
    #[doc = "Bit 16 - ??"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<16> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 17 - ??"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<17> {
        HALT_W::new(self)
    }
    #[doc = "Bits 20:29 - ??"]
    #[inline(always)]
    #[must_use]
    pub fn linked_list_counter(&mut self) -> LINKED_LIST_COUNTER_W<20> {
        LINKED_LIST_COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets config to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

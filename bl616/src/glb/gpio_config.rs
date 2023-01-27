#[doc = "Register `gpio_config[%s]` reader"]
pub struct R(crate::R<GPIO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_config[%s]` writer"]
pub struct W(crate::W<GPIO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CONFIG_SPEC>;
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
impl From<crate::W<GPIO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `input_function` reader - Enable input signal"]
pub type INPUT_FUNCTION_R = crate::BitReader<bool>;
#[doc = "Field `input_function` writer - Enable input signal"]
pub type INPUT_FUNCTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CONFIG_SPEC, bool, O>;
#[doc = "Field `schmitt` reader - Enable schmitt trigger"]
pub type SCHMITT_R = crate::BitReader<bool>;
#[doc = "Field `schmitt` writer - Enable schmitt trigger"]
pub type SCHMITT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CONFIG_SPEC, bool, O>;
#[doc = "Field `drive` reader - Drive strength"]
pub type DRIVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `drive` writer - Drive strength"]
pub type DRIVE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIO_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `pull_up` reader - Enable internal pull-up"]
pub type PULL_UP_R = crate::BitReader<bool>;
#[doc = "Field `pull_up` writer - Enable internal pull-up"]
pub type PULL_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CONFIG_SPEC, bool, O>;
#[doc = "Field `pull_down` reader - Enable internal pull-down"]
pub type PULL_DOWN_R = crate::BitReader<bool>;
#[doc = "Field `pull_down` writer - Enable internal pull-down"]
pub type PULL_DOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CONFIG_SPEC, bool, O>;
#[doc = "Field `output_function` reader - Enable output signal"]
pub type OUTPUT_FUNCTION_R = crate::BitReader<bool>;
#[doc = "Field `output_function` writer - Enable output signal"]
pub type OUTPUT_FUNCTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CONFIG_SPEC, bool, O>;
#[doc = "Field `alternate` reader - Pin alternate function switch"]
pub type ALTERNATE_R = crate::FieldReader<u8, ALTERNATE_A>;
#[doc = "Pin alternate function switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALTERNATE_A {
    #[doc = "0: Secure Digital host"]
    SDH = 0,
    #[doc = "1: Serial Peripheral Interface 0"]
    SPI0 = 1,
    #[doc = "2: Flash control"]
    FLASH = 2,
    #[doc = "3: Inter-IC Sound 0"]
    I2S0 = 3,
    #[doc = "4: Pulse Density Modulation"]
    PDM = 4,
    #[doc = "5: Inter-Integrated Circuit bus 0"]
    I2C0 = 5,
    #[doc = "6: Inter-Integrated Circuit bus 1"]
    I2C1 = 6,
    #[doc = "7: Universal Asynchronous Receiver/Transmitter 0"]
    UART0 = 7,
    #[doc = "8: Ethernet Media Access Control"]
    EMAC = 8,
    #[doc = "9: ??"]
    CAM = 9,
    #[doc = "10: ??"]
    ANALOG = 10,
    #[doc = "11: Generic Purpose Input/Output"]
    GPIO = 11,
    #[doc = "12: ??"]
    SDIO = 12,
    #[doc = "16: Pulse-Width Modulation module 0"]
    PWM0 = 16,
    #[doc = "17: ??"]
    JTAG = 17,
    #[doc = "18: Universal Asynchronous Receiver/Transmitter 1"]
    UART1 = 18,
    #[doc = "19: Pulse-Width Modulation 1"]
    PWM1 = 19,
    #[doc = "20: Serial Peripheral Interface 1"]
    SPI1 = 20,
    #[doc = "21: Inter-IC Sound 1"]
    I2S1 = 21,
    #[doc = "22: ??"]
    DBI_B = 22,
    #[doc = "23: ??"]
    DBI_C = 23,
    #[doc = "24: ??"]
    QSPI = 24,
    #[doc = "25: Audio Pulse-Width Modulation"]
    APWM = 25,
    #[doc = "31: Clock output"]
    CLOCK_OUT = 31,
}
impl From<ALTERNATE_A> for u8 {
    #[inline(always)]
    fn from(variant: ALTERNATE_A) -> Self {
        variant as _
    }
}
impl ALTERNATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALTERNATE_A> {
        match self.bits {
            0 => Some(ALTERNATE_A::SDH),
            1 => Some(ALTERNATE_A::SPI0),
            2 => Some(ALTERNATE_A::FLASH),
            3 => Some(ALTERNATE_A::I2S0),
            4 => Some(ALTERNATE_A::PDM),
            5 => Some(ALTERNATE_A::I2C0),
            6 => Some(ALTERNATE_A::I2C1),
            7 => Some(ALTERNATE_A::UART0),
            8 => Some(ALTERNATE_A::EMAC),
            9 => Some(ALTERNATE_A::CAM),
            10 => Some(ALTERNATE_A::ANALOG),
            11 => Some(ALTERNATE_A::GPIO),
            12 => Some(ALTERNATE_A::SDIO),
            16 => Some(ALTERNATE_A::PWM0),
            17 => Some(ALTERNATE_A::JTAG),
            18 => Some(ALTERNATE_A::UART1),
            19 => Some(ALTERNATE_A::PWM1),
            20 => Some(ALTERNATE_A::SPI1),
            21 => Some(ALTERNATE_A::I2S1),
            22 => Some(ALTERNATE_A::DBI_B),
            23 => Some(ALTERNATE_A::DBI_C),
            24 => Some(ALTERNATE_A::QSPI),
            25 => Some(ALTERNATE_A::APWM),
            31 => Some(ALTERNATE_A::CLOCK_OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDH`"]
    #[inline(always)]
    pub fn is_sdh(&self) -> bool {
        *self == ALTERNATE_A::SDH
    }
    #[doc = "Checks if the value of the field is `SPI0`"]
    #[inline(always)]
    pub fn is_spi0(&self) -> bool {
        *self == ALTERNATE_A::SPI0
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == ALTERNATE_A::FLASH
    }
    #[doc = "Checks if the value of the field is `I2S0`"]
    #[inline(always)]
    pub fn is_i2s0(&self) -> bool {
        *self == ALTERNATE_A::I2S0
    }
    #[doc = "Checks if the value of the field is `PDM`"]
    #[inline(always)]
    pub fn is_pdm(&self) -> bool {
        *self == ALTERNATE_A::PDM
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == ALTERNATE_A::I2C0
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == ALTERNATE_A::I2C1
    }
    #[doc = "Checks if the value of the field is `UART0`"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == ALTERNATE_A::UART0
    }
    #[doc = "Checks if the value of the field is `EMAC`"]
    #[inline(always)]
    pub fn is_emac(&self) -> bool {
        *self == ALTERNATE_A::EMAC
    }
    #[doc = "Checks if the value of the field is `CAM`"]
    #[inline(always)]
    pub fn is_cam(&self) -> bool {
        *self == ALTERNATE_A::CAM
    }
    #[doc = "Checks if the value of the field is `ANALOG`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == ALTERNATE_A::ANALOG
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == ALTERNATE_A::GPIO
    }
    #[doc = "Checks if the value of the field is `SDIO`"]
    #[inline(always)]
    pub fn is_sdio(&self) -> bool {
        *self == ALTERNATE_A::SDIO
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == ALTERNATE_A::PWM0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == ALTERNATE_A::JTAG
    }
    #[doc = "Checks if the value of the field is `UART1`"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == ALTERNATE_A::UART1
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == ALTERNATE_A::PWM1
    }
    #[doc = "Checks if the value of the field is `SPI1`"]
    #[inline(always)]
    pub fn is_spi1(&self) -> bool {
        *self == ALTERNATE_A::SPI1
    }
    #[doc = "Checks if the value of the field is `I2S1`"]
    #[inline(always)]
    pub fn is_i2s1(&self) -> bool {
        *self == ALTERNATE_A::I2S1
    }
    #[doc = "Checks if the value of the field is `DBI_B`"]
    #[inline(always)]
    pub fn is_dbi_b(&self) -> bool {
        *self == ALTERNATE_A::DBI_B
    }
    #[doc = "Checks if the value of the field is `DBI_C`"]
    #[inline(always)]
    pub fn is_dbi_c(&self) -> bool {
        *self == ALTERNATE_A::DBI_C
    }
    #[doc = "Checks if the value of the field is `QSPI`"]
    #[inline(always)]
    pub fn is_qspi(&self) -> bool {
        *self == ALTERNATE_A::QSPI
    }
    #[doc = "Checks if the value of the field is `APWM`"]
    #[inline(always)]
    pub fn is_apwm(&self) -> bool {
        *self == ALTERNATE_A::APWM
    }
    #[doc = "Checks if the value of the field is `CLOCK_OUT`"]
    #[inline(always)]
    pub fn is_clock_out(&self) -> bool {
        *self == ALTERNATE_A::CLOCK_OUT
    }
}
#[doc = "Field `alternate` writer - Pin alternate function switch"]
pub type ALTERNATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CONFIG_SPEC, u8, ALTERNATE_A, 5, O>;
impl<'a, const O: u8> ALTERNATE_W<'a, O> {
    #[doc = "Secure Digital host"]
    #[inline(always)]
    pub fn sdh(self) -> &'a mut W {
        self.variant(ALTERNATE_A::SDH)
    }
    #[doc = "Serial Peripheral Interface 0"]
    #[inline(always)]
    pub fn spi0(self) -> &'a mut W {
        self.variant(ALTERNATE_A::SPI0)
    }
    #[doc = "Flash control"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(ALTERNATE_A::FLASH)
    }
    #[doc = "Inter-IC Sound 0"]
    #[inline(always)]
    pub fn i2s0(self) -> &'a mut W {
        self.variant(ALTERNATE_A::I2S0)
    }
    #[doc = "Pulse Density Modulation"]
    #[inline(always)]
    pub fn pdm(self) -> &'a mut W {
        self.variant(ALTERNATE_A::PDM)
    }
    #[doc = "Inter-Integrated Circuit bus 0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut W {
        self.variant(ALTERNATE_A::I2C0)
    }
    #[doc = "Inter-Integrated Circuit bus 1"]
    #[inline(always)]
    pub fn i2c1(self) -> &'a mut W {
        self.variant(ALTERNATE_A::I2C1)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn uart0(self) -> &'a mut W {
        self.variant(ALTERNATE_A::UART0)
    }
    #[doc = "Ethernet Media Access Control"]
    #[inline(always)]
    pub fn emac(self) -> &'a mut W {
        self.variant(ALTERNATE_A::EMAC)
    }
    #[doc = "??"]
    #[inline(always)]
    pub fn cam(self) -> &'a mut W {
        self.variant(ALTERNATE_A::CAM)
    }
    #[doc = "??"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(ALTERNATE_A::ANALOG)
    }
    #[doc = "Generic Purpose Input/Output"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(ALTERNATE_A::GPIO)
    }
    #[doc = "??"]
    #[inline(always)]
    pub fn sdio(self) -> &'a mut W {
        self.variant(ALTERNATE_A::SDIO)
    }
    #[doc = "Pulse-Width Modulation module 0"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(ALTERNATE_A::PWM0)
    }
    #[doc = "??"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(ALTERNATE_A::JTAG)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn uart1(self) -> &'a mut W {
        self.variant(ALTERNATE_A::UART1)
    }
    #[doc = "Pulse-Width Modulation 1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(ALTERNATE_A::PWM1)
    }
    #[doc = "Serial Peripheral Interface 1"]
    #[inline(always)]
    pub fn spi1(self) -> &'a mut W {
        self.variant(ALTERNATE_A::SPI1)
    }
    #[doc = "Inter-IC Sound 1"]
    #[inline(always)]
    pub fn i2s1(self) -> &'a mut W {
        self.variant(ALTERNATE_A::I2S1)
    }
    #[doc = "??"]
    #[inline(always)]
    pub fn dbi_b(self) -> &'a mut W {
        self.variant(ALTERNATE_A::DBI_B)
    }
    #[doc = "??"]
    #[inline(always)]
    pub fn dbi_c(self) -> &'a mut W {
        self.variant(ALTERNATE_A::DBI_C)
    }
    #[doc = "??"]
    #[inline(always)]
    pub fn qspi(self) -> &'a mut W {
        self.variant(ALTERNATE_A::QSPI)
    }
    #[doc = "Audio Pulse-Width Modulation"]
    #[inline(always)]
    pub fn apwm(self) -> &'a mut W {
        self.variant(ALTERNATE_A::APWM)
    }
    #[doc = "Clock output"]
    #[inline(always)]
    pub fn clock_out(self) -> &'a mut W {
        self.variant(ALTERNATE_A::CLOCK_OUT)
    }
}
#[doc = "Field `interrupt_mode` reader - Select pin interrupt mode"]
pub type INTERRUPT_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `interrupt_mode` writer - Select pin interrupt mode"]
pub type INTERRUPT_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `interrupt_clear` reader - Clear pin interrupt flag"]
pub type INTERRUPT_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `interrupt_clear` writer - Clear pin interrupt flag"]
pub type INTERRUPT_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CONFIG_SPEC, bool, O>;
#[doc = "Field `interrupt_state` reader - Pin interrupt state"]
pub type INTERRUPT_STATE_R = crate::BitReader<bool>;
#[doc = "Field `interrupt_mask` reader - Pin interrupt mask"]
pub type INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `interrupt_mask` writer - Pin interrupt mask"]
pub type INTERRUPT_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CONFIG_SPEC, bool, O>;
#[doc = "Field `output_value` reader - Output value"]
pub type OUTPUT_VALUE_R = crate::BitReader<bool>;
#[doc = "Field `output_value` writer - Output value"]
pub type OUTPUT_VALUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CONFIG_SPEC, bool, O>;
#[doc = "Field `output_set` writer - Set output value to 1\n\n When sets and clears at the same, only set will take effect."]
pub type OUTPUT_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CONFIG_SPEC, bool, O>;
#[doc = "Field `output_clear` writer - Clear output value to 0\n\n When sets and clears at the same, only set will take effect."]
pub type OUTPUT_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CONFIG_SPEC, bool, O>;
#[doc = "Field `input_value` reader - Input value"]
pub type INPUT_VALUE_R = crate::BitReader<bool>;
#[doc = "Field `pin_mode` reader - Pin input/output mode switch"]
pub type PIN_MODE_R = crate::FieldReader<u8, PIN_MODE_A>;
#[doc = "Pin input/output mode switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PIN_MODE_A {
    #[doc = "0: Output by `output_value` field"]
    OUTPUT_VALUE = 0,
    #[doc = "1: Output set by `output_set` and `output_clear` fields"]
    SET_CLEAR = 1,
    #[doc = "2: Source from GPIO DMA, output by `output_value`"]
    DMA_OUTPUT_VALUE = 2,
    #[doc = "3: Source from GPIO DMA, set by `output_set` and `output_clear`"]
    DMA_SET_CLEAR = 3,
}
impl From<PIN_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN_MODE_A) -> Self {
        variant as _
    }
}
impl PIN_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_MODE_A {
        match self.bits {
            0 => PIN_MODE_A::OUTPUT_VALUE,
            1 => PIN_MODE_A::SET_CLEAR,
            2 => PIN_MODE_A::DMA_OUTPUT_VALUE,
            3 => PIN_MODE_A::DMA_SET_CLEAR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT_VALUE`"]
    #[inline(always)]
    pub fn is_output_value(&self) -> bool {
        *self == PIN_MODE_A::OUTPUT_VALUE
    }
    #[doc = "Checks if the value of the field is `SET_CLEAR`"]
    #[inline(always)]
    pub fn is_set_clear(&self) -> bool {
        *self == PIN_MODE_A::SET_CLEAR
    }
    #[doc = "Checks if the value of the field is `DMA_OUTPUT_VALUE`"]
    #[inline(always)]
    pub fn is_dma_output_value(&self) -> bool {
        *self == PIN_MODE_A::DMA_OUTPUT_VALUE
    }
    #[doc = "Checks if the value of the field is `DMA_SET_CLEAR`"]
    #[inline(always)]
    pub fn is_dma_set_clear(&self) -> bool {
        *self == PIN_MODE_A::DMA_SET_CLEAR
    }
}
#[doc = "Field `pin_mode` writer - Pin input/output mode switch"]
pub type PIN_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPIO_CONFIG_SPEC, u8, PIN_MODE_A, 2, O>;
impl<'a, const O: u8> PIN_MODE_W<'a, O> {
    #[doc = "Output by `output_value` field"]
    #[inline(always)]
    pub fn output_value(self) -> &'a mut W {
        self.variant(PIN_MODE_A::OUTPUT_VALUE)
    }
    #[doc = "Output set by `output_set` and `output_clear` fields"]
    #[inline(always)]
    pub fn set_clear(self) -> &'a mut W {
        self.variant(PIN_MODE_A::SET_CLEAR)
    }
    #[doc = "Source from GPIO DMA, output by `output_value`"]
    #[inline(always)]
    pub fn dma_output_value(self) -> &'a mut W {
        self.variant(PIN_MODE_A::DMA_OUTPUT_VALUE)
    }
    #[doc = "Source from GPIO DMA, set by `output_set` and `output_clear`"]
    #[inline(always)]
    pub fn dma_set_clear(self) -> &'a mut W {
        self.variant(PIN_MODE_A::DMA_SET_CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Enable input signal"]
    #[inline(always)]
    pub fn input_function(&self) -> INPUT_FUNCTION_R {
        INPUT_FUNCTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable schmitt trigger"]
    #[inline(always)]
    pub fn schmitt(&self) -> SCHMITT_R {
        SCHMITT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Drive strength"]
    #[inline(always)]
    pub fn drive(&self) -> DRIVE_R {
        DRIVE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable internal pull-up"]
    #[inline(always)]
    pub fn pull_up(&self) -> PULL_UP_R {
        PULL_UP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable internal pull-down"]
    #[inline(always)]
    pub fn pull_down(&self) -> PULL_DOWN_R {
        PULL_DOWN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable output signal"]
    #[inline(always)]
    pub fn output_function(&self) -> OUTPUT_FUNCTION_R {
        OUTPUT_FUNCTION_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Pin alternate function switch"]
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Select pin interrupt mode"]
    #[inline(always)]
    pub fn interrupt_mode(&self) -> INTERRUPT_MODE_R {
        INTERRUPT_MODE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Clear pin interrupt flag"]
    #[inline(always)]
    pub fn interrupt_clear(&self) -> INTERRUPT_CLEAR_R {
        INTERRUPT_CLEAR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pin interrupt state"]
    #[inline(always)]
    pub fn interrupt_state(&self) -> INTERRUPT_STATE_R {
        INTERRUPT_STATE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pin interrupt mask"]
    #[inline(always)]
    pub fn interrupt_mask(&self) -> INTERRUPT_MASK_R {
        INTERRUPT_MASK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Output value"]
    #[inline(always)]
    pub fn output_value(&self) -> OUTPUT_VALUE_R {
        OUTPUT_VALUE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Input value"]
    #[inline(always)]
    pub fn input_value(&self) -> INPUT_VALUE_R {
        INPUT_VALUE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Pin input/output mode switch"]
    #[inline(always)]
    pub fn pin_mode(&self) -> PIN_MODE_R {
        PIN_MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable input signal"]
    #[inline(always)]
    #[must_use]
    pub fn input_function(&mut self) -> INPUT_FUNCTION_W<0> {
        INPUT_FUNCTION_W::new(self)
    }
    #[doc = "Bit 1 - Enable schmitt trigger"]
    #[inline(always)]
    #[must_use]
    pub fn schmitt(&mut self) -> SCHMITT_W<1> {
        SCHMITT_W::new(self)
    }
    #[doc = "Bits 2:3 - Drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn drive(&mut self) -> DRIVE_W<2> {
        DRIVE_W::new(self)
    }
    #[doc = "Bit 4 - Enable internal pull-up"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up(&mut self) -> PULL_UP_W<4> {
        PULL_UP_W::new(self)
    }
    #[doc = "Bit 5 - Enable internal pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down(&mut self) -> PULL_DOWN_W<5> {
        PULL_DOWN_W::new(self)
    }
    #[doc = "Bit 6 - Enable output signal"]
    #[inline(always)]
    #[must_use]
    pub fn output_function(&mut self) -> OUTPUT_FUNCTION_W<6> {
        OUTPUT_FUNCTION_W::new(self)
    }
    #[doc = "Bits 8:12 - Pin alternate function switch"]
    #[inline(always)]
    #[must_use]
    pub fn alternate(&mut self) -> ALTERNATE_W<8> {
        ALTERNATE_W::new(self)
    }
    #[doc = "Bits 16:19 - Select pin interrupt mode"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_mode(&mut self) -> INTERRUPT_MODE_W<16> {
        INTERRUPT_MODE_W::new(self)
    }
    #[doc = "Bit 20 - Clear pin interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_clear(&mut self) -> INTERRUPT_CLEAR_W<20> {
        INTERRUPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 22 - Pin interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_mask(&mut self) -> INTERRUPT_MASK_W<22> {
        INTERRUPT_MASK_W::new(self)
    }
    #[doc = "Bit 24 - Output value"]
    #[inline(always)]
    #[must_use]
    pub fn output_value(&mut self) -> OUTPUT_VALUE_W<24> {
        OUTPUT_VALUE_W::new(self)
    }
    #[doc = "Bit 25 - Set output value to 1\n\n When sets and clears at the same, only set will take effect."]
    #[inline(always)]
    #[must_use]
    pub fn output_set(&mut self) -> OUTPUT_SET_W<25> {
        OUTPUT_SET_W::new(self)
    }
    #[doc = "Bit 26 - Clear output value to 0\n\n When sets and clears at the same, only set will take effect."]
    #[inline(always)]
    #[must_use]
    pub fn output_clear(&mut self) -> OUTPUT_CLEAR_W<26> {
        OUTPUT_CLEAR_W::new(self)
    }
    #[doc = "Bits 30:31 - Pin input/output mode switch"]
    #[inline(always)]
    #[must_use]
    pub fn pin_mode(&mut self) -> PIN_MODE_W<30> {
        PIN_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic Purpose Input/Output config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_config](index.html) module"]
pub struct GPIO_CONFIG_SPEC;
impl crate::RegisterSpec for GPIO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_config::R](R) reader structure"]
impl crate::Readable for GPIO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_config::W](W) writer structure"]
impl crate::Writable for GPIO_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_config[%s] to value 0"]
impl crate::Resettable for GPIO_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<Apb2enSpec>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<Apb2enSpec>;
#[doc = "Field `SYSCFGCMPEN` reader - Syscfg and comparator clock enable"]
pub type SyscfgcmpenR = crate::BitReader;
#[doc = "Field `SYSCFGCMPEN` writer - Syscfg and comparator clock enable"]
pub type SyscfgcmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1EN` reader - Timer1 clock enable"]
pub type Tmr1enR = crate::BitReader;
#[doc = "Field `TMR1EN` writer - Timer1 clock enable"]
pub type Tmr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type Usart1enR = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR15EN` reader - Timer15 clock enable"]
pub type Tmr15enR = crate::BitReader;
#[doc = "Field `TMR15EN` writer - Timer15 clock enable"]
pub type Tmr15enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR16EN` reader - Timer16 clock enable"]
pub type Tmr16enR = crate::BitReader;
#[doc = "Field `TMR16EN` writer - Timer16 clock enable"]
pub type Tmr16enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR17EN` reader - Timer17 clock enable"]
pub type Tmr17enR = crate::BitReader;
#[doc = "Field `TMR17EN` writer - Timer17 clock enable"]
pub type Tmr17enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Syscfg and comparator clock enable"]
    #[inline(always)]
    pub fn syscfgcmpen(&self) -> SyscfgcmpenR {
        SyscfgcmpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer1 clock enable"]
    #[inline(always)]
    pub fn tmr1en(&self) -> Tmr1enR {
        Tmr1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer15 clock enable"]
    #[inline(always)]
    pub fn tmr15en(&self) -> Tmr15enR {
        Tmr15enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer16 clock enable"]
    #[inline(always)]
    pub fn tmr16en(&self) -> Tmr16enR {
        Tmr16enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer17 clock enable"]
    #[inline(always)]
    pub fn tmr17en(&self) -> Tmr17enR {
        Tmr17enR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Syscfg and comparator clock enable"]
    #[inline(always)]
    pub fn syscfgcmpen(&mut self) -> SyscfgcmpenW<'_, Apb2enSpec> {
        SyscfgcmpenW::new(self, 0)
    }
    #[doc = "Bit 9 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> AdcenW<'_, Apb2enSpec> {
        AdcenW::new(self, 9)
    }
    #[doc = "Bit 11 - Timer1 clock enable"]
    #[inline(always)]
    pub fn tmr1en(&mut self) -> Tmr1enW<'_, Apb2enSpec> {
        Tmr1enW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> Spi1enW<'_, Apb2enSpec> {
        Spi1enW::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> Usart1enW<'_, Apb2enSpec> {
        Usart1enW::new(self, 14)
    }
    #[doc = "Bit 16 - Timer15 clock enable"]
    #[inline(always)]
    pub fn tmr15en(&mut self) -> Tmr15enW<'_, Apb2enSpec> {
        Tmr15enW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer16 clock enable"]
    #[inline(always)]
    pub fn tmr16en(&mut self) -> Tmr16enW<'_, Apb2enSpec> {
        Tmr16enW::new(self, 17)
    }
    #[doc = "Bit 18 - Timer17 clock enable"]
    #[inline(always)]
    pub fn tmr17en(&mut self) -> Tmr17enW<'_, Apb2enSpec> {
        Tmr17enW::new(self, 18)
    }
}
#[doc = "APB2 peripheral clock enable register (CRM_APB2EN)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2enSpec;
impl crate::RegisterSpec for Apb2enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2en::R`](R) reader structure"]
impl crate::Readable for Apb2enSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2en::W`](W) writer structure"]
impl crate::Writable for Apb2enSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2EN to value 0"]
impl crate::Resettable for Apb2enSpec {}

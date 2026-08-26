#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<Apb2rstSpec>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<Apb2rstSpec>;
#[doc = "Field `SCFGCMPRST` reader - System config and comparator reset"]
pub type ScfgcmprstR = crate::BitReader;
#[doc = "Field `SCFGCMPRST` writer - System config and comparator reset"]
pub type ScfgcmprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXINTRST` reader - External interrupt reset"]
pub type ExintrstR = crate::BitReader;
#[doc = "Field `EXINTRST` writer - External interrupt reset"]
pub type ExintrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type AdcrstR = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR15RST` reader - Timer15 reset"]
pub type Tmr15rstR = crate::BitReader;
#[doc = "Field `TMR15RST` writer - Timer15 reset"]
pub type Tmr15rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR16RST` reader - Timer16 reset"]
pub type Tmr16rstR = crate::BitReader;
#[doc = "Field `TMR16RST` writer - Timer16 reset"]
pub type Tmr16rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR17RST` reader - Timer17 reset"]
pub type Tmr17rstR = crate::BitReader;
#[doc = "Field `TMR17RST` writer - Timer17 reset"]
pub type Tmr17rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System config and comparator reset"]
    #[inline(always)]
    pub fn scfgcmprst(&self) -> ScfgcmprstR {
        ScfgcmprstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    pub fn exintrst(&self) -> ExintrstR {
        ExintrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer15 reset"]
    #[inline(always)]
    pub fn tmr15rst(&self) -> Tmr15rstR {
        Tmr15rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer16 reset"]
    #[inline(always)]
    pub fn tmr16rst(&self) -> Tmr16rstR {
        Tmr16rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer17 reset"]
    #[inline(always)]
    pub fn tmr17rst(&self) -> Tmr17rstR {
        Tmr17rstR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System config and comparator reset"]
    #[inline(always)]
    pub fn scfgcmprst(&mut self) -> ScfgcmprstW<'_, Apb2rstSpec> {
        ScfgcmprstW::new(self, 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    pub fn exintrst(&mut self) -> ExintrstW<'_, Apb2rstSpec> {
        ExintrstW::new(self, 1)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> AdcrstW<'_, Apb2rstSpec> {
        AdcrstW::new(self, 9)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> Spi1rstW<'_, Apb2rstSpec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> Usart1rstW<'_, Apb2rstSpec> {
        Usart1rstW::new(self, 14)
    }
    #[doc = "Bit 16 - Timer15 reset"]
    #[inline(always)]
    pub fn tmr15rst(&mut self) -> Tmr15rstW<'_, Apb2rstSpec> {
        Tmr15rstW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer16 reset"]
    #[inline(always)]
    pub fn tmr16rst(&mut self) -> Tmr16rstW<'_, Apb2rstSpec> {
        Tmr16rstW::new(self, 17)
    }
    #[doc = "Bit 18 - Timer17 reset"]
    #[inline(always)]
    pub fn tmr17rst(&mut self) -> Tmr17rstW<'_, Apb2rstSpec> {
        Tmr17rstW::new(self, 18)
    }
}
#[doc = "APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstSpec;
impl crate::RegisterSpec for Apb2rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for Apb2rstSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for Apb2rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for Apb2rstSpec {}

#[doc = "Register `I2SCLK` reader"]
pub type R = crate::R<I2sclkSpec>;
#[doc = "Register `I2SCLK` writer"]
pub type W = crate::W<I2sclkSpec>;
#[doc = "Field `I2SDIV7_0` reader - I2S division bit7 to bit0"]
pub type I2sdiv7_0R = crate::FieldReader;
#[doc = "Field `I2SDIV7_0` writer - I2S division bit7 to bit0"]
pub type I2sdiv7_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2SODD` reader - Odd result for I2S division"]
pub type I2soddR = crate::BitReader;
#[doc = "Field `I2SODD` writer - Odd result for I2S division"]
pub type I2soddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SMCLKOE` reader - I2S master clock output enable"]
pub type I2smclkoeR = crate::BitReader;
#[doc = "Field `I2SMCLKOE` writer - I2S master clock output enable"]
pub type I2smclkoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SDIV9_8` reader - I2S division bit9 and bit8"]
pub type I2sdiv9_8R = crate::FieldReader;
#[doc = "Field `I2SDIV9_8` writer - I2S division bit9 and bit8"]
pub type I2sdiv9_8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - I2S division bit7 to bit0"]
    #[inline(always)]
    pub fn i2sdiv7_0(&self) -> I2sdiv7_0R {
        I2sdiv7_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd result for I2S division"]
    #[inline(always)]
    pub fn i2sodd(&self) -> I2soddR {
        I2soddR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2S master clock output enable"]
    #[inline(always)]
    pub fn i2smclkoe(&self) -> I2smclkoeR {
        I2smclkoeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - I2S division bit9 and bit8"]
    #[inline(always)]
    pub fn i2sdiv9_8(&self) -> I2sdiv9_8R {
        I2sdiv9_8R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S division bit7 to bit0"]
    #[inline(always)]
    pub fn i2sdiv7_0(&mut self) -> I2sdiv7_0W<'_, I2sclkSpec> {
        I2sdiv7_0W::new(self, 0)
    }
    #[doc = "Bit 8 - Odd result for I2S division"]
    #[inline(always)]
    pub fn i2sodd(&mut self) -> I2soddW<'_, I2sclkSpec> {
        I2soddW::new(self, 8)
    }
    #[doc = "Bit 9 - I2S master clock output enable"]
    #[inline(always)]
    pub fn i2smclkoe(&mut self) -> I2smclkoeW<'_, I2sclkSpec> {
        I2smclkoeW::new(self, 9)
    }
    #[doc = "Bits 10:11 - I2S division bit9 and bit8"]
    #[inline(always)]
    pub fn i2sdiv9_8(&mut self) -> I2sdiv9_8W<'_, I2sclkSpec> {
        I2sdiv9_8W::new(self, 10)
    }
}
#[doc = "I2S clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sclkSpec;
impl crate::RegisterSpec for I2sclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sclk::R`](R) reader structure"]
impl crate::Readable for I2sclkSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sclk::W`](W) writer structure"]
impl crate::Writable for I2sclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SCLK to value 0x0a"]
impl crate::Resettable for I2sclkSpec {
    const RESET_VALUE: u32 = 0x0a;
}

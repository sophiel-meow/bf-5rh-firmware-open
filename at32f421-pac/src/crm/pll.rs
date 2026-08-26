#[doc = "Register `PLL` reader"]
pub type R = crate::R<PllSpec>;
#[doc = "Register `PLL` writer"]
pub type W = crate::W<PllSpec>;
#[doc = "Field `PLL_FR` reader - PLL_FR"]
pub type PllFrR = crate::FieldReader;
#[doc = "Field `PLL_FR` writer - PLL_FR"]
pub type PllFrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLL_MS` reader - PLL_MS"]
pub type PllMsR = crate::FieldReader;
#[doc = "Field `PLL_MS` writer - PLL_MS"]
pub type PllMsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLL_NS` reader - PLL_NS"]
pub type PllNsR = crate::FieldReader<u16>;
#[doc = "Field `PLL_NS` writer - PLL_NS"]
pub type PllNsW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLL_FREF` reader - PLL entry clock reference frequency"]
pub type PllFrefR = crate::FieldReader;
#[doc = "Field `PLL_FREF` writer - PLL entry clock reference frequency"]
pub type PllFrefW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLCFGEN` reader - PLL config enable"]
pub type PllcfgenR = crate::BitReader;
#[doc = "Field `PLLCFGEN` writer - PLL config enable"]
pub type PllcfgenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - PLL_FR"]
    #[inline(always)]
    pub fn pll_fr(&self) -> PllFrR {
        PllFrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - PLL_MS"]
    #[inline(always)]
    pub fn pll_ms(&self) -> PllMsR {
        PllMsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:16 - PLL_NS"]
    #[inline(always)]
    pub fn pll_ns(&self) -> PllNsR {
        PllNsR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:26 - PLL entry clock reference frequency"]
    #[inline(always)]
    pub fn pll_fref(&self) -> PllFrefR {
        PllFrefR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - PLL config enable"]
    #[inline(always)]
    pub fn pllcfgen(&self) -> PllcfgenR {
        PllcfgenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PLL_FR"]
    #[inline(always)]
    pub fn pll_fr(&mut self) -> PllFrW<'_, PllSpec> {
        PllFrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - PLL_MS"]
    #[inline(always)]
    pub fn pll_ms(&mut self) -> PllMsW<'_, PllSpec> {
        PllMsW::new(self, 4)
    }
    #[doc = "Bits 8:16 - PLL_NS"]
    #[inline(always)]
    pub fn pll_ns(&mut self) -> PllNsW<'_, PllSpec> {
        PllNsW::new(self, 8)
    }
    #[doc = "Bits 24:26 - PLL entry clock reference frequency"]
    #[inline(always)]
    pub fn pll_fref(&mut self) -> PllFrefW<'_, PllSpec> {
        PllFrefW::new(self, 24)
    }
    #[doc = "Bit 31 - PLL config enable"]
    #[inline(always)]
    pub fn pllcfgen(&mut self) -> PllcfgenW<'_, PllSpec> {
        PllcfgenW::new(self, 31)
    }
}
#[doc = "PLL configuration register (CRM_PLL)\n\nYou can [`read`](crate::Reg::read) this register and get [`pll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllSpec;
impl crate::RegisterSpec for PllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll::R`](R) reader structure"]
impl crate::Readable for PllSpec {}
#[doc = "`write(|w| ..)` method takes [`pll::W`](W) writer structure"]
impl crate::Writable for PllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL to value 0x1f10"]
impl crate::Resettable for PllSpec {
    const RESET_VALUE: u32 = 0x1f10;
}

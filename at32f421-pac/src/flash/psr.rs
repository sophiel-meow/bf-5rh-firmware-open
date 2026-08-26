#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PsrSpec>;
#[doc = "Field `WTCYC` reader - Wait cycle"]
pub type WtcycR = crate::FieldReader;
#[doc = "Field `WTCYC` writer - Wait cycle"]
pub type WtcycW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HFCYC_EN` reader - Half cycle acceleration access enable"]
pub type HfcycEnR = crate::BitReader;
#[doc = "Field `HFCYC_EN` writer - Half cycle acceleration access enable"]
pub type HfcycEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFT_EN` reader - Prefetch enable"]
pub type PftEnR = crate::BitReader;
#[doc = "Field `PFT_EN` writer - Prefetch enable"]
pub type PftEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFT_ENF` reader - Prefetch enabled flag"]
pub type PftEnfR = crate::BitReader;
#[doc = "Field `PFT_EN2` reader - Prefetch enable 2"]
pub type PftEn2R = crate::BitReader;
#[doc = "Field `PFT_EN2` writer - Prefetch enable 2"]
pub type PftEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFT_ENF2` reader - Prefetch enabled flag 2"]
pub type PftEnf2R = crate::BitReader;
#[doc = "Field `PFT_LAT_DIS` reader - Prefetch latency disable"]
pub type PftLatDisR = crate::BitReader;
#[doc = "Field `PFT_LAT_DIS` writer - Prefetch latency disable"]
pub type PftLatDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Wait cycle"]
    #[inline(always)]
    pub fn wtcyc(&self) -> WtcycR {
        WtcycR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Half cycle acceleration access enable"]
    #[inline(always)]
    pub fn hfcyc_en(&self) -> HfcycEnR {
        HfcycEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Prefetch enable"]
    #[inline(always)]
    pub fn pft_en(&self) -> PftEnR {
        PftEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Prefetch enabled flag"]
    #[inline(always)]
    pub fn pft_enf(&self) -> PftEnfR {
        PftEnfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Prefetch enable 2"]
    #[inline(always)]
    pub fn pft_en2(&self) -> PftEn2R {
        PftEn2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Prefetch enabled flag 2"]
    #[inline(always)]
    pub fn pft_enf2(&self) -> PftEnf2R {
        PftEnf2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Prefetch latency disable"]
    #[inline(always)]
    pub fn pft_lat_dis(&self) -> PftLatDisR {
        PftLatDisR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Wait cycle"]
    #[inline(always)]
    pub fn wtcyc(&mut self) -> WtcycW<'_, PsrSpec> {
        WtcycW::new(self, 0)
    }
    #[doc = "Bit 3 - Half cycle acceleration access enable"]
    #[inline(always)]
    pub fn hfcyc_en(&mut self) -> HfcycEnW<'_, PsrSpec> {
        HfcycEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Prefetch enable"]
    #[inline(always)]
    pub fn pft_en(&mut self) -> PftEnW<'_, PsrSpec> {
        PftEnW::new(self, 4)
    }
    #[doc = "Bit 6 - Prefetch enable 2"]
    #[inline(always)]
    pub fn pft_en2(&mut self) -> PftEn2W<'_, PsrSpec> {
        PftEn2W::new(self, 6)
    }
    #[doc = "Bit 8 - Prefetch latency disable"]
    #[inline(always)]
    pub fn pft_lat_dis(&mut self) -> PftLatDisW<'_, PsrSpec> {
        PftLatDisW::new(self, 8)
    }
}
#[doc = "Performance selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSR to value 0x30"]
impl crate::Resettable for PsrSpec {
    const RESET_VALUE: u32 = 0x30;
}

#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `HICKEN` reader - High speed internal clock enable"]
pub type HickenR = crate::BitReader;
#[doc = "Field `HICKEN` writer - High speed internal clock enable"]
pub type HickenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICKSTBL` reader - High speed internal clock ready flag"]
pub type HickstblR = crate::BitReader;
#[doc = "Field `HICKTRIM` reader - High speed internal clock trimming"]
pub type HicktrimR = crate::FieldReader;
#[doc = "Field `HICKTRIM` writer - High speed internal clock trimming"]
pub type HicktrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HICKCAL` reader - High speed internal clock calibration"]
pub type HickcalR = crate::FieldReader;
#[doc = "Field `HEXTEN` reader - High speed exernal crystal enable"]
pub type HextenR = crate::BitReader;
#[doc = "Field `HEXTEN` writer - High speed exernal crystal enable"]
pub type HextenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEXTSTBL` reader - High speed exernal crystal ready flag"]
pub type HextstblR = crate::BitReader;
#[doc = "Field `HEXTBYPS` reader - High speed exernal crystal bypass"]
pub type HextbypsR = crate::BitReader;
#[doc = "Field `HEXTBYPS` writer - High speed exernal crystal bypass"]
pub type HextbypsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEN` reader - Clock failure detection enable"]
pub type CfdenR = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock failure detection enable"]
pub type CfdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PllenR = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTBL` reader - PLL clock ready flag"]
pub type PllstblR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - High speed internal clock enable"]
    #[inline(always)]
    pub fn hicken(&self) -> HickenR {
        HickenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High speed internal clock ready flag"]
    #[inline(always)]
    pub fn hickstbl(&self) -> HickstblR {
        HickstblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hicktrim(&self) -> HicktrimR {
        HicktrimR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - High speed internal clock calibration"]
    #[inline(always)]
    pub fn hickcal(&self) -> HickcalR {
        HickcalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - High speed exernal crystal enable"]
    #[inline(always)]
    pub fn hexten(&self) -> HextenR {
        HextenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High speed exernal crystal ready flag"]
    #[inline(always)]
    pub fn hextstbl(&self) -> HextstblR {
        HextstblR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - High speed exernal crystal bypass"]
    #[inline(always)]
    pub fn hextbyps(&self) -> HextbypsR {
        HextbypsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock failure detection enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CfdenR {
        CfdenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PllenR {
        PllenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllstbl(&self) -> PllstblR {
        PllstblR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High speed internal clock enable"]
    #[inline(always)]
    pub fn hicken(&mut self) -> HickenW<'_, CtrlSpec> {
        HickenW::new(self, 0)
    }
    #[doc = "Bits 2:7 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hicktrim(&mut self) -> HicktrimW<'_, CtrlSpec> {
        HicktrimW::new(self, 2)
    }
    #[doc = "Bit 16 - High speed exernal crystal enable"]
    #[inline(always)]
    pub fn hexten(&mut self) -> HextenW<'_, CtrlSpec> {
        HextenW::new(self, 16)
    }
    #[doc = "Bit 18 - High speed exernal crystal bypass"]
    #[inline(always)]
    pub fn hextbyps(&mut self) -> HextbypsW<'_, CtrlSpec> {
        HextbypsW::new(self, 18)
    }
    #[doc = "Bit 19 - Clock failure detection enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CfdenW<'_, CtrlSpec> {
        CfdenW::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PllenW<'_, CtrlSpec> {
        PllenW::new(self, 24)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x83"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x83;
}

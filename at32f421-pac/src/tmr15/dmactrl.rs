#[doc = "Register `DMACTRL` reader"]
pub type R = crate::R<DmactrlSpec>;
#[doc = "Register `DMACTRL` writer"]
pub type W = crate::W<DmactrlSpec>;
#[doc = "Field `ADDR` reader - DMA transfer address offset"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - DMA transfer address offset"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DTB` reader - DMA transfer bytes"]
pub type DtbR = crate::FieldReader;
#[doc = "Field `DTB` writer - DMA transfer bytes"]
pub type DtbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA transfer address offset"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA transfer bytes"]
    #[inline(always)]
    pub fn dtb(&self) -> DtbR {
        DtbR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA transfer address offset"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, DmactrlSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA transfer bytes"]
    #[inline(always)]
    pub fn dtb(&mut self) -> DtbW<'_, DmactrlSpec> {
        DtbW::new(self, 8)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactrlSpec;
impl crate::RegisterSpec for DmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactrl::R`](R) reader structure"]
impl crate::Readable for DmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactrl::W`](W) writer structure"]
impl crate::Writable for DmactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTRL to value 0"]
impl crate::Resettable for DmactrlSpec {}

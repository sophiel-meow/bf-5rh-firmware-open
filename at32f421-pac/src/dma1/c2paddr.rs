#[doc = "Register `C2PADDR` reader"]
pub type R = crate::R<C2paddrSpec>;
#[doc = "Register `C2PADDR` writer"]
pub type W = crate::W<C2paddrSpec>;
#[doc = "Field `PADDR` reader - Peripheral address"]
pub type PaddrR = crate::FieldReader<u32>;
#[doc = "Field `PADDR` writer - Peripheral address"]
pub type PaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn paddr(&self) -> PaddrR {
        PaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn paddr(&mut self) -> PaddrW<'_, C2paddrSpec> {
        PaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 2 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2paddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2paddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2paddrSpec;
impl crate::RegisterSpec for C2paddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2paddr::R`](R) reader structure"]
impl crate::Readable for C2paddrSpec {}
#[doc = "`write(|w| ..)` method takes [`c2paddr::W`](W) writer structure"]
impl crate::Writable for C2paddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C2PADDR to value 0"]
impl crate::Resettable for C2paddrSpec {}

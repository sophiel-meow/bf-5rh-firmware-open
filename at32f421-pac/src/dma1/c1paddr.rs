#[doc = "Register `C1PADDR` reader"]
pub type R = crate::R<C1paddrSpec>;
#[doc = "Register `C1PADDR` writer"]
pub type W = crate::W<C1paddrSpec>;
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
    pub fn paddr(&mut self) -> PaddrW<'_, C1paddrSpec> {
        PaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 1 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1paddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1paddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1paddrSpec;
impl crate::RegisterSpec for C1paddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1paddr::R`](R) reader structure"]
impl crate::Readable for C1paddrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1paddr::W`](W) writer structure"]
impl crate::Writable for C1paddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1PADDR to value 0"]
impl crate::Resettable for C1paddrSpec {}

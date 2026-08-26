#[doc = "Register `C5PADDR` reader"]
pub type R = crate::R<C5paddrSpec>;
#[doc = "Register `C5PADDR` writer"]
pub type W = crate::W<C5paddrSpec>;
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
    pub fn paddr(&mut self) -> PaddrW<'_, C5paddrSpec> {
        PaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 5 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`c5paddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5paddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C5paddrSpec;
impl crate::RegisterSpec for C5paddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c5paddr::R`](R) reader structure"]
impl crate::Readable for C5paddrSpec {}
#[doc = "`write(|w| ..)` method takes [`c5paddr::W`](W) writer structure"]
impl crate::Writable for C5paddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C5PADDR to value 0"]
impl crate::Resettable for C5paddrSpec {}

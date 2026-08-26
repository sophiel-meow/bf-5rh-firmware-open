#[doc = "Register `C1MADDR` reader"]
pub type R = crate::R<C1maddrSpec>;
#[doc = "Register `C1MADDR` writer"]
pub type W = crate::W<C1maddrSpec>;
#[doc = "Field `MADDR` reader - Memory address"]
pub type MaddrR = crate::FieldReader<u32>;
#[doc = "Field `MADDR` writer - Memory address"]
pub type MaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn maddr(&self) -> MaddrR {
        MaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn maddr(&mut self) -> MaddrW<'_, C1maddrSpec> {
        MaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 1 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1maddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1maddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1maddrSpec;
impl crate::RegisterSpec for C1maddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1maddr::R`](R) reader structure"]
impl crate::Readable for C1maddrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1maddr::W`](W) writer structure"]
impl crate::Writable for C1maddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1MADDR to value 0"]
impl crate::Resettable for C1maddrSpec {}

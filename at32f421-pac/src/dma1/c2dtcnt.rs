#[doc = "Register `C2DTCNT` reader"]
pub type R = crate::R<C2dtcntSpec>;
#[doc = "Register `C2DTCNT` writer"]
pub type W = crate::W<C2dtcntSpec>;
#[doc = "Field `CNT` reader - Number of data to transfer"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Number of data to transfer"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, C2dtcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "DMA channel 2 number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2dtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2dtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2dtcntSpec;
impl crate::RegisterSpec for C2dtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2dtcnt::R`](R) reader structure"]
impl crate::Readable for C2dtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`c2dtcnt::W`](W) writer structure"]
impl crate::Writable for C2dtcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C2DTCNT to value 0"]
impl crate::Resettable for C2dtcntSpec {}

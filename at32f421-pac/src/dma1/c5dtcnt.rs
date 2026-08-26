#[doc = "Register `C5DTCNT` reader"]
pub type R = crate::R<C5dtcntSpec>;
#[doc = "Register `C5DTCNT` writer"]
pub type W = crate::W<C5dtcntSpec>;
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
    pub fn cnt(&mut self) -> CntW<'_, C5dtcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "DMA channel 5 number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`c5dtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5dtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C5dtcntSpec;
impl crate::RegisterSpec for C5dtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c5dtcnt::R`](R) reader structure"]
impl crate::Readable for C5dtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`c5dtcnt::W`](W) writer structure"]
impl crate::Writable for C5dtcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C5DTCNT to value 0"]
impl crate::Resettable for C5dtcntSpec {}

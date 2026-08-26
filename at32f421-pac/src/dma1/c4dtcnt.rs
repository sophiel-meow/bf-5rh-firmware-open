#[doc = "Register `C4DTCNT` reader"]
pub type R = crate::R<C4dtcntSpec>;
#[doc = "Register `C4DTCNT` writer"]
pub type W = crate::W<C4dtcntSpec>;
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
    pub fn cnt(&mut self) -> CntW<'_, C4dtcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "DMA channel 4 number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`c4dtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4dtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C4dtcntSpec;
impl crate::RegisterSpec for C4dtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c4dtcnt::R`](R) reader structure"]
impl crate::Readable for C4dtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`c4dtcnt::W`](W) writer structure"]
impl crate::Writable for C4dtcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C4DTCNT to value 0"]
impl crate::Resettable for C4dtcntSpec {}

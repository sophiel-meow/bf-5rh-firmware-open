#[doc = "Register `C3DTCNT` reader"]
pub type R = crate::R<C3dtcntSpec>;
#[doc = "Register `C3DTCNT` writer"]
pub type W = crate::W<C3dtcntSpec>;
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
    pub fn cnt(&mut self) -> CntW<'_, C3dtcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "DMA channel 3 number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3dtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3dtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3dtcntSpec;
impl crate::RegisterSpec for C3dtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3dtcnt::R`](R) reader structure"]
impl crate::Readable for C3dtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`c3dtcnt::W`](W) writer structure"]
impl crate::Writable for C3dtcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C3DTCNT to value 0"]
impl crate::Resettable for C3dtcntSpec {}

#[doc = "Register `DT` reader"]
pub type R = crate::R<DtSpec>;
#[doc = "Register `DT` writer"]
pub type W = crate::W<DtSpec>;
#[doc = "Field `DT` reader - data register"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - data register"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data register"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data register"]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<'_, DtSpec> {
        DtW::new(self, 0)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtSpec;
impl crate::RegisterSpec for DtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt::R`](R) reader structure"]
impl crate::Readable for DtSpec {}
#[doc = "`write(|w| ..)` method takes [`dt::W`](W) writer structure"]
impl crate::Writable for DtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT to value 0"]
impl crate::Resettable for DtSpec {}

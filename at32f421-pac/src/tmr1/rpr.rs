#[doc = "Register `RPR` reader"]
pub type R = crate::R<RprSpec>;
#[doc = "Register `RPR` writer"]
pub type W = crate::W<RprSpec>;
#[doc = "Field `RPR` reader - Repetition of period value"]
pub type RprR = crate::FieldReader;
#[doc = "Field `RPR` writer - Repetition of period value"]
pub type RprW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repetition of period value"]
    #[inline(always)]
    pub fn rpr(&self) -> RprR {
        RprR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition of period value"]
    #[inline(always)]
    pub fn rpr(&mut self) -> RprW<'_, RprSpec> {
        RprW::new(self, 0)
    }
}
#[doc = "Repetition of period value\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RprSpec;
impl crate::RegisterSpec for RprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr::R`](R) reader structure"]
impl crate::Readable for RprSpec {}
#[doc = "`write(|w| ..)` method takes [`rpr::W`](W) writer structure"]
impl crate::Writable for RprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RPR to value 0"]
impl crate::Resettable for RprSpec {}

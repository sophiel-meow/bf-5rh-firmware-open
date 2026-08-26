#[doc = "Register `C4DT` reader"]
pub type R = crate::R<C4dtSpec>;
#[doc = "Register `C4DT` writer"]
pub type W = crate::W<C4dtSpec>;
#[doc = "Field `C4DT` reader - Channel 4 data register"]
pub type C4dtR = crate::FieldReader<u16>;
#[doc = "Field `C4DT` writer - Channel 4 data register"]
pub type C4dtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel 4 data register"]
    #[inline(always)]
    pub fn c4dt(&self) -> C4dtR {
        C4dtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel 4 data register"]
    #[inline(always)]
    pub fn c4dt(&mut self) -> C4dtW<'_, C4dtSpec> {
        C4dtW::new(self, 0)
    }
}
#[doc = "Channel 4 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`c4dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C4dtSpec;
impl crate::RegisterSpec for C4dtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c4dt::R`](R) reader structure"]
impl crate::Readable for C4dtSpec {}
#[doc = "`write(|w| ..)` method takes [`c4dt::W`](W) writer structure"]
impl crate::Writable for C4dtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C4DT to value 0"]
impl crate::Resettable for C4dtSpec {}

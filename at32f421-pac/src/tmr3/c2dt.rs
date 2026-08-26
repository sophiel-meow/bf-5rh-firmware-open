#[doc = "Register `C2DT` reader"]
pub type R = crate::R<C2dtSpec>;
#[doc = "Register `C2DT` writer"]
pub type W = crate::W<C2dtSpec>;
#[doc = "Field `C2DT` reader - Channel 2 data register"]
pub type C2dtR = crate::FieldReader<u16>;
#[doc = "Field `C2DT` writer - Channel 2 data register"]
pub type C2dtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel 2 data register"]
    #[inline(always)]
    pub fn c2dt(&self) -> C2dtR {
        C2dtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel 2 data register"]
    #[inline(always)]
    pub fn c2dt(&mut self) -> C2dtW<'_, C2dtSpec> {
        C2dtW::new(self, 0)
    }
}
#[doc = "Channel 2 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2dtSpec;
impl crate::RegisterSpec for C2dtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2dt::R`](R) reader structure"]
impl crate::Readable for C2dtSpec {}
#[doc = "`write(|w| ..)` method takes [`c2dt::W`](W) writer structure"]
impl crate::Writable for C2dtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C2DT to value 0"]
impl crate::Resettable for C2dtSpec {}

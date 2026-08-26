#[doc = "Register `PCDTO2` reader"]
pub type R = crate::R<Pcdto2Spec>;
#[doc = "Register `PCDTO2` writer"]
pub type W = crate::W<Pcdto2Spec>;
#[doc = "Field `PCDTO2` reader - Data offset for Preempted channel 2"]
pub type Pcdto2R = crate::FieldReader<u16>;
#[doc = "Field `PCDTO2` writer - Data offset for Preempted channel 2"]
pub type Pcdto2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 2"]
    #[inline(always)]
    pub fn pcdto2(&self) -> Pcdto2R {
        Pcdto2R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 2"]
    #[inline(always)]
    pub fn pcdto2(&mut self) -> Pcdto2W<'_, Pcdto2Spec> {
        Pcdto2W::new(self, 0)
    }
}
#[doc = "Preempted channel 2 data offset register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcdto2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcdto2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcdto2Spec;
impl crate::RegisterSpec for Pcdto2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto2::R`](R) reader structure"]
impl crate::Readable for Pcdto2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcdto2::W`](W) writer structure"]
impl crate::Writable for Pcdto2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCDTO2 to value 0"]
impl crate::Resettable for Pcdto2Spec {}

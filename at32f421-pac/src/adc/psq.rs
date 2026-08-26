#[doc = "Register `PSQ` reader"]
pub type R = crate::R<PsqSpec>;
#[doc = "Register `PSQ` writer"]
pub type W = crate::W<PsqSpec>;
#[doc = "Field `PSN1` reader - Number of 1st conversion in Preempted sequence"]
pub type Psn1R = crate::FieldReader;
#[doc = "Field `PSN1` writer - Number of 1st conversion in Preempted sequence"]
pub type Psn1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PSN2` reader - Number of 2nd conversion in Preempted sequence"]
pub type Psn2R = crate::FieldReader;
#[doc = "Field `PSN2` writer - Number of 2nd conversion in Preempted sequence"]
pub type Psn2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PSN3` reader - Number of 3rd conversion in Preempted sequence"]
pub type Psn3R = crate::FieldReader;
#[doc = "Field `PSN3` writer - Number of 3rd conversion in Preempted sequence"]
pub type Psn3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PSN4` reader - Number of 4th conversion in Preempted sequence"]
pub type Psn4R = crate::FieldReader;
#[doc = "Field `PSN4` writer - Number of 4th conversion in Preempted sequence"]
pub type Psn4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PCLEN` reader - Preempted conversion sequence length"]
pub type PclenR = crate::FieldReader;
#[doc = "Field `PCLEN` writer - Preempted conversion sequence length"]
pub type PclenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - Number of 1st conversion in Preempted sequence"]
    #[inline(always)]
    pub fn psn1(&self) -> Psn1R {
        Psn1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 2nd conversion in Preempted sequence"]
    #[inline(always)]
    pub fn psn2(&self) -> Psn2R {
        Psn2R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 3rd conversion in Preempted sequence"]
    #[inline(always)]
    pub fn psn3(&self) -> Psn3R {
        Psn3R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 4th conversion in Preempted sequence"]
    #[inline(always)]
    pub fn psn4(&self) -> Psn4R {
        Psn4R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - Preempted conversion sequence length"]
    #[inline(always)]
    pub fn pclen(&self) -> PclenR {
        PclenR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 1st conversion in Preempted sequence"]
    #[inline(always)]
    pub fn psn1(&mut self) -> Psn1W<'_, PsqSpec> {
        Psn1W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 2nd conversion in Preempted sequence"]
    #[inline(always)]
    pub fn psn2(&mut self) -> Psn2W<'_, PsqSpec> {
        Psn2W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 3rd conversion in Preempted sequence"]
    #[inline(always)]
    pub fn psn3(&mut self) -> Psn3W<'_, PsqSpec> {
        Psn3W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 4th conversion in Preempted sequence"]
    #[inline(always)]
    pub fn psn4(&mut self) -> Psn4W<'_, PsqSpec> {
        Psn4W::new(self, 15)
    }
    #[doc = "Bits 20:21 - Preempted conversion sequence length"]
    #[inline(always)]
    pub fn pclen(&mut self) -> PclenW<'_, PsqSpec> {
        PclenW::new(self, 20)
    }
}
#[doc = "Preempted sequence register\n\nYou can [`read`](crate::Reg::read) this register and get [`psq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsqSpec;
impl crate::RegisterSpec for PsqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psq::R`](R) reader structure"]
impl crate::Readable for PsqSpec {}
#[doc = "`write(|w| ..)` method takes [`psq::W`](W) writer structure"]
impl crate::Writable for PsqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSQ to value 0"]
impl crate::Resettable for PsqSpec {}
